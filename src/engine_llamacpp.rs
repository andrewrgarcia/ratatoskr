use std::process::{Command, Stdio};
use std::path::PathBuf;

use crate::engine::Engine;

pub struct LlamaCppEngine {
    pub binary: PathBuf,
    pub model: PathBuf,
    pub n_predict: usize,
}

impl Engine for LlamaCppEngine {
    fn run(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        if !self.binary.exists() {
            return Err(format!(
                "llama.cpp binary not found: {}",
                self.binary.display()
            ).into());
        }

        if !self.model.exists() {
            return Err(format!(
                "model file not found: {}",
                self.model.display()
            ).into());
        }

        let child = Command::new(&self.binary)
            .arg("-m")
            .arg(&self.model)
            .arg("--n-predict")
            .arg(self.n_predict.to_string())
            .arg("--prompt")
            .arg(prompt)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        if !output.status.success() {
            return Err(format!(
                "llama.cpp execution failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ).into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn describe(&self) -> String {
        format!(
            "engine: llama.cpp\nbinary: {}\nmodel: {}\nn_predict: {}\n",
            self.binary.display(),
            self.model.display(),
            self.n_predict
        )
    }
}
