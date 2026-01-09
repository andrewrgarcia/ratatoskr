mod engine;
mod engine_llamacpp;
mod task;
mod trace;
mod fur_atom;
mod fur_logger;
mod citation;
mod validate;
mod execute;
mod interactive;

use crate::execute::execute_task;
use crate::interactive::run_interactive;
use crate::task::load_task;
use crate::engine::EngineSpec;

fn main() {
    if let Err(e) = entrypoint() {
        eprintln!("RATATOSKR ERROR: {}", e);
        std::process::exit(1);
    }
}

fn entrypoint() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // ---------------------------------
    // Default: interactive mode
    // ---------------------------------
    if args.len() == 1 {
        let engine = default_engine();
        return run_interactive(engine);
    }

    // ---------------------------------
    // Batch mode: rat run
    // ---------------------------------
    if args.len() == 2 && args[1] == "run" {
        let task = load_task()?;
        execute_task(task)?;
        return Ok(());
    }

    Err("usage: rat [run]".into())
}

/// Hardcoded default engine for interactive mode.
/// This is intentional and can be made configurable later.
fn default_engine() -> EngineSpec {
    EngineSpec {
        engine_type: "llama.cpp".into(),
        name: "/home/andrew/engines/llama.cpp/llama".into(),
        model: "/home/andrew/models/capybarahermes-2.5-mistral-7b.Q4_K_M.gguf".into(),
    }
}
