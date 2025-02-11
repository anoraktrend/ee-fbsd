use std::path::PathBuf;
use std::fs;
use std::io;
use super::cursor::{Cursor, CursorMove};  // Add CursorMove to import
use super::selection::Selection;

pub struct Tab {
    lines: Vec<String>,
    cursor: Cursor,
    modified: bool,
    filename: Option<PathBuf>,
    selection: Option<Selection>,
    pub extension: Option<String>, // Only extension needs to be public
}

impl Tab {
    // Add public getter methods
    pub fn get_lines(&self) -> &Vec<String> { &self.lines }
    pub fn get_lines_mut(&mut self) -> &mut Vec<String> { &mut self.lines }
    pub fn get_cursor(&self) -> &Cursor { &self.cursor }
    pub fn get_cursor_mut(&mut self) -> &mut Cursor { &mut self.cursor }
    pub fn is_modified(&self) -> bool { self.modified }
    pub fn set_modified(&mut self, modified: bool) { self.modified = modified; }
    pub fn get_filename(&self) -> Option<&PathBuf> { self.filename.as_ref() }
    pub fn get_selection(&self) -> Option<&Selection> { self.selection.as_ref() }
    pub fn get_selection_mut(&mut self) -> &mut Option<Selection> { &mut self.selection }

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

    pub fn save(&mut self, filename: Option<PathBuf>) -> io::Result<()> {
        let path = filename.as_ref().or(self.filename.as_ref())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No filename specified"))?;
        fs::write(path, self.lines.join("\n"))?;
        self.modified = false;
        if filename.is_some() {
            self.filename = filename;
        }
        Ok(())
    }

    pub fn load(&mut self, path: PathBuf) -> io::Result<()> {
        let content = fs::read_to_string(&path)?;
        self.lines = content.lines().map(String::from).collect();
        self.filename = Some(path);
        self.modified = false;
        Ok(())
    }

    pub fn move_cursor(&mut self, movement: CursorMove) {
        match movement {
            CursorMove::Left => {
                if self.cursor.column > 0 {
                    self.cursor.column -= 1;
                } else if self.cursor.line > 0 {
                    self.cursor.line -= 1;
                    self.cursor.column = self.lines[self.cursor.line].len();
                }
            },
            CursorMove::Right => {
                if self.cursor.column < self.lines[self.cursor.line].len() {
                    self.cursor.column += 1;
                } else if self.cursor.line < self.lines.len() - 1 {
                    self.cursor.line += 1;
                    self.cursor.column = 0;
                }
            },
            CursorMove::Up => {
                if self.cursor.line > 0 {
                    self.cursor.line -= 1;
                    self.clamp_cursor_column();
                }
            },
            CursorMove::Down => {
                if self.cursor.line < self.lines.len() - 1 {
                    self.cursor.line += 1;
                    self.clamp_cursor_column();
                }
            },
            CursorMove::PageUp(n) => {
                if self.cursor.line >= n {
                    self.cursor.line -= n;
                } else {
                    self.cursor.line = 0;
                }
                self.clamp_cursor_column();
            },
            CursorMove::PageDown(n) => {
                let max_line = self.lines.len().saturating_sub(1);
                if self.cursor.line.saturating_add(n) < max_line {
                    self.cursor.line += n;
                } else {
                    self.cursor.line = max_line;
                }
                self.clamp_cursor_column();
            }
        }
    }

    // Make clamp_cursor_column public
    pub fn clamp_cursor_column(&mut self) {
        if let Some(line) = self.lines.get(self.cursor.line) {
            self.cursor.column = self.cursor.column.min(line.len());
        }
    }

    pub fn get_selected_text(&self) -> Option<String> {
        self.selection.as_ref().map(|sel| {
            if sel.start.line == sel.end.line {
                self.lines[sel.start.line][sel.start.column..sel.end.column].to_string()
            } else {
                let mut result = String::new();
                // Add first line
                result.push_str(&self.lines[sel.start.line][sel.start.column..]);
                result.push('\n');
                // Add middle lines
                for line in &self.lines[sel.start.line + 1..sel.end.line] {
                    result.push_str(line);
                    result.push('\n');
                }
                // Add last line
                result.push_str(&self.lines[sel.end.line][..sel.end.column]);
                result
            }
        })
    }

    pub fn start_selection(&mut self) {
        // ...existing start_selection implementation...
    }

    pub fn update_selection(&mut self) {
        // ...existing update_selection implementation...
    }

    pub fn clear_selection(&mut self) {
        // ...existing clear_selection implementation...
    }

    pub fn delete_selection(&mut self) {
        if let Some(sel) = self.selection.take() {
            let _text = self.get_selected_text().unwrap(); // Mark as intentionally unused
            let start = sel.start;
            let end = sel.end;

            if start.line == end.line {
                self.lines[start.line].replace_range(start.column..end.column, "");
            } else {
                // Remove end line portion
                self.lines[end.line].replace_range(..end.column, "");
                // Remove middle lines
                self.lines.drain(start.line + 1..end.line);
                // Remove start line portion
                self.lines[start.line].replace_range(start.column.., "");
                // Join the lines
                let end_portion = self.lines.remove(start.line + 1);
                self.lines[start.line].push_str(&end_portion);
            }
            self.cursor = start;
            self.modified = true;
        }
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
