use crate::buffer::Buffer;
use crate::ui::UI;
use crate::config::Config;
use crate::error::Result;
use crossterm::event::KeyCode;

pub struct Editor {
    buffer: Buffer,
    ui: UI,
    config: Config,
    quit: bool,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            buffer: Buffer::new(),
            ui: UI::new()?,
            config: Config::default(),
            quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.quit {
            self.ui.refresh()?;
            self.handle_input()?;
        }
        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if let Some(key) = self.ui.get_key()? {
            match key {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char(c) => self.buffer.insert_char(c),
                KeyCode::Backspace => self.buffer.delete_char(),
                KeyCode::Enter => self.buffer.insert_char('\n'),
                KeyCode::Tab => self.buffer.insert_char('\t'),
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    // TODO: Handle cursor movement
                }
                _ => (), // Ignore other keys
            }
        }
        Ok(())
    }

    // Convert other methods to return Result
    // ...existing code...
}
