use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FurAtom {
    /// Memory system identifier (always "fur")
    pub system: String,

    /// Conversation / thread ID
    pub convo_id: String,

    /// Message ID inside the conversation
    pub message_id: String,

    /// Linear order in the conversation (0-based)
    pub order: usize,

    /// Source type: "text" or "markdown"
    pub source: String,

    /// Fully resolved content used for inference
    pub content: String,

    /// SHA256 of `content`
    pub sha256: String,
}

pub fn load_fur_thread(
    fur_dir: &Path,
    convo_id: &str,
) -> Result<Vec<FurAtom>, Box<dyn std::error::Error>> {
    let thread_path = fur_dir
        .join("threads")
        .join(format!("{}.json", convo_id));

    if !thread_path.exists() {
        return Err(format!(
            "FUR thread not found: {}",
            thread_path.display()
        ).into());
    }

    let thread: Value =
        serde_json::from_str(&fs::read_to_string(&thread_path)?)?;

    let msgs = thread["messages"]
        .as_array()
        .ok_or("thread.messages must be array")?;

    let mut atoms = Vec::new();

    for (order, mid) in msgs.iter().enumerate() {
        let message_id = mid
            .as_str()
            .ok_or("invalid message id")?
            .to_string();

        let msg_path = fur_dir
            .join("messages")
            .join(format!("{}.json", message_id));

        let msg: Value =
            serde_json::from_str(&fs::read_to_string(&msg_path)?)?;

        let text = msg.get("text").and_then(|v| v.as_str());
        let markdown = msg.get("markdown").and_then(|v| v.as_str());

        let (source, content) = match (text, markdown) {
            (Some(t), None) => ("text".to_string(), t.to_string()),
            (None, Some(md)) => {
                let md_path = fur_dir
                    .parent()
                    .ok_or("fur_dir has no parent")?
                    .join(md);

                (
                    "markdown".to_string(),
                    fs::read_to_string(&md_path)?,
                )
            }
            _ => {
                return Err(format!(
                    "invalid content in message {} (must have exactly one of text|markdown)",
                    message_id
                )
                .into())
            }
        };

        let sha256 = sha256_str(&content);

        atoms.push(FurAtom {
            system: "fur".to_string(),
            convo_id: convo_id.to_string(),
            message_id,
            order,
            source,
            content,
            sha256,
        });
    }

    Ok(atoms)
}

fn sha256_str(s: &str) -> String {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    format!("{:x}", h.finalize())
}
