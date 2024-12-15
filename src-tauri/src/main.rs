// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::env;
use log::info;
use tokio::sync::RwLock;

use pylon_lib::mcp::prompts::FileSystemPromptProvider;
use pylon_lib::mcp::server::MCPServer;
use pylon_lib::start_server;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    // Get the port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    info!("Starting server on port {}", port);

    // Create the prompt provider with the current directory
    let prompt_provider = FileSystemPromptProvider::new("prompts");
    
    // Create and start the server
    let server_result = start_server("0.0.0.0", port).await;
    
    if let Err(e) = server_result {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}