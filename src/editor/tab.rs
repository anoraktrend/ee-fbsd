use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};
use super::cursor::{Cursor, Selection, CursorMove};

pub struct Tab {
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub modified: bool,
    pub filename: Option<PathBuf>,
    pub selection: Option<Selection>,
    pub extension: Option<String>,
}

impl Tab {
    pub fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.cursor.line];
        line.insert(self.cursor.column, c);
        self.cursor.column += 1;
        self.modified = true;
    }

    pub fn delete_char(&mut self) {
        if self.cursor.column > 0 {
            let line = &mut self.lines[self.cursor.line];
            line.remove(self.cursor.column - 1);
            self.cursor.column -= 1;
            self.modified = true;
        }
    }

    pub fn move_cursor(&mut self, direction: CursorMove) {
        match direction {
            CursorMove::Left => if self.cursor.column > 0 {
                self.cursor.column -= 1;
            },
            CursorMove::Right => {
                let line_len = self.lines[self.cursor.line].len();
                if self.cursor.column < line_len {
                    self.cursor.column += 1;
                }
            },
            CursorMove::Up => if self.cursor.line > 0 {
                self.cursor.line -= 1;
                self.clamp_cursor_column();
            },
            CursorMove::Down => {
                if self.cursor.line < self.lines.len() - 1 {
                    self.cursor.line += 1;
                    self.clamp_cursor_column();
                }
            },
            CursorMove::PageUp(lines) => {
                self.cursor.line = self.cursor.line.saturating_sub(lines);
                self.clamp_cursor_column();
            },
            CursorMove::PageDown(lines) => {
                self.cursor.line = (self.cursor.line + lines).min(self.lines.len() - 1);
                self.clamp_cursor_column();
            },
        }
    }

    pub fn clamp_cursor_column(&mut self) {
        let line_len = self.lines[self.cursor.line].len();
        if self.cursor.column > line_len {
            self.cursor.column = line_len;
        }
    }

    pub fn save(&mut self, filename: Option<PathBuf>) -> io::Result<()> {
        let path = filename.unwrap_or_else(|| self.filename.clone().unwrap());
        let mut file = fs::File::create(&path)?;
        for line in &self.lines {
            file.write_all(line.as_bytes())?;
            file.write_all(b"\n")?;
        }
        self.filename = Some(path);
        self.modified = false;
        Ok(())
    }

    pub fn load(&mut self, path: PathBuf) -> io::Result<()> {
        let content = fs::read_to_string(&path)?;
        self.lines = content.lines().map(|s| s.to_string()).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.filename = Some(path);
        self.modified = false;
        Ok(())
    }

    pub fn get_selected_text(&self) -> Option<String> {
        self.selection.as_ref().map(|sel| {
            let (start, end) = if sel.start <= sel.end {
                (&sel.start, &sel.end)
            } else {
                (&sel.end, &sel.start)
            };

            if start.line == end.line {
                self.lines[start.line][start.column..end.column].to_string()
            } else {
                let mut text = String::new();
                text.push_str(&self.lines[start.line][start.column..]);
                text.push('\n');
                
                for line in &self.lines[start.line + 1..end.line] {
                    text.push_str(line);
                    text.push('\n');
                }
                
                text.push_str(&self.lines[end.line][..end.column]);
                text
            }
        })
    }

    pub fn delete_selection(&mut self) -> Option<String> {
        self.selection.take().map(|sel| {
            let (start, end) = if sel.start <= sel.end {
                (sel.start, sel.end)
            } else {
                (sel.end, sel.start)
            };

            let text = self.get_selected_text().unwrap_or_default();

            if start.line == end.line {
                let line = &mut self.lines[start.line];
                line.replace_range(start.column..end.column, "");
            } else {
                // Keep first part of start line
                let start_line = self.lines[start.line][..start.column].to_string();
                // Keep last part of end line
                let end_line = self.lines[end.line][end.column..].to_string();
                
                // Remove lines in between
                self.lines.drain(start.line + 1..=end.line);
                
                // Join remaining parts
                self.lines[start.line] = start_line + &end_line;
            }

            self.cursor = start;
            self.modified = true;
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
        if let Some(selection) = &mut self.selection {
            selection.end = self.cursor.clone();
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: Cursor::default(),
            modified: false,
            filename: None,
            selection: None,
            extension: None,
        }
    }
}
