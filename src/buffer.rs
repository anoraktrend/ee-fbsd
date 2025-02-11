use std::path::PathBuf;
use std::fs;
use std::io::{self, Read, Write};

pub struct Buffer {
    lines: Vec<String>,
    cursor: Cursor,
    modified: bool,
    syntax_name: Option<String>,
    filename: Option<PathBuf>,
}

pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: Cursor { line: 0, column: 0 },
            modified: false,
            syntax_name: None,
            filename: None,
        }
    }

    pub fn should_quit(&self) -> bool {
        !self.modified
    }

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
