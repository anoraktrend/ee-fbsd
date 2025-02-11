mod client;
mod config;

pub use client::LspClient;
pub use config::{LspConfig, LanguageServerConfig};

use std::process::Stdio;
use tokio::process::Command;
use tower_lsp::Client;
use tower_lsp::lsp_types::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LspClient {
    client: Option<Client>,
    server_capabilities: Arc<Mutex<Option<ServerCapabilities>>>,
}

impl LspClient {
    pub async fn new(language_id: &str) -> Option<Self> {
        // Choose LSP server based on language
        let (command, args) = match language_id {
            "rust" => ("rust-analyzer", vec![]),
            "javascript" | "typescript" => ("typescript-language-server", vec!["--stdio"]),
            "python" => ("pyls", vec![]),
            _ => return None,
        };

        // Start LSP server process
        let process = Command::new(command)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .ok()?;

        // Initialize LSP connection
        // TODO: Implement actual LSP client connection
        
        Some(Self {
            client: None, // TODO: Replace with actual client
            server_capabilities: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn get_completions(&self, uri: &str, position: Position) -> Option<Vec<CompletionItem>> {
        // TODO: Implement completion request
        None
    }

    pub async fn get_hover(&self, uri: &str, position: Position) -> Option<Hover> {
        // TODO: Implement hover request
        None
    }

    pub async fn get_diagnostics(&self, uri: &str) -> Vec<Diagnostic> {
        // TODO: Implement diagnostics
        vec![]
    }
}
