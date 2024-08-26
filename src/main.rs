mod lnk_finder;
mod lnk_handler;

use std::path::PathBuf;
use clap::Parser;
use lnk_finder::find_lnk_files_streaming;

/// 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {

    /// Target path for the shortcut
    target: PathBuf,

    /// Start directory to search for .lnk files
    #[clap(default_value = "C:\\")]
    search_dir: PathBuf,

    /// If specified, runs in dry run mode without modifying .lnk files
    #[clap(short, long)]
    dry_run: bool,
}

fn main() -> windows::core::Result<()> {
    // Parse command-line arguments using clap
    let cli = Cli::parse();

    // Use the parsed arguments
    let search_directory = &cli.search_dir;
    let target_path = &cli.target;
    let dry_run = cli.dry_run;

    // Use streaming I/O to process files one at a time
    if let Err(e) = find_lnk_files_streaming(search_directory, target_path, dry_run) {
        eprintln!("Error processing .lnk files: {}", e);
    }

    Ok(())
}
