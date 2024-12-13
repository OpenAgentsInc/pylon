use log::info;
use std::thread;
use std::sync::Arc;
use crate::mcp::{MCPServer, MCPProtocol};

mod commands;
mod mcp;

fn main() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting MCP server...");
    
    // Create MCP server instance
    let mcp_server = Arc::new(MCPServer::new());
    let protocol = mcp_server.protocol.clone();
    
    // Start MCP server in a separate thread
    let server_clone = mcp_server.clone();
    thread::spawn(move || {
        let system = actix_web::rt::System::new();
        system.block_on(async {
            if let Err(e) = server_clone.start("127.0.0.1", 8080).await {
                log::error!("MCP server error: {}", e);
            }
        });
    });

    // Run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(protocol)
        .invoke_handler(tauri::generate_handler![
            commands::get_connected_clients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}