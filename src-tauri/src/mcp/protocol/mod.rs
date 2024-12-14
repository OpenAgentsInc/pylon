mod handlers;
mod types;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::sync::Arc;
use std::env;
use log::{info, debug};

use crate::mcp::clients::ClientManager;
use crate::mcp::providers::{
    filesystem::FileSystemProvider,
    ollama::OllamaProvider,
};
use crate::mcp::types::*;

pub use types::*;

pub struct MCPProtocol {
    server_info: Implementation,
    server_capabilities: ServerCapabilities,
    fs_provider: Arc<FileSystemProvider>,
    ollama_provider: Arc<OllamaProvider>,
    client_manager: Arc<ClientManager>,
}

impl Default for MCPProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl MCPProtocol {
    pub fn new() -> Self {
        let ollama_provider = Arc::new(OllamaProvider::default());

        // Get the project root directory
        let root_path = env::current_dir()
            .map(|mut d| {
                // Log the initial directory
                debug!("Initial directory: {:?}", d);
                
                // If we're in src-tauri, go up one level
                if d.ends_with("src-tauri") {
                    d.pop();
                }
                
                // Log the final directory
                info!("Using root path: {:?}", d);
                d
            })
            .unwrap_or_else(|e| {
                info!("Failed to get current directory: {}, using '.'", e);
                PathBuf::from(".")
            });

        Self {
            server_info: Implementation::default(),
            server_capabilities: ServerCapabilities {
                resources: Some(ResourcesCapability {
                    list_changed: true,
                    subscribe: true,
                }),
                tools: Some(ToolsCapability { list_changed: true }),
                prompts: Some(PromptsCapability { list_changed: true }),
                ollama: Some(OllamaCapability {
                    available_models: Vec::new(),
                    endpoint: "http://localhost:11434".to_string(),
                    streaming: true,
                }),
                ..Default::default()
            },
            fs_provider: Arc::new(FileSystemProvider::new(root_path)),
            ollama_provider,
            client_manager: Arc::new(ClientManager::new()),
        }
    }

    pub fn get_client_manager(&self) -> Arc<ClientManager> {
        self.client_manager.clone()
    }
}