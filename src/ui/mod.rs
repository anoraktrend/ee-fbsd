mod dialog;
mod menu;
pub mod theme;  // Make theme module public

pub use dialog::{DialogResult, DialogType};
pub use menu::{Menu, MenuOption};
pub use theme::Theme;

use std::io;
use std::path::PathBuf;
use crate::buffer::Buffer;
use crate::syntax::SyntaxHighlighter;
use crate::config::Config;
use tui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph, Clear},
    text::{Spans, Span, Text},
    style::{Style, Modifier, Color},
};
use crossterm::event::{self, Event, KeyCode, MouseEvent};
use crossterm::{
    terminal::{Clear as TermClear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    ExecutableCommand,
};
use copypasta::{ClipboardContext, ClipboardProvider};

pub type Result<T> = std::result::Result<T, io::Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Menu,
    Dialog,
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    menu: Menu,
    dialog_input: String,
    dialog_type: Option<DialogType>,
    input_mode: InputMode,
    error_message: Option<String>,
    clipboard: ClipboardContext,
    show_hints: bool,
    syntax_highlighter: SyntaxHighlighter,
    config: Config,
}

impl UI {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        // Enable mouse support if configured
        if let Ok(()) = crossterm::execute!(
            std::io::stdout(),
            crossterm::event::EnableMouseCapture
        ) {}

