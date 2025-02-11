use std::path::PathBuf;
use std::fs;
use std::io::{self, Read};  // Remove unused Write import
use std::path::Path;
use crate::syntax::SyntaxHighlighter;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Default)]
pub struct Selection {
    pub start: Cursor,
    pub end: Cursor,
}

pub struct Tab {
    pub lines: Vec<String>,  // Make public
    pub cursor: Cursor,      // Make public
    pub modified: bool,      // Make public
    pub filename: Option<PathBuf>,    // Make public
    pub selection: Option<Selection>,
    pub extension: Option<String>,
}

pub struct Buffer {
    pub tabs: Vec<Tab>,     // Make public
    pub current_tab: usize, // Make public
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

    // Delegate methods to current tab
    pub fn lines(&self) -> &Vec<String> { &self.current_tab().lines }
    pub fn get_cursor(&self) -> &Cursor { &self.current_tab().cursor }
    pub fn is_modified(&self) -> bool { self.current_tab().modified }
    pub fn get_filename(&self) -> Option<&PathBuf> { self.current_tab().filename.as_ref() }
    
    // Move existing methods to Tab impl and delegate from Buffer
    pub fn insert_char(&mut self, c: char) {
        self.current_tab_mut().insert_char(c);
    }

    pub fn delete_char(&mut self) {
        self.current_tab_mut().delete_char();
    }

    pub fn save(&mut self, filename: Option<PathBuf>) -> io::Result<()> {
        self.current_tab_mut().save(filename)
    }

    pub fn load(&mut self, path: PathBuf) -> io::Result<()> {
        self.current_tab_mut().load(path)
    }

    pub fn move_cursor(&mut self, direction: CursorMove) {
        self.current_tab_mut().move_cursor(direction);
    }

    // Add delegation methods for selection
    pub fn start_selection(&mut self) {
        self.current_tab_mut().start_selection();
    }

    pub fn update_selection(&mut self) {
        self.current_tab_mut().update_selection();
    }

    pub fn clear_selection(&mut self) {
        self.current_tab_mut().clear_selection();
    }

    pub fn get_selected_text(&self) -> Option<String> {
        self.current_tab().get_selected_text()
    }

    pub fn delete_selection(&mut self) -> Option<String> {
        self.current_tab_mut().delete_selection()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.len() == 1 && 
        self.tabs[0].lines.len() == 1 && 
        self.tabs[0].lines[0].is_empty() &&
        !self.tabs[0].modified
    }
}

impl Tab {
    pub fn insert_char(&mut self, c: char) {
        // Ensure there's at least one line
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }

        // Handle newline specially
        if c == '\n' {
            let current_line = &self.lines[self.cursor.line];
            let remainder = if self.cursor.column < current_line.len() {
                current_line[self.cursor.column..].to_string()
            } else {
                String::new()
            };
            self.lines[self.cursor.line].truncate(self.cursor.column);
            self.lines.insert(self.cursor.line + 1, remainder);
            self.cursor.line += 1;
            self.cursor.column = 0;
        } else {
            let current_line = &mut self.lines[self.cursor.line];
            if self.cursor.column > current_line.len() {
                current_line.push_str(&" ".repeat(self.cursor.column - current_line.len()));
            }
            current_line.insert(self.cursor.column, c);
            self.cursor.column += 1;
        }
        self.modified = true;
    }

    pub fn delete_char(&mut self) {
        if self.lines.is_empty() {
            return;
        }

        if self.cursor.column > 0 {
            let current_line = &mut self.lines[self.cursor.line];
            if self.cursor.column <= current_line.len() {
                current_line.remove(self.cursor.column - 1);
                self.cursor.column -= 1;
                self.modified = true;
            }
        }
    }

    pub fn save(&mut self, filename: Option<PathBuf>) -> io::Result<()> {
        let path = if let Some(path) = filename {
            self.filename = Some(path.clone());
            path
        } else if let Some(path) = &self.filename {
            path.clone()
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "No filename specified"));
        };

        let content = self.lines.join("\n");
        fs::write(&path, content)?;
        self.modified = false;
        Ok(())
    }

    pub fn load(&mut self, path: PathBuf) -> io::Result<()> {
        let mut content = String::new();
        fs::File::open(&path)?.read_to_string(&mut content)?;
        
        self.lines = content.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        
        self.filename = Some(path.clone());
        self.modified = false;
        self.cursor.line = 0;
        self.cursor.column = 0;
        
        self.extension = path.extension()
            .and_then(|os_str| os_str.to_str())
            .map(String::from);
        
        Ok(())
    }

    pub fn move_cursor(&mut self, direction: CursorMove) {
        match direction {
            CursorMove::Left => {
                if self.cursor.column > 0 {
                    self.cursor.column -= 1;
                }
            }
            CursorMove::Right => {
                let line_len = self.lines[self.cursor.line].len();
                if self.cursor.column < line_len {
                    self.cursor.column += 1;
                }
            }
            CursorMove::Up => {
                if self.cursor.line > 0 {
                    self.cursor.line -= 1;
                    self.clamp_cursor_column();
                }
            }
            CursorMove::Down => {
                if self.cursor.line < self.lines.len() - 1 {
                    self.cursor.line += 1;
                    self.clamp_cursor_column();
                }
            }
            CursorMove::PageUp(page_size) => {
                if self.cursor.line > page_size {
                    self.cursor.line -= page_size;
                } else {
                    self.cursor.line = 0;
                }
                self.clamp_cursor_column();
            }
            CursorMove::PageDown(page_size) => {
                let max_line = self.lines.len().saturating_sub(1);
                self.cursor.line = (self.cursor.line + page_size).min(max_line);
                self.clamp_cursor_column();
            }
        }
    }

    fn clamp_cursor_column(&mut self) {
        let line_len = self.lines[self.cursor.line].len();
        self.cursor.column = self.cursor.column.min(line_len);
    }

    pub fn get_selected_text(&self) -> Option<String> {
        self.selection.as_ref().map(|sel| {
            let (start, end) = if sel.start <= sel.end {
                (&sel.start, &sel.end)
            } else {
                (&sel.end, &sel.start)
            };

            let mut text = String::new();
            for line_idx in start.line..=end.line {
                let line = &self.lines[line_idx];
                if line_idx == start.line && line_idx == end.line {
                    text.push_str(&line[start.column.min(line.len())..end.column.min(line.len())]);
                } else if line_idx == start.line {
                    text.push_str(&line[start.column.min(line.len())..]);
                    text.push('\n');
                } else if line_idx == end.line {
                    text.push_str(&line[..end.column.min(line.len())]);
                } else {
                    text.push_str(line);
                    text.push('\n');
                }
            }
            text
        })
    }

    pub fn start_selection(&mut self) {
        self.selection = Some(Selection {
            start: self.cursor.clone(),
            end: self.cursor.clone(),
        });
    }

    pub fn update_selection(&mut self) {
        if let Some(sel) = &mut self.selection {
            sel.end = self.cursor.clone();
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn delete_selection(&mut self) -> Option<String> {
        let text = self.get_selected_text()?;
        // TODO: Implement delete selected text
        self.clear_selection();
        Some(text)
    }
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],  // Initialize with one empty line
            cursor: Cursor::default(),
            modified: false,
            filename: None,
            selection: None,
            extension: None,
        }
    }
}

pub enum CursorMove {
    Left,
    Right,
    Up,
    Down,
    PageUp(usize),    // Parameter is page size
    PageDown(usize),  // Parameter is page size
}
