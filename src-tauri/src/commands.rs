use tauri::State;
use std::sync::Arc;
use crate::mcp::protocol::RequestHandler;

#[tauri::command]
pub async fn get_connected_clients(handler: State<'_, Arc<RequestHandler>>) -> Result<Vec<String>, String> {
    Ok(vec![]) // TODO: Implement this once we have client tracking
}