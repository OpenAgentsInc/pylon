use log::info;

fn main() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting MCP server...");
    
    // Create a new tokio runtime for the MCP server
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    
    // Spawn MCP server in the background
    rt.spawn(async {
        if let Err(e) = pylon_lib::start_mcp_server("127.0.0.1".to_string(), 8080).await {
            log::error!("MCP server error: {}", e);
        }
    });

    // Run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}