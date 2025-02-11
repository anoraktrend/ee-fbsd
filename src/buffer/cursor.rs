#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
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
