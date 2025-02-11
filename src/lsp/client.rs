use super::config::{LspConfig, LanguageServerConfig};
use tokio::process::{Command, Child};
use tokio::io::{BufReader, BufWriter};
use tokio::sync::Mutex;
use tower_lsp::{Client, LanguageClient, LspService};
use tower_lsp::lsp_types::*;
use std::sync::Arc;
use std::io;
use std::process::Stdio;

pub struct LspClient {
    client: Option<Client>,
    server_process: Option<Child>,
    server_capabilities: Arc<Mutex<Option<ServerCapabilities>>>,
    config: LspConfig,
}

impl LspClient {
    pub async fn new(language_id: &str) -> io::Result<Self> {
        let config = LspConfig::default();
        let server_config = config.get_server_config(language_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Language server not configured"))?;

        Ok(Self {
            client: None,
            server_process: None,
            server_capabilities: Arc::new(Mutex::new(None)),
            config,
        })
    }

    pub async fn connect(&mut self, language_id: &str) -> io::Result<()> {
        let server_config = self.config.get_server_config(language_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Language server not configured"))?;

        let mut child = self.start_server(server_config)?;
        
        let stdin = child.stdin.take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get stdin"))?;
        let stdout = child.stdout.take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get stdout"))?;

        let (service, messages) = LspService::new(|client| {
            Backend {
                client,
                capabilities: self.server_capabilities.clone(),
            }
        });

        let stdin = BufWriter::new(stdin);
        let stdout = BufReader::new(stdout);

        tokio::spawn(async move {
            tower_lsp::Server::new(stdin, stdout, messages).serve().await;
        });

        self.server_process = Some(child);

        // Initialize the server
        if let Some(ref client) = self.client {
            let params = InitializeParams {
                process_id: Some(std::process::id()),
                root_uri: None, // TODO: Add workspace support
                capabilities: ClientCapabilities::default(),
                workspace_folders: None,
                client_info: Some(ClientInfo {
                    name: "ee-fbsd".to_string(),
                    version: Some(env!("CARGO_PKG_VERSION").to_string()),
                }),
                ..InitializeParams::default()
            };

            let result = client.initialize(params).await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            let mut capabilities = self.server_capabilities.lock().await;
            *capabilities = Some(result.capabilities);
        }

        Ok(())
    }

    fn start_server(&self, config: &LanguageServerConfig) -> io::Result<Child> {
        Command::new(&config.command)
            .args(&config.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
    }

    pub async fn stop(&mut self) -> io::Result<()> {
        if let Some(ref client) = self.client {
            let _ = client.shutdown().await;
        }

        if let Some(mut child) = self.server_process.take() {
            let _ = child.kill().await;
        }

        Ok(())
    }

    pub async fn get_completions(&self, uri: &str, position: Position) -> Option<Vec<CompletionItem>> {
        let client = self.client.as_ref()?;
        let params = CompletionParams {
            text_document: TextDocumentIdentifier::new(uri.to_string()),
            position,
            context: None,
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        };

        client.completion(params).await.ok().map(|resp| {
            match resp {
                CompletionResponse::Array(items) => items,
                CompletionResponse::List(list) => list.items,
            }
        })
    }

    pub async fn get_hover(&self, uri: &str, position: Position) -> Option<Hover> {
        let client = self.client.as_ref()?;
        let params = HoverParams {
            text_document: TextDocumentIdentifier::new(uri.to_string()),
            position,
            work_done_progress_params: WorkDoneProgressParams::default(),
        };

        client.hover(params).await.ok()?
    }
}

struct Backend {
    client: Client,
    capabilities: Arc<Mutex<Option<ServerCapabilities>>>,
}

#[tower_lsp::async_trait]
impl tower_lsp::LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities::default(),
            server_info: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        // Server is now ready to receive requests
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
}
