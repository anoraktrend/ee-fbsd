pub mod buffer;
pub mod cursor;
pub mod tab;

pub use buffer::Buffer;
pub use cursor::{Cursor, CursorMove, Selection};
pub use tab::Tab;

use crate::config::Config;
use crate::error::{Result, EditorError};
use std::path::Path;

pub struct Editor {
    buffer: Buffer,
    config: Config,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            buffer: Buffer::new(),
            config: Config::load(),
        })
    }

    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, _path: P) -> Result<()> {
        // TODO: Implement file loading
        // For now just return Ok to get the code compiling
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        // TODO: Implement main editor loop
        // For now just return Ok to get the code compiling
        Ok(())
    }
}
