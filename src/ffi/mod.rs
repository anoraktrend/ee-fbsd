use ncurses::*;

pub struct Terminal {
    window: WINDOW
}

impl Terminal {
    pub fn init() -> Option<Self> {
        initscr();
        raw();
        keypad(stdscr(), true);
        noecho();
        
        Some(Self {
            window: stdscr()
        })
    }

    pub fn get_key(&self) -> Option<i32> {
        let ch = wgetch(self.window);
        if ch == -1 {
            None
        } else {
            Some(ch)
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        endwin();
    }
}
