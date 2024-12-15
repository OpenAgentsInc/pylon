use log::{debug, info, error};
use serde_json::Value;
use std::collections::HashMap;

use crate::mcp::prompts::{provider::PromptProvider, FileSystemPromptProvider};
use crate::mcp::types::{JsonRpcRequest, JSONRPC_VERSION, ClientCapabilities};
use crate::mcp::{
    clients::{ClientManager, ClientInfo},
    types::Implementation,
};
use super::types::create_error_response;

pub struct RequestHandler {
    client_manager: ClientManager,
    prompt_provider: FileSystemPromptProvider,
}

impl RequestHandler {
    pub fn new(client_manager: ClientManager, prompt_provider: FileSystemPromptProvider) -> Self {
        Self {
            client_manager,
            prompt_provider,
        }
    }

    pub async fn handle_message(
        &self,
        client_id: &str,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request: JsonRpcRequest = serde_json::from_str(message)?;
        debug!("Handling message from {}: {}", client_id, request.method);

        match request.method.as_str() {
            "initialize" => self.handle_initialize(&request, client_id).await,
            "prompts/list" => self.handle_list_prompts(&request).await,
            "prompts/get" => self.handle_get_prompt(&request).await,
            "ping" => self.handle_ping(&request).await,
            _ => {
                error!("Unknown method: {}", request.method);
                Ok(create_error_response(
                    request.id,
                    -32601,
                    "Method not found".to_string(),
                ))
            }
        }
    }

    pub async fn handle_initialize(
        &self,
        request: &JsonRpcRequest,
        client_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params = request.params.as_object().ok_or("Invalid params")?;
        let client_info = serde_json::from_value::<Implementation>(
            params
                .get("clientInfo")
                .ok_or("Missing clientInfo")?
                .clone(),
        )?;

        info!(
            "Received initialize request from client {}: {:?}",
            client_id, client_info
        );

        // Convert Implementation to ClientInfo
        let client_info = ClientInfo {
            name: client_info.name.clone(),
            version: client_info.version.clone(),
        };

        // Add the client with default capabilities
        self.client_manager.add_client(
            client_id.to_string(), 
            client_info,
            ClientCapabilities::default()
        ).await;

        // Send response with server info and capabilities
        let response = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": request.id,
            "result": {
                "serverInfo": Implementation {
                    name: "pylon".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                },
                "protocolVersion": "0.1.0",
                "capabilities": {
                    "prompts": {
                        "list_changed": true
                    }
                }
            }
        });

        Ok(response.to_string())
    }

    pub async fn handle_get_prompt(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params = request.params.as_object().ok_or("Invalid params")?;
        let name = params.get("name").ok_or("Missing name")?.as_str().ok_or("Invalid name")?;
        let arguments = params.get("arguments").map(|args| {
            args.as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect::<HashMap<String, String>>()
                })
                .unwrap_or_default()
        });

        debug!("Getting prompt {} with arguments {:?}", name, arguments);
        let messages = self.prompt_provider.get_prompt(name, arguments).await?;

        let response = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": request.id,
            "result": {
                "messages": messages
            }
        });

        Ok(response.to_string())
    }

    pub async fn handle_list_prompts(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cursor = request
            .params
            .get("cursor")
            .and_then(Value::as_str)
            .map(String::from);

        let (prompts, next_cursor) = self.prompt_provider.list_prompts(cursor).await?;

        let response = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": request.id,
            "result": {
                "prompts": prompts,
                "nextCursor": next_cursor
            }
        });

        Ok(response.to_string())
    }

    pub async fn handle_ping(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let response = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": request.id,
            "result": {}
        });

        Ok(response.to_string())
    }
}