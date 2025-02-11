use std::ptr;
use ncurses::WINDOW;

mod editor;
mod window;
mod text;
mod menu;

use editor::Editor;

// Convert global state into struct fields
pub struct EeState {
    pub initialized: bool,
    pub curscr: WINDOW,
    pub stdscr: WINDOW,
}

impl Default for EeState {
    fn default() -> Self {
        Self {
            initialized: false,
            curscr: ptr::null_mut(),
            stdscr: ptr::null_mut(),
        }
    }
}

fn main() {
    // Initialize editor state
    let mut state = EeState::default();
    
    // Set up signal handlers
    for i in 1..24 {
        unsafe {
            libc::signal(i, libc::SIG_IGN);
        }
    }

    // Check input/output are terminals
    if !isatty(0) || !isatty(1) {
        eprintln!("ee's standard input and output must be a terminal");
        std::process::exit(1);
    }

    // Initialize editor with state
    let mut editor = Editor::new(&mut state);
    
    // Parse command line args
    editor.get_options();

    // Set up terminal
    editor.set_up_term();

    // Main edit loop
    while editor.edit {
        editor.process_input();
    }
}

// Helper functions
fn isatty(fd: i32) -> bool {
    unsafe { libc::isatty(fd) == 1 }
}
