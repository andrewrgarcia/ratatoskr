use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::engine::EngineSpec;

/* ================================
   Task schema
   ================================ */

pub const TASK_FILE: &str = "task.yaml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub task_type: String,
    pub prompt: String,
    pub memory_scope: String,

    #[serde(default)]
    pub context: Vec<ContextRef>,

    pub engine: EngineSpec,

    #[serde(default)]
    pub memory_refs: Vec<MemoryRef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextRef {
    pub path: String,

    // Exactly one of these may be present
    pub lines: Option<(usize, usize)>,
    pub section: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemoryRef {
    pub system: String,
    pub id: String,
}

/* ================================
   Loading
   ================================ */

pub fn load_task() -> Result<Task, Box<dyn std::error::Error>> {
    let path = Path::new(TASK_FILE);

    if !path.exists() {
        return Err(format!("missing {}", TASK_FILE).into());
    }

    let contents = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&contents)?)
}
