use std::path::PathBuf;
use std::fs;
use std::io::{self, Read};  // Remove unused Write import

#[derive(Default)]
pub struct Tab {
    pub lines: Vec<String>,  // Make public
    pub cursor: Cursor,      // Make public
    pub modified: bool,      // Make public
    pub syntax_name: Option<String>,  // Make public
    pub filename: Option<PathBuf>,    // Make public
}

pub struct Buffer {
    pub tabs: Vec<Tab>,     // Make public
    pub current_tab: usize, // Make public
}

#[derive(Default)]  // Add Default derive
pub struct Cursor {
    pub line: usize,
    pub column: usize,
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

    pub fn syntax_name(&self) -> Option<&str> {
        self.current_tab().syntax_name.as_deref()
    }

    pub fn set_syntax_name(&mut self, name: Option<String>) {
        self.current_tab_mut().set_syntax_name(name);
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
}

impl Tab {
    pub fn insert_char(&mut self, c: char) {
        let Cursor { line, column } = self.cursor;
        let current_line = &mut self.lines[line];
        current_line.insert(column, c);
        self.cursor.column += 1;
        self.modified = true;
    }

    pub fn delete_char(&mut self) {
        let Cursor { line, column } = self.cursor;
        if column > 0 {
            let current_line = &mut self.lines[line];
            current_line.remove(column - 1);
            self.cursor.column -= 1;
            self.modified = true;
        }
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn syntax_name(&self) -> Option<&str> {
        self.syntax_name.as_deref()
    }

    pub fn set_syntax_name(&mut self, name: Option<String>) {
        self.syntax_name = name;
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
        
        self.filename = Some(path);
        self.modified = false;
        self.cursor.line = 0;
        self.cursor.column = 0;
        Ok(())
    }

    pub fn get_filename(&self) -> Option<&PathBuf> {
        self.filename.as_ref()
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

    pub fn get_cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn is_modified(&self) -> bool {
        self.modified
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
