use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageServerConfig {
    pub command: String,
    pub args: Vec<String>,
    pub initialization_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspConfig {
    servers: HashMap<String, LanguageServerConfig>,
}

impl Default for LspConfig {
    fn default() -> Self {
        let mut servers = HashMap::new();
        
        // Rust
        servers.insert(
            "rust".to_string(),
            LanguageServerConfig {
                command: "rust-analyzer".to_string(),
                args: vec![],
                initialization_options: None,
            }
        );

        // TypeScript/JavaScript
        servers.insert(
            "typescript".to_string(),
            LanguageServerConfig {
                command: "typescript-language-server".to_string(),
                args: vec!["--stdio".to_string()],
                initialization_options: None,
            }
        );

        Self { servers }
    }
}

impl LspConfig {
    pub fn get_server_config(&self, language_id: &str) -> Option<&LanguageServerConfig> {
        self.servers.get(language_id)
    }
}
