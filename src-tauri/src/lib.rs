use std::sync::Arc;

pub mod mcp;
pub mod utils;

#[cfg(test)]
pub mod tests;

pub async fn start_mcp_server(host: String, port: u16) -> std::io::Result<()> {
    let mcp_server = Arc::new(mcp::server::MCPServer::new());
    
    // Run the server directly instead of spawning
    mcp_server.start(&host, port).await
}