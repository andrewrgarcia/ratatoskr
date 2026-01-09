use std::path::Path;
use std::process::Command;
use std::io;

/// Thin delegation layer between RATATOSKR and FUR.
///
/// RULES:
/// - Ratatoskr NEVER writes FUR JSON directly
/// - Ratatoskr NEVER guesses schema
/// - All persistence happens via `fur` CLI
///
/// This module is intentionally boring.
pub struct FurBridge;

impl FurBridge {
    /// Jot a short text message as the MAIN avatar
    /// Equivalent to: `fur jot "text"`
    pub fn jot_main(text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("fur")
            .arg("jot")
            .arg(text)
            .status()?;

        if !status.success() {
            return Err("fur jot (main avatar) failed".into());
        }

        Ok(())
    }

    /// Jot a markdown file as a SECONDARY avatar (e.g. mistral)
    /// Equivalent to: `fur jot <avatar> --markdown <path>`
    ///
    /// NOTE:
    /// - `avatar` MUST already be registered in FUR
    pub fn jot_markdown_as(
        avatar: &str,
        markdown_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("fur")
            .arg("jot")
            .arg(avatar)                 // positional avatar
            .arg("--markdown")
            .arg(markdown_path)
            .status()?;

        if !status.success() {
            return Err(format!(
                "fur jot {} --markdown {} failed",
                avatar,
                markdown_path.display()
            ).into());
        }

        Ok(())
    }

    /// Delegate to FUR's interactive chat capture
    /// Equivalent to: `fur chat`
    ///
    /// This is used for long-form paste.
    /// 
    pub fn chat() -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("fur")
            .arg("chat")
            .status()?;

        if !status.success() {
            return Err("fur chat failed".into());
        }

        Ok(())
    }

    /// Sanity check that FUR is available on PATH
    pub fn ensure_available() -> Result<(), io::Error> {
        Command::new("fur")
            .arg("--version")
            .status()
            .map(|_| ())
    }

    /// Jot a markdown file as the MAIN avatar
    /// Equivalent to: `fur jot --markdown <path>`
    pub fn jot_main_markdown(
        markdown_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("fur")
            .arg("jot")
            .arg("--markdown")
            .arg(markdown_path)
            .status()?;

        if !status.success() {
            return Err(format!(
                "fur jot --markdown {} failed",
                markdown_path.display()
            ).into());
        }

        Ok(())
    }

}
