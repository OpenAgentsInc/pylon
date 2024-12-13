use std::sync::Arc;
use log::error;

pub mod mcp;
pub mod tests;

pub async fn start_mcp_server(host: &str, port: u16) -> std::io::Result<()> {
    let mcp_server = Arc::new(mcp::MCPServer::new());
    let mcp_server_clone = mcp_server.clone();

    tokio::spawn(async move {
        if let Err(e) = mcp_server_clone.start(host, port).await {
            error!("MCP server error: {}", e);
        }
    });

    Ok(())
}