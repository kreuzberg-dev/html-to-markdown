// ~keep reason: CLI application modules do not expose docs to users; doc coverage not required
#![allow(missing_docs)]

use std::fs;
use std::path::PathBuf;

pub fn write_output(output_path: Option<PathBuf>, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    match output_path {
        Some(path) => {
            fs::write(&path, content.as_bytes())
                .map_err(|e| format!("Error writing to file '{}': {}", path.display(), e))?;
        }
        None => {
            // ~keep: converted Markdown is the CLI's machine-readable result output, not a diagnostic.
            #[expect(clippy::print_stdout, reason = "primary result output of the CLI, not a diagnostic")]
            {
                print!("{content}");
            }
        }
    }
    Ok(())
}
