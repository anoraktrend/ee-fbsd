use super::cursor::{Cursor, CursorMove};
use super::tab::Tab;
use std::path::PathBuf;
use std::io;

macro_rules! delegate_to_tab {
    ($self:ident, $method:ident $(,$arg:expr)*) => {
        $self.current_tab_mut().$method($($arg),*)
    }
}

pub struct Buffer {
    pub tabs: Vec<Tab>,
    pub current_tab: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            tabs: vec![Tab::default()],
            current_tab: 0,
        }
    }

    // Keep core tab management methods
    pub fn current_tab(&self) -> &Tab {
        &self.tabs[self.current_tab]
    }

    pub fn current_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.current_tab]
    }

    // Tab navigation methods
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.current_tab = (self.current_tab + 1) % self.tabs.len();
        }
    }

    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.current_tab = (self.current_tab + self.tabs.len() - 1) % self.tabs.len();
        }
    }

    pub fn new_tab(&mut self) {
        self.tabs.push(Tab::default());
        self.current_tab = self.tabs.len() - 1;
    }

    pub fn close_tab(&mut self) -> bool {
        if self.tabs.len() > 1 {
            self.tabs.remove(self.current_tab);
            if self.current_tab >= self.tabs.len() {
                self.current_tab = self.tabs.len() - 1;
            }
            true
        } else {
            false
        }
    }

    // Delegate methods
    pub fn lines(&self) -> &Vec<String> {
        self.current_tab().lines()
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

    pub fn insert_char(&mut self, c: char) {
        delegate_to_tab!(self, insert_char, c)
    }

    pub fn delete_char(&mut self) {
        delegate_to_tab!(self, delete_char)
    }

    pub fn save(&mut self, filename: Option<PathBuf>) -> io::Result<()> {
        delegate_to_tab!(self, save, filename)
    }

    pub fn load(&mut self, path: PathBuf) -> io::Result<()> {
        delegate_to_tab!(self, load, path)
    }

    pub fn move_cursor(&mut self, direction: CursorMove) {
        delegate_to_tab!(self, move_cursor, direction)
    }

    pub fn start_selection(&mut self) {
        delegate_to_tab!(self, start_selection)
    }

    pub fn update_selection(&mut self) {
        delegate_to_tab!(self, update_selection)
    }

    pub fn clear_selection(&mut self) {
        delegate_to_tab!(self, clear_selection)
    }

    pub fn get_selected_text(&mut self) -> Option<String> {
        delegate_to_tab!(self, get_selected_text)
    }

    pub fn delete_selection(&mut self) -> Option<String> {
        delegate_to_tab!(self, delete_selection)
    }

    // Utility methods
    pub fn is_empty(&self) -> bool {
        self.tabs.len() == 1 && 
        self.tabs[0].lines.len() == 1 && 
        self.tabs[0].lines[0].is_empty() &&
        !self.tabs[0].modified
    }

    pub fn set_cursor_position(&mut self, row: usize, col: usize) {
        let tab = self.current_tab_mut();
        tab.cursor.line = row.min(tab.lines.len().saturating_sub(1));
        tab.cursor.column = col;
        tab.clamp_cursor_column();
    }

    pub fn update_selection_to(&mut self, row: usize, col: usize) {
        let tab = self.current_tab_mut();
        if tab.selection.is_none() {
            tab.start_selection();
        }
        tab.cursor.line = row.min(tab.lines.len().saturating_sub(1));
        tab.cursor.column = col;
        tab.clamp_cursor_column();
        tab.update_selection();
    }
}

pub trait TabTrait {
    fn lines(&self) -> &Vec<String>;
    fn get_cursor(&self) -> &Cursor;
    fn is_modified(&self) -> bool;
    fn get_filename(&self) -> Option<&PathBuf>;
}

impl TabTrait for Tab {
    fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    fn get_cursor(&self) -> &Cursor {
        &self.cursor
    }

    fn is_modified(&self) -> bool {
        self.modified
    }

    fn get_filename(&self) -> Option<&PathBuf> {
        self.filename.as_ref()
    }
}
