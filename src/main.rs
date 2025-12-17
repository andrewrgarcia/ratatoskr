use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    if let Err(e) = run() {
        eprintln!("RATATOSKR ERROR: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure trace root exists
    let trace_root = Path::new("trace");
    if !trace_root.exists() {
        fs::create_dir(trace_root)?;
    }

    // Generate a trace ID using UNIX timestamp
    let trace_id = generate_trace_id()?;
    let trace_dir = trace_root.join(&trace_id);

    // Refuse to overwrite an existing trace
    if trace_dir.exists() {
        return Err(format!("trace already exists: {}", trace_id).into());
    }

    fs::create_dir(&trace_dir)?;

    // Create mandatory trace files
    write_file(&trace_dir, "input.yaml", "# placeholder\n")?;
    write_file(&trace_dir, "prompt.txt", "# placeholder\n")?;
    write_file(&trace_dir, "response.txt", "# placeholder\n")?;
    write_file(&trace_dir, "memory_delta.yaml", "# placeholder\n")?;
    write_file(&trace_dir, "engine.yaml", "# placeholder\n")?;
    write_file(&trace_dir, "metadata.yaml", &metadata_contents(&trace_id)?)?;

    println!("Trace initialized: {}", trace_id);
    Ok(())
}

fn generate_trace_id() -> Result<String, io::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "time went backwards"))?;

    Ok(format!("{}", now.as_secs()))
}

fn write_file(dir: &Path, name: &str, contents: &str) -> Result<(), io::Error> {
    let path = dir.join(name);
    fs::write(path, contents)
}

fn metadata_contents(trace_id: &str) -> Result<String, io::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "time went backwards"))?;

    Ok(format!(
        "trace_id: {}\ntimestamp_unix: {}\nstatus: initialized\n",
        trace_id,
        now.as_secs()
    ))
}
