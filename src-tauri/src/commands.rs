use crate::mcp::protocol::MCPProtocol;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_connected_clients(protocol: State<'_, Arc<MCPProtocol>>) -> Result<Vec<crate::mcp::clients::ConnectedClient>, String> {
    let clients = protocol.get_client_manager().get_clients().await;
    Ok(clients)
}