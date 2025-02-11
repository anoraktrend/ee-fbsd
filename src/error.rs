use thiserror::Error;

pub type Result<T> = std::result::Result<T, EditorError>;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Failed to initialize terminal UI")]
    UiInit,
    
    #[error("Failed to read file: {0}")]
    FileRead(String),
    
    #[error("Failed to write file: {0}")]
    FileWrite(String),
}
