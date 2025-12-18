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
        Ok(format!(
            "STUB ENGINE RESPONSE\n--------------------\n{}",
            prompt
        ))
    }

    fn describe(&self) -> String {
        "engine: stub\nmodel: none\npurpose: lifecycle validation\n".to_string()
    }
}
