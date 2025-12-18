use std::fs;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
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
    let mut prompt = assemble_prompt(&task);

    let context_chunks = resolve_context(&task, &trace_dir)?;
    for chunk in context_chunks {
        prompt.push_str("\n\n--- CONTEXT ---\n");
        prompt.push_str(&chunk);
    }

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
struct EngineSpec {
    #[serde(rename = "type")]
    engine_type: String,
    name: String,
    model: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ContextRef {
    path: String,

    // Exactly one of these may be present
    lines: Option<(usize, usize)>,
    section: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    task_type: String,
    prompt: String,
    memory_scope: String,

    #[serde(default)]
    context: Vec<ContextRef>,

    engine: EngineSpec,
}


fn load_task() -> Result<Task, Box<dyn std::error::Error>> {
    let path = Path::new(TASK_FILE);

    if !path.exists() {
        return Err(format!("missing {}", TASK_FILE).into());
    }

    let contents = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&contents)?)
}


#[derive(Debug, Serialize)]
struct SelectionEntry {
    source: String,
    chunk: String,
    rationale: String,
}

fn write_selection(
    trace_dir: &PathBuf,
    entries: &Vec<SelectionEntry>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = trace_dir
        .join("resolved_context")
        .join("selection.yaml");

    let yaml = serde_yaml::to_string(entries)?;
    fs::write(path, yaml)?;
    Ok(())
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

fn resolve_context(
    task: &Task,
    trace_dir: &PathBuf,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ctx_root = trace_dir.join("resolved_context");
    let docs_dir = ctx_root.join("documents");
    let chunks_dir = ctx_root.join("chunks");

    ensure_dir(&docs_dir)?;
    ensure_dir(&chunks_dir)?;

    let mut used_chunks = Vec::new();
    let mut selection = Vec::new();

    for ctx in &task.context {
        let src_path = Path::new(&ctx.path);

        if !src_path.exists() {
            return Err(format!("missing context file: {}", ctx.path).into());
        }

        let filename = src_path.file_name().unwrap();
        let dst_doc = docs_dir.join(filename);
        fs::copy(src_path, &dst_doc)?;

        // ---- LINE-BASED EXTRACTION ----
        if let Some((start, end)) = ctx.lines {
            let file = File::open(src_path)?;
            let reader = BufReader::new(file);

            let mut extracted = String::new();
            for (idx, line) in reader.lines().enumerate() {
                let line_no = idx + 1;
                if line_no >= start && line_no <= end {
                    extracted.push_str(&line?);
                    extracted.push('\n');
                }
            }

            let chunk_name = format!(
                "{}__lines_{}_{}.txt",
                filename.to_string_lossy(),
                start,
                end
            );

            let chunk_path = chunks_dir.join(&chunk_name);
            fs::write(&chunk_path, &extracted)?;

            used_chunks.push(extracted.clone());

            selection.push(SelectionEntry {
                source: ctx.path.clone(),
                chunk: chunk_name,
                rationale: "explicit line range".to_string(),
            });
        }

        // ---- SECTION-BASED EXTRACTION (simple marker) ----
        if let Some(section) = &ctx.section {
            let contents = fs::read_to_string(src_path)?;
            let mut capture = false;
            let mut extracted = String::new();

            for line in contents.lines() {
                if line.contains(section) {
                    capture = true;
                }
                if capture {
                    extracted.push_str(line);
                    extracted.push('\n');
                }
            }

            let chunk_name = format!(
                "{}__section_{}.txt",
                filename.to_string_lossy(),
                section.replace(' ', "_")
            );

            let chunk_path = chunks_dir.join(&chunk_name);
            fs::write(&chunk_path, &extracted)?;

            used_chunks.push(extracted.clone());

            selection.push(SelectionEntry {
                source: ctx.path.clone(),
                chunk: chunk_name,
                rationale: format!("explicit section: {}", section),
            });
        }
    }

    write_selection(trace_dir, &selection)?;
    Ok(used_chunks)
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
