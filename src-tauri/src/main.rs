use log::info;
use std::thread;

fn main() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting MCP server...");
    
    // Start MCP server in a separate thread
    thread::spawn(|| {
        let system = actix_web::rt::System::new();
        system.block_on(async {
            if let Err(e) = pylon_lib::start_mcp_server("127.0.0.1".to_string(), 8080).await {
                log::error!("MCP server error: {}", e);
            }
        });
    });

    // Run Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}