mod editor;
mod ui;
mod buffer;
mod config;
mod error;

use std::process;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::panic;
use editor::Editor;

fn main() {
    // Set up proper panic handling
    panic::set_hook(Box::new(|_| {
        let mut stdout = stdout();
        let _ = stdout.execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }));

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
