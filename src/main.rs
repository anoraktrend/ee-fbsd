mod editor;
mod ui;
mod buffer;
mod config;
mod ffi;
mod error;

use editor::Editor;
use error::Result;

fn main() -> Result<()> {
    let mut editor = Editor::new()?;
    editor.run()?;
    Ok(())
}
