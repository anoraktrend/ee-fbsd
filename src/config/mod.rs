use crate::ui::Theme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]  // Add Clone
pub struct Settings {
    mouse_enabled: bool,
    dos_mode: bool,
    auto_indent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]  // Add Clone
pub struct Config {
    settings: Settings,
    theme_name: String,
    #[serde(skip)]
    theme: Theme,
}

impl Settings {
    pub fn is_mouse_enabled(&self) -> bool {
        self.mouse_enabled
    }

    pub fn is_dos_mode(&self) -> bool {
        self.dos_mode
    }

    pub fn is_auto_indent(&self) -> bool {
        self.auto_indent
    }

    pub fn toggle_mouse(&mut self) -> bool {
        self.mouse_enabled = !self.mouse_enabled;
        self.mouse_enabled
    }

    pub fn toggle_dos_mode(&mut self) -> bool {
        self.dos_mode = !self.dos_mode;
        self.dos_mode
    }

    pub fn toggle_auto_indent(&mut self) -> bool {
        self.auto_indent = !self.auto_indent;
        self.auto_indent
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        let mut config = if let Ok(contents) = fs::read_to_string(&config_path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Config::default()
        };

        config.theme = Theme::load(&config.theme_name);
        config
    }

    pub fn save(&self) -> std::io::Result<()> {
        let config_path = Self::get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(config_path, contents)
    }

    fn get_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ee-fbsd")
            .join("config.json")
    }

    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }

    pub fn set_theme(&mut self, theme_name: String) {
        self.theme_name = theme_name.clone();
        self.theme = Theme::load(&theme_name);
        self.save().ok();
    }

    pub fn get_theme_name(&self) -> &str {
        &self.theme_name
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn get_settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            settings: Settings {
                mouse_enabled: true,
                dos_mode: false,
                auto_indent: true,
            },
            theme_name: "default".to_string(),
            theme: Theme::default(),
        }
    }
}
