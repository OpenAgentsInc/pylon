use std::sync::Arc;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::Message;
use futures_util::StreamExt as _;
use log::{error, info};
use tokio::net::TcpListener;

use super::protocol::MCPProtocol;

pub struct MCPServer {
    pub protocol: Arc<MCPProtocol>,
}

impl MCPServer {
    pub fn new() -> Self {
        Self {
            protocol: Arc::new(MCPProtocol::new()),
        }
    }

    pub async fn start(&self, host: &str, port: u16) -> std::io::Result<()> {
        let protocol = self.protocol.clone();

        info!("Starting MCP server on {}:{}", host, port);

        // First check if we can bind to the port
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&addr).await?;
        drop(listener); // Release the port

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(protocol.clone()))
                .route("/mcp", web::get().to(handle_connection))
        })
        .workers(4) // Reduce number of workers
        .bind(&addr)?;

        // Spawn the server in a background task
        tokio::spawn(server.run());

        // Add a small delay to ensure server is fully initialized
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(())
    }
}

impl Default for MCPServer {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn handle_connection(
    req: HttpRequest,
    body: web::Payload,
    protocol: web::Data<Arc<MCPProtocol>>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let client_id = uuid::Uuid::new_v4().to_string();
    info!("New WebSocket connection: {}", client_id);

    // Spawn client handler
    let protocol_clone = protocol.clone();
    let client_id_clone = client_id.clone();
    
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    info!("Received message from {}: {}", client_id, text);

                    match protocol.handle_message(&client_id, &text).await {
                        Ok(response) => {
                            if let Err(e) = session.text(response).await {
                                error!("Error sending response to {}: {}", client_id, e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error handling message from {}: {}", client_id, e);
                            if let Err(e) = session
                                .text(format!("{{\"error\": \"{}\"}}", e.to_string()))
                                .await
                            {
                                error!("Error sending error response to {}: {}", client_id, e);
                                break;
                            }
                        }
                    }
                }
                Message::Close(reason) => {
                    info!(
                        "Client {} disconnected: {:?}",
                        client_id,
                        reason
                    );
                    break;
                }
                Message::Ping(bytes) => {
                    if let Err(e) = session.pong(&bytes).await {
                        error!("Error sending pong to {}: {}", client_id, e);
                        break;
                    }
                }
                _ => {}
            }
        }

        // Clean up when client disconnects
        protocol_clone.get_client_manager().remove_client(&client_id_clone).await;
        
        if let Err(e) = session.close(None).await {
            error!("Error closing session for {}: {}", client_id, e);
        }
    });

    Ok(response)
}