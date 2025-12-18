mod engine;
mod task;
mod trace;
mod fur_atom;

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
    TraceStatus,
};
use crate::fur_atom::{load_fur_thread, FurAtom};

fn main() {
    if let Err(e) = run() {
        eprintln!("RATATOSKR ERROR: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // ----------------------------
    // LOAD + FREEZE INTENT
    // ----------------------------
    let task = load_task()?; // immutable intent

    let trace_root = Path::new("trace");
    ensure_dir(trace_root)?;

    let trace_id = generate_trace_id()?;
    let trace_dir = trace_root.join(&trace_id);

    if trace_dir.exists() {
        return Err(format!("trace already exists: {}", trace_id).into());
    }

    ensure_dir(&trace_dir)?;
    initialize_trace_layout(&trace_dir, &trace_id)?;

    write_file(
        &trace_dir,
        "input.yaml",
        &serde_yaml::to_string(&task)?,
    )?;

    // ----------------------------
    // RESOLVE FUR → ATOMS
    // ----------------------------
    let mut atoms: Vec<FurAtom> = Vec::new();

    for mem in &task.memory_refs {
        if mem.system != "fur" {
            return Err("unsupported memory system".into());
        }

        let fur_dir = Path::new(".fur");
        let mut convo_atoms = load_fur_thread(fur_dir, &mem.convo_id)?;
        atoms.append(&mut convo_atoms);
    }

    // Persist full material ledger (THIS IS THE POINT)
    write_file(
        &trace_dir,
        "materials.yaml",
        &serde_yaml::to_string(&atoms)?,
    )?;

    // ----------------------------
    // PROMPT ASSEMBLY (DETERMINISTIC)
    // ----------------------------
    let mut prompt = String::new();

    prompt.push_str(
        "SYSTEM:\n\
         You may ONLY use the following material atoms.\n\
         Cite atoms as [FUR:<message_id>].\n\n",
    );

    for atom in &atoms {
        prompt.push_str(&format!(
            "=== ATOM FUR:{}:{} ===\n{}\n\n",
            atom.message_id,
            atom.sha256,
            atom.content
        ));
    }

    prompt.push_str("\n=== TASK ===\n");
    prompt.push_str(&assemble_prompt(&task));

    // Optional filesystem context (non-FUR)
    let context_chunks = resolve_context(&task, &trace_dir)?;
    for chunk in context_chunks {
        prompt.push_str("\n\n--- CONTEXT ---\n");
        prompt.push_str(&chunk);
    }

    write_file(&trace_dir, "prompt.txt", &prompt)?;

    // ----------------------------
    // ENGINE EXECUTION
    // ----------------------------
    let engine = StubEngine {};
    let response = engine.run(&prompt)?;

    write_file(&trace_dir, "response.txt", &response)?;
    write_file(&trace_dir, "engine.yaml", &engine.describe())?;

    // ----------------------------
    // FINALIZE TRACE
    // ----------------------------
    update_trace_status(&trace_dir, TraceStatus::Executed)?;
    println!("Trace executed: {}", trace_id);

    Ok(())
}

/* ================================
   Prompt assembly (task-only)
   ================================ */

fn assemble_prompt(task: &Task) -> String {
    format!(
        "TASK TYPE: {}\nMEMORY SCOPE: {}\n\n{}",
        task.task_type,
        task.memory_scope,
        task.prompt
    )
}
