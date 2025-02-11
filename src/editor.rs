use ncurses::*;
use crate::{EeState, text::TextBuffer, window::Windows};

pub struct Editor<'a> {
    pub edit: bool,
    pub state: &'a mut EeState,
    pub windows: Windows,
    pub text: TextBuffer,
    pub input_file: Option<String>,
    pub case_sensitive: bool,
    pub expand_tabs: bool, 
    pub info_window: bool,
    pub margins: bool,
    pub auto_format: bool,
    pub eight_bit: bool,
    pub right_margin: i32,
    pub current_line: i32,
    pub current_col: i32,
    pub screen_cols: i32,
    pub screen_lines: i32,
}

impl<'a> Editor<'a> {
    pub fn new(state: &'a mut EeState) -> Self {
        Self {
            edit: true,
            state,
            windows: Windows::new(),
            text: TextBuffer::new(),
            input_file: None,
            case_sensitive: false,
            expand_tabs: true,
            info_window: true,
            margins: true,
            auto_format: false,
            eight_bit: true,
            right_margin: 72,
            current_line: 0,
            current_col: 0,
            screen_cols: 0,
            screen_lines: 0,
        }
    }

    pub fn get_options(&mut self) {
        // Parse command line arguments
        for arg in std::env::args().skip(1) {
            if arg.starts_with('+') {
                // Go to line number
                if let Ok(line) = arg[1..].parse::<i32>() {
                    self.current_line = line.max(0);
                }
            } else {
                // Treat as input file
                self.input_file = Some(arg);
                self.read_file();
            }
        }
    }

    pub fn set_up_term(&mut self) {
        if (!self.state.initialized) {
            initscr();
            raw();
            keypad(stdscr(), true);
            noecho();
            self.state.initialized = true;
        }

        // Setup dimensions 
        self.screen_lines = LINES();
        self.screen_cols = COLS();

        // Create windows
        self.windows.setup(self.info_window);
    }

    pub fn process_input(&mut self) {
        // Get input
        let ch = getch();

        // Handle resize
        self.check_resize();

        // Process keystroke
        match ch {
            // Control keys
            KEY_BACKSPACE | 127 => self.delete_char(),
            KEY_DC => self.delete_char(),
            KEY_LEFT => self.cursor_left(),  
            KEY_RIGHT => self.cursor_right(),
            KEY_UP => self.cursor_up(),
            KEY_DOWN => self.cursor_down(),
            
            // Menu/commands
            27 => self.show_menu(), // ESC
            
            // Regular characters
            _ if ch > 31 && ch < 127 => {
                self.insert_char(ch as u8 as char);
            }
            
            _ => {} // Ignore other keys
        }

        // Update display
        self.refresh();
    }
    
