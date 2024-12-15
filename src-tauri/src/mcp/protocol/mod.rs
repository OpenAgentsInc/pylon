mod handlers;
mod types;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::sync::Arc;
use log::{info, debug};

use crate::mcp::clients::ClientManager;
use crate::mcp::providers::{
    filesystem::FileSystemProvider,
    ollama::OllamaProvider,
    roots::get_root_paths,
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

        // Get the configured root paths
        let root_paths = get_root_paths();
        
        // Log the root paths
        for path in &root_paths {
            info!("Using root path: {:?}", path);
        }
        
        // Use the first root path as the default
        let root_path = root_paths.first()
            .cloned()
            .unwrap_or_else(|| {
                info!("No valid root paths found, using '.'");
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