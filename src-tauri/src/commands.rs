use tauri::State;
use std::sync::Arc;
use pylon_lib::mcp::protocol::RequestHandler;

#[tauri::command]
pub async fn get_connected_clients(handler: State<'_, Arc<RequestHandler>>) -> Result<Vec<String>, String> {
    Ok(vec![]) // TODO: Implement this once we have client tracking
}