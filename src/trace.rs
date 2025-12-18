use std::fs;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::task::{Task, MemoryRef};

pub const RATATOSKR_VERSION: &str = "0.2.0-dev";

/* ================================
   Trace state
   ================================ */

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceStatus {
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

/* ================================
   Trace initialization & lifecycle
   ================================ */

pub fn initialize_trace_layout(
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

pub fn update_trace_status(
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
   Context resolution & provenance
   ================================ */

#[derive(Debug, Serialize)]
struct SelectionEntry {
    source: String,
    chunk: String,
    rationale: String,
}

pub fn resolve_context(
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
        fs::copy(src_path, docs_dir.join(filename))?;

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

            fs::write(chunks_dir.join(&chunk_name), &extracted)?;
            used_chunks.push(extracted.clone());

            selection.push(SelectionEntry {
                source: ctx.path.clone(),
                chunk: chunk_name,
                rationale: "explicit line range".to_string(),
            });
        }

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

            fs::write(chunks_dir.join(&chunk_name), &extracted)?;
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

fn write_selection(
    trace_dir: &PathBuf,
    entries: &Vec<SelectionEntry>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = trace_dir
        .join("resolved_context")
        .join("selection.yaml");

    fs::write(path, serde_yaml::to_string(entries)?)?;
    Ok(())
}

pub fn write_memory_refs(
    trace_dir: &PathBuf,
    refs: &Vec<MemoryRef>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = trace_dir
        .join("resolved_context")
        .join("memory_refs.yaml");

    fs::write(path, serde_yaml::to_string(refs)?)?;
    Ok(())
}

/* ================================
   Utilities
   ================================ */

pub fn ensure_dir(path: &Path) -> Result<(), io::Error> {
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub fn generate_trace_id() -> Result<String, io::Error> {
    Ok(now_unix()?.to_string())
}

fn now_unix() -> Result<u64, io::Error> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "system time error"))?
        .as_secs())
}

pub fn write_file(dir: &PathBuf, name: &str, contents: &str) -> Result<(), io::Error> {
    fs::write(dir.join(name), contents)
}
