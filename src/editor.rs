use crate::buffer::Buffer;
use crate::ui::UI;
use crate::error::{Result, EditorError};
use crossterm::event::{KeyCode, KeyModifiers};
use crate::ui::DialogResult;
use crate::buffer::CursorMove;
use crate::ui::MenuOption;
use crate::ui::InputMode;
use std::path::PathBuf;

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
        self.ui.clear_error(); // Clear any previous error
        
        if let Some(key_event) = self.ui.get_key()? {
            match self.ui.get_input_mode() {
                InputMode::Normal => {
                    match (key_event.code, key_event.modifiers) {
                        // Exit to menu
                        (KeyCode::Esc, _) => self.ui.show_menu(),
                        
                        // Control key shortcuts
                        (KeyCode::Char(c), m) if m.contains(KeyModifiers::CONTROL) => {
                            match c {
                                'q' | 'Q' => self.quit = true,
                                's' | 'S' => self.handle_save()?,  // Changed from O to S
                                'o' | 'O' => self.ui.show_load_dialog(), // Changed from R to O
                                't' | 'T' => self.buffer.new_tab(),
                                'n' | 'N' => self.buffer.next_tab(),
                                'p' | 'P' => self.buffer.prev_tab(),
                                'w' | 'W' => {
                                    if !self.buffer.close_tab() {
                                        self.quit = true;
                                    }
                                }
                                'h' | 'H' => self.ui.toggle_hints(),
                                _ => (),
                            }
                        },

                        // Regular typing
                        (KeyCode::Char(c), m) if m.is_empty() => self.buffer.insert_char(c),
                        
                        // Selection mode with shift
                        (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down, m) 
                        if m.contains(KeyModifiers::SHIFT) => {
                            self.buffer.start_selection();
                            match key_event.code {
                                KeyCode::Left => self.buffer.move_cursor(CursorMove::Left),
                                KeyCode::Right => self.buffer.move_cursor(CursorMove::Right),
                                KeyCode::Up => self.buffer.move_cursor(CursorMove::Up),
                                KeyCode::Down => self.buffer.move_cursor(CursorMove::Down),
                                _ => unreachable!(),
                            }
                            self.buffer.update_selection();
                        },

                        // Copy/Cut/Paste
                        (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CONTROL) => {
                            if let Some(text) = self.buffer.get_selected_text() {
                                self.ui.copy_to_clipboard(&text)?;
                                self.buffer.clear_selection();
                            }
                        },
                        (KeyCode::Char('x'), m) if m.contains(KeyModifiers::CONTROL) => {
                            if let Some(text) = self.buffer.delete_selection() {
                                self.ui.copy_to_clipboard(&text)?;
                            }
                        },
                        (KeyCode::Char('v'), m) if m.contains(KeyModifiers::CONTROL) => {
                            if let Some(text) = self.ui.paste_from_clipboard()? {
                                self.buffer.clear_selection();
                                for c in text.chars() {
                                    self.buffer.insert_char(c);
                                }
                            }
                        },

                        // Regular cursor movement and basic keys
                        (KeyCode::Left, m) if m.is_empty() => self.buffer.move_cursor(CursorMove::Left),
                        (KeyCode::Right, m) if m.is_empty() => self.buffer.move_cursor(CursorMove::Right),
                        (KeyCode::Up, m) if m.is_empty() => self.buffer.move_cursor(CursorMove::Up),
                        (KeyCode::Down, m) if m.is_empty() => self.buffer.move_cursor(CursorMove::Down),
                        (KeyCode::Enter, _) => self.buffer.insert_char('\n'),
                        (KeyCode::Tab, _) => self.buffer.insert_char('\t'),
                        (KeyCode::Backspace, _) => self.buffer.delete_char(),
                        (KeyCode::PageUp, _) => {
                            let page_size = self.ui.get_page_size();
                            self.buffer.move_cursor(CursorMove::PageUp(page_size));
                        },
                        (KeyCode::PageDown, _) => {
                            let page_size = self.ui.get_page_size();
                            self.buffer.move_cursor(CursorMove::PageDown(page_size));
                        },
                        _ => (), // Ignore other keys
                    }
                },
                InputMode::Menu => {
                    match key_event.code {
                        KeyCode::Esc => self.ui.hide_menu(),
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
                            self.ui.hide_menu();  // Hide menu after selection
                        }
                        KeyCode::Char(c) => {
                            if let Some(digit) = c.to_digit(10) {
                                if digit > 0 && digit <= 8 {
                                    self.ui.menu_goto(digit as usize - 1);
                                    // Auto-execute the selected menu item
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
                                    self.ui.hide_menu();
                                }
                            }
                        }
                        _ => (), // Ignore other keys when menu is active
                    }
                }
                InputMode::Dialog => {
                    match self.ui.handle_dialog(key_event.code) {
                        DialogResult::Save(path) => {
                            if let Err(e) = self.buffer.save(Some(path.clone())) {
                                self.ui.set_error(format!("Failed to save {}: {}", path.display(), e));
                            }
                        }
                        DialogResult::Load(path) => {
                            if let Err(e) = self.buffer.load(path.clone()) {
                                self.ui.set_error(format!("Failed to load {}: {}", path.display(), e));
                            }
                        }
                        DialogResult::Cancel | DialogResult::None => {}
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
                self.ui.set_error(format!("Failed to save: {}", e));
            }
        } else {
            self.ui.show_save_dialog();
        }
        Ok(())
    }

    pub fn load_file(&mut self, path: &str) -> Result<()> {
        // Create a new tab for each file except the first one
        if !self.buffer.is_empty() {
            self.buffer.new_tab();
        }
        self.buffer.load(PathBuf::from(path))
            .map_err(EditorError::from)
    }
}
