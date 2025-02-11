use ncurses::*;

pub struct Windows {
    main: WINDOW,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            main: initscr(),
        }
    }

    pub fn setup(&mut self, _show_info: bool) {
        // TODO: Implement window setup
    }

    pub fn resize(&mut self, _show_info: bool) {
        // TODO: Implement resize
    }

    pub fn refresh(&self) {
        refresh();
    }
}

impl Drop for Windows {
    fn drop(&mut self) {
        delwin(self.main);
        endwin();
    }
}
