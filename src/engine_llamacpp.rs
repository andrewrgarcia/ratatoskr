use std::process::{Command, Stdio};
use std::error::Error;

use crate::engine::Engine;

pub struct LlamaCppEngine {
    pub binary: String,
    pub model: String,
    pub n_predict: usize,
}

impl Engine for LlamaCppEngine {
    fn run(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let output = Command::new(&self.binary)
            .arg("-m")
            .arg(&self.model)
            .arg("--prompt")           // ✅ or just -p
            .arg(prompt)
            .arg("-n")
            .arg(self.n_predict.to_string())
            .arg("--temp")
            .arg("0.7")
            .arg("-b")                 // ✅ batch mode
            .arg("512")
            .stdin(Stdio::null())      // ✅ close stdin
            .stdout(Stdio::piped())
            .stderr(Stdio::null())     // ✅ discard stderr noise
            .output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("llama.cpp failed:\n{}", err).into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn describe(&self) -> String {
        format!(
            "engine: llama.cpp\nmodel: {}\nn_predict: {}\n",
            self.model, self.n_predict
        )
    }
}
