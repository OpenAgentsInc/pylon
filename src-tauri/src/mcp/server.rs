use std::sync::Arc;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::Message;
use futures_util::{StreamExt as _, SinkExt};
use log::{error, info};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::protocol::MCPProtocol;

pub struct MCPServer {
    protocol: Arc<MCPProtocol>,
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

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(protocol.clone()))
                .route("/mcp", web::get().to(handle_connection))
        })
        .bind((host, port))?
        .run()
        .await
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

    let client_id = Uuid::new_v4();
    info!("New WebSocket connection: {}", client_id);

    // Spawn client handler
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    info!("Received message from {}: {}", client_id, text);

                    match protocol.handle_message(&text).await {
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
                        reason.map(|r| r.to_string())
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
        if let Err(e) = session.close(None).await {
            error!("Error closing session for {}: {}", client_id, e);
        }
    });

    Ok(response)
}