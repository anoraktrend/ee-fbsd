use std::io;
use std::path::PathBuf;
use crate::buffer::Buffer;
use tui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph, Clear},
    text::{Spans, Span, Text},
    style::{Style, Modifier, Color},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent};
use crossterm::{
    terminal::{Clear as TermClear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    ExecutableCommand,
};
use copypasta::{ClipboardContext, ClipboardProvider};
use crate::syntax::SyntaxHighlighter;
use crate::settings::Settings;

pub type Result<T> = std::result::Result<T, io::Error>;

pub enum DialogResult {
    None,
    Save(PathBuf),
    Load(PathBuf),
    Cancel,
}

pub enum MenuOption {
    Exit,
    Save,
    SaveAs,
    Read,
    Goto,
    Find,
    Replace,
    Help,
    Settings,  // Add settings option
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Menu,
    Dialog,
    Settings,  // Add settings mode
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    menu_active: bool,
    dialog_input: String,
    dialog_type: Option<DialogType>,
    pub menu_selection: usize,
    input_mode: InputMode,
    error_message: Option<String>,
    clipboard: ClipboardContext,
    show_hints: bool,  // Add this field
    syntax_highlighter: SyntaxHighlighter,
    settings: Settings,
}

enum DialogType {
    Save,
    Load,
}

impl DialogType {
    fn title(&self) -> &str {
        match self {
            DialogType::Save => "Save",
            DialogType::Load => "Load",
        }
    }
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
            menu_active: false,
            dialog_input: String::new(),
            dialog_type: None,
            menu_selection: 0,
            input_mode: InputMode::Normal,
            error_message: None,
            show_hints: true,  // Initialize hints as visible
            clipboard: ClipboardContext::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
            syntax_highlighter: SyntaxHighlighter::new(),
            settings: Settings::new(),
        })
    }

    pub fn refresh(&mut self, buffer: &Buffer) -> Result<()> {
        // Store all values we need before the closure
        let menu_active = self.menu_active;
        let menu_selection = self.menu_selection;
        let dialog_type = self.dialog_type.as_ref();
        let dialog_input = &self.dialog_input;
        let error_message = &self.error_message;
        let editor_content = self.render_editor_content(buffer);

        self.terminal.draw(|f| {
            let size = f.size();

            // Create base layout with explicit type
            let constraints: Vec<Constraint> = if self.show_hints {
                vec![
                    Constraint::Length(1),  // hints bar
                    Constraint::Length(1),  // tab bar
                    Constraint::Min(1),     // main area
                    Constraint::Length(1),  // status bar
                ]
            } else {
                vec![
                    Constraint::Length(1),  // tab bar
                    Constraint::Min(1),     // main area
                    Constraint::Length(1),  // status bar
                ]
            };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(size);

            // Draw hints if enabled
            if self.show_hints {
                let hints_text = if menu_active {
                    "ESC=exit menu | ↑↓=select | ENTER=choose | 1-8=quick select"
                } else {
                    "ESC=menu | ^S=save | ^O=open | ^Q=exit | ^W=close | ^T=new tab | ^H=toggle hints"
                };
                let hints = Paragraph::new(hints_text)
                    .style(Style::default().add_modifier(Modifier::REVERSED));
                f.render_widget(hints, chunks[0]);
            }

            // Draw tab bar with boxed tabs
            let tabs_spans: Vec<Span> = buffer.tabs.iter().enumerate().flat_map(|(idx, tab)| {
                let name = tab.filename
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("untitled");
                let modified = if tab.modified { "+" } else { "" };
                
                let style = if idx == buffer.current_tab {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                
                vec![
                    Span::raw("│"), // Left border
                    Span::styled(" ", style),
                    Span::styled(format!("{}{}", name, modified), style),
                    Span::styled(" ", style),
                    Span::raw("│"), // Right border
                ]
            }).collect();

            let tabs = Paragraph::new(Spans::from(tabs_spans))
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(tabs, chunks[if self.show_hints { 1 } else { 0 }]);

            // Adjust main area index based on hints visibility
            let main_idx = if self.show_hints { 2 } else { 1 };
            let status_idx = if self.show_hints { 3 } else { 2 };

            // Main editor area with optional menu
            let main_area = if menu_active {
                // Split for menu
                let splits = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(30),  // menu width
                        Constraint::Min(1),      // editor width
                    ].as_ref())
                    .split(chunks[main_idx]);

                // Update menu items to include settings
                let menu_items = if self.input_mode == InputMode::Settings {
                    vec![
                        format!("Mouse support [{}]", if self.settings.is_mouse_enabled() { "x" } else { " " }),
                        format!("DOS mode [{}]", if self.settings.is_dos_mode() { "x" } else { " " }),
                        format!("Auto indent [{}]", if self.settings.is_auto_indent() { "x" } else { " " }),
                        "Back to menu".to_string(),
                    ]
                } else {
                    vec![
                        "leave editor".into(),
                        "save file".into(),
                        "save as".into(),
                        "read file".into(),
                        "goto line".into(),
                        "find string".into(),
                        "replace".into(),
                        "settings".into(),
                        "help".into(),
                    ]
                };

                // Draw menu
                let menu_text: Vec<Spans> = menu_items.iter().enumerate().map(|(idx, text)| {
                    let style = if idx == menu_selection {
                        Style::default().add_modifier(Modifier::REVERSED)
                    } else {
                        Style::default()
                    };
                    Spans::from(vec![
                        Span::styled(format!(" {:>2}. ", idx + 1), style),
                        Span::styled(text, style),
                    ])
                }).collect();

                let menu = Paragraph::new(Text::from(menu_text))
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title("Menu"));
                
                f.render_widget(menu, splits[0]);

                // Return editor area
                splits[1]
            } else {
                chunks[main_idx]
            };

            // Draw editor content - now using pre-rendered content
            let editor = Paragraph::new(editor_content)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Editor"));
            f.render_widget(editor, main_area);

            // Draw dialog if active
            if let Some(dtype) = dialog_type {
                // Calculate dialog area using stored size
                let dialog_area = UI::calc_centered_rect(60, 3, size);
                f.render_widget(Clear, dialog_area);
                let dialog = Paragraph::new(format!("{}: {}", dtype.title(), dialog_input))
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title(dtype.title()));
                f.render_widget(dialog, dialog_area);
            }

            // Draw status bar at correct position
            let status_text = if let Some(error) = error_message {
                Spans::from(Span::styled(error, Style::default().fg(Color::Red)))
            } else {
                let file_info = buffer.get_filename()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|| String::from("New File"));
                let modified = if buffer.is_modified() { "[+]" } else { "" };
                Spans::from(Span::raw(format!("{} {}", file_info, modified)))
            };
            let status_bar = Paragraph::new(status_text)
                .style(Style::default().add_modifier(Modifier::REVERSED));
            f.render_widget(status_bar, chunks[status_idx]);
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
        self.menu_active = true;
        self.input_mode = InputMode::Menu;
        self.menu_selection = 0;
    }

    pub fn hide_menu(&mut self) {
        self.menu_active = false;
        self.input_mode = InputMode::Normal;
        self.menu_selection = 0;
    }

    pub fn get_key(&mut self) -> Result<Option<KeyEvent>> {
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => Ok(Some(key)),
                Event::Resize(_, _) => {
                    self.terminal.backend_mut().execute(TermClear(ClearType::All))?;
                    Ok(None)
                }
                Event::Mouse(mouse_event) => {
                    self.handle_mouse(mouse_event);
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
        self.menu_active = false;
        self.input_mode = InputMode::Dialog;
    }

    pub fn show_load_dialog(&mut self) {
        self.dialog_input.clear();
        self.dialog_type = Some(DialogType::Load);
        self.menu_active = false;
        self.input_mode = InputMode::Dialog;
    }

    pub fn get_page_size(&self) -> usize {
        let (_, height) = self.terminal.size()
            .map(|r| (r.width as usize, r.height as usize))
            .unwrap_or((80, 24));
        height.saturating_sub(3)
    }

    pub fn menu_select(&mut self) -> Option<MenuOption> {
        match self.menu_selection {
            0 => Some(MenuOption::Exit),
            1 => Some(MenuOption::Save),
            2 => Some(MenuOption::SaveAs),
            3 => Some(MenuOption::Read),
            4 => Some(MenuOption::Goto),
            5 => Some(MenuOption::Find),
            6 => Some(MenuOption::Replace),
            7 => Some(MenuOption::Settings),
            8 => Some(MenuOption::Help),
            _ => None,
        }
    }

    pub fn menu_next(&mut self) {
        self.menu_selection = (self.menu_selection + 1) % 9;
    }

    pub fn menu_prev(&mut self) {
        self.menu_selection = (self.menu_selection + 8) % 9;
    }

    pub fn menu_goto(&mut self, index: usize) {
        if index < 9 {
            self.menu_selection = index;
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
        if !self.settings.is_mouse_enabled() {
            return None;
        }

        // Convert mouse position to buffer coordinates
        // TODO: Implement proper coordinate translation
        Some((mouse_event.row as usize, mouse_event.column as usize))
    }

    pub fn toggle_setting(&mut self, index: usize) {
        match index {
            0 => { self.settings.toggle_mouse(); }
            1 => { self.settings.toggle_dos_mode(); }
            2 => { self.settings.toggle_auto_indent(); }
            3 => { self.input_mode = InputMode::Menu; }
            _ => {}
        }
    }

    pub fn show_settings(&mut self) {
        self.menu_active = true;
        self.input_mode = InputMode::Settings;
        self.menu_selection = 0;
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
