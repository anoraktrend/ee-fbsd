use ncurses::*;
use crate::config::EditorConfig;

pub struct HintBar {
    window: WINDOW,
    hints: Vec<String>,
}

impl HintBar {
    pub fn new(cols: i32) -> Self {
        let width = cols;
        Self {
            window: newwin(1, width, LINES() - 1, 0),
            hints: Vec::new(),
        }
    }

    fn create_hints(config: &EditorConfig) -> Vec<String> {
        vec![
            format!("^{} Exit", char::from(config.key_bindings.exit as u8)),
            format!("^{} Write", char::from(config.key_bindings.write as u8)),
            format!("^{} Search", char::from(config.key_bindings.search as u8)),
            format!("^{} Menu", char::from(config.key_bindings.menu as u8)),
            format!("^{} Next", char::from(config.key_bindings.next as u8)),
            format!("^{} Prev", char::from(config.key_bindings.prev as u8)),
            format!("^{} Cut", char::from(config.key_bindings.cut as u8)),
            format!("^{} Paste", char::from(config.key_bindings.paste as u8)),
        ]
    }

    pub fn update_hints(&mut self, config: &EditorConfig) {
        self.hints = Self::create_hints(config);
    }

    pub fn refresh(&self) {
        werase(self.window);
        let hint_text = self.hints.join(" | ");
        mvwaddstr(self.window, 0, 1, &hint_text);
        wrefresh(self.window);
    }
}

pub struct TabBar {
    window: WINDOW,
    pub tabs: Vec<String>,
    pub active: usize,
}

impl TabBar {
    pub fn new(width: i32) -> Self {
        let win = newwin(1, width, 1, 0);
        Self {
            window: win,
            tabs: vec![String::from("untitled")],
            active: 0,
        }
    }

    pub fn add_tab(&mut self, name: String) -> usize {
        self.tabs.push(name);
        self.tabs.len() - 1
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active = index;
        }
    }

    pub fn remove_tab(&mut self, index: usize) {
        if index < self.tabs.len() && self.tabs.len() > 1 {
            self.tabs.remove(index);
            if self.active >= self.tabs.len() {
                self.active = self.tabs.len() - 1;
            }
        }
    }

    pub fn refresh(&self) {
        werase(self.window);
        wattron(self.window, A_REVERSE());
        
        let mut x = 0;
        for (i, tab) in self.tabs.iter().enumerate() {
            // Draw left border
            mvwaddch(self.window, 0, x, ACS_VLINE());
            x += 1;

            // Draw tab title
            if i == self.active {
                wattron(self.window, A_BOLD());
            }
            mvwaddstr(self.window, 0, x, tab);
            if i == self.active {
                wattroff(self.window, A_BOLD());
            }
            x += tab.len() as i32;

            // Draw right border
            mvwaddch(self.window, 0, x, ACS_VLINE());
            x += 2; // Add space between tabs
        }
        
        wattroff(self.window, A_REVERSE());
        wrefresh(self.window);
    }

    pub fn get_active_tab_name(&self) -> Option<&str> {
        self.tabs.get(self.active).map(|s| s.as_str())
    }

    pub fn rename_tab(&mut self, index: usize, name: String) {
        if index < self.tabs.len() {
            self.tabs[index] = name;
            self.refresh();
        }
    }

    pub fn get_tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn get_tab_name(&self, index: usize) -> Option<&str> {
        self.tabs.get(index).map(|s| s.as_str())
    }
}

pub struct Windows {
    pub main: WINDOW,
    pub hint_bar: Option<HintBar>,
    pub tab_bar: TabBar,
}

impl Windows {
    pub fn new(config: &EditorConfig) -> Self {
        let main = initscr();
        // Enable scrolling in main window
        scrollok(main, true);
        
        let mut windows = Self {
            main,
            hint_bar: Some(HintBar::new(COLS())),
            tab_bar: TabBar::new(COLS()),
        };
        
        // Initial setup
        windows.setup(true);
        
        // Initialize hint bar if present
        if let Some(ref mut hint_bar) = windows.hint_bar {
            hint_bar.update_hints(config);
        }

        windows
    }

    pub fn setup(&mut self, show_hints: bool) {
        // Calculate vertical positions
        let hint_height = if show_hints { 1 } else { 0 };
        let tab_y = hint_height;
        let main_y = hint_height + 1;
        
        // Position windows
        mvwin(self.tab_bar.window, tab_y, 0);
        mvwin(self.main, main_y, 0);
        
        // Resize main window to fill remaining space
        let main_height = LINES() - main_y - 1; // -1 for status line
        wresize(self.main, main_height, COLS());
    }

    pub fn resize(&mut self, show_hints: bool) {
        if show_hints {
            if let Some(ref _hint_bar) = self.hint_bar {
                // Recreate hint bar with new dimensions
                self.hint_bar = Some(HintBar::new(COLS()));
            }
        }
        self.tab_bar = TabBar::new(COLS());
        self.setup(show_hints);
    }

    pub fn refresh(&self) {
        if let Some(ref hint_bar) = self.hint_bar {
            hint_bar.refresh();
        }
        self.tab_bar.refresh();
        refresh();
    }

    pub fn refresh_hints(&mut self, config: &EditorConfig) {
        if let Some(ref mut hint_bar) = self.hint_bar {
            hint_bar.update_hints(config);
            hint_bar.refresh();
        }
    }

    pub fn refresh_all(&self) {
        // Clear all windows first
        werase(self.main);
        if let Some(ref hint_bar) = self.hint_bar {
            werase(hint_bar.window);
        }
        werase(self.tab_bar.window);
        
        // Refresh components in order
        if let Some(ref hint_bar) = self.hint_bar {
            hint_bar.refresh();
        }
        self.tab_bar.refresh();
        wrefresh(self.main);
        
        // Update physical screen
        doupdate();
    }

    // Add status line clearing
    pub fn clear_status_line(&self) {
        let line = LINES() - 1;
        mvhline(line, 0, ' ' as chtype, COLS());
        refresh();
    }
}

impl Drop for Windows {
    fn drop(&mut self) {
        delwin(self.main);
        endwin();
    }
}
