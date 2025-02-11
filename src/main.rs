mod editor;
mod ui;
mod buffer;
mod config;
mod ffi;
mod error;

use std::process;
use editor::Editor;

fn main() {
    let mut editor = match Editor::new() {
        Ok(e) => e,
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
