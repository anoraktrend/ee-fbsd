use ncurses::*;
use std::io;
use crate::{EeState, config::EditorConfig, ui::UI};
use crate::buffer::{Buffer, BufferCollection};
use crate::text::{TextProcessor, ThemeConfig};
use crate::menu::MenuAction;
use crate::error::{Result, EditorError};

pub struct Editor<'a> {
    pub edit: bool,
    pub state: &'a mut EeState,
    pub ui: UI,
    pub buffers: BufferCollection,
    pub input_file: Option<String>,
    pub case_sensitive: bool,
    pub expand_tabs: bool, 
    pub info_window: bool,
    pub margins: bool,
    pub current_line: i32,
    pub current_col: i32,
    pub screen_cols: i32,
    pub screen_lines: i32,
    search_term: Option<String>,
    search_position: Option<(i32, i32)>,
    config: EditorConfig,
    clipboard: Option<String>,
    text_processor: TextProcessor,
    theme_config: ThemeConfig,
}

impl<'a> Editor<'a> {
    pub fn new(state: &'a mut EeState) -> Result<Self> {
        let config = EditorConfig::load();
        let ui = UI::new()?;
        Ok(Self {
            edit: true,
            state,
            ui,
            buffers: BufferCollection::new(),
            input_file: None,
            case_sensitive: false,
            expand_tabs: true,
            info_window: true,
            margins: true,
            current_line: 0,
            current_col: 0,
            screen_cols: 0,
            screen_lines: 0,
            search_term: None,
            search_position: None,
            config,
            clipboard: None,
            text_processor: TextProcessor::new(),
            theme_config: TextProcessor::load_theme_config(),
        })
    }

    pub fn get_options(&mut self) {
        let files: Vec<_> = std::env::args()
            .skip(1)
            .collect();
            
        if !files.is_empty() {
            // Remove initial empty buffer
            self.buffers.remove_buffer(0);
            self.ui.windows.tab_bar.remove_tab(0);
            
            // Open each file in a new tab
            for filename in files {
                self.input_file = Some(filename);
                self.read_file();
            }
            
            // Activate first tab
            self.switch_to_buffer(0);
        }
    }

    pub fn set_up_term(&mut self) -> Result<()> {
        if !self.state.initialized {
            initscr();
            raw();
            keypad(stdscr(), true);
            noecho();
            
            // Initialize colors
            if !has_colors() {
                endwin();
                return Err(EditorError::TerminalInit);
            }
            
            start_color();
            use_default_colors();
            
            // Initialize color pairs for syntax highlighting
            for i in 0..=15 {
                init_pair(i as i16, i as i16, -1);
            }
            
            self.state.initialized = true;
        }

        // Setup dimensions 
        self.screen_lines = LINES();
        self.screen_cols = COLS();

        // Always show hints
        self.info_window = true;
        self.ui.windows.setup(true);
        
        // Initial display
        self.redraw_screen();
        Ok(())
    }

    pub fn process_input(&mut self) -> Result<()> {
        let ch = getch();
        self.check_resize();

        match ch {
            ch if ch == self.config.key_bindings.exit => {
                if self.check_unsaved_changes()? {
                    self.edit = false;
                }
            }
            ch if ch == self.config.key_bindings.write => self.write_file()?,
            ch if ch == self.config.key_bindings.search => {
                if let Some(_) = self.search_term.as_ref() {
                    self.continue_search();
                } else {
                    self.show_search_menu();
                }
            },
            ch if ch == self.config.key_bindings.menu => self.show_menu(),
            ch if ch == self.config.key_bindings.next => self.cursor_down(),
            ch if ch == self.config.key_bindings.prev => self.cursor_up(),
            ch if ch == self.config.key_bindings.cut => self.cut_line(),
            ch if ch == self.config.key_bindings.paste => self.paste_line(),
            KEY_BACKSPACE | 127 => self.delete_char(),
            KEY_DC => self.delete_char(),
            KEY_LEFT => self.cursor_left(),  
            KEY_RIGHT => self.cursor_right(),
            KEY_UP => self.cursor_up(),
            KEY_DOWN => self.cursor_down(),
            27 => self.show_menu(), // ESC
            _ if ch > 31 && ch < 127 => {
                self.insert_char(ch as u8 as char);
                self.buffers.get_active_mut().mark_modified();
                let _ = self.buffers.get_active().write_temp_file();
            }
            _ => {} // Ignore other keys
        }
        
        self.refresh()?;
        Ok(())
    }
    
