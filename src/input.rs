use crate::buffer::CursorMove;
use crate::ui::{InputMode, MenuOption};
use crossterm::event::{Event, KeyCode, KeyModifiers};

pub enum EditorCommand {
    Quit,
    Save,
    SaveAs,
    Load,
    NewTab,
    NextTab,
    PrevTab,
    CloseTab,
    ToggleHints,
    InsertChar(char),
    MoveCursor(CursorMove),
    DeleteChar,
    Copy,
    Cut,
    Paste,
    ShowMenu,
    HideMenu,
    MenuSelect(MenuOption),
    MenuNext,
    MenuPrev,
    MenuGoto(usize),
    None,
}

pub fn process_input(event: Event, mode: InputMode) -> EditorCommand {
    match event {
        Event::Key(key) => match mode {
            InputMode::Normal => process_normal_mode(key.code, key.modifiers),
            InputMode::Menu => process_menu_mode(key.code),
            InputMode::Dialog => EditorCommand::None,
        },
        _ => EditorCommand::None,
    }
}

fn process_normal_mode(code: KeyCode, modifiers: KeyModifiers) -> EditorCommand {
    match (code, modifiers) {
        (KeyCode::Esc, _) => EditorCommand::ShowMenu,
        (KeyCode::Char(c), m) if m.contains(KeyModifiers::CONTROL) => {
            match c {
                'q' | 'Q' => EditorCommand::Quit,
                's' | 'S' => EditorCommand::Save,
                'o' | 'O' => EditorCommand::Load,
                't' | 'T' => EditorCommand::NewTab,
                'n' | 'N' => EditorCommand::NextTab,
                'p' | 'P' => EditorCommand::PrevTab,
                'w' | 'W' => EditorCommand::CloseTab,
                'h' | 'H' => EditorCommand::ToggleHints,
                _ => EditorCommand::None,
            }
        },
        (KeyCode::Char(c), m) if m.is_empty() => EditorCommand::InsertChar(c),
        _ => EditorCommand::None,
    }
}

fn process_menu_mode(code: KeyCode) -> EditorCommand {
    match code {
        KeyCode::Esc => EditorCommand::HideMenu,
        KeyCode::Up => EditorCommand::MenuPrev,
        KeyCode::Down => EditorCommand::MenuNext,
        KeyCode::Enter => EditorCommand::MenuSelect(MenuOption::None),
        KeyCode::Char(c) => {
            if let Some(digit) = c.to_digit(10) {
                if digit > 0 && digit <= 9 {
                    EditorCommand::MenuGoto((digit as usize) - 1)
                } else {
                    EditorCommand::None
                }
            } else {
                EditorCommand::None
            }
        }
        _ => EditorCommand::None,
    }
}
