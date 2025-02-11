use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use tokio::io::{self, AsyncRead, AsyncWrite};

#[derive(Debug, thiserror::Error)]
pub enum LspError {
    #[error("Failed to start LSP server: {0}")]
    ServerStartError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct LspClient {
    client: Client,
    language_id: String,
    document_map: HashMap<String, TextDocumentItem>,
}

#[tower_lsp::async_trait]
impl LanguageServer for LspClient {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                // Add more capabilities as needed
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "LSP initialized").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

pub fn setup_lsp_client(language_id: &str) -> Result<Client, LspError> {
    // Map language ID to LSP server command
    let (cmd, args) = match language_id {
        "rust" => ("rust-analyzer", vec![]),
        "python" => ("pyls", vec![]),
        "javascript" => ("typescript-language-server", vec!["--stdio"]),
        _ => return Err(LspError::ServerStartError("Unsupported language".into())),
    };

    // Start LSP server process
    let process = Command::new(cmd)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| LspError::ServerStartError(e.to_string()))?;

    let (service, socket) = LspService::new(|client| LspClient {
        client: client.clone(),
        language_id: language_id.to_string(),
        document_map: HashMap::new(),
    });

    // Run server in background
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime");
        rt.block_on(async {
            let server = Server::new(socket);
            if let Err(e) = server.serve().await {
                eprintln!("LSP server error: {}", e);
            }
        });
    });

    Ok(service.inner().client.clone())
}
