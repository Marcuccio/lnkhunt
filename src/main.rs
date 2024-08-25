mod lnk_finder;
mod lnk_handler;

use lnk_finder::find_lnk_files;
use lnk_handler::handle_lnk_file;

use std::path::PathBuf;

fn main() {
    let target_directory = PathBuf::from("C:/path/to/search"); // Set your search directory here
    let dry_run = true; // Toggle dry run mode

    // Find all .lnk files in the target directory
    match find_lnk_files(&target_directory) {
        Ok(lnk_files) => {
            for lnk_file in lnk_files {
                handle_lnk_file(&lnk_file, dry_run);
            }
        }
        Err(e) => eprintln!("Error finding .lnk files: {}", e),
    }
}
