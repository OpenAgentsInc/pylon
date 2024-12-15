use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Data, Payload};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::mcp::protocol::RequestHandler;
use crate::mcp::clients::ClientManager;
use crate::mcp::prompts::FileSystemPromptProvider;

pub struct MCPServer {
    handler: Arc<RequestHandler>,
}

impl MCPServer {
    pub fn new(prompt_provider: FileSystemPromptProvider) -> Self {
        let client_manager = ClientManager::new();
        let handler = RequestHandler::new(client_manager, prompt_provider);
        Self {
            handler: Arc::new(handler),
        }
    }

    pub fn configure(self) -> impl FnOnce(&mut web::ServiceConfig) {
        let handler = self.handler;
        move |cfg: &mut web::ServiceConfig| {
            cfg.app_data(Data::new(handler))
                .route("/mcp", web::get().to(handle_connection));
        }
    }
}

async fn handle_connection(
    req: HttpRequest,
    payload: Payload,
    handler: Data<Arc<RequestHandler>>,
) -> Result<HttpResponse, actix_web::Error> {
    let client_id = Uuid::new_v4().to_string();
    info!("New WebSocket connection: {}", client_id);

    let (response, mut session, mut msg_stream) = actix_web::web::WebSocket::handle_request(req, payload)?;

    // Spawn task to handle WebSocket messages
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                actix_web::ws::Message::Text(text) => {
                    debug!("Received message from {}: {}", client_id, text);
                    match handler.handle_message(&client_id, &text).await {
                        Ok(response) => {
                            if let Err(e) = session.text(response).await {
                                error!("Error sending response: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error handling message: {}", e);
                            if let Err(e) = session.text(format!("Error: {}", e)).await {
                                error!("Error sending error response: {}", e);
                                break;
                            }
                        }
                    }
                }
                actix_web::ws::Message::Close(reason) => {
                    info!(
                        "Client {} disconnected with reason: {:?}",
                        client_id, reason
                    );
                    break;
                }
                _ => {}
            }
        }

        info!("Client {} connection ended, cleaning up", client_id);
    });

    Ok(response)
}