    pub fn delete_char(&mut self) {
        if self.current_col > 0 {
            self.text.delete_at_cursor(self.current_line as usize, (self.current_col - 1) as usize);
            self.current_col -= 1;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.text.insert_at_cursor(
            self.current_line as usize,
            self.current_col as usize,
            ch
        );
        self.current_col += 1;
    }
    
    pub fn cursor_left(&mut self) {
        if self.current_col > 0 {
            self.current_col -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        let line_len = self.text.line_length(self.current_line as usize) as i32;
        if self.current_col < line_len {
            self.current_col += 1;
        }
    }

    pub fn cursor_up(&mut self) {
        if self.current_line > 0 {
            self.current_line -= 1;
            let line_len = self.text.line_length(self.current_line as usize) as i32;
            self.current_col = self.current_col.min(line_len);
        }
    }

    pub fn cursor_down(&mut self) {
        let line_count = self.text.line_count() as i32;
        if self.current_line < line_count - 1 {
            self.current_line += 1;
            let line_len = self.text.line_length(self.current_line as usize) as i32;
            self.current_col = self.current_col.min(line_len);
        }
    }

    pub fn show_menu(&mut self) {
        let mut menu = crate::menu::create_main_menu();
        if let Some(choice) = menu.show() {
            match choice {
                1 => self.edit = false, // Leave Editor
                2 => self.show_help(),
                3 => self.show_file_menu(),
                4 => self.redraw_screen(),
                5 => self.show_settings(),
                6 => self.show_search_menu(),
                7 => self.show_misc_menu(),
                _ => {}
            }
        }
        self.redraw_screen();
    }

    pub fn refresh(&mut self) {
        self.windows.refresh();
    }

    fn check_resize(&mut self) {
        let new_lines = LINES();
        let new_cols = COLS();
        
        if new_lines != self.screen_lines || new_cols != self.screen_cols {
            self.screen_lines = new_lines;
            self.screen_cols = new_cols;
            self.windows.resize(self.info_window);
            self.refresh();
        }
    }

    pub fn display_text(&self) {
        let mut y = 0;
        
        // Clear screen
        wclear(stdscr());
        
        // Draw text content
        for line in self.text.lines() {
            if y >= self.screen_lines - 1 {
                break;
            }
            mvaddstr(y, 0, line);
            y += 1;
        }

        // Draw status line
        self.draw_status_line();
        
        // Position cursor
        mv(self.current_line, self.current_col);
        refresh();
    }

    fn draw_status_line(&self) {
        let line = self.screen_lines - 1;
        mvaddch(line, 0, ACS_HLINE());
        
        // Show file name if exists
        if let Some(ref name) = self.input_file {
            mvaddstr(line, 2, &format!("File: {}", name));
        }

        // Show position
        let pos_str = format!("Line: {} Col: {}", self.current_line + 1, self.current_col + 1);
        mvaddstr(line, self.screen_cols - pos_str.len() as i32 - 2, &pos_str);
    }

    fn show_help(&self) {
        let mut help = crate::menu::Menu::new("Help");
        help.add_item("ESC - Main Menu", 1, false);
        help.add_item("Arrow keys - Move cursor", 2, false);
        help.add_item("Backspace/Delete - Delete character", 3, false);
        help.add_item("Enter - New line", 4, false);
        help.show();
    }

    fn show_file_menu(&mut self) {
        let mut menu = crate::menu::create_file_menu();
        if let Some(choice) = menu.show() {
            match choice {
                1 => self.read_file(),
                2 => self.write_file(),
                3 => self.save_file(),
                4 => self.print_buffer(),
                _ => {}
            }
        }
    }

    fn show_settings(&mut self) {
        let mut menu = crate::menu::Menu::new("Settings");
        menu.add_item(&format!("Case Sensitive: {}", if self.case_sensitive {"Yes"} else {"No"}), 1, true);
        menu.add_item(&format!("Expand Tabs: {}", if self.expand_tabs {"Yes"} else {"No"}), 2, true);
        menu.add_item(&format!("Info Window: {}", if self.info_window {"Yes"} else {"No"}), 3, true);
        menu.add_item(&format!("Margins: {}", if self.margins {"Yes"} else {"No"}), 4, true);
        
        if let Some(choice) = menu.show() {
            match choice {
                1 => self.case_sensitive = !self.case_sensitive,
                2 => self.expand_tabs = !self.expand_tabs,
                3 => {
                    self.info_window = !self.info_window;
                    self.windows.setup(self.info_window);
                },
                4 => self.margins = !self.margins,
                _ => {}
            }
        }
    }

    fn show_search_menu(&mut self) {
        let mut menu = crate::menu::create_search_menu();
        if let Some(choice) = menu.show() {
            match choice {
                1 => self.search_forward(),
                2 => self.continue_search(),
                _ => {}
            }
        }
    }

    fn show_misc_menu(&mut self) {
        // TODO: Implement misc menu
    }

    fn redraw_screen(&mut self) {
        clear();
        self.display_text();
        refresh();
    }

    fn read_file(&mut self) {
        if let Some(ref filename) = self.input_file {
            if let Ok(content) = std::fs::read_to_string(filename) {
                self.text = TextBuffer::new();
                for line in content.lines() {
                    self.text.insert_line(line.to_string());
                }
                self.current_line = 0;
                self.current_col = 0;
            }
        }
    }

    fn write_file(&mut self) {
        if let Some(ref filename) = self.input_file {
            let content: String = self.text.lines()
                .join("\n");
            let _ = std::fs::write(filename, content);
        }
    }

    fn save_file(&mut self) {
        self.write_file();
    }

    fn print_buffer(&mut self) {
        // Print to terminal
        let content: String = self.text.lines()
            .join("\n");
        println!("{}", content);
    }

    fn search_forward(&mut self) {
        // Implement simple search
        let mut menu = crate::menu::Menu::new("Search");
        menu.add_item("Enter search term:", 1, false);
        menu.show();
        // TODO: Get search input and find matches
    }

    fn continue_search(&mut self) {
        // Continue from last search position
        // TODO: Implement search continuation
    }
}
