use pylon_lib::mcp::protocol::MCPProtocol;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClientInfoResponse {
    pub name: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct RootsCapabilityResponse {
    pub list_changed: bool,
}

#[derive(Serialize)]
pub struct CapabilitiesResponse {
    pub experimental: HashMap<String, HashMap<String, serde_json::Value>>,
    pub roots: RootsCapabilityResponse,
    pub sampling: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
pub struct ClientResponse {
    pub id: String,
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfoResponse,
    #[serde(rename = "connectedAt")]
    pub connected_at: String,
    #[serde(rename = "lastMessage")]
    pub last_message: String,
    pub capabilities: CapabilitiesResponse,
}

#[tauri::command]
pub async fn get_connected_clients(protocol: State<'_, Arc<MCPProtocol>>) -> Result<Vec<ClientResponse>, String> {
    let clients = protocol.get_client_manager().get_clients().await;
    
    Ok(clients.into_iter().map(|client| ClientResponse {
        id: client.id,
        client_info: ClientInfoResponse {
            name: client.client_info.name,
            version: client.client_info.version,
        },
        connected_at: client.connected_at.to_rfc3339(),
        last_message: client.last_message,
        capabilities: CapabilitiesResponse {
            experimental: client.capabilities.experimental.unwrap_or_default(),
            roots: RootsCapabilityResponse {
                list_changed: client.capabilities.roots
                    .map(|r| r.list_changed)
                    .unwrap_or_default(),
            },
            sampling: client.capabilities.sampling.unwrap_or_default(),
        },
    }).collect())
}