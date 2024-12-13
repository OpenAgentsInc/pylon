use pylon_lib::mcp::protocol::MCPProtocol;
use tauri::State;

#[tauri::command]
pub async fn get_connected_clients(protocol: State<'_, MCPProtocol>) -> Result<Vec<String>, String> {
    let clients = protocol.get_client_manager().get_clients().await;
    Ok(clients.iter().map(|c| c.id.clone()).collect())
}