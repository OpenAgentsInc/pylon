use log::{error, info, debug};
use crate::mcp::types::*;
use crate::mcp::clients::ClientInfo;
use crate::mcp::providers::ResourceProvider;
use super::{MCPProtocol, types::*};

impl MCPProtocol {
    pub async fn handle_message(
        &self,
        client_id: &str,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request: JsonRpcRequest = serde_json::from_str(message)?;
        debug!("Handling message from {}: {}", client_id, request.method);

        // Only update last message if it's not an initialize request
        if request.method != "initialize" {
            self.client_manager
                .update_last_message(client_id, request.method.clone())
                .await;
        }

        match request.method.as_str() {
            "initialize" => self.handle_initialize(client_id, &request).await,
            "resource/list" => self.handle_list_resources(&request).await,
            "resource/read" => self.handle_read_resource(&request).await,
            "resource/watch" => self.handle_watch_resource(&request).await,
            "resource/unwatch" => self.handle_unwatch_resource(&request).await,
            "ollama/chat" => self.handle_ollama_chat(&request).await,
            "ollama/models" => self.handle_ollama_models(&request).await,
            "prompts/list" => self.handle_list_prompts(&request).await,
            "prompts/get" => self.handle_get_prompt(&request).await,
            _ => {
                error!("Unknown method: {}", request.method);
                Ok(create_error_response(
                    request.id.clone(),
                    -32601,
                    "Method not found".to_string(),
                ))
            }
        }
    }

    pub(crate) async fn handle_list_prompts(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let prompts = self.prompt_provider.list_prompts(None).await?;
        let response = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": request.id,
            "result": {
                "prompts": prompts.0,
                "cursor": prompts.1
            }
        });
        Ok(serde_json::to_string(&response)?)
    }

    pub(crate) async fn handle_get_prompt(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        #[derive(serde::Deserialize)]
        struct GetPromptParams {
            name: String,
            arguments: Option<std::collections::HashMap<String, String>>,
        }

        let params: GetPromptParams = serde_json::from_value(request.params.clone())?;
        let messages = self.prompt_provider.get_prompt(&params.name, params.arguments).await?;
        
        let response = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": request.id,
            "result": {
                "messages": messages
            }
        });
        Ok(serde_json::to_string(&response)?)
    }

    pub(crate) async fn handle_ollama_chat(&self, request: &JsonRpcRequest) -> Result<String, Box<dyn std::error::Error>> {
        let params: ChatParams = serde_json::from_value(request.params.clone())?;

        match self
            .ollama_provider
            .chat(&params.model, params.messages)
            .await
        {
            Ok(response) => {
                let json_response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id,
                    "result": {
                        "message": response.message,
                        "done": response.done
                    }
                });
                Ok(serde_json::to_string(&json_response)?)
            }
            Err(e) => Ok(create_error_response(
                request.id.clone(),
                -32000,
                format!("Chat error: {}", e),
            )),
        }
    }

    pub(crate) async fn handle_ollama_models(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self.ollama_provider.list_models().await {
            Ok(models) => {
                let json_response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id,
                    "result": {
                        "models": models
                    }
                });
                Ok(serde_json::to_string(&json_response)?)
            }
            Err(e) => Ok(create_error_response(
                request.id.clone(),
                -32000,
                format!("Error listing models: {}", e),
            )),
        }
    }

    pub(crate) async fn handle_initialize(
        &self,
        client_id: &str,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params: InitializeParams = serde_json::from_value(request.params.clone())?;
        info!(
            "Received initialize request from client {}: {:?}",
            client_id, params.client_info
        );

        // Store client info
        let client_info = ClientInfo {
            name: params.client_info.name.clone(),
            version: params.client_info.version.clone(),
        };

        let capabilities = ClientCapabilities {
            experimental: Some(params.capabilities.experimental.unwrap_or_default()),
            roots: Some(RootsCapability {
                list_changed: params
                    .capabilities
                    .roots
                    .map(|r| r.list_changed)
                    .unwrap_or_default(),
            }),
            sampling: Some(params.capabilities.sampling.unwrap_or_default()),
            ollama: params.capabilities.ollama,
        };

        // Add client to manager
        self.client_manager
            .add_client(client_id.to_string(), client_info, capabilities)
            .await;

        // Log current clients
        let current_clients = self.client_manager.get_clients().await;
        debug!("Current clients after adding {}: {:?}", client_id, current_clients);

        // Create initialize result
        let result = InitializeResult {
            capabilities: self.server_capabilities.clone(),
            instructions: Some("Pylon MCP Server ready for connections".to_string()),
            protocol_version: MCP_VERSION.to_string(),
            server_info: self.server_info.clone(),
        };

        // Create JSON-RPC response
        let response = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": request.id,
            "result": {
                "capabilities": result.capabilities,
                "instructions": result.instructions,
                "protocol_version": result.protocol_version,
                "server_info": result.server_info
            }
        });

        Ok(serde_json::to_string(&response)?)
    }

    pub(crate) async fn handle_list_resources(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params: ListParams = serde_json::from_value(request.params.clone())?;
        let path = params.path.unwrap_or_else(|| ".".to_string());

        match self.fs_provider.as_ref().list(&path).await {
            Ok(resources) => {
                let response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id.clone(),
                    "result": resources
                });
                Ok(serde_json::to_string(&response)?)
            }
            Err(e) => Ok(create_error_response(
                request.id.clone(),
                -32000,
                format!("Error listing resources: {}", e),
            )),
        }
    }

    pub(crate) async fn handle_read_resource(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params: ReadParams = serde_json::from_value(request.params.clone())?;

        match self.fs_provider.as_ref().read(&params.path).await {
            Ok(contents) => {
                let response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id.clone(),
                    "result": contents
                });
                Ok(serde_json::to_string(&response)?)
            }
            Err(e) => Ok(create_error_response(
                request.id.clone(),
                -32000,
                format!("Error reading resource: {}", e),
            )),
        }
    }

    pub(crate) async fn handle_watch_resource(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params: WatchParams = serde_json::from_value(request.params.clone())?;

        match self.fs_provider.as_ref().watch(&params.path).await {
            Ok(()) => {
                let response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id.clone(),
                    "result": true
                });
                Ok(serde_json::to_string(&response)?)
            }
            Err(e) => Ok(create_error_response(
                request.id.clone(),
                -32000,
                format!("Error watching resource: {}", e),
            )),
        }
    }

    pub(crate) async fn handle_unwatch_resource(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params: UnwatchParams = serde_json::from_value(request.params.clone())?;

        match self.fs_provider.as_ref().unwatch(&params.path).await {
            Ok(()) => {
                let response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id.clone(),
                    "result": true
                });
                Ok(serde_json::to_string(&response)?)
            }
            Err(e) => Ok(create_error_response(
                request.id.clone(),
                -32000,
                format!("Error unwatching resource: {}", e),
            )),
        }
    }
}