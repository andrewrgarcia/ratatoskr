mod engine;
mod task;
mod trace;

use std::path::Path;

use crate::engine::{Engine, StubEngine};
use crate::task::{Task, load_task};
use crate::trace::{
    ensure_dir,
    generate_trace_id,
    initialize_trace_layout,
    resolve_context,
    update_trace_status,
    write_file,
    write_memory_refs,
    TraceStatus,
};

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

    write_memory_refs(&trace_dir, &task.memory_refs)?;

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
   Prompt assembly (deterministic)
   ================================ */

fn assemble_prompt(task: &Task) -> String {
    format!(
        "TASK TYPE: {}\nMEMORY SCOPE: {}\n\n{}",
        task.task_type, task.memory_scope, task.prompt
    )
}