    pub fn delete_char(&mut self) {
        if self.current_col > 0 {
            self.buffers.get_active_mut().delete_at_cursor(self.current_line as usize, (self.current_col - 1) as usize);
            self.current_col -= 1;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.buffers.get_active_mut().insert_at_cursor(
            self.current_line as usize,
            self.current_col as usize,
            ch
        );
        self.current_col += 1;
    }
    
    pub fn cursor_left(&mut self) {
        if self.current_col > 0 {
            self.current_col -= 1;
        } else if self.current_line > 0 {
            // Move to end of previous line
            self.current_line -= 1;
            self.current_col = self.buffers.get_active()
                .line_length(self.current_line as usize) as i32;
        }
    }

    pub fn cursor_right(&mut self) {
        let line_len = self.buffers.get_active().line_length(self.current_line as usize) as i32;
        if self.current_col < line_len {
            self.current_col += 1;
        } else if self.current_line < self.buffers.get_active().line_count() as i32 - 1 {
            // Move to start of next line
            self.current_line += 1;
            self.current_col = 0;
        }
    }

    pub fn cursor_up(&mut self) {
        if self.current_line > 0 {
            self.current_line -= 1;
            let line_len = self.buffers.get_active().line_length(self.current_line as usize) as i32;
            self.current_col = self.current_col.min(line_len);
        }
    }

    pub fn cursor_down(&mut self) {
        let line_count = self.buffers.get_active().line_count() as i32;
        if self.current_line < line_count - 1 {
            self.current_line += 1;
            let line_len = self.buffers.get_active().line_length(self.current_line as usize) as i32;
            self.current_col = self.current_col.min(line_len);
        }
    }

    pub fn show_menu(&mut self) {
        let mut menu = crate::menu::create_main_menu();
        if let Some(choice) = menu.show() {
            match choice {
                MenuAction::Save => { self.save_file().unwrap(); }
                MenuAction::SaveAs => { self.save_as(); }
                MenuAction::Exit => { self.edit = false; }
                MenuAction::NewFile => { self.new_file(); }
                MenuAction::CloseTab => { self.close_current_tab(); }
                MenuAction::NextTab => { self.next_tab(); }
                MenuAction::PrevTab => { self.prev_tab(); }
                _ => {}
            }
        }
        self.redraw_screen();
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.ui.windows.refresh_all();
        Ok(())
    }

    fn check_resize(&mut self) {
        let new_lines = LINES();
        let new_cols = COLS();
        
        if new_lines != self.screen_lines || new_cols != self.screen_cols {
            self.screen_lines = new_lines;
            self.screen_cols = new_cols;
            self.ui.windows.resize(self.info_window);
            self.refresh().unwrap();
        }
    }

    pub fn display_text(&self) {
        // Clear screen
        clear();
        
        // Refresh window components
        self.ui.windows.refresh_all();

        let start_y = 2; // Leave space for tab bar and hint bar
        let text_width = self.screen_cols as usize;
        let text_height = (self.screen_lines - start_y - 1) as usize; // Leave space for status line

        // Get file extension for syntax detection
        let syntax_name = self.detect_syntax();
        let wrapped = self.buffers.get_active().get_wrapped_lines(text_width);

        // Draw text with syntax highlighting
        self.draw_text_content(start_y, text_height, &wrapped, syntax_name);

        // Draw status line
        self.draw_status_line();

        // Position cursor
        let (visual_line, visual_col) = self.buffers.get_active()
            .get_visual_position(text_width, 
                               self.current_line as usize,
                               self.current_col as usize);
        mv(visual_line as i32 + start_y, visual_col as i32);
        
        // Refresh screen
        refresh();
    }

    fn detect_syntax(&self) -> &str {
        self.input_file.as_ref()
            .and_then(|f| std::path::Path::new(f).extension())
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext {
                "rs" => "Rust",
                "py" => "Python",
                "js" => "JavaScript",
                _ => "Plain Text"
            })
            .unwrap_or("Plain Text")
    }

