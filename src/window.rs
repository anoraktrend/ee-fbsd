use ncurses::*;

pub struct HintBar {
    window: WINDOW,
    hints: Vec<String>,
}

impl HintBar {
    pub fn new(width: i32) -> Self {
        let win = newwin(1, width, 0, 0);
        wattron(win, A_REVERSE());
        Self {
            window: win,
            hints: vec![
                "^X Exit".to_string(),
                "^O Write".to_string(),
                "^W Search".to_string(),
                "ESC Menu".to_string(),
            ],
        }
    }

    pub fn refresh(&self) {
        werase(self.window);
        let hint_text = self.hints.join(" | ");
        mvwaddstr(self.window, 0, 1, &hint_text);
        wrefresh(self.window);
    }
}

pub struct Windows {
    pub main: WINDOW,
    pub hint_bar: Option<HintBar>,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            main: initscr(),
            hint_bar: None,
        }
    }

    pub fn setup(&mut self, show_hints: bool) {
        if show_hints {
            self.hint_bar = Some(HintBar::new(COLS()));
        }
    }

    pub fn resize(&mut self, show_hints: bool) {
        if show_hints {
            if let Some(ref _hint_bar) = self.hint_bar {
                // Recreate hint bar with new dimensions
                self.hint_bar = Some(HintBar::new(COLS()));
            }
        }
    }

    pub fn refresh(&self) {
        if let Some(ref hint_bar) = self.hint_bar {
            hint_bar.refresh();
        }
        refresh();
    }
}

impl Drop for Windows {
    fn drop(&mut self) {
        delwin(self.main);
        endwin();
    }
}
