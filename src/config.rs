use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditorConfig {
    pub key_bindings: KeyBindings,
    pub display: DisplayConfig,
    pub editing: EditingConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyBindings {
    pub exit: i32,
    pub write: i32,
    pub search: i32,
    pub menu: i32,
    pub next: i32,
    pub prev: i32,
    pub cut: i32,
    pub paste: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DisplayConfig {
    pub show_line_numbers: bool,
    pub tab_width: usize,
    pub color_theme: String,
    pub syntax_highlighting: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditingConfig {
    pub auto_indent: bool,
    pub word_wrap: bool,
    pub tab_to_spaces: bool,
    pub auto_save: bool,
    pub backup_files: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            key_bindings: KeyBindings::default(),
            display: DisplayConfig::default(),
            editing: EditingConfig::default(),
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            exit: 24,  // Ctrl-X
            write: 23, // Ctrl-W
            search: 6, // Ctrl-F
            menu: 13,  // Ctrl-M
            next: 14,  // Ctrl-N
            prev: 16,  // Ctrl-P
            cut: 11,   // Ctrl-K
            paste: 25, // Ctrl-Y
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            tab_width: 4,
            color_theme: "default".to_string(),
            syntax_highlighting: true,
        }
    }
}

impl Default for EditingConfig {
    fn default() -> Self {
        Self {
            auto_indent: true,
            word_wrap: true,
            tab_to_spaces: true,
            auto_save: true,
            backup_files: true,
        }
    }
}

impl EditorConfig {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        if let Ok(contents) = fs::read_to_string(config_path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            let default = Self::default();
            let _ = default.save();
            default
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let config_path = Self::get_config_path();
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(config_path, contents)
    }

    pub fn get_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ee")
            .join("config.json")
    }

    pub fn get_theme_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ee")
            .join("themes")
    }

    pub fn get_backup_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("ee")
            .join("backups")
    }

    pub fn update_key_binding(&mut self, key: &str, value: i32) -> bool {
        match key {
            "exit" => self.key_bindings.exit = value,
            "write" => self.key_bindings.write = value,
            "search" => self.key_bindings.search = value,
            "menu" => self.key_bindings.menu = value,
            "next" => self.key_bindings.next = value,
            "prev" => self.key_bindings.prev = value,
            "cut" => self.key_bindings.cut = value,
            "paste" => self.key_bindings.paste = value,
            _ => return false,
        }
        true
    }

    pub fn get_key_binding(&self, key: &str) -> Option<i32> {
        match key {
            "exit" => Some(self.key_bindings.exit),
            "write" => Some(self.key_bindings.write),
            "search" => Some(self.key_bindings.search),
            "menu" => Some(self.key_bindings.menu),
            "next" => Some(self.key_bindings.next),
            "prev" => Some(self.key_bindings.prev),
            "cut" => Some(self.key_bindings.cut),
            "paste" => Some(self.key_bindings.paste),
            _ => None,
        }
    }
    
    pub fn update_display_setting(&mut self, key: &str, value: String) -> bool {
        match key {
            "theme" => self.display.color_theme = value,
            "tab_width" => {
                if let Ok(width) = value.parse() {
                    self.display.tab_width = width;
                } else {
                    return false;
                }
            }
            "line_numbers" => {
                self.display.show_line_numbers = value.parse().unwrap_or(true);
            }
            "syntax" => {
                self.display.syntax_highlighting = value.parse().unwrap_or(true);
            }
            _ => return false,
        }
        true
    }

    pub fn update_editing_setting(&mut self, key: &str, value: bool) -> bool {
        match key {
            "auto_indent" => self.editing.auto_indent = value,
            "word_wrap" => self.editing.word_wrap = value,
            "tab_to_spaces" => self.editing.tab_to_spaces = value,
            "auto_save" => self.editing.auto_save = value,
            "backup_files" => self.editing.backup_files = value,
            _ => return false,
        }
        true
    }
}
