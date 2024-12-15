use actix_web::{App, HttpServer};
use log::info;

use crate::mcp::prompts::FileSystemPromptProvider;

pub mod mcp;

pub async fn start_server(host: &str, port: u16) -> std::io::Result<()> {
    info!("Starting MCP server on {}:{}", host, port);

    // Create the prompt provider with the current directory
    let prompt_provider = FileSystemPromptProvider::new("prompts");
    
    // Create the MCP server
    let mcp_server = mcp::server::MCPServer::new(prompt_provider);
    let configure = mcp_server.configure();

    // Start the HTTP server
    HttpServer::new(move || {
        App::new().configure(configure.clone())
    })
    .workers(4)
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}