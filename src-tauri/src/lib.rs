use log::info;
use std::sync::Arc;

pub mod mcp;
#[cfg(test)]
mod tests;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Create runtime for MCP server
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Create and spawn MCP server
    let mcp_server = Arc::new(mcp::MCPServer::new(8080));
    let mcp_server_clone = Arc::clone(&mcp_server);
    
    rt.spawn(async move {
        info!("Starting MCP server...");
        if let Err(e) = mcp_server_clone.run().await {
            log::error!("MCP server error: {}", e);
        }
    });

    // Run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Runtime will be dropped here, cleaning up the MCP server
}