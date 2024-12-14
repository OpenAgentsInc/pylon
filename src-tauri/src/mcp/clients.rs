use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use log::{info, debug};

use crate::mcp::types::ClientCapabilities;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectedClient {
    pub id: String,
    pub client_info: ClientInfo,
    pub connected_at: DateTime<Utc>,
    pub last_message: String,
    pub capabilities: ClientCapabilities,
}

#[derive(Debug, Default)]
pub struct ClientManager {
    clients: Arc<RwLock<HashMap<String, ConnectedClient>>>,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_client(&self, id: String, info: ClientInfo, capabilities: ClientCapabilities) {
        let client = ConnectedClient {
            id: id.clone(),
            client_info: info.clone(),
            connected_at: Utc::now(),
            last_message: "Connected".to_string(),
            capabilities,
        };

        info!("Adding client {} ({} v{})", id, info.name, info.version);
        self.clients.write().await.insert(id, client);
        debug!("Current clients: {:?}", self.clients.read().await.keys().collect::<Vec<_>>());
    }

    pub async fn remove_client(&self, id: &str) {
        info!("Removing client {}", id);
        self.clients.write().await.remove(id);
        debug!("Current clients: {:?}", self.clients.read().await.keys().collect::<Vec<_>>());
    }

    pub async fn clear_clients(&self) {
        info!("Clearing all clients");
        self.clients.write().await.clear();
    }

    pub async fn update_last_message(&self, id: &str, message: String) {
        if let Some(client) = self.clients.write().await.get_mut(id) {
            debug!("Updating last message for client {}: {}", id, message);
            client.last_message = message;
        }
    }

    pub async fn get_clients(&self) -> Vec<ConnectedClient> {
        let clients = self.clients.read().await.values().cloned().collect::<Vec<_>>();
        debug!("Getting clients: {} connected", clients.len());
        clients
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::types::OllamaCapability;

    #[tokio::test]
    async fn test_client_management() {
        let manager = ClientManager::new();

        // Add a client
        let info = ClientInfo {
            name: "test".to_string(),
            version: "1.0".to_string(),
        };
        
        // Create a function to generate fresh capabilities for each test
        let make_capabilities = || ClientCapabilities {
            experimental: None,
            roots: None,
            sampling: None,
            ollama: Some(OllamaCapability {
                available_models: vec!["llama2".to_string()],
                endpoint: "http://localhost:11434".to_string(),
                streaming: true,
            }),
        };

        // Use fresh capabilities for each call
        manager.add_client("test-id".to_string(), info.clone(), make_capabilities()).await;

        // Check client was added
        let clients = manager.get_clients().await;
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0].id, "test-id");
        assert_eq!(clients[0].client_info.name, "test");

        // Update last message
        manager.update_last_message("test-id", "Test message".to_string()).await;
        let clients = manager.get_clients().await;
        assert_eq!(clients[0].last_message, "Test message");

        // Remove client
        manager.remove_client("test-id").await;
        let clients = manager.get_clients().await;
        assert_eq!(clients.len(), 0);

        // Test clear_clients
        manager.add_client("test-id1".to_string(), info.clone(), make_capabilities()).await;
        manager.add_client("test-id2".to_string(), info.clone(), make_capabilities()).await;
        assert_eq!(manager.get_clients().await.len(), 2);
        manager.clear_clients().await;
        assert_eq!(manager.get_clients().await.len(), 0);
    }
}