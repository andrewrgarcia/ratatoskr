use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EngineSpec {
    pub engine_type: String,
    pub name: String,
    pub model: String,
    pub avatar: Option<String>, 
}


/* ================================
   Engine (minimal)
   ================================ */

pub trait Engine {
    fn run(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn describe(&self) -> String;
}

pub struct StubEngine;

impl Engine for StubEngine {
    fn run(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let line = prompt
            .lines()
            .find(|l| l.starts_with("=== ATOM FUR:"))
            .ok_or("no atoms found in prompt")?;

        // === ATOM FUR:<message_id>:<sha> ===
        let message_id = line
            .split(':')
            .nth(1)                // ✅ MESSAGE ID
            .ok_or("malformed atom header")?
            .trim();

        Ok(format!(
            "Grounded answer using cited material [FUR:{}].",
            message_id
        ))
    }

    fn describe(&self) -> String {
        "engine: stub\nmodel: none\npurpose: citation-enforcement test\n".to_string()
    }
}
