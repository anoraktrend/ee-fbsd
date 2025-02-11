use crate::buffer::Buffer;
use crate::ui::UI;
use crate::error::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};  // Remove unused imports
use crate::ui::DialogResult;
use crate::buffer::CursorMove;
use crate::ui::MenuOption;
use crate::ui::InputMode;

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
        if let Some(event) = self.ui.get_key()? {
            match self.ui.get_input_mode() {
                InputMode::Normal => {
                    match event.code {
                        KeyCode::Esc => self.ui.set_input_mode(InputMode::WaitingForControl),
                        KeyCode::Left => self.buffer.move_cursor(CursorMove::Left),
                        KeyCode::Right => self.buffer.move_cursor(CursorMove::Right),
                        KeyCode::Up => self.buffer.move_cursor(CursorMove::Up),
                        KeyCode::Down => self.buffer.move_cursor(CursorMove::Down),
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
                InputMode::WaitingForControl => {
                    match event {
                        KeyEvent {
                            code: KeyCode::Char(c),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } => {
                            match c {
                                'X' | 'x' => self.quit = true,
                                'O' | 'o' => self.ui.show_save_dialog(),
                                'R' | 'r' => self.ui.show_load_dialog(),
                                'T' | 't' => self.buffer.new_tab(),
                                'W' | 'w' => {
                                    if !self.buffer.close_tab() {
                                        self.quit = true;
                                    }
                                }
                                'N' | 'n' => self.buffer.next_tab(),
                                'P' | 'p' => self.buffer.prev_tab(),
                                _ => self.ui.set_input_mode(InputMode::Normal),
                            }
                        }
                        KeyEvent { code: KeyCode::Esc, .. } => {
                            self.ui.toggle_menu();
                        }
                        _ => self.ui.set_input_mode(InputMode::Normal),
                    }
                }
                InputMode::Menu => {
                    match event.code {
                        KeyCode::Up => self.ui.menu_prev(),
                        KeyCode::Down => self.ui.menu_next(),
                        KeyCode::Enter => {
                            match self.ui.menu_select() {
                                Some(MenuOption::Exit) => self.quit = true,
                                Some(MenuOption::Save) => self.handle_save()?,
                                Some(MenuOption::SaveAs) => self.ui.show_save_dialog(),
                                Some(MenuOption::Read) => self.ui.show_load_dialog(),
                                Some(MenuOption::Goto) => { /* TODO */ },
                                Some(MenuOption::Find) => { /* TODO */ },
                                Some(MenuOption::Replace) => { /* TODO */ },
                                Some(MenuOption::Help) => { /* TODO */ },
                                None => (),
                            }
                        }
                        KeyCode::Char(c) => {
                            if let Some(digit) = c.to_digit(10) {
                                if digit > 0 && digit <= 8 {
                                    self.ui.menu_goto(digit as usize - 1);
                                }
                            }
                        }
                        KeyCode::Esc => self.ui.toggle_menu(),
                        _ => (), // Ignore other keys when menu is active
                    }
                }
                InputMode::Dialog => {
                    match self.ui.handle_dialog(event.code) {
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
                }
            }
        }
        self.ui.refresh(&self.buffer)?;
        Ok(())
    }

    fn handle_save(&mut self) -> Result<()> {
        if self.buffer.get_filename().is_some() {
            if let Err(e) = self.buffer.save(None) {
                eprintln!("Failed to save: {}", e);
            }
        } else {
            self.ui.show_save_dialog();
        }
        Ok(())
    }
}
