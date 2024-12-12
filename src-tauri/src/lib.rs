use log::info;

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
    
    // Create and spawn MCP server
    let mcp_server = mcp::MCPServer::new(8080);
    let mcp_handle = tokio::spawn(async move {
        info!("Starting MCP server...");
        if let Err(e) = mcp_server.run().await {
            log::error!("MCP server error: {}", e);
        }
    });

    // Run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Ensure MCP server is properly shutdown
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            if let Err(e) = mcp_handle.await {
                log::error!("Error joining MCP server task: {}", e);
            }
        });
}