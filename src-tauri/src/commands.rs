use crate::mcp::protocol::MCPProtocol;
use std::sync::Arc;
use tauri::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClientInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub connected_at: String,
    pub last_message: String,
}

#[tauri::command]
pub async fn get_connected_clients(protocol: State<'_, Arc<MCPProtocol>>) -> Result<Vec<ClientInfo>, String> {
    let clients = protocol.get_client_manager().get_clients().await;
    
    Ok(clients.into_iter().map(|client| ClientInfo {
        id: client.id,
        name: client.client_info.name,
        version: client.client_info.version,
        connected_at: client.connected_at.to_rfc3339(),
        last_message: client.last_message,
    }).collect())
}