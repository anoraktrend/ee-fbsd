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
    text::{Text, Spans, Span},
    style::{Style, Modifier},
};
use crossterm::event::{self, Event, KeyCode};

pub type Result<T> = std::result::Result<T, io::Error>;

pub enum DialogResult {
    None,
    Save(PathBuf),
    Load(PathBuf),
    Cancel,
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    menu_active: bool,
    dialog_input: String,
    dialog_type: Option<DialogType>,
}

enum DialogType {
    Save,
    Load,
}

impl UI {
    pub fn new() -> Result<Self> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self { 
            terminal,
            menu_active: false,
            dialog_input: String::new(),
            dialog_type: None,
        })
    }

    pub fn refresh(&mut self, buffer: &Buffer) -> Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),  // menu bar
                    Constraint::Min(1),     // editor area
                    Constraint::Length(2),  // status bars
                ].as_ref())
                .split(f.size());

            // Editor content with cursor
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
            f.render_widget(editor, chunks[1]);

            // Status bar
            let status = format!(
                "{} {}",
                buffer.get_filename().map_or("New Buffer", |p| p.to_str().unwrap_or("?")),
                if buffer.is_modified() { "[+]" } else { "" }
            );
            let status_bar = Paragraph::new(status)
                .style(Style::default().add_modifier(Modifier::REVERSED));
            f.render_widget(status_bar, chunks[2]);

            // Menu or dialog
            match &self.dialog_type {
                Some(DialogType::Save) => {
                    let dialog = Paragraph::new(format!("Save as: {}", self.dialog_input))
                        .style(Style::default().add_modifier(Modifier::REVERSED));
                    f.render_widget(dialog, chunks[0]);
                }
                Some(DialogType::Load) => {
                    let dialog = Paragraph::new(format!("Open file: {}", self.dialog_input))
                        .style(Style::default().add_modifier(Modifier::REVERSED));
                    f.render_widget(dialog, chunks[0]);
                }
                None if self.menu_active => {
                    let menu = Paragraph::new("^X Exit | ^O Save | ^W Where | ^R Read | ^Y Prev | ^V Next")
                        .style(Style::default().add_modifier(Modifier::REVERSED));
                    f.render_widget(menu, chunks[0]);
                }
                None => {}
            }
        })?;
        Ok(())
    }

    pub fn toggle_menu(&mut self) {
        self.menu_active = !self.menu_active;
    }

    pub fn is_menu_active(&self) -> bool {
        self.menu_active
    }

    pub fn get_key(&self) -> Result<Option<KeyCode>> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                return Ok(Some(key.code));
            }
        }
        Ok(None)
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
}
