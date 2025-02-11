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

pub enum CursorMove {
    Left,
    Right,
    Up,
    Down,
    PageUp(usize),
    PageDown(usize),
}
