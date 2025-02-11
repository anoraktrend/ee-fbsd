use std::env;
use std::ffi::{CStr, CString};
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use std::ptr;
use ncurses::WINDOW;

mod editor;
mod window;
mod text;
mod menu;

use editor::Editor;

// Convert global state into struct fields
pub struct EeState {
    curscr: WINDOW,
    virtual_scr: WINDOW, 
    stdscr: WINDOW,
    last_window_refreshed: WINDOW,
    
    bufp: i32,
    terminal_type: Option<String>,
    cfound: bool,
    data_line_len: i32,
    max_key_len: i32,
    data_line: Option<String>,
    term_path: Option<String>,
    term_data_ptr: Option<String>,
    term_file_name: Option<String>,
    stand: bool,
    term_info: bool,
    time_out: bool,
    curr_x: i32,
    curr_y: i32,
    lines: i32,
    cols: i32,
    move_it: bool,
    initialized: bool,
    speed: f32,
    chars_per_millisecond: f32,
    repaint_screen: bool,
    intr: i32,
    parity: i32,
    noblock: bool,
    num_bits: i32,
    flip_bytes: bool,
    interrupt_flag: bool,
}

impl Default for EeState {
    fn default() -> Self {
        Self {
            curscr: ptr::null_mut(),
            virtual_scr: ptr::null_mut(),
            stdscr: ptr::null_mut(), 
            last_window_refreshed: ptr::null_mut(),
            
            bufp: 0,
            terminal_type: None,
            cfound: false,
            data_line_len: 0,
            max_key_len: 0,
            data_line: None,
            term_path: None,
            term_data_ptr: None, 
            term_file_name: None,
            stand: false,
            term_info: false,
            time_out: false,
            curr_x: 0,
            curr_y: 0,
            lines: 0,
            cols: 0,
            move_it: false,
            initialized: false,
            speed: 0.0,
            chars_per_millisecond: 0.0,
            repaint_screen: false,
            intr: 0,
            parity: 0,
            noblock: false,
            num_bits: 0,
            flip_bytes: false,
            interrupt_flag: false
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
