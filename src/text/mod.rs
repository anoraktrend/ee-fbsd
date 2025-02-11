use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use syntect::{
    parsing::SyntaxSet,
    highlighting::{ThemeSet},  // Remove Style import
};
use serde::{Deserialize, Serialize};
use tower_lsp::Client;

pub mod highlighting;

#[derive(Debug, thiserror::Error)]
pub enum TextError {
    #[error("Theme error: {0}")]
    ThemeError(String),
    #[error("LSP error: {0}")]
    LspError(String),
}

#[derive(Deserialize, Serialize, Default)]
pub struct ThemeConfig {
    pub themes: HashMap<String, String>,
    pub current_theme: String,
}

pub struct TextProcessor {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub current_theme: String,
    pub theme_config: ThemeConfig,
    lsp_clients: HashMap<String, Arc<Mutex<Client>>>,
    active_language: Option<String>,
}

impl TextProcessor {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            current_theme: "base16-ocean.dark".to_string(),
            theme_config: ThemeConfig::default(),
            lsp_clients: HashMap::new(),
            active_language: None,
        }
    }

    pub fn highlight_line(&self, line: &str, syntax_name: &str) -> Vec<(i16, String)> {
        if let Some(syntax) = self.syntax_set.find_syntax_by_name(syntax_name) {
            let mut h = syntect::easy::HighlightLines::new(
                syntax,
                &self.theme_set.themes[&self.current_theme]
            );
            
            h.highlight_line(line, &self.syntax_set)
                .unwrap_or_default()
                .into_iter()
                .map(|(style, text)| {
                    let color_pair = self.get_color_pair(&style);
                    (color_pair, text.to_string())
                })
                .collect()
        } else {
            vec![(1, line.to_string())]
        }
    }

    fn get_color_pair(&self, style: &syntect::highlighting::Style) -> i16 {
        // Map syntect colors to our color pairs
        match style.foreground.r {
            n if n > 200 => 4, // keyword
            n if n > 150 => 5, // function
            n if n > 100 => 3, // string
            _ => 1,      // normal
        }
    }

    pub async fn initialize_lsp(&mut self, _language_id: &str) -> Result<(), TextError> {
        // Remove LSP functionality for now
        Ok(())
    }

    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), TextError> {
        if self.theme_set.themes.contains_key(theme_name) {
            self.current_theme = theme_name.to_string();
            Ok(())
        } else {
            Err(TextError::ThemeError(format!("Theme {} not found", theme_name)))
        }
    }

    pub fn get_active_lsp_client(&self) -> Option<Arc<Mutex<Client>>> {
        self.active_language.as_ref()
            .and_then(|lang| self.lsp_clients.get(lang))
            .map(Arc::clone)
    }

    pub fn get_available_themes(&self) -> Vec<String> {
        self.theme_set.themes.keys()
            .map(|k| k.to_string())
            .collect()
    }

    pub fn detect_language(&self, filename: &str) -> Option<String> {
        let ext = std::path::Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())?;

        Some(match ext {
            "rs" => "rust",
            "py" => "python",
            "js" => "javascript",
            "ts" => "typescript",
            "go" => "go",
            "c" | "h" => "c",
            "cpp" | "hpp" => "cpp",
            _ => return None
        }.to_string())
    }

    pub fn cleanup(&mut self) {
        // Empty cleanup for now
    }

    pub fn save_theme_config(&self) -> std::io::Result<()> {
        let config_path = crate::config::EditorConfig::get_config_path();
        let theme_path = config_path.with_file_name("themes.json");
        let contents = serde_json::to_string_pretty(&self.theme_config)?;
        std::fs::write(theme_path, contents)?;
        Ok(())
    }

    pub fn load_theme_config() -> ThemeConfig {
        let config_path = crate::config::EditorConfig::get_config_path();
        let theme_path = config_path.with_file_name("themes.json");
        if let Ok(contents) = std::fs::read_to_string(theme_path) {
            if let Ok(config) = serde_json::from_str(&contents) {
                return config;
            }
        }
        ThemeConfig::default()
    }
}

impl Drop for TextProcessor {
    fn drop(&mut self) {
        self.cleanup();
    }
}
