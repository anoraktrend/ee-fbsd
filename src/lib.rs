use std::ptr;
use ncurses::WINDOW;

pub mod buffer;
pub mod config;
pub mod editor;
pub mod ui;
pub mod window;
pub mod text;
pub mod menu; // Using src/menu.rs
pub mod ffi;  // Will use the directory-based module

// Re-export commonly used items
pub use editor::Editor;
pub use text::TextProcessor;

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