mod highlight;
mod theme;

use std::io;
use std::path::PathBuf;
use crate::buffer::Buffer;  // Add this import
use tui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph},
    text::{Spans, Span},  // Remove Text import
    style::{Style, Modifier},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    ExecutableCommand,
};

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
}

pub enum InputMode {
    Normal,
    WaitingForControl,
    Menu,
    Dialog,
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    menu_active: bool,
    dialog_input: String,
    dialog_type: Option<DialogType>,
    menu_selection: usize,
    input_mode: InputMode,
}

enum DialogType {
    Save,
    Load,
}

impl UI {
    pub fn new() -> Result<Self> {
        // Set up terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self { 
            terminal,
            menu_active: false,
            dialog_input: String::new(),
            dialog_type: None,
            menu_selection: 0,
            input_mode: InputMode::Normal,
        })
    }

    pub fn refresh(&mut self, buffer: &Buffer) -> Result<()> {
        let show_tabs = !self.menu_active && !self.is_dialog_active();
        
        self.terminal.draw(|f| {
            // Create a three-panel layout: menu bar, main area, status bar
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),  // menu/tab bar
                    Constraint::Min(1),     // editor area
                    Constraint::Length(1),  // status bar
                ].as_ref())
                .split(f.size());

            // Always show menu bar but change content based on mode
            let menu_bar = Paragraph::new(
                if self.menu_active {
                    "ESC=exit menu | Enter=select | Up/Down=navigate"
                } else {
                    "ESC=menu | ^O=save | ^R=read | ^X=exit"
                }
            ).style(Style::default().add_modifier(Modifier::REVERSED));
            f.render_widget(menu_bar, chunks[0]);

            // Split main area for menu if active
            let main_area = if self.menu_active {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(30), // Menu width
                        Constraint::Min(1),     // Editor width
                    ].as_ref())
                    .split(chunks[1])
            } else {
                vec![chunks[1]] // Just use full width for editor
            };

            // Render menu if active
            if self.menu_active {
                let menu_items = vec![
                    "leave editor", "save file", "save as", "read file",
                    "goto line", "find string", "replace", "help"
                ];
                
                let menu_text: Vec<Spans> = menu_items.iter().enumerate().map(|(idx, &text)| {
                    let style = if idx == self.menu_selection {
                        Style::default().add_modifier(Modifier::REVERSED)
                    } else {
                        Style::default()
                    };
                    Spans::from(vec![
                        Span::styled(format!("{:>2}. ", idx + 1), style),
                        Span::styled(text, style),
                    ])
                }).collect();

                let menu = Paragraph::new(menu_text)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title("Menu"));
                
                f.render_widget(menu, main_area[0]);
            }

            // Editor content
            let editor_area = if self.menu_active { main_area[1] } else { main_area[0] };
            let content: Vec<Spans> = buffer.lines().iter().enumerate().map(|(idx, line)| {
                let line_no = format!("{:>4} ", idx + 1);
                let mut spans = vec![
                    Span::styled(line_no, Style::default().add_modifier(Modifier::DIM))
                ];
                
                if idx == buffer.get_cursor().line {
                    let (before, after) = line.split_at(buffer.get_cursor().column.min(line.len()));
                    spans.push(Span::raw(before));
                    spans.push(Span::styled("█", Style::default().add_modifier(Modifier::REVERSED)));
                    spans.push(Span::raw(after));
                } else {
                    spans.push(Span::raw(line));
                }
                
                Spans::from(spans)
            }).collect();

            let editor = Paragraph::new(content)
                .block(Block::default().borders(Borders::ALL).title("Editor"));
            f.render_widget(editor, editor_area);

            // Status bar - always show
            let status = format!(
                "{} {}",
                buffer.get_filename().map_or("New Buffer", |p| p.to_str().unwrap_or("?")),
                if buffer.is_modified() { "[+]" } else { "" }
            );
            let status_bar = Paragraph::new(status)
                .style(Style::default().add_modifier(Modifier::REVERSED));
            f.render_widget(status_bar, chunks[2]);
        })?;
        Ok(())
    }

    pub fn get_input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    pub fn toggle_menu(&mut self) {
        self.menu_active = !self.menu_active;
        self.input_mode = if self.menu_active {
            InputMode::Menu
        } else {
            InputMode::Normal
        };
    }

    pub fn is_menu_active(&self) -> bool {
        self.menu_active
    }

    pub fn get_key(&mut self) -> Result<Option<KeyEvent>> {
        // Properly handle input with timeout
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    Ok(Some(key))
                }
                Event::Resize(_, _) => {
                    // Handle terminal resize
                    self.terminal.backend_mut().execute(Clear(ClearType::All))?;
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
                self.dialog_input.clear();
                self.dialog_type = None;
                result
            }
            KeyCode::Esc => {
                self.dialog_input.clear();
                self.dialog_type = None;
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

    pub fn show_save_dialog(&mut self) {
        self.dialog_type = Some(DialogType::Save);
        self.menu_active = false;
    }

    pub fn show_load_dialog(&mut self) {
        self.dialog_type = Some(DialogType::Load);
        self.menu_active = false;
    }

    pub fn is_dialog_active(&self) -> bool {
        self.dialog_type.is_some()
    }

    pub fn get_page_size(&self) -> usize {
        // Get terminal size and subtract 3 for menu and status bars
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
            7 => Some(MenuOption::Help),
            _ => None,
        }
    }

    pub fn menu_next(&mut self) {
        self.menu_selection = (self.menu_selection + 1) % 8;
    }

    pub fn menu_prev(&mut self) {
        self.menu_selection = (self.menu_selection + 7) % 8;
    }

    pub fn menu_goto(&mut self, index: usize) {
        if index < 8 {
            self.menu_selection = index;
        }
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        // Proper cleanup sequence
        let mut stdout = std::io::stdout();
        let _ = stdout.execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
