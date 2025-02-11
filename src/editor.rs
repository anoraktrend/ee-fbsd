use crate::buffer::Buffer;
use crate::ui::UI;
use crate::config::Config;
use crate::error::Result;
use crossterm::event::KeyCode;

pub struct Editor {
    buffer: Buffer,
    ui: UI,
    config: Config,
    quit: bool,
    window: Window,
    case_sensitive: bool,
    expand_tabs: bool,
    info_window: bool,
    observe_margins: bool,
    auto_format: bool,
    eight_bit: bool,
    emacs_keys: bool,
    highlight: bool,
    right_margin: usize,
    text_changes: bool,
    restricted: bool,
}

pub struct Window {
    lines: usize,
    cols: usize,
    y: usize,
    x: usize,
    content: Vec<String>,
}

impl Window {
    pub fn new(lines: usize, cols: usize, y: usize, x: usize) -> std::io::Result<Window> {
        Ok(Window {
            lines,
            cols,
            y,
            x,
            content: Vec::with_capacity(lines),
        })
    }
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            buffer: Buffer::new(),
            ui: UI::new()?,
            config: Config::default(),
            quit: false,
            window: Window::new(24, 80, 0, 0)?,
            case_sensitive: false,
            expand_tabs: true,
            info_window: true,
            observe_margins: true,
            auto_format: false,
            eight_bit: true,
            emacs_keys: false,
            highlight: true,
            right_margin: 79,
            text_changes: false,
            restricted: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.quit {
            self.ui.refresh()?;
            self.handle_input()?;
        }
        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if let Some(key) = self.ui.get_key()? {
            match key {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char(c) => self.buffer.insert_char(c),
                KeyCode::Backspace => self.buffer.delete_char(),
                KeyCode::Enter => self.buffer.insert_char('\n'),
                KeyCode::Tab => self.buffer.insert_char('\t'),
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    // TODO: Handle cursor movement
                }
                _ => (), // Ignore other keys
            }
        }
        Ok(())
    }

    // Convert other methods to return Result
    // ...existing code...
}
