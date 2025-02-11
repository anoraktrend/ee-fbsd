// Represents a line of text in the buffer
pub struct TextLine {
    pub text: String,
    pub line_number: usize,
    pub next: Option<Box<TextLine>>,
    pub prev: Option<Box<TextLine>>,
}

impl TextLine {
    pub fn new(text: String, line_number: usize) -> Self {
        Self {
            text,
            line_number,
            next: None,
            prev: None,
        }
    }
}

// Main text buffer containing lines
pub struct TextBuffer {
    lines: Vec<String>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()]
        }
    }

    pub fn insert_char(&mut self, ch: char, pos: usize) {
        if let Some(line) = self.lines.get_mut(0) {
            line.insert(pos, ch);
        }
    }

    pub fn delete_char(&mut self, pos: usize) -> Option<char> {
        if let Some(line) = self.lines.get_mut(0) {
            if pos < line.len() {
                return Some(line.remove(pos));
            }
        }
        None
    }

    pub fn insert_line(&mut self, text: String) {
        self.lines.push(text);
    }

    pub fn delete_line(&mut self) -> Option<String> {
        if self.lines.len() > 1 {
            return self.lines.pop();
        }
        None
    }

    pub fn get_current_line(&self) -> Option<String> {
        self.lines.get(0).cloned()
    }
    
    pub fn get_line_number(&self) -> usize {
        1
    }

    pub fn next_line(&mut self) -> bool {
        false
    }

    pub fn prev_line(&mut self) -> bool {
        false
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn insert_at_cursor(&mut self, line: usize, col: usize, ch: char) {
        if let Some(line_str) = self.lines.get_mut(line) {
            if col <= line_str.len() {
                line_str.insert(col, ch);
            }
        }
    }

    pub fn delete_at_cursor(&mut self, line: usize, col: usize) -> Option<char> {
        if let Some(line_str) = self.lines.get_mut(line) {
            if col < line_str.len() {
                return Some(line_str.remove(col));
            }
        }
        None
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line_length(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.len()).unwrap_or(0)
    }
}
