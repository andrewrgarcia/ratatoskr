use std::io::{self, Write};
use std::path::{PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::task::{Task, MemoryRef};
use crate::engine::EngineSpec;
use crate::fur_logger::{log_text, log_markdown};

/// Interactive session state
pub struct InteractiveSession {
    pub pending_ask: Option<String>,
    pub pending_attachments: Vec<PathBuf>,
}

impl InteractiveSession {
    pub fn new() -> Self {
        Self {
            pending_ask: None,
            pending_attachments: Vec::new(),
        }
    }
}

/// Entry point for interactive mode
pub fn run_interactive(default_engine: EngineSpec) -> Result<(), Box<dyn std::error::Error>> {
    let mut session = InteractiveSession::new();

    println!("RATATOSKR interactive mode");
    println!("Commands: ask | attach | run | exit\n");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)?;
        let cmd = cmd.trim();

        match cmd {
            "ask" => handle_ask(&mut session)?,
            "attach" => handle_attach(&mut session)?,
            "run" => {
                let task = lower_to_task(&mut session, &default_engine)?;
                let response_path = crate::execute_task(task)?;
                let id = log_markdown(&response_path)?;
                println!("✔ response logged (FUR:{})", id);
            }
            "exit" | "quit" => break,
            "" => continue,
            _ => println!("Unknown command: {}", cmd),
        }
    }

    Ok(())
}

fn handle_ask(session: &mut InteractiveSession) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter prompt (single line):");

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let text = line.trim().to_string();
    if text.is_empty() {
        println!("Empty prompt ignored");
        return Ok(());
    }

    let id = log_text(&text)?;
    session.pending_ask = Some(text);
    println!("✔ ask recorded (FUR:{})", id);

    Ok(())
}

fn handle_attach(session: &mut InteractiveSession) -> Result<(), Box<dyn std::error::Error>> {
    println!("Attachment mode [enter | chat | path] :");

    let mut mode = String::new();
    io::stdin().read_line(&mut mode)?;
    let mode = mode.trim();

    match mode {
        "" => {
            println!("(no attachment)");
        }

        "chat" => {
            println!("Paste text. End with a single line containing `EOF`.");

            let mut buf = String::new();
            loop {
                let mut line = String::new();
                io::stdin().read_line(&mut line)?;
                if line.trim() == "EOF" {
                    break;
                }
                buf.push_str(&line);
            }

            let path = write_markdown_attachment(&buf)?;
            let id = log_markdown(&path)?;
            session.pending_attachments.push(path);
            println!("✔ chat attachment recorded (FUR:{})", id);

            println!("✔ chat attachment recorded");
        }

        "path" => {
            println!("Enter file path:");

            let mut p = String::new();
            io::stdin().read_line(&mut p)?;
            let p = PathBuf::from(p.trim());

            if !p.exists() {
                println!("File does not exist");
                return Ok(());
            }

            let id = log_markdown(&p)?;
            session.pending_attachments.push(p);
            println!("✔ path attachment recorded (FUR:{})", id);
        }

        _ => println!("Unknown attach mode"),
    }

    Ok(())
}

fn write_markdown_attachment(contents: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = PathBuf::from("chats");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }


    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    let fname = format!("CHAT-{}.md", ts);

    let path = dir.join(fname);

    fs::write(&path, contents)?;
    Ok(path)
}

fn lower_to_task(
    session: &mut InteractiveSession,
    engine: &EngineSpec,
) -> Result<Task, Box<dyn std::error::Error>> {
    let prompt = session
        .pending_ask
        .take()
        .ok_or("no ask provided")?;

    let memory_refs = session
        .pending_attachments
        .drain(..)
        .map(|path| MemoryRef {
            system: "fur".into(),
            convo_id: path.to_string_lossy().to_string(), // FUR will resolve
        })
        .collect();

    Ok(Task {
        task_type: "interactive".into(),
        prompt,
        memory_scope: "explicit".into(),
        context: vec![],
        engine: engine.clone(),
        memory_refs,
    })
}
