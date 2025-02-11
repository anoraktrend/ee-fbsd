use ncurses::*;
use std::cmp::max;

pub struct MenuItem {
    pub text: String,
    pub id: usize,
    pub enabled: bool,
}

pub struct Menu {
    pub title: String,
    pub items: Vec<MenuItem>,
    pub selected: usize,
    pub active: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Menu {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            items: Vec::new(),
            selected: 0,
            active: false,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn add_item(&mut self, text: &str, id: usize, enabled: bool) {
        self.items.push(MenuItem {
            text: text.to_string(),
            id,
            enabled,
        });
        
        // Update dimensions
        self.width = max(self.width, text.len() as i32 + 4);
        self.height = self.items.len() as i32 + 2;
    }

    pub fn show(&mut self) -> Option<usize> {
        let win = self.create_window();
        self.active = true;
        self.draw(win);

        loop {
            match getch() {
                KEY_UP => {
                    if self.selected > 0 {
                        self.selected -= 1;
                        self.draw(win);
                    }
                }
                
                KEY_DOWN => {
                    if self.selected < self.items.len() - 1 {
                        self.selected += 1; 
                        self.draw(win);
                    }
                }
                
                10 | KEY_ENTER => { // Enter
                    let id = self.items[self.selected].id;
                    self.cleanup(win);
                    return Some(id);
                }
                
                27 => { // Escape
                    self.cleanup(win);
                    return None;
                }
                
                _ => {}
            }
        }
    }

    fn create_window(&mut self) -> WINDOW {
        // Center the menu
        let cols = COLS();
        let lines = LINES();
        
        self.x = (cols - self.width) / 2;
        self.y = (lines - self.height) / 2;

        let win = newwin(self.height, self.width, self.y, self.x);
        keypad(win, true);
        win
    }

    fn draw(&self, win: WINDOW) {
        werase(win);
        box_(win, 0, 0);
        
        // Draw title
        mvwaddstr(win, 0, (self.width - self.title.len() as i32) / 2, 
                 &self.title);

        // Draw menu items
        for (i, item) in self.items.iter().enumerate() {
            let y = i as i32 + 1;
            let style = if !item.enabled {
                A_DIM()
            } else if i == self.selected {
                A_REVERSE()
            } else {
                A_NORMAL()
            };

            wattron(win, style);
            mvwaddstr(win, y, 2, &item.text);
            wattroff(win, style);
        }

        wrefresh(win);
    }

    fn cleanup(&mut self, win: WINDOW) {
        self.active = false;
        werase(win);
        wrefresh(win);
        delwin(win);
    }
}

// Static menu definitions
pub fn create_main_menu() -> Menu {
    let mut menu = Menu::new("Main Menu");
    menu.add_item("Leave Editor", 1, true);
    menu.add_item("Help", 2, true); 
    menu.add_item("File Operations", 3, true);
    menu.add_item("Redraw Screen", 4, true);
    menu.add_item("Settings", 5, true);
    menu.add_item("Search", 6, true);
    menu.add_item("Miscellaneous", 7, true);
    menu
}

pub fn create_file_menu() -> Menu {
    let mut menu = Menu::new("File Menu");
    menu.add_item("Read File", 1, true);
    menu.add_item("Write File", 2, true);
    menu.add_item("Save File", 3, true);
    menu.add_item("Print Buffer", 4, true);
    menu
}

pub fn create_search_menu() -> Menu {
    let mut menu = Menu::new("Search Menu");
    menu.add_item("Search For...", 1, true);
    menu.add_item("Continue Search", 2, true);
    menu
}
