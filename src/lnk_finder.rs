use std::fs::{self, OpenOptions};
use std::path::Path;
use std::io::{self, ErrorKind};

use crate::lnk_handler::handle_lnk_file;


/// Streams .lnk files in a directory recursively without storing them in memory all at once
/// Improved error handling and reporting
pub fn find_lnk_files_streaming(dir: &Path, target_path: &Path, dry_run: bool) -> Result<(), io::Error> {
    // Attempt to read the directory entries
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Directory '{}' not found.", dir.display());
                }
                ErrorKind::PermissionDenied => {}
                _ => {
                    eprintln!("[!] Unexpected error '{}': {}", dir.display(), e);
                }
            }
            return Err(e); // Return the error if the directory can't be read
        }
    };

    // Iterate over the entries one by one
    for entry in entries {
        // Handle any errors that occur when reading the directory entry
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to read an entry in directory '{}': {}", dir.display(), e);
                continue; // Skip this entry and continue with the next one
            }
        };

        let path = entry.path();

        // If the entry is a directory, recurse into it
        if path.is_dir() {
            let _ = find_lnk_files_streaming(&path, target_path, dry_run);
        }
        // If the entry is a .lnk file, attempt to handle it
        else if path.extension().and_then(|ext| ext.to_str()) == Some("lnk") {
            if let Ok(true) = is_lnk_file_writable(&path) {
                if let Ok(()) = handle_lnk_file(&path, target_path, dry_run) {
                    println!("[+] .lnk file '{}'", path.display());
                }
            }
        }
    }

    Ok(())
}

/// Checks if a `.lnk` file is writable by attempting to open it in write mode.
pub fn is_lnk_file_writable(lnk_file: &Path) -> Result<bool, io::Error> {
    let open_result = OpenOptions::new()
        .write(true)
        .open(lnk_file);
    
    match open_result {
        Ok(_) => Ok(true), // Successfully opened with write permissions
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => Ok(false), // Not writable due to permissions
        Err(e) => Err(e), // Return other errors (e.g., file not found)
    }
}

