use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const RATATOSKR_VERSION: &str = "0.1.0-dev";

fn main() {
    if let Err(e) = run() {
        eprintln!("RATATOSKR ERROR: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let trace_root = Path::new("trace");
    ensure_dir(trace_root)?;

    let trace_id = generate_trace_id()?;
    let trace_dir = trace_root.join(&trace_id);

    if trace_dir.exists() {
        return Err(format!("trace already exists: {}", trace_id).into());
    }

    ensure_dir(&trace_dir)?;
    initialize_trace_layout(&trace_dir, &trace_id)?;

    println!("Trace initialized: {}", trace_id);
    Ok(())
}

fn ensure_dir(path: &Path) -> Result<(), io::Error> {
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

fn generate_trace_id() -> Result<String, io::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "system time error"))?;

    Ok(now.as_secs().to_string())
}

fn initialize_trace_layout(
    trace_dir: &PathBuf,
    trace_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Mandatory subdirectories
    ensure_dir(&trace_dir.join("resolved_context"))?;

    // Mandatory files
    write_file(trace_dir, "input.yaml", "# task specification (placeholder)\n")?;
    write_file(trace_dir, "prompt.txt", "# assembled prompt (placeholder)\n")?;
    write_file(trace_dir, "response.txt", "# model response (placeholder)\n")?;
    write_file(
        trace_dir,
        "memory_delta.yaml",
        "# append-only memory changes (placeholder)\n",
    )?;
    write_file(
        trace_dir,
        "engine.yaml",
        "# engine declaration (placeholder)\n",
    )?;
    write_file(
        trace_dir,
        "metadata.yaml",
        &metadata_contents(trace_id)?,
    )?;

    Ok(())
}

fn write_file(dir: &PathBuf, name: &str, contents: &str) -> Result<(), io::Error> {
    let path = dir.join(name);
    fs::write(path, contents)
}

fn metadata_contents(trace_id: &str) -> Result<String, io::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "system time error"))?;

    Ok(format!(
        "trace_id: {}\n\
         timestamp_unix: {}\n\
         ratatoskr_version: {}\n\
         status: initialized\n",
        trace_id,
        now.as_secs(),
        RATATOSKR_VERSION
    ))
}