        Ok(Self { 
            terminal,
            menu: Menu::new(),
            dialog_input: String::new(),
            dialog_type: None,
            input_mode: InputMode::Normal,
            error_message: None,
            show_hints: true,  // Initialize hints as visible
            clipboard: ClipboardContext::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
            syntax_highlighter: SyntaxHighlighter::new(),
            config: Config::load(),  // Load config instead of creating new Settings
        })
    }

    pub fn refresh(&mut self, buffer: &Buffer) -> Result<()> {
        // Pre-render content to avoid borrow checker issues
        let rendered_tabs = self.render_tabs(buffer);
        let rendered_content = self.render_editor_content(buffer);
        let show_hints = self.show_hints;
        let menu_active = self.menu.active;
        let menu_selection = self.menu.selection;
        let menu_in_settings = self.menu.in_settings;
        let dialog = self.dialog_type.clone();
        let dialog_input = self.dialog_input.clone();
        let error_msg = self.error_message.clone();
        let config = self.config.clone();

        self.terminal.draw(|f| {
            let area = f.size();

            // Use the pre-rendered content here without borrowing self
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    if show_hints {
                        vec![
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Min(1),
                            Constraint::Length(1),
                        ]
                    } else {
                        vec![
                            Constraint::Length(1),
                            Constraint::Min(1),
                            Constraint::Length(1),
                        ]
                    }
                )
                .split(area);

            // TODO: Implement actual drawing using pre-rendered content
            // and variables...
        })?;
        Ok(())
    }

    fn render_editor_content<'a>(&self, buffer: &'a Buffer) -> Vec<Spans<'a>> {
        buffer.lines().iter().enumerate().map(|(idx, line)| {
            let mut spans = vec![
                Span::styled(
                    format!("{:4} ", idx + 1),
                    Style::default().fg(Color::DarkGray)
                )
            ];

            // Get file extension for syntax highlighting
            let extension = buffer.current_tab().extension.as_deref();
            
            // Highlight the line
            let highlighted = self.syntax_highlighter.highlight_line(line, extension);
            
            if idx == buffer.get_cursor().line {
                // Handle cursor in highlighted text
                // TODO: Implement cursor positioning with highlighted text
                spans.extend(highlighted.into_iter().map(|(color, text)| {
                    Span::styled(text.to_string(), Style::default().fg(color))
                }));
            } else {
                spans.extend(highlighted.into_iter().map(|(color, text)| {
                    Span::styled(text.to_string(), Style::default().fg(color))
                }));
            }

            Spans::from(spans)
        }).collect()
    }

    pub fn get_input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    pub fn show_menu(&mut self) {
        self.menu.active = true;
        self.input_mode = InputMode::Menu;
        self.menu.selection = 0;
    }

    pub fn hide_menu(&mut self) {
        self.menu.active = false;
        self.input_mode = InputMode::Normal;
        self.menu.selection = 0;
    }

    pub fn get_key(&mut self) -> Result<Option<Event>> {  // Changed return type
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                event @ Event::Key(_) => Ok(Some(event)),
                event @ Event::Mouse(_) => Ok(Some(event)),
                Event::Resize(_, _) => {
                    self.terminal.backend_mut().execute(TermClear(ClearType::All))?;
                    Ok(None)
                }
                _ => Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn handle_dialog(&mut self, key: KeyCode) -> DialogResult {
        match key {
            KeyCode::Enter => {
                let path = PathBuf::from(&self.dialog_input);
                let result = match self.dialog_type {
                    Some(DialogType::Save) => DialogResult::Save(path),
                    Some(DialogType::Load) => DialogResult::Load(path),
                    None => DialogResult::None,
                };
                self.cleanup_dialog();
                result
            }
            KeyCode::Esc => {
                self.cleanup_dialog();
                DialogResult::Cancel
            }
            KeyCode::Char(c) => {
                self.dialog_input.push(c);
                DialogResult::None
            }
            KeyCode::Backspace => {
                self.dialog_input.pop();
                DialogResult::None
            }
            _ => DialogResult::None,
        }
    }

    fn cleanup_dialog(&mut self) {
        self.dialog_input.clear();
        self.dialog_type = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn show_save_dialog(&mut self) {
        self.dialog_input.clear();
        self.dialog_type = Some(DialogType::Save);
        self.menu.active = false;
        self.input_mode = InputMode::Dialog;
    }

    pub fn show_load_dialog(&mut self) {
        self.dialog_input.clear();
        self.dialog_type = Some(DialogType::Load);
        self.menu.active = false;
        self.input_mode = InputMode::Dialog;
    }

    pub fn get_page_size(&self) -> usize {
        let (_, height) = self.terminal.size()
            .map(|r| (r.width as usize, r.height as usize))
            .unwrap_or((80, 24));
        height.saturating_sub(3)
    }

    pub fn menu_select(&self) -> Option<MenuOption> {
        self.menu.select()
    }

    pub fn menu_next(&mut self) {
        self.menu.selection = (self.menu.selection + 1) % 10;
    }

    pub fn menu_prev(&mut self) {
        self.menu.selection = (self.menu.selection + 9) % 10;
    }

    pub fn menu_goto(&mut self, index: usize) {
        if index < 10 {
            self.menu.selection = index;
        }
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    // Make this a static method instead of an instance method
    fn calc_centered_rect(width: u16, height: u16, area: tui::layout::Rect) -> tui::layout::Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - height) / 2),
                Constraint::Length(height),
                Constraint::Min(0),
            ].as_ref())
            .split(area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - width) / 2),
                Constraint::Percentage(width),
                Constraint::Percentage((100 - width) / 2),
            ].as_ref())
            .split(vertical[1])[1]
    }

    pub fn copy_to_clipboard(&mut self, text: &str) -> Result<()> {
        self.clipboard.set_contents(text.to_owned())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn paste_from_clipboard(&mut self) -> Result<Option<String>> {
        self.clipboard.get_contents()
            .map(Some)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn toggle_hints(&mut self) {
        self.show_hints = !self.show_hints;
    }

    pub fn handle_mouse(&mut self, mouse_event: MouseEvent) -> Option<(usize, usize)> {
        if !self.config.get_settings().is_mouse_enabled() {
            return None;
        }

        let row = mouse_event.row as usize;
        let col = mouse_event.column as usize;

        // Handle UI element clicks
        if self.menu.active {
            self.handle_menu_click(row, col);
            return None;
        } else if self.show_hints && row == 0 {
            // Clicking anywhere on hints bar shows menu
            self.show_menu();
            return None;
        }

        // Default to cursor positioning in editor area
        Some((row, col))
    }

    fn handle_menu_click(&mut self, row: usize, col: usize) -> Option<MenuOption> {
        // Menu starts at row 2 (after hints and tabs)
        let menu_row = if self.show_hints { row.saturating_sub(2) } else { row.saturating_sub(1) };
        
        // Check if click is in menu area
        if col < 30 && menu_row < 10 {
            self.menu.selection = menu_row;
            let option = self.menu_select()?;
            
            if self.is_toggleable(&option) {
                self.toggle_setting(option.clone());
            }
            
            Some(option)
        } else {
            None
        }
    }

    pub fn toggle_setting(&mut self, option: MenuOption) -> bool {
        match option {
            MenuOption::MouseSupport => { 
                let result = self.config.get_settings_mut().toggle_mouse();
                self.config.save().ok();
                result
            }
            MenuOption::DosMode => { 
                let result = self.config.get_settings_mut().toggle_dos_mode();
                self.config.save().ok();
                result
            }
            MenuOption::AutoIndent => { 
                let result = self.config.get_settings_mut().toggle_auto_indent();
                self.config.save().ok();
                result
            }
            _ => false
        }
    }

    // Add helper method to check if menu item is toggleable
    pub fn is_toggleable(&self, option: &MenuOption) -> bool {
        Menu::is_toggleable(option)
    }

    pub fn handle_menu_option(&mut self, option: MenuOption) {
        match option {
            MenuOption::Settings => {
                self.menu.in_settings = true;
                self.menu.selection = 0;
            }
            MenuOption::Back => {
                self.menu.in_settings = false;
                self.menu.selection = 0;
            }
            MenuOption::Theme(name) => {
                self.config.set_theme(name);
            }
            _ => { /* handle other menu options as before */ }
        }
    }

    fn render_tabs(&self, buffer: &Buffer) -> Vec<Span> {
        let mut spans = Vec::new();

        for (idx, tab) in buffer.iter_tabs().enumerate() {
            let name = tab.get_filename()
                .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
                .unwrap_or_else(|| "[No Name]".to_string());

            let style = if idx == buffer.current_tab_index() {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            let modified = if tab.is_modified() { "+" } else { "" };
            spans.push(Span::styled(format!(" {} {} ", name, modified), style));
        }

        spans
    }

    pub fn draw(&mut self, buffer: &Buffer) -> std::io::Result<()> {
        let _rendered_tabs = self.render_tabs(buffer);
        let _rendered_content = self.render_editor_content(buffer);
        
        let _menu_active = self.menu.active;
        let _menu_selection = self.menu.selection;
        let _menu_in_settings = self.menu.in_settings;
        let _dialog = self.dialog_type.clone();
        let _dialog_input = self.dialog_input.clone();
        let _error_msg = self.error_message.clone();
        let _config = self.config.clone();

        self.terminal.draw(|f| {
            let _layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    if self.show_hints {
                        vec![
                            Constraint::Length(1),  // Status line
                            Constraint::Length(1),  // Tabs
                            Constraint::Min(1),     // Editor content
                            Constraint::Length(1),  // Command line
                        ]
                    } else {
                        vec![
                            Constraint::Length(1),  // Tabs
                            Constraint::Min(1),     // Editor content 
                            Constraint::Length(1),  // Command line
                        ]
                    }
                )
                .split(f.size());

                // Allow additional content to be rendered through dynamic dispatch
                if let Some(extension) = buffer.current_tab().extension.as_deref() {
                    if extension == "rs" {
                        // Add cargo-specific rendering here if needed
                        // Example: Show build status, cargo commands, etc
                    }
                }
        })?;

        Ok(())
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        // Disable mouse capture on exit
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::event::DisableMouseCapture
        );
        let mut stdout = std::io::stdout();
        let _ = stdout.execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
