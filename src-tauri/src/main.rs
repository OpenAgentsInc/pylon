use log::info;
use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;
use pylon_lib::mcp::MCPServer;

mod commands;

fn main() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting MCP server...");
    
    // Create MCP server instance
    let mcp_server = Arc::new(MCPServer::new());
    let protocol = mcp_server.protocol.clone();
    
    // Create a channel to signal server start
    let (tx, rx) = mpsc::channel();
    
    // Start MCP server in a separate thread
    let server_clone = mcp_server.clone();
    thread::spawn(move || {
        let system = actix_web::rt::System::new();
        system.block_on(async {
            // Try ports 8080, 8081, 8082 in sequence
            let mut server_started = false;
            
            for port in 8080..8083 {
                match server_clone.start("0.0.0.0", port).await {
                    Ok(_) => {
                        info!("MCP server started successfully on port {}", port);
                        server_started = true;
                        // Send confirmation immediately after successful start
                        tx.send(Some(port)).unwrap_or_default();
                        break;
                    },
                    Err(e) => {
                        log::error!("Failed to start MCP server on port {}: {}", port, e);
                        if port == 8082 {
                            log::error!("Failed to start MCP server on any port");
                            tx.send(None).unwrap_or_default();
                        }
                    }
                }
            }
            
            // Keep the system running if server started
            if server_started {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });
    });

    // Wait for server to start with a timeout
    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(Some(port)) => {
            info!("Server confirmed running on port {}", port);
            
            // Run Tauri application only after server is confirmed running
            tauri::Builder::default()
                .plugin(tauri_plugin_shell::init())
                .manage(protocol) // protocol is already an Arc<MCPProtocol>
                .invoke_handler(tauri::generate_handler![
                    commands::get_connected_clients
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        },
        Ok(None) => {
            log::error!("Failed to start server on any port");
            std::process::exit(1);
        },
        Err(_) => {
            log::error!("Timeout waiting for server to start");
            std::process::exit(1);
        }
    }
}