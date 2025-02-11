use thiserror::Error;
use std::path::PathBuf;

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

    #[error("Invalid buffer index: {0}")]
    InvalidBuffer(usize),

    #[error("No active buffer")]
    NoActiveBuffer,

    #[error("Buffer is empty")]
    EmptyBuffer,

    #[error("Invalid cursor position: line {line}, column {column}")]
    InvalidCursor {
        line: usize,
        column: usize,
    },

    #[error("File already exists: {0}")]
    FileExists(PathBuf),

    #[error("No such file: {0}")]
    FileNotFound(PathBuf),

    #[error("Failed to parse config: {0}")]
    ConfigParse(String),

    #[error("Syntax highlighting error: {0}")]
    SyntaxError(String),

    #[error("Invalid key binding: {0}")]
    InvalidKeyBinding(String),

    #[error("Operation cancelled")]
    Cancelled,
}

impl EditorError {
    pub fn is_fatal(&self) -> bool {
        matches!(self, 
            EditorError::TerminalInit |
            EditorError::ConfigParse(_)
        )
    }

    pub fn is_user_error(&self) -> bool {
        matches!(self,
            EditorError::InvalidBuffer(_) |
            EditorError::InvalidCursor { .. } |
            EditorError::InvalidKeyBinding(_)
        )
    }

    pub fn requires_redraw(&self) -> bool {
        matches!(self,
            EditorError::FileRead(_) |
            EditorError::FileWrite(_) |
            EditorError::FileExists(_) |
            EditorError::FileNotFound(_)
        )
    }

    pub fn display_message(&self) -> String {
        match self {
            EditorError::FileRead(path) => format!("Could not read file: {}", path),
            EditorError::FileWrite(path) => format!("Could not write file: {}", path),
            EditorError::FileExists(path) => format!("File already exists: {}", path.display()),
            EditorError::FileNotFound(path) => format!("No such file: {}", path.display()),
            EditorError::InvalidKeyBinding(key) => format!("Invalid key binding: {}", key),
            _ => self.to_string()
        }
    }
}

impl From<serde_json::Error> for EditorError {
    fn from(err: serde_json::Error) -> Self {
        EditorError::ConfigParse(err.to_string())
    }
}

impl From<syntect::Error> for EditorError {
    fn from(err: syntect::Error) -> Self {
        EditorError::SyntaxError(err.to_string())
    }
}
