use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tui::style::Color;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiColors {
    pub background: String,
    pub foreground: String,
    pub selection: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxColors {
    pub comment: String,
    pub string: String,
    pub keyword: String,
    pub function: String,
    pub number: String,
    pub r#type: String,
    pub constant: String,
    pub operator: String,
    pub namespace: String,
    pub variable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticColors {
    pub warning: String,
    pub error: String,
    pub info: String,
    pub hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub ui: UiColors,
    pub syntax: SyntaxColors,
    pub diagnostic: DiagnosticColors,
}

#[derive(Debug, Clone)]  // Add Clone derivation
pub struct Theme {
    pub ui: UiTheme,
    pub syntax: SyntaxTheme,
    pub diagnostic: DiagnosticTheme,
}

#[derive(Debug, Clone)]  // Add Clone derivation
pub struct UiTheme {
    pub background: Color,
    pub foreground: Color,
    pub selection: Color,
    pub status: Color,
}

#[derive(Debug, Clone)]  // Add Clone derivation
pub struct SyntaxTheme {
    pub comment: Color,
    pub string: Color,
    pub keyword: Color,
    pub function: Color,
    pub number: Color,
    pub r#type: Color,
    pub constant: Color,
    pub operator: Color,
    pub namespace: Color,
    pub variable: Color,
}

#[derive(Debug, Clone)]  // Add Clone derivation
pub struct DiagnosticTheme {
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub hint: Color,
}

impl Theme {
    pub fn load(theme_name: &str) -> Self {
        if let Ok(content) = std::fs::read_to_string("config/colors.json") {
            if let Ok(themes) = serde_json::from_str::<HashMap<String, ThemeConfig>>(&content) {
                if let Some(config) = themes.get(theme_name) {
                    return Self::from_config(config);
                }
            }
        }
        Self::default()
    }

    fn from_config(config: &ThemeConfig) -> Self {
        Self {
            ui: UiTheme {
                background: Self::parse_hex(&config.ui.background),
                foreground: Self::parse_hex(&config.ui.foreground),
                selection: Self::parse_hex(&config.ui.selection),
                status: Self::parse_hex(&config.ui.status),
            },
            syntax: SyntaxTheme {
                comment: Self::parse_hex(&config.syntax.comment),
                string: Self::parse_hex(&config.syntax.string),
                keyword: Self::parse_hex(&config.syntax.keyword),
                function: Self::parse_hex(&config.syntax.function),
                number: Self::parse_hex(&config.syntax.number),
                r#type: Self::parse_hex(&config.syntax.r#type),
                constant: Self::parse_hex(&config.syntax.constant),
                operator: Self::parse_hex(&config.syntax.operator),
                namespace: Self::parse_hex(&config.syntax.namespace),
                variable: Self::parse_hex(&config.syntax.variable),
            },
            diagnostic: DiagnosticTheme {
                warning: Self::parse_hex(&config.diagnostic.warning),
                error: Self::parse_hex(&config.diagnostic.error),
                info: Self::parse_hex(&config.diagnostic.info),
                hint: Self::parse_hex(&config.diagnostic.hint),
            },
        }
    }

    fn parse_hex(hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            Color::Rgb(
                ((rgb >> 16) & 0xFF) as u8,
                ((rgb >> 8) & 0xFF) as u8,
                (rgb & 0xFF) as u8,
            )
        } else {
            Color::Reset
        }
    }

    pub fn get_available_themes() -> Vec<String> {
        if let Ok(content) = fs::read_to_string("config/colors.json") {
            if let Ok(themes) = serde_json::from_str::<HashMap<String, ThemeConfig>>(&content) {
                return themes.keys().cloned().collect();
            }
        }
        vec!["default".to_string()]
    }

    pub fn exists(theme_name: &str) -> bool {
        if let Ok(content) = fs::read_to_string("config/colors.json") {
            if let Ok(themes) = serde_json::from_str::<HashMap<String, ThemeConfig>>(&content) {
                return themes.contains_key(theme_name);
            }
        }
        theme_name == "default"
    }

    pub fn format_theme_label(name: &str, current: &str) -> String {
        format!("Theme: {} {}", name, if name == current { "(*)" } else { "" })
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::load("default")
    }
}
