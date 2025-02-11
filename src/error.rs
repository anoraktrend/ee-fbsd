use thiserror::Error;

pub type Result<T> = std::result::Result<T, EditorError>;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
