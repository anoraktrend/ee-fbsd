use std::path::PathBuf;

pub struct Buffer {
    lines: Vec<String>,
    cursor: Cursor,
    filename: Option<PathBuf>,
    modified: bool,
    syntax_name: Option<String>,
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
            filename: None,
            modified: false,
            syntax_name: None,
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
        // ...existing code...
    }
    
    pub fn syntax_name(&self) -> Option<&str> {
        self.syntax_name.as_deref()
    }

    pub fn set_syntax_name(&mut self, name: Option<String>) {
        self.syntax_name = name;
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
    // Add other buffer manipulation methods...
}