    fn draw_text_content(&self, start_y: i32, text_height: usize, wrapped: &[String], syntax_name: &str) {
        for (i, line) in wrapped.iter().enumerate() {
            if i >= text_height {
                break;
            }

            let highlighted = self.text_processor.highlight_line(line, syntax_name);
            let mut current_x = 0;

            for (color_pair, text) in highlighted {
                attron(COLOR_PAIR(color_pair));
                if mvaddstr(i as i32 + start_y, current_x, &text) == ERR {
                    eprintln!("Failed to write text at ({}, {})", current_x, i + start_y as usize);
                }
                attroff(COLOR_PAIR(color_pair));
                current_x += text.len() as i32;
            }
        }
    }

    fn draw_status_line(&self) {
        let line = self.screen_lines - 1;
        mvaddch(line, 0, ACS_HLINE());
        
        // Show file name and status
        if let Some(ref name) = self.input_file {
            let status = if self.buffers.get_active().is_modified() {
                "Modified"
            } else {
                "Saved"
            };
            mvaddstr(line, 2, &format!("File: {} [{}]", name, status));
        }

        // Show position
        let pos_str = format!("Line: {} Col: {}", self.current_line + 1, self.current_col + 1);
        mvaddstr(line, self.screen_cols - pos_str.len() as i32 - 2, &pos_str);
    }

    #[cfg(feature = "full_ui")]
    pub fn show_help(&self) {
        let mut help = crate::menu::Menu::new("Help");
        help.add_item("ESC - Main Menu", MenuAction::None);
        help.add_item("Arrow keys - Move cursor", MenuAction::None);
        help.add_item("Backspace/Delete - Delete character", MenuAction::None);
        help.add_item("Enter - New line", MenuAction::None);
        help.show();
    }

    #[cfg(feature = "full_ui")]
    pub fn show_file_menu(&mut self) {
        let mut menu = crate::menu::create_file_menu();
        if let Some(choice) = menu.show() {
            match choice {
                MenuAction::Save => { self.save_file().unwrap(); }
                MenuAction::SaveAs => { self.save_as(); }
                MenuAction::Exit => { self.edit = false; }
                MenuAction::NewFile => { self.new_file(); }
                MenuAction::CloseTab => { self.close_current_tab(); }
                MenuAction::NextTab => { self.next_tab(); }
                MenuAction::PrevTab => { self.prev_tab(); }
                _ => {}
            }
        }
    }

