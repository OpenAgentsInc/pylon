use crate::mcp::types::*;
use log::{error, info};
use serde_json::Value;
use std::error::Error;

pub struct MCPProtocol {
    server_info: Implementation,
    server_capabilities: ServerCapabilities,
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
        }
    }

    pub fn handle_message(&self, message: &str) -> Result<String, Box<dyn Error>> {
        let request: JsonRpcRequest = serde_json::from_str(message)?;

        match request.method.as_str() {
            "initialize" => self.handle_initialize(&request),
            _ => {
                error!("Unknown method: {}", request.method);
                Ok(self.create_error_response(request.id, -32601, "Method not found".to_string()))
            }
        }
    }

    fn handle_initialize(&self, request: &JsonRpcRequest) -> Result<String, Box<dyn Error>> {
        let params: InitializeParams = serde_json::from_value(request.params.clone())?;
        info!(
            "Received initialize request from client: {:?}",
            params.client_info
        );

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
                r#"{{"jsonrpc":"2.0","id":null,"error":{{"code":-32603,"message":"Error creating error response: {}"}}"#,
                e
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_request() {
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

        let message = serde_json::to_string(&request).unwrap();
        let response = protocol.handle_message(&message).unwrap();

        let response: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["jsonrpc"], JSONRPC_VERSION);
        assert_eq!(response["id"], 1);

        let result = &response["result"];
        assert!(result.is_object());
        assert!(result["capabilities"].is_object());
        assert_eq!(result["protocol_version"], MCP_VERSION);
        assert!(result["server_info"].is_object());
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

        let message = serde_json::to_string(&request).unwrap();
        let response = protocol.handle_message(&message).unwrap();

        let error: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(error["jsonrpc"], JSONRPC_VERSION);
        assert_eq!(error["id"], 1);
        assert_eq!(error["error"]["code"], -32601);
        assert_eq!(error["error"]["message"], "Method not found");
    }
}
