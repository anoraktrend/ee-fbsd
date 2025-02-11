mod editor;
mod ui;
mod buffer;
mod config;
mod ffi;
mod error;

use std::env;
use std::process;
use editor::Editor;
use error::Result;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Initialize editor
    let mut editor = match Editor::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to initialize editor: {}", e);
            process::exit(1);
        }
    };

    // Run editor
    if let Err(e) = editor.run() {
        eprintln!("Editor error: {}", e);
        process::exit(1);
    }
}
