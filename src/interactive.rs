use std::io::{self, Write, Read};
use std::path::PathBuf;
use crate::task::ContextRef;
use crate::engine::EngineSpec;
use crate::fur_bridge::FurBridge;
use std::process::Command;

/// Interactive session state
pub struct InteractiveSession {
    pub pending_ask: Option<String>,
    pub pending_context: Vec<ContextRef>,
}

impl InteractiveSession {
    pub fn new() -> Self {
        Self {
            pending_ask: None,
            pending_context: Vec::new(),
        }
    }
}

const RATATOSKR_BANNER: &str = r#"        
                                                            
     ############## ===================  ##############     
       ###########  ===================== ###########       
           #########  =====        ======  ######           
          ######     ======        ======   ######          
           ########  ===================  #######           
             ###### ==================  #######             
                    ======  =======  ###                    
                   ======     =======                       
                   ======       =======                     
                                                                                                                          
"#;

/// Entry point for interactive mode
pub fn run_interactive(default_engine: EngineSpec) -> Result<(), Box<dyn std::error::Error>> {
    FurBridge::ensure_available()?;

    let mut session = InteractiveSession::new();

    println!("{}", RATATOSKR_BANNER);
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
                let prompt = session.pending_ask.take().ok_or("no ask")?;
                
                println!("\n=== Running llama.cpp ===\n");
                
                Command::new(&default_engine.name)
                    .arg("-m")
                    .arg(&default_engine.model)
                    .arg("-p")
                    .arg(&prompt)
                    .arg("-n")
                    .arg("256")
                    .status()?;
                
                println!("\n=== Paste the AI's response (Ctrl+D when done) ===\n");
                
                let mut response = String::new();
                io::stdin().read_to_string(&mut response)?;
                
                if response.trim().is_empty() {
                    println!("No response captured");
                    return Ok(());
                }
                
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();
                let response_file = PathBuf::from(format!("responses/RESPONSE-{}.md", ts));
                std::fs::create_dir_all("responses")?;
                std::fs::write(&response_file, &response)?;
                
                match default_engine.avatar.as_deref() {
                    Some(avatar) => {
                        FurBridge::jot_markdown_as(avatar, &response_file)?;
                        println!("✔ Logged as avatar `{}`", avatar);
                    }
                    None => {
                        FurBridge::jot_main_markdown(&response_file)?;
                        println!("✔ Logged to FUR");
                    }
                }
                
                session.pending_context.clear();
                println!();
            },
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
    let text = line.trim();

    if text.is_empty() {
        println!("Empty prompt ignored");
        return Ok(());
    }

    // Log user ask as MAIN avatar
    FurBridge::jot_main(text)?;

    session.pending_ask = Some(text.to_string());
    println!("✔ ask recorded");

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
            // Delegate entirely to FUR
            FurBridge::chat()?;

            println!("✔ chat attached (FUR-managed)");
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

            session.pending_context.push(ContextRef {
                path: p.to_string_lossy().to_string(),
                lines: None,
                section: None,
            });

            println!("✔ path attached as context");
        }

        _ => println!("Unknown attach mode"),
    }

    Ok(())
}