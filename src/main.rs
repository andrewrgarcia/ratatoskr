use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const RATATOSKR_VERSION: &str = "0.1.0-dev";
const TASK_FILE: &str = "task.yaml";

fn main() {
    if let Err(e) = run() {
        eprintln!("RATATOSKR ERROR: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let task = load_task()?;

    let trace_root = Path::new("trace");
    ensure_dir(trace_root)?;

    let trace_id = generate_trace_id()?;
    let trace_dir = trace_root.join(&trace_id);

    if trace_dir.exists() {
        return Err(format!("trace already exists: {}", trace_id).into());
    }

    ensure_dir(&trace_dir)?;
    initialize_trace_layout(&trace_dir, &trace_id)?;

    // Freeze intent
    let task_yaml = serde_yaml::to_string(&task)?;
    write_file(&trace_dir, "input.yaml", &task_yaml)?;

    // ---- PROMPT ASSEMBLY ----
    let prompt = assemble_prompt(&task);
    write_file(&trace_dir, "prompt.txt", &prompt)?;

    // ---- ENGINE INVOCATION ----
    let engine = StubEngine {};
    let response = engine.run(&prompt)?;

    write_file(&trace_dir, "response.txt", &response)?;
    write_file(&trace_dir, "engine.yaml", &engine.describe())?;

    // ---- FINALIZE TRACE ----
    update_trace_status(&trace_dir, TraceStatus::Executed)?;

    println!("Trace executed: {}", trace_id);
    Ok(())
}

/* ================================
   Task
   ================================ */

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    task_type: String,
    prompt: String,
    memory_scope: String,
    engine: EngineSpec,
}

#[derive(Debug, Deserialize, Serialize)]
struct EngineSpec {
    #[serde(rename = "type")]
    engine_type: String,
    name: String,
    model: String,
}

fn load_task() -> Result<Task, Box<dyn std::error::Error>> {
    let path = Path::new(TASK_FILE);

    if !path.exists() {
        return Err(format!("missing {}", TASK_FILE).into());
    }

    let contents = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&contents)?)
}

/* ================================
   Prompt assembly (deterministic)
   ================================ */

fn assemble_prompt(task: &Task) -> String {
    format!(
        "TASK TYPE: {}\nMEMORY SCOPE: {}\n\n{}",
        task.task_type, task.memory_scope, task.prompt
    )
}

/* ================================
   Engine (minimal)
   ================================ */

trait Engine {
    fn run(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn describe(&self) -> String;
}

struct StubEngine;

impl Engine for StubEngine {
    fn run(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "STUB ENGINE RESPONSE\n--------------------\n{}",
            prompt
        ))
    }

    fn describe(&self) -> String {
        "engine: stub\nmodel: none\npurpose: lifecycle validation\n".to_string()
    }
}

/* ================================
   Trace state machine
   ================================ */

#[derive(Debug, Serialize, Deserialize)]
enum TraceStatus {
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "executed")]
    Executed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
struct TraceMetadata {
    trace_id: String,
    timestamp_unix: u64,
    ratatoskr_version: String,
    status: TraceStatus,
}

fn update_trace_status(
    trace_dir: &PathBuf,
    new_status: TraceStatus,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = trace_dir.join("metadata.yaml");
    let contents = fs::read_to_string(&path)?;
    let mut meta: TraceMetadata = serde_yaml::from_str(&contents)?;

    validate_transition(&meta.status, &new_status)?;
    meta.status = new_status;

    fs::write(path, serde_yaml::to_string(&meta)?)?;
    Ok(())
}

fn validate_transition(
    from: &TraceStatus,
    to: &TraceStatus,
) -> Result<(), Box<dyn std::error::Error>> {
    match (from, to) {
        (TraceStatus::Initialized, TraceStatus::Executed) => Ok(()),
        (TraceStatus::Initialized, TraceStatus::Failed) => Ok(()),
        _ => Err(format!("illegal state transition {:?} → {:?}", from, to).into()),
    }
}

/* ================================
   Trace initialization
   ================================ */

fn initialize_trace_layout(
    trace_dir: &PathBuf,
    trace_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_dir(&trace_dir.join("resolved_context"))?;

    write_file(trace_dir, "prompt.txt", "# placeholder\n")?;
    write_file(trace_dir, "response.txt", "# placeholder\n")?;
    write_file(trace_dir, "memory_delta.yaml", "# placeholder\n")?;
    write_file(trace_dir, "engine.yaml", "# placeholder\n")?;

    let meta = TraceMetadata {
        trace_id: trace_id.to_string(),
        timestamp_unix: now_unix()?,
        ratatoskr_version: RATATOSKR_VERSION.to_string(),
        status: TraceStatus::Initialized,
    };

    write_file(trace_dir, "metadata.yaml", &serde_yaml::to_string(&meta)?)?;
    Ok(())
}

/* ================================
   Utilities
   ================================ */

fn ensure_dir(path: &Path) -> Result<(), io::Error> {
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

fn generate_trace_id() -> Result<String, io::Error> {
    Ok(now_unix()?.to_string())
}

fn now_unix() -> Result<u64, io::Error> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "system time error"))?
        .as_secs())
}

fn write_file(dir: &PathBuf, name: &str, contents: &str) -> Result<(), io::Error> {
    fs::write(dir.join(name), contents)
}
