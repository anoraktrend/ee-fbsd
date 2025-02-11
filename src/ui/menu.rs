use super::theme::Theme;

#[derive(Clone, PartialEq)]
pub enum MenuOption {
    None,  // Add None variant
    Settings,
    Back,
    // Settings menu options
    MouseSupport,
    DosMode,
    AutoIndent,
    Theme(String),
    // Main menu options
    Save,
    SaveAs,
    Read,
    Find,
    Help,
    Exit,
}

pub struct Menu {
    pub active: bool,
    pub selection: usize,
    pub in_settings: bool,
    theme_options: Vec<String>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            active: false,
            selection: 0,
            in_settings: false,
            theme_options: Theme::get_available_themes(),
        }
    }

    pub fn get_settings_items(&self, current_theme: &str) -> Vec<(String, Option<bool>)> {
        let mut items = vec![
            ("Mouse support".into(), Some(true)),
            ("DOS mode".into(), Some(false)),
            ("Auto indent".into(), Some(true)),
        ];

        // Add theme options using Theme's formatting
        for theme in &self.theme_options {
            items.push((Theme::format_theme_label(theme, current_theme), None));
        }

        items.push(("Back to main menu".into(), None));
        items
    }

    pub fn select(&self) -> Option<MenuOption> {
        if self.in_settings {
            match self.selection {
                0 => Some(MenuOption::MouseSupport),
                1 => Some(MenuOption::DosMode),
                2 => Some(MenuOption::AutoIndent),
                i if i < self.theme_options.len() + 3 => {
                    Some(MenuOption::Theme(self.theme_options[i - 3].clone()))
                }
                i if i == self.theme_options.len() + 3 => Some(MenuOption::Back),
                _ => None,
            }
        } else {
            match self.selection {
                0 => Some(MenuOption::Settings),
                1 => Some(MenuOption::Save),
                2 => Some(MenuOption::SaveAs),
                3 => Some(MenuOption::Read),
                4 => Some(MenuOption::Find),
                5 => Some(MenuOption::Help),
                6 => Some(MenuOption::Exit),
                _ => None,
            }
        }
    }

    pub fn is_toggleable(option: &MenuOption) -> bool {
        matches!(option,
            MenuOption::MouseSupport |
            MenuOption::DosMode |
            MenuOption::AutoIndent
        )
    }

    pub fn is_theme(option: &MenuOption) -> bool {
        matches!(option, MenuOption::Theme(_))
    }

    pub fn get_theme_name(option: &MenuOption) -> Option<String> {
        match option {
            MenuOption::Theme(name) => Some(name.clone()),
            _ => None,
        }
    }
}
