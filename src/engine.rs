use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EngineSpec {
    #[serde(rename = "type")]
    pub engine_type: String,
    pub name: String,
    pub model: String,
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
        // Extract first atom id from the prompt and cite it
        let citation = prompt
            .lines()
            .find(|l| l.starts_with("=== ATOM FUR:"))
            .and_then(|l| l.split(':').nth(2))
            .map(|s| s.trim())
            .ok_or("no atoms found in prompt")?;

        Ok(format!(
            "The following atom was used to produce this response: [{}].",
            citation
        ))
    }

    fn describe(&self) -> String {
        "engine: stub\nmodel: none\npurpose: lifecycle validation\n".to_string()
    }
}
