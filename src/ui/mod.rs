mod highlight;
mod theme;

use crate::error::{Result, EditorError};
use ncurses::*;
use std::io;

pub struct UI {
    pub windows: crate::window::Windows,
}

impl UI {
    pub fn new() -> Result<Self> {
        // Initialize ncurses
        let config = crate::config::EditorConfig::load();
        if initscr().is_null() {
            return Err(EditorError::TerminalInit);
        }

        raw();
        keypad(stdscr(), true);
        noecho();
        
        if !has_colors() {
            endwin();
            return Err(EditorError::TerminalInit);
        }
        
        start_color();
        use_default_colors();

        // Initialize colors for syntax highlighting
        for i in 0..=15 {
            init_pair(i as i16, i as i16, -1);
        }

        // Create window components
        let windows = crate::window::Windows::new(&config);

        Ok(Self { windows })
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.windows.refresh_all();
        Ok(())
    }

    pub fn get_key(&self) -> Result<Option<crossterm::event::KeyCode>> {
        let ch = getch();
        if ch == ERR {
            return Ok(None);
        }

        // Convert ncurses key to crossterm KeyCode
        let key = match ch {
            KEY_UP => Some(crossterm::event::KeyCode::Up),
            KEY_DOWN => Some(crossterm::event::KeyCode::Down),
            KEY_LEFT => Some(crossterm::event::KeyCode::Left), 
            KEY_RIGHT => Some(crossterm::event::KeyCode::Right),
            KEY_BACKSPACE => Some(crossterm::event::KeyCode::Backspace),
            KEY_DC => Some(crossterm::event::KeyCode::Delete),
            KEY_ENTER | 10 | 13 => Some(crossterm::event::KeyCode::Enter),
            27 => Some(crossterm::event::KeyCode::Esc), // Just use literal 27 for ESC
            _ if ch >= 32 && ch <= 126 => {
                Some(crossterm::event::KeyCode::Char(ch as u8 as char))
            }
            _ => None
        };

        Ok(key)
    }

    pub fn resize(&mut self) {
        endwin();
        refresh();
        self.windows.resize(true);
    }

    pub fn shutdown(&self) {
        endwin();
    }

    pub fn display_error(&self, error: &EditorError) {
        let msg = error.display_message();
        let line = LINES() - 1;
        mvaddstr(line, 0, &format!("Error: {}", msg));
        refresh();
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        self.shutdown();
    }
}

// Helper function to highlight text
pub fn highlight_buffer(buffer: &crate::buffer::Buffer, theme: &Theme) -> Vec<String> {
    let content = buffer.lines().join("\n");
    highlight::highlight_content(&content, theme)
        .into_iter()
        .map(|(_, text)| text)
        .collect()
}
