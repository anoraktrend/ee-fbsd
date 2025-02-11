mod highlight;
mod theme;

use std::io;
pub use highlight::highlight_content;
pub use theme::Theme;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::event::{self, Event, KeyCode};

pub type Result<T> = std::result::Result<T, io::Error>;

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    theme: theme::Theme,
}

impl UI {
    pub fn new() -> Result<Self> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self {
            terminal,
            theme: theme::Theme::default(),
        })
    }

    pub fn set_theme(&mut self, theme: theme::Theme) {
        self.theme = theme;
    }

    pub fn get_theme(&self) -> &theme::Theme {
        &self.theme
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.terminal.draw(|_| {
            // TODO: Implement drawing logic
        })?;
        Ok(())
    }

    pub fn get_key(&self) -> Result<Option<KeyCode>> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                return Ok(Some(key.code));
            }
        }
        Ok(None)
    }
}

pub fn highlight_buffer(buffer: &crate::buffer::Buffer) -> Vec<String> {
    let theme = Theme::default();
    highlight_content(buffer, &theme)
        .into_iter()
        .map(|spans| {
            spans.0.into_iter()
                .map(|span| span.content)
                .collect::<Vec<_>>()
                .join("")
        })
        .collect()
}
