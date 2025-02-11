use crate::buffer::Buffer;
use crate::ui::UI;
use crate::error::Result;
use crossterm::event::KeyCode;
use crate::ui::DialogResult;
use crate::buffer::CursorMove;

pub struct Editor {
    buffer: Buffer,
    ui: UI,
    quit: bool,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            buffer: Buffer::new(),
            ui: UI::new()?,
            quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.quit {
            self.ui.refresh(&self.buffer)?;
            self.handle_input()?;
        }
        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if let Some(key) = self.ui.get_key()? {
            if self.ui.is_dialog_active() {
                match self.ui.handle_dialog(key) {
                    DialogResult::Save(path) => {
                        if let Err(e) = self.buffer.save(Some(path)) {
                            // TODO: Show error message
                            eprintln!("Failed to save: {}", e);
                        }
                    }
                    DialogResult::Load(path) => {
                        if let Err(e) = self.buffer.load(path) {
                            // TODO: Show error message
                            eprintln!("Failed to load: {}", e);
                        }
                    }
                    DialogResult::Cancel => {}
                    DialogResult::None => {}
                }
            } else if self.ui.is_menu_active() {
                match key {
                    KeyCode::Char('x') | KeyCode::Char('X') => self.quit = true,
                    KeyCode::Char('o') | KeyCode::Char('O') => self.ui.show_save_dialog(),
                    KeyCode::Char('r') | KeyCode::Char('R') => self.ui.show_load_dialog(),
                    KeyCode::Esc => self.ui.toggle_menu(),
                    _ => (), // Ignore other keys when menu is active
                }
            } else {
                match key {
                    KeyCode::Left => self.buffer.move_cursor(CursorMove::Left),
                    KeyCode::Right => self.buffer.move_cursor(CursorMove::Right),
                    KeyCode::Up => self.buffer.move_cursor(CursorMove::Up),
                    KeyCode::Down => self.buffer.move_cursor(CursorMove::Down),
                    KeyCode::Esc => self.ui.toggle_menu(),
                    KeyCode::Char('q') => self.quit = true,
                    KeyCode::Char(c) => self.buffer.insert_char(c),
                    KeyCode::Backspace => self.buffer.delete_char(),
                    KeyCode::Enter => self.buffer.insert_char('\n'),
                    KeyCode::Tab => self.buffer.insert_char('\t'),
                    KeyCode::PageUp => {
                        let page_size = self.ui.get_page_size();
                        self.buffer.move_cursor(CursorMove::PageUp(page_size));
                    }
                    KeyCode::PageDown => {
                        let page_size = self.ui.get_page_size();
                        self.buffer.move_cursor(CursorMove::PageDown(page_size));
                    }
                    _ => (), // Ignore other keys
                }
            }
        }
        self.ui.refresh(&self.buffer)?;
        Ok(())
    }
}
