use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct FurEntry {
    pub attachment: Option<String>,
    pub avatar: String,
    pub branches: Vec<String>,
    pub children: Vec<String>,
    pub id: String,
    pub markdown: Option<String>,
    pub parent: Option<String>,
    pub schema_version: String,
    pub text: Option<String>,
    pub timestamp: String,
}

fn now_ts() -> (u64, String) {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    (
        t.as_secs(),
        format!("{:?}", SystemTime::now()),
    )
}

fn ensure_dirs() -> PathBuf {
    let root = PathBuf::from(".fur/messages");
    if !root.exists() {
        fs::create_dir_all(&root).unwrap();
    }
    root
}

pub fn log_text(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (id_num, ts) = now_ts();
    let id = format!("fur-{}", id_num);

    let entry = FurEntry {
        attachment: None,
        avatar: "user".into(),
        branches: vec![],
        children: vec![],
        id: id.clone(),
        markdown: None,
        parent: None,
        schema_version: "0.2".into(),
        text: Some(text.to_string()),
        timestamp: ts,
    };

    let path = ensure_dirs().join(format!("{}.json", id));
    fs::write(path, serde_json::to_string_pretty(&entry)?)?;

    Ok(id)
}

pub fn log_markdown(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let (id_num, ts) = now_ts();
    let id = format!("fur-{}", id_num);

    let entry = FurEntry {
        attachment: None,
        avatar: "llm".into(),
        branches: vec![],
        children: vec![],
        id: id.clone(),
        markdown: Some(path.to_string_lossy().to_string()),
        parent: None,
        schema_version: "0.2".into(),
        text: None,
        timestamp: ts,
    };

    let out = ensure_dirs().join(format!("{}.json", id));
    fs::write(out, serde_json::to_string_pretty(&entry)?)?;

    Ok(id)
}
