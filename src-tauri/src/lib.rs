use log::info;
use std::sync::Arc;
use actix_web::rt as actix_rt;

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
    
    // Create MCP server
    let mcp_server = Arc::new(mcp::MCPServer::new(8080));
    let mcp_server_clone = Arc::clone(&mcp_server);

    // Start actix system in a separate thread
    std::thread::spawn(move || {
        actix_rt::System::new().block_on(async {
            info!("Starting MCP server...");
            if let Err(e) = mcp_server_clone.run().await {
                log::error!("MCP server error: {}", e);
            }
        });
    });

    // Run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}