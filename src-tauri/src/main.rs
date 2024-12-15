// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::env;
use log::info;
use actix_web::{App, HttpServer};

use pylon_lib::mcp::prompts::FileSystemPromptProvider;
use pylon_lib::mcp::server::MCPServer;

mod commands;

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
    
    // Create the MCP server
    let mcp_server = MCPServer::new(prompt_provider);
    let handler = mcp_server.get_handler();

    // Create and start the server in a background task
    let server_handle = {
        let configure = mcp_server.configure();
        tokio::spawn(async move {
            HttpServer::new(move || {
                App::new().configure(configure.clone())
            })
            .workers(4)
            .bind(format!("0.0.0.0:{}", port))?
            .run()
            .await
        })
    };

    // Start Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(handler)
        .invoke_handler(tauri::generate_handler![
            commands::get_connected_clients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Wait for server to finish
    if let Err(e) = server_handle.await.unwrap() {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}