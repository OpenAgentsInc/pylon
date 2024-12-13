use crate::mcp::types::*;
use crate::mcp::providers::{filesystem::FileSystemProvider, ResourceProvider};
use crate::mcp::clients::{ClientManager, ClientInfo, ClientCapabilities, RootsCapability};
use log::{error, info};
use serde_json::Value;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

pub struct MCPProtocol {
    server_info: Implementation,
    server_capabilities: ServerCapabilities,
    fs_provider: Arc<FileSystemProvider>,
    client_manager: Arc<ClientManager>,
}

impl Default for MCPProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl MCPProtocol {
    pub fn new() -> Self {
        Self {
            server_info: Implementation::default(),
            server_capabilities: ServerCapabilities {
                resources: Some(ResourcesCapability {
                    list_changed: true,
                    subscribe: true,
                }),
                tools: Some(ToolsCapability { list_changed: true }),
                prompts: Some(PromptsCapability { list_changed: true }),
                ..Default::default()
            },
            fs_provider: Arc::new(FileSystemProvider::new(PathBuf::from("/Users/christopherdavid/code/pylon"))),
            client_manager: Arc::new(ClientManager::new()),
        }
    }

    pub fn get_client_manager(&self) -> Arc<ClientManager> {
        self.client_manager.clone()
    }

    pub async fn handle_message(&self, client_id: &str, message: &str) -> Result<String, Box<dyn Error>> {
        let request: JsonRpcRequest = serde_json::from_str(message)?;

        // Update last message for client
        self.client_manager.update_last_message(client_id, request.method.clone()).await;

        match request.method.as_str() {
            "initialize" => self.handle_initialize(client_id, &request).await,
            "resource/list" => self.handle_list_resources(&request).await,
            "resource/read" => self.handle_read_resource(&request).await,
            "resource/watch" => self.handle_watch_resource(&request).await,
            "resource/unwatch" => self.handle_unwatch_resource(&request).await,
            _ => {
                error!("Unknown method: {}", request.method);
                Ok(self.create_error_response(request.id.clone(), -32601, "Method not found".to_string()))
            }
        }
    }

    async fn handle_initialize(&self, client_id: &str, request: &JsonRpcRequest) -> Result<String, Box<dyn Error>> {
        let params: InitializeParams = serde_json::from_value(request.params.clone())?;
        info!(
            "Received initialize request from client: {:?}",
            params.client_info
        );

        // Store client info
        let client_info = ClientInfo {
            name: params.client_info.name.clone(),
            version: params.client_info.version.clone(),
        };

        let capabilities = ClientCapabilities {
            experimental: Some(params.capabilities.experimental.unwrap_or_default()),
            roots: Some(RootsCapability {
                list_changed: params.capabilities.roots
                    .map(|r| r.list_changed)
                    .unwrap_or_default(),
            }),
            sampling: Some(params.capabilities.sampling.unwrap_or_default()),
        };

        self.client_manager.add_client(
            client_id.to_string(),
            client_info,
            capabilities,
        ).await;

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

    async fn handle_list_resources(&self, request: &JsonRpcRequest) -> Result<String, Box<dyn Error>> {
        #[derive(serde::Deserialize)]
        struct ListParams {
            path: Option<String>,
        }

        let params: ListParams = serde_json::from_value(request.params.clone())?;
        let path = params.path.unwrap_or_else(|| ".".to_string());

        match self.fs_provider.list(&path).await {
            Ok(resources) => {
                let response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id.clone(),
                    "result": resources
                });
                Ok(serde_json::to_string(&response)?)
            }
            Err(e) => Ok(self.create_error_response(
                request.id.clone(),
                -32000,
                format!("Error listing resources: {}", e),
            )),
        }
    }

    async fn handle_read_resource(&self, request: &JsonRpcRequest) -> Result<String, Box<dyn Error>> {
        #[derive(serde::Deserialize)]
        struct ReadParams {
            path: String,
        }

        let params: ReadParams = serde_json::from_value(request.params.clone())?;

        match self.fs_provider.read(&params.path).await {
            Ok(contents) => {
                let response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id.clone(),
                    "result": contents
                });
                Ok(serde_json::to_string(&response)?)
            }
            Err(e) => Ok(self.create_error_response(
                request.id.clone(),
                -32000,
                format!("Error reading resource: {}", e),
            )),
        }
    }

    async fn handle_watch_resource(&self, request: &JsonRpcRequest) -> Result<String, Box<dyn Error>> {
        #[derive(serde::Deserialize)]
        struct WatchParams {
            path: String,
        }

        let params: WatchParams = serde_json::from_value(request.params.clone())?;

        match self.fs_provider.watch(&params.path).await {
            Ok(()) => {
                let response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id.clone(),
                    "result": true
                });
                Ok(serde_json::to_string(&response)?)
            }
            Err(e) => Ok(self.create_error_response(
                request.id.clone(),
                -32000,
                format!("Error watching resource: {}", e),
            )),
        }
    }

    async fn handle_unwatch_resource(&self, request: &JsonRpcRequest) -> Result<String, Box<dyn Error>> {
        #[derive(serde::Deserialize)]
        struct UnwatchParams {
            path: String,
        }

        let params: UnwatchParams = serde_json::from_value(request.params.clone())?;

        match self.fs_provider.unwatch(&params.path).await {
            Ok(()) => {
                let response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id.clone(),
                    "result": true
                });
                Ok(serde_json::to_string(&response)?)
            }
            Err(e) => Ok(self.create_error_response(
                request.id.clone(),
                -32000,
                format!("Error unwatching resource: {}", e),
            )),
        }
    }

    fn create_error_response(&self, id: Value, code: i32, message: String) -> String {
        let error = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": id,
            "error": {
                "code": code,
                "message": message,
                "data": null
            }
        });

        serde_json::to_string(&error).unwrap_or_else(|e| {
            format!(
                r#"{{"jsonrpc":"2.0","id":null,"error":{{"code":-32603,"message":"Error creating error response: {}"}}"}}"#,
                e
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_request() {
        let protocol = MCPProtocol::new();

        let request = JsonRpcRequest {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: serde_json::Value::Number(serde_json::Number::from(1)),
            method: "initialize".to_string(),
            params: serde_json::to_value(InitializeParams {
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: "test-client".to_string(),
                    version: "1.0.0".to_string(),
                },
                protocol_version: MCP_VERSION.to_string(),
            })
            .unwrap(),
        };

        let response = protocol.handle_initialize("test-id", &request).await.unwrap();

        let response: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["jsonrpc"], JSONRPC_VERSION);
        assert_eq!(response["id"], 1);

        let result = &response["result"];
        assert!(result.is_object());
        assert!(result["capabilities"].is_object());
        assert_eq!(result["protocol_version"], MCP_VERSION);
        assert!(result["server_info"].is_object());

        // Verify client was added
        let clients = protocol.get_client_manager().get_clients().await;
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0].id, "test-id");
        assert_eq!(clients[0].client_info.name, "test-client");
    }

    #[test]
    fn test_unknown_method() {
        let protocol = MCPProtocol::new();

        let request = JsonRpcRequest {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: serde_json::Value::Number(serde_json::Number::from(1)),
            method: "unknown".to_string(),
            params: serde_json::Value::Null,
        };

        let response = protocol.create_error_response(
            request.id,
            -32601,
            "Method not found".to_string()
        );

        let error: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(error["jsonrpc"], JSONRPC_VERSION);
        assert_eq!(error["id"], 1);
        assert_eq!(error["error"]["code"], -32601);
        assert_eq!(error["error"]["message"], "Method not found");
    }
}