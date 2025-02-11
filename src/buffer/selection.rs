use super::cursor::Cursor;

#[derive(Clone, Default)]
pub struct Selection {
    pub start: Cursor,
    pub end: Cursor,
}
