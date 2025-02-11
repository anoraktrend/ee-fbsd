mod cursor;
mod selection;
mod tab;

pub use cursor::{Cursor, CursorMove};
pub use selection::Selection;
pub use tab::Tab;

use std::path::{Path, PathBuf};
use std::io;

pub struct Buffer {
    tabs: Vec<Tab>,
    current_tab: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            tabs: vec![Tab::default()],
            current_tab: 0,
        }
    }

    pub fn current_tab(&self) -> &Tab {
        &self.tabs[self.current_tab]
    }

    pub fn current_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.current_tab]
    }

    // Delegate methods for read access
    pub fn lines(&self) -> &Vec<String> {
        self.current_tab().get_lines()
    }

    pub fn get_cursor(&self) -> &Cursor {
        self.current_tab().get_cursor()
    }

    pub fn is_modified(&self) -> bool {
        self.current_tab().is_modified()
    }

    pub fn get_filename(&self) -> Option<&PathBuf> {
        self.current_tab().get_filename()
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let tab = self.current_tab_mut();
        tab.load(path.as_ref().to_path_buf())
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let tab = self.current_tab_mut();
        tab.save(Some(path.as_ref().to_path_buf()))
    }

    pub fn insert_char(&mut self, c: char) {
        let tab = self.current_tab_mut();
        if tab.get_selection().is_some() {
            let selection = tab.get_selection().unwrap().clone();
            tab.delete_selection();
            tab.get_cursor_mut().column = selection.start.column;
            tab.get_cursor_mut().line = selection.start.line;
        }
        tab.insert_char(c);
    }

    pub fn delete_char(&mut self) {
        let tab = self.current_tab_mut();
        if tab.get_selection().is_some() {
            tab.delete_selection();
        } else {
            tab.delete_char();
        }
    }

    pub fn move_cursor(&mut self, movement: CursorMove) {
        let tab = self.current_tab_mut();
        let lines = tab.get_lines().len();
        let line = tab.get_cursor().line;
        let _col = tab.get_cursor().column;  // Mark as intentionally unused

        match movement {
            CursorMove::PageUp(n) => {
                let new_line = if line >= n { line - n } else { 0 };
                tab.get_cursor_mut().line = new_line;
                tab.clamp_cursor_column();
            },
            CursorMove::PageDown(n) => {
                let new_line = if line + n < lines { line + n } else { lines - 1 };
                tab.get_cursor_mut().line = new_line;
                tab.clamp_cursor_column();
            },
            _ => tab.move_cursor(movement),
        }
    }

    pub fn start_selection(&mut self) {
        let tab = self.current_tab_mut();
        if tab.get_selection().is_none() {
            let cursor = *tab.get_cursor();
            let selection = Selection {
                start: cursor,
                end: cursor,
            };
            *tab.get_selection_mut() = Some(selection);
        }
    }

    pub fn update_selection(&mut self) {
        let tab = self.current_tab_mut();
        let cursor = *tab.get_cursor();
        if let Some(selection) = tab.get_selection_mut() {
            selection.end = cursor;
        }
    }

    pub fn get_selected_text(&self) -> Option<String> {
        let tab = self.current_tab();
        tab.get_selection().map(|selection| {
            let start = selection.start;
            let end = selection.end;
            if start.line == end.line {
                tab.get_lines()[start.line][start.column..end.column].to_string()
            } else {
                let mut text = String::new();
                text.push_str(&tab.get_lines()[start.line][start.column..]);
                for line in &tab.get_lines()[start.line + 1..end.line] {
                    text.push_str(line);
                }
                text.push_str(&tab.get_lines()[end.line][..end.column]);
                text
            }
        })
    }

    pub fn delete_selection(&mut self) -> Option<String> {
        let tab = self.current_tab_mut();
        let selection = tab.get_selection()?.clone();
        let text = tab.get_selected_text()?;

        // Do the deletion
        {
            let lines = tab.get_lines_mut();
            if selection.start.line == selection.end.line {
                lines[selection.start.line].replace_range(
                    selection.start.column..selection.end.column, ""
                );
            } else {
                // ...existing deletion logic...
            }
        }

        *tab.get_cursor_mut() = selection.start;
        tab.set_modified(true);
        tab.get_selection_mut().take();

        Some(text)
    }

    pub fn end_selection(&mut self) {
        let tab = self.current_tab_mut();
        if let Some(selection) = tab.get_selection() {
            if selection.start == selection.end {
                *tab.get_selection_mut() = None;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.len() == 1 && 
        self.tabs[0].get_lines().len() == 1 && 
        self.tabs[0].get_lines()[0].is_empty() &&
        !self.tabs[0].is_modified()
    }

    pub fn set_cursor_position(&mut self, row: usize, col: usize) {
        let tab = self.current_tab_mut();
        tab.get_cursor_mut().line = row.min(tab.get_lines().len().saturating_sub(1));
        tab.get_cursor_mut().column = col;
        tab.clamp_cursor_column();
    }

    pub fn update_selection_to(&mut self, row: usize, col: usize) {
        let tab = self.current_tab_mut();
        if tab.get_selection().is_none() {
            tab.start_selection();
        }
        tab.get_cursor_mut().line = row.min(tab.get_lines().len().saturating_sub(1));
        tab.get_cursor_mut().column = col;
        tab.clamp_cursor_column();
        tab.update_selection();
    }

    // Add methods for UI access
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn current_tab_index(&self) -> usize {
        self.current_tab
    }

    pub fn iter_tabs(&self) -> impl Iterator<Item = &Tab> {
        self.tabs.iter()
    }
}