    #[cfg(feature = "full_ui")]
    pub fn show_settings(&mut self) {
        let mut menu = crate::menu::Menu::new("Settings");
        menu.add_item(&format!("Case Sensitive: {}", if self.case_sensitive {"Yes"} else {"No"}), 
            MenuAction::ToggleSetting("case_sensitive".to_string()));
        menu.add_item(&format!("Expand Tabs: {}", if self.expand_tabs {"Yes"} else {"No"}),
            MenuAction::ToggleSetting("expand_tabs".to_string()));
        menu.add_item(&format!("Info Window: {}", if self.info_window {"Yes"} else {"No"}),
            MenuAction::ToggleSetting("info_window".to_string()));
        menu.add_item(&format!("Margins: {}", if self.margins {"Yes"} else {"No"}),
            MenuAction::ToggleSetting("margins".to_string()));
        menu.add_item("Keyboard Shortcuts", MenuAction::KeyBinding("shortcuts".to_string()));
        menu.add_item("Themes", MenuAction::KeyBinding("themes".to_string()));
        
        if let Some(action) = menu.show() {
            match action {
                MenuAction::ToggleSetting(setting) => match setting.as_str() {
                    "case_sensitive" => self.case_sensitive = !self.case_sensitive,
                    "expand_tabs" => self.expand_tabs = !self.expand_tabs,
                    "info_window" => {
                        self.info_window = !self.info_window;
                        self.ui.windows.setup(self.info_window);
                    },
                    "margins" => self.margins = !self.margins,
                    _ => {}
                },
                MenuAction::KeyBinding(binding) => match binding.as_str() {
                    "shortcuts" => self.show_keyboard_config(),
                    "themes" => self.show_theme_settings(),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn show_theme_settings(&mut self) {
        let mut menu = crate::menu::Menu::new("Theme Settings");
        for theme_name in self.theme_config.themes.keys() {
            let action = MenuAction::ToggleSetting(theme_name.clone());
            menu.add_item(
                &format!("{}{}", 
                    if theme_name == &self.theme_config.current_theme { "* " } else { "  " },
                    theme_name
                ),
                action
            );
        }

        if let Some(MenuAction::ToggleSetting(theme_name)) = menu.show() {
            // Handle theme selection
            if self.theme_config.themes.contains_key(&theme_name) {
                self.theme_config.current_theme = theme_name;
                self.apply_theme();
            }
        }
    }

    fn show_keyboard_config(&mut self) {
        let mut menu = crate::menu::create_keyboard_menu(&self.config);
        let mut updated = false;
        let mut dialog = crate::menu::InputDialog::new("", 40);
        
        while let Some(choice) = menu.show() {
            match choice {
                MenuAction::KeyExit => {
                    dialog.set_prompt("Press key for Exit");
                    if let Some(_) = dialog.show() {
                        let key = dialog.get_key();
                        if key > 0 {
                            self.config.key_bindings.exit = key;
                            updated = true;
                        }
                    }
                }
                MenuAction::KeyWrite => {
                    dialog.set_prompt("Press key for Write");
                    if let Some(_) = dialog.show() {
                        let key = dialog.get_key();
                        if key > 0 {
                            self.config.key_bindings.write = key;
                            updated = true;
                        }
                    }
                }
                MenuAction::KeySearch => {
                    dialog.set_prompt("Press key for Search");
                    if let Some(_) = dialog.show() {
                        let key = dialog.get_key();
                        if key > 0 {
                            self.config.key_bindings.search = key;
                            updated = true;
                        }
                    }
                }
                MenuAction::KeyMenu => {
                    dialog.set_prompt("Press key for Menu");
                    if let Some(_) = dialog.show() {
                        let key = dialog.get_key();
                        if key > 0 {
                            self.config.key_bindings.menu = key;
                            updated = true;
                        }
                    }
                }
                MenuAction::KeyNext => {
                    dialog.set_prompt("Press key for Next");
                    if let Some(_) = dialog.show() {
                        let key = dialog.get_key();
                        if key > 0 {
                            self.config.key_bindings.next = key;
                            updated = true;
                        }
                    }
                }
                MenuAction::KeyPrev => {
                    dialog.set_prompt("Press key for Previous");
                    if let Some(_) = dialog.show() {
                        let key = dialog.get_key();
                        if key > 0 {
                            self.config.key_bindings.prev = key;
                            updated = true;
                        }
                    }
                }
                MenuAction::KeyCut => {
                    dialog.set_prompt("Press key for Cut");
                    if let Some(_) = dialog.show() {
                        let key = dialog.get_key();
                        if key > 0 {
                            self.config.key_bindings.cut = key;
                            updated = true;
                        }
                    }
                }
                MenuAction::KeyPaste => {
                    dialog.set_prompt("Press key for Paste");
                    if let Some(_) = dialog.show() {
                        let key = dialog.get_key();
                        if key > 0 {
                            self.config.key_bindings.paste = key;
                            updated = true;
                        }
                    }
                }
                MenuAction::SaveConfig => {
                    if updated {
                        self.config.save().unwrap_or(());
                        self.ui.windows.refresh_hints(&self.config);
                    }
                    break;
                }
                _ => break
            }
            if updated {
                menu = crate::menu::create_keyboard_menu(&self.config);
            }
        }
        self.redraw_screen();
    }

    #[cfg(feature = "full_ui")]
    fn show_search_menu(&mut self) {
        let mut menu = crate::menu::create_search_menu();
        if let Some(choice) = menu.show() {
            match choice {
                MenuAction::Search => self.search_forward(),
                MenuAction::FindNext => self.continue_search(),
                MenuAction::Replace => {
                    // Get search term first if needed
                    if self.search_term.is_none() {
                        self.search_forward();
                    }
                    let mut dialog = crate::menu::InputDialog::new("Replace with:", 40);
                    if let Some(replace_text) = dialog.show() {
                        self.replace_text(replace_text);
                    }
                }
                _ => {}
            }
        }
        self.redraw_screen();
    }

    #[cfg(feature = "full_ui")]
    fn show_misc_menu(&mut self) {
        let mut menu = crate::menu::Menu::new("Misc Menu");
        menu.add_item("New Tab", MenuAction::NewFile);
        menu.add_item("Close Tab", MenuAction::CloseTab);
        menu.add_item("Next Tab", MenuAction::NextTab);
        menu.add_item("Previous Tab", MenuAction::PrevTab);
        
        if let Some(action) = menu.show() {
            match action {
                MenuAction::NewFile => self.new_file(),
                MenuAction::CloseTab => self.close_current_tab(),
                MenuAction::NextTab => self.next_tab(),
                MenuAction::PrevTab => self.prev_tab(),
                _ => {}
            }
        }
    }

    fn close_current_tab(&mut self) {
        let current = self.ui.windows.tab_bar.active;
        if self.buffers.is_modified(current) {
            if !self.check_unsaved_changes().unwrap() {
                return;
            }
        }
        
        self.buffers.remove_buffer(current);
        self.ui.windows.tab_bar.remove_tab(current);
        
        // Switch to new active buffer
        let new_active = self.ui.windows.tab_bar.active;
        self.switch_to_buffer(new_active);
        self.redraw_screen();
    }

    fn switch_to_buffer(&mut self, index: usize) {
        // Save current cursor position
        let current = self.buffers.get_active_mut();
        current.set_cursor(self.current_line as usize, self.current_col as usize);

        // Switch buffers and sync UI
        self.buffers.set_active(index);
        self.ui.windows.tab_bar.set_active(index);

        // Restore cursor position from new buffer
        let (line, col) = self.buffers.get_active().get_cursor();
        self.current_line = line as i32;
        self.current_col = col as i32;
        
        // Update input file reference
        self.input_file = self.buffers.get_active()
            .get_filename()
            .map(|p| p.to_string_lossy().into_owned());
    }

    fn next_tab(&mut self) {
        let current = self.ui.windows.tab_bar.active;
        let next = (current + 1) % self.ui.windows.tab_bar.tabs.len();
        self.switch_to_buffer(next);
        self.redraw_screen();
    }

    fn prev_tab(&mut self) {
        let current = self.ui.windows.tab_bar.active;
        let prev = if current == 0 {
            self.ui.windows.tab_bar.tabs.len() - 1
        } else {
            current - 1
        };
        self.switch_to_buffer(prev);
        self.redraw_screen();
    }

    fn redraw_screen(&mut self) {
        clear();
        self.display_text();
        refresh();
    }

    fn read_file(&mut self) {
        if let Some(ref filename) = self.input_file {
            if let Ok(idx) = self.buffers.open_file(filename) {
                if let Some(name) = self.buffers.get_filename(idx) {
                    let tab_name = std::path::Path::new(&name)
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "untitled".to_string());
                    self.ui.windows.tab_bar.add_tab(tab_name);
                    self.current_line = 0;
                    self.current_col = 0;
                }
            }
        }
    }

    pub fn write_file(&mut self) -> Result<()> {
        let current = self.ui.windows.tab_bar.active;
        let path = self.input_file.as_deref();
        
        match self.buffers.save_file(current, path) {
            Ok(_) => Ok(()),
            Err(e) => {
                if path.is_none() {
                    self.prompt_save_file();
                    Ok(())
                } else {
                    Err(EditorError::Io(e))
                }
            }
        }
    }

    fn prompt_save_file(&mut self) {
        let mut dialog = crate::menu::InputDialog::new("Save as:", 40);
        if let Some(filename) = dialog.show() {
            self.input_file = Some(filename);
            self.write_file().unwrap();
        }
    }

    #[cfg(feature = "full_ui")]
    fn draw_status_message(&self, msg: &str) {
        let line = self.screen_lines - 1;
        mvaddstr(line, 2, &format!("{:<40}", msg));
        refresh();
    }

    fn check_unsaved_changes(&mut self) -> Result<bool> {
        if self.buffers.get_active().is_modified() {
            let mut menu = crate::menu::Menu::new("Unsaved Changes");
            menu.add_item("Save changes?", MenuAction::SaveChanges);
            menu.add_item("Discard changes", MenuAction::DiscardChanges);
            menu.add_item("Cancel", MenuAction::Cancel);

            match menu.show() {
                Some(MenuAction::SaveChanges) => {
                    self.write_file()?;
                    Ok(true)
                }
                Some(MenuAction::DiscardChanges) => Ok(true),
                _ => Ok(false),
            }
        } else {
            Ok(true)
        }
    }

    fn save_file(&mut self) -> Result<()> {
        self.write_file()?;
        let _ = self.buffers.get_active().cleanup_temp_file();
        Ok(())
    }

    #[cfg(feature = "full_ui")]
    pub fn print_buffer(&mut self) -> io::Result<()> {
        let content = self.buffers.get_active().lines()
            .join("\n");
            
        // Create temp file for printing
        let tmp_path = std::env::temp_dir().join("ee_print.tmp");
        std::fs::write(&tmp_path, content)?;
        
        // Print via system command
        let status = std::process::Command::new("lpr")
            .arg(tmp_path)
            .status()?;
            
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Print command failed"
            ));
        }
        
        Ok(())
    }

