use syntect::highlighting::{Theme};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ThemeConfig {
    pub themes: HashMap<String, String>,
    pub current_theme: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let mut themes = HashMap::new();
        themes.insert("Default".to_string(), "base16-ocean.dark".to_string());
        Self {
            themes,
            current_theme: "Default".to_string(),
        }
    }
}

impl ThemeConfig {
    pub fn save(&self) -> std::io::Result<()> {
        let config_path = crate::config::EditorConfig::get_config_path();
        let theme_path = config_path.with_file_name("themes.json");
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(theme_path, contents)?;
        Ok(())
    }

    pub fn load_theme(&self, name: &str) -> Option<Theme> {
        if let Some(theme_path) = self.themes.get(name) {
            if let Ok(contents) = std::fs::read_to_string(theme_path) {
                let mut reader = std::io::Cursor::new(contents);
                return syntect::highlighting::ThemeSet::load_from_reader(&mut reader).ok();
            }
        }
        None
    }
}

pub fn load_theme_config() -> ThemeConfig {
    let config_path = crate::config::EditorConfig::get_config_path();
    let theme_path = config_path.with_file_name("themes.json");
    
    if theme_path.exists() {
        if let Ok(contents) = std::fs::read_to_string(theme_path) {
            if let Ok(config) = serde_json::from_str(&contents) {
                return config;
            }
        }
    }
    ThemeConfig::default()
}

pub fn load_theme(contents: &str) -> Option<Theme> {
    let mut reader = std::io::Cursor::new(contents);
    syntect::highlighting::ThemeSet::load_from_reader(&mut reader).ok()
}
