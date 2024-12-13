use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootsCapability {
    pub list_changed: bool,
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
            client_info: info,
            connected_at: Utc::now(),
            last_message: "Connected".to_string(),
            capabilities,
        };

        self.clients.write().await.insert(id, client);
    }

    pub async fn remove_client(&self, id: &str) {
        self.clients.write().await.remove(id);
    }

    pub async fn update_last_message(&self, id: &str, message: String) {
        if let Some(client) = self.clients.write().await.get_mut(id) {
            client.last_message = message;
        }
    }

    pub async fn get_clients(&self) -> Vec<ConnectedClient> {
        self.clients.read().await.values().cloned().collect()
    }
}

impl Default for ClientCapabilities {
    fn default() -> Self {
        Self {
            experimental: None,
            roots: None,
            sampling: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_management() {
        let manager = ClientManager::new();

        // Add a client
        let info = ClientInfo {
            name: "test".to_string(),
            version: "1.0".to_string(),
        };
        let capabilities = ClientCapabilities {
            experimental: None,
            roots: Some(RootsCapability { list_changed: true }),
            sampling: None,
        };
        manager.add_client("test-id".to_string(), info.clone(), capabilities).await;

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
    }
}