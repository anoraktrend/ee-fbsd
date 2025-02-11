mod editor;
mod ui;
mod buffer;
mod config;
mod error;
mod syntax;

use std::process;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::panic;
use clap::Parser;
use editor::Editor;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Files to edit
    files: Vec<String>,
}

fn main() {
    // Set up proper panic handling
    panic::set_hook(Box::new(|_| {
        let mut stdout = stdout();
        let _ = stdout.execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }));

    let args = Args::parse();
    
    let mut editor = match Editor::new() {
        Ok(mut e) => {
            // Load files from command line
            for file in args.files {
                if let Err(e) = e.load_file(&file) {
                    eprintln!("Warning: Failed to load {}: {}", file, e);
                }
            }
            e
        }
        Err(e) => {
            eprintln!("Failed to initialize editor: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = editor.run() {
        eprintln!("Editor error: {}", e);
        process::exit(1);
    }
}