    fn search_forward(&mut self) {
        let mut dialog = crate::menu::InputDialog::new("Search:", 40);
        if let Some(term) = dialog.show() {
            self.search_term = Some(term);
            self.search_position = Some((self.current_line, self.current_col));
            self.do_search();
        }
        self.redraw_screen();
    }

    #[cfg(feature = "full_ui")]
    fn continue_search(&mut self) {
        if self.search_term.is_some() {
            if let Some((line, col)) = self.search_position {
                self.current_line = line;
                self.current_col = col + 1;
                self.do_search();
            }
        }
        self.redraw_screen();
    }

    fn do_search(&mut self) {
        if let Some(ref term) = self.search_term {
            let start_line = self.current_line as usize;
            let start_col = self.current_col as usize;

            // Search from current position
            for (line_num, line) in self.buffers.get_active().lines()[start_line..].iter().enumerate() {
                let search_start = if line_num == 0 { start_col } else { 0 };
                if let Some(pos) = line[search_start..].find(term) {
                    self.current_line = (start_line + line_num) as i32;
                    self.current_col = (search_start + pos) as i32;
                    self.search_position = Some((self.current_line, self.current_col));
                    return;
                }
            }
        }
    }

    fn replace_text(&mut self, new_text: String) {
        if let Some(ref term) = self.search_term {
            // Get current line
            if let Some(mut line) = self.buffers.get_active()
                .get_line(self.current_line as usize)
                .cloned() 
            {
                // Replace at current position
                if let Some(pos) = line[self.current_col as usize..].find(term) {
                    let abs_pos = self.current_col as usize + pos;
                    line.replace_range(abs_pos..abs_pos+term.len(), &new_text);
                    
                    // Update buffer
                    self.buffers.get_active_mut()
                        .insert_line_at(self.current_line as usize, line);
                    self.buffers.get_active_mut()
                        .delete_line(self.current_line as usize + 1);
                    self.buffers.get_active_mut().mark_modified();
                }
            }
        }
    }

