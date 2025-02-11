extern crate libc;

mod buffer;
mod error;
mod ffi;
mod ui;

use error::{Result, EditorError};

fn main() {
    match run() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            if e.is_fatal() {
                std::process::exit(1);
            }
            std::process::exit(0)
        }
    }
}

fn run() -> Result<()> {
    let mut ui = ui::UI::new()?;
    
    loop {
        if let Err(e) = ui.refresh() {
            ui.display_error(&e);
            if e.is_fatal() {
                return Err(e);
            }
            continue;
        }
        
        match ui.get_key()? {
            Some(crossterm::event::KeyCode::Char('q')) => break,
            Some(crossterm::event::KeyCode::Char('r')) => ui.resize(),
            Some(key) => {
                // Handle other keys
                // You might want to pass this to an editor/buffer handler
            }
            None => {}
        }
    }
    
    Ok(())
}

// Helper functions
fn isatty(fd: i32) -> bool {
    unsafe { libc::isatty(fd) == 1 }
}
