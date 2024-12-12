use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::Message;
use futures_util::StreamExt;
use log::{error, info};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::sync::Arc;

use super::protocol::MCPProtocol;

static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

pub struct MCPServer {
    port: u16,
    protocol: Arc<MCPProtocol>,
}

impl MCPServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            protocol: Arc::new(MCPProtocol::new()),
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        info!("Starting MCP server on port {}", self.port);

        let protocol = Arc::clone(&self.protocol);
        let factory = move || {
            let protocol = Arc::clone(&protocol);
            App::new()
                .app_data(web::Data::new(protocol))
                .route("/mcp", web::get().to(handle_connection))
        };

        HttpServer::new(factory)
            .workers(2)
            .bind(("127.0.0.1", self.port))?
            .run()
            .await
    }
}

async fn handle_connection(
    req: HttpRequest,
    body: web::Payload,
    protocol: web::Data<Arc<MCPProtocol>>,
) -> Result<HttpResponse, Error> {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    info!("New client connection: {}", client_id);

    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
    let protocol = Arc::clone(&protocol);

    // Spawn client handler
    actix_web::rt::spawn(async move {
        let mut last_heartbeat = Instant::now();

        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    info!("Client {}: Received text message: {}", client_id, text);
                    
                    // Handle message using protocol
                    match protocol.handle_message(&text) {
                        Ok(response) => {
                            if let Err(e) = session.text(response).await {
                                error!("Error sending response: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error handling message: {}", e);
                            // Send error response
                            if let Err(e) = session.text(format!("{{\"jsonrpc\":\"2.0\",\"id\":null,\"error\":{{\"code\":-32603,\"message\":\"Internal error: {}\"}}}}",
                                e
                            )).await {
                                error!("Error sending error response: {}", e);
                                break;
                            }
                        }
                    }
                }
                Message::Binary(bin) => {
                    info!("Client {}: Received binary message", client_id);
                    
                    if let Err(e) = session.binary(bin).await {
                        error!("Error sending binary: {}", e);
                        break;
                    }
                }
                Message::Ping(bytes) => {
                    last_heartbeat = Instant::now();
                    if let Err(e) = session.pong(&bytes).await {
                        error!("Error sending pong: {}", e);
                        break;
                    }
                }
                Message::Pong(_) => {
                    last_heartbeat = Instant::now();
                }
                Message::Close(reason) => {
                    info!("Client {} disconnected: {:?}", client_id, reason);
                    break;
                }
                Message::Continuation(_) => {
                    info!("Client {}: Received continuation frame", client_id);
                }
                Message::Nop => (),
            }

            // Check heartbeat
            if Instant::now().duration_since(last_heartbeat) > Duration::from_secs(10) {
                error!("Client {} heartbeat missing, disconnecting!", client_id);
                break;
            }
        }

        // Send close message
        let _ = session.close(None).await;
        info!("Client {} connection closed", client_id);
    });

    Ok(response)
}