    pub fn cut_line(&mut self) {
        if let Some(line) = self.buffers.get_active().get_line(self.current_line as usize).cloned() {
            self.clipboard = Some(line.clone());
            self.buffers.get_active_mut().delete_line(self.current_line as usize);
            if self.current_line as usize >= self.buffers.get_active().line_count() {
                self.current_line = (self.buffers.get_active().line_count() - 1) as i32;
            }
            self.current_col = 0;
        }
    }

    pub fn paste_line(&mut self) {
        if let Some(ref line) = self.clipboard {
            self.buffers.get_active_mut().insert_line_at(self.current_line as usize, line.clone());
            self.current_line += 1;
            self.current_col = 0;
        }
    }

    fn new_file(&mut self) {
        if self.check_unsaved_changes().unwrap() {
            let buffer = Buffer::new();
            let idx = self.buffers.add_buffer_with(buffer);  // Using new method
            self.input_file = None;
            let tab_name = String::from("untitled");
            self.ui.windows.tab_bar.add_tab(tab_name);
            self.buffers.set_active(idx);
            self.ui.windows.tab_bar.set_active(idx);
            self.current_line = 0;
            self.current_col = 0;
            self.redraw_screen();
        }
    }

    // Fix the highlighted text handling
    pub fn draw_highlighted_text(&self, text: &str, start_y: i32, current_x: i32) -> io::Result<()> {
        let highlighted = self.text_processor.highlight_line(text, "text");
        for (i, (_style, text)) in highlighted.into_iter().enumerate() {
            if mvaddstr(i as i32 + start_y, current_x, &text) == ERR {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to write text"));
            }
        }
        Ok(())
    }

    // Add save_as implementation
    fn save_as(&mut self) {
        self.prompt_save_file();
    }

    pub fn apply_theme(&mut self) {
        if let Some(theme) = self.theme_config.themes.get(&self.theme_config.current_theme) {
            if let Err(e) = self.text_processor.set_theme(theme) {
                eprintln!("Failed to apply theme: {}", e);
            }
            self.redraw_screen();
        }
    }
}
