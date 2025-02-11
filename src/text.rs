pub struct TextBuffer {
    lines: Vec<String>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()]
        }
    }

    pub fn insert_line(&mut self, text: String) {
        self.lines.push(text);
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line_length(&self, line: usize) -> usize {
        self.lines.get(line).map(|l| l.len()).unwrap_or(0)
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
}
