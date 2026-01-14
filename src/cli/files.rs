#![cfg(feature = "cli")]

use anyhow::Result;
use glob::glob;
use std::collections::BTreeSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Writes content to a file atomically using a streaming closure.
/// This prevents loading the entire output string into memory before writing.
pub fn atomic_write_stream<F>(p: &Path, write_op: F) -> Result<()>
where
    F: FnOnce(&mut dyn Write) -> io::Result<()>,
{
    // Create temp file in the same directory to ensure atomic move works (same filesystem)
    let mut t = tempfile::NamedTempFile::new_in(p.parent().unwrap_or_else(|| Path::new(".")))?;

    // Wrap in BufWriter for syscall performance
    let mut writer = io::BufWriter::new(&mut t);

    // Execute the streaming operation (Zero-Copy flow)
    write_op(&mut writer)?;

    // Flush to ensure all bytes hit the temp file
    writer.flush()?;

    // Drop writer to release borrow, then persist
    drop(writer);
    t.persist(p)?;

    Ok(())
}

/// Discovers files based on direct paths and glob patterns.
pub fn discover_files(files: &[PathBuf], pattern: Option<&str>) -> Result<BTreeSet<PathBuf>> {
    let mut ts = BTreeSet::new();

    // 1. Explicit files
    for p in files {
        if p.exists() {
            ts.insert(p.clone());
        }
    }

    // 2. Glob patterns
    if let Some(q) = pattern {
        for entry in glob(q)? {
            match entry {
                Ok(path) if path.is_file() => {
                    ts.insert(path);
                }
                _ => {} // Ignore errors or non-files in glob results
            }
        }
    }
    Ok(ts)
}

/// Generates a new path by appending a suffix to the file stem.
pub fn get_output_path(path: &Path, suffix: &str) -> PathBuf {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let new_filename = if extension.is_empty() {
        format!("{stem}{suffix}")
    } else {
        format!("{stem}{suffix}.{extension}")
    };

    path.with_file_name(new_filename)
}

/// Helper function to enforce path-safe characters in user-provided suffixes.
pub fn sanitize_string(s: &mut String) {
    s.retain(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.');
}
