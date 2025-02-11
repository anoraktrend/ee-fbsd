use thiserror::Error;

pub type Result<T> = std::result::Result<T, EditorError>;

/// Custom error types for the editor
#[derive(Error, Debug)]
pub enum EditorError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to initialize terminal UI")]
    TerminalInit,

    #[error("Failed to read file: {0}")]
    FileRead(String),

    #[error("Failed to write file: {0}")]
    FileWrite(String),
}
