use log::info;
use std::thread;
use std::sync::Arc;
use pylon_lib::mcp::MCPServer;

mod commands;

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
            // Try ports 8080, 8081, 8082 in sequence
            for port in 8080..8083 {
                match server_clone.start("0.0.0.0", port).await {
                    Ok(_) => {
                        info!("MCP server started successfully on port {}", port);
                        break;
                    },
                    Err(e) => {
                        log::error!("Failed to start MCP server on port {}: {}", port, e);
                        if port == 8082 {
                            log::error!("Failed to start MCP server on any port");
                        }
                    }
                }
            }
        });
    });

    // Run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(protocol.clone()) // Clone to ensure it's managed even if server fails
        .invoke_handler(tauri::generate_handler![
            commands::get_connected_clients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}