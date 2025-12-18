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
use regex::Regex;
use std::collections::HashSet;

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
    // POST-INFERENCE VALIDATION
    // ----------------------------
    let cited = extract_citations(&response);

    if cited.is_empty() {
        return Err("model produced no material citations".into());
    }

    let known: HashSet<&str> =
        atoms.iter().map(|a| a.message_id.as_str()).collect();

    for cid in &cited {
        if !known.contains(cid.as_str()) {
            return Err(format!("invalid citation: {}", cid).into());
        }
    }

    // ----------------------------
    // MATERIAL USAGE LEDGER
    // ----------------------------
    let used: Vec<UsedAtom> = atoms
        .iter()
        .filter(|a| cited.contains(&a.message_id))
        .map(|a| UsedAtom {
            convo_id: &a.convo_id,
            message_id: &a.message_id,
            sha256: &a.sha256,
            order: a.order,
        })
        .collect();

    write_file(
        &trace_dir,
        "usage.yaml",
        &serde_yaml::to_string(&used)?,
    )?;

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

/* ================================
   Citation extraction & validation
   ================================ */

fn extract_citations(response: &str) -> Vec<String> {
    let re = Regex::new(r"\[FUR:([a-f0-9\-]{8,})\]").unwrap();

    let mut ids = Vec::new();
    for cap in re.captures_iter(response) {
        ids.push(cap[1].to_string());
    }

    ids.sort();
    ids.dedup();
    ids
}

#[derive(serde::Serialize)]
struct UsedAtom<'a> {
    convo_id: &'a str,
    message_id: &'a str,
    sha256: &'a str,
    order: usize,
}
