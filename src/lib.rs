pub mod buffer;
pub mod config;
pub mod editor;
pub mod error;
pub mod input;
pub mod syntax;
pub mod ui;

pub use buffer::Buffer;
pub use config::Config;
pub use editor::Editor;
pub use error::EditorError;
pub use input::{EditorCommand, process_input};
pub use syntax::SyntaxHighlighter;
pub use ui::{UI, Theme};