use super::*;
use crate::utils::ollama::is_ollama_running;
use crate::mcp::types::{JSONRPC_VERSION, MCP_VERSION};
use serde_json::Value;

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

    let response = protocol
        .handle_initialize("test-id", &request)
        .await
        .unwrap();

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
    let request = JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: serde_json::Value::Number(serde_json::Number::from(1)),
        method: "unknown".to_string(),
        params: serde_json::Value::Null,
    };

    let response = create_error_response(request.id, -32601, "Method not found".to_string());

    let error: Value = serde_json::from_str(&response).unwrap();
    assert_eq!(error["jsonrpc"], JSONRPC_VERSION);
    assert_eq!(error["id"], 1);
    assert_eq!(error["error"]["code"], -32601);
    assert_eq!(error["error"]["message"], "Method not found");
}

#[tokio::test]
async fn test_ollama_chat() {
    if !is_ollama_running().await {
        eprintln!("Skipping test: Ollama is not running");
        return;
    }

    let protocol = MCPProtocol::new();

    let request = JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: serde_json::Value::Number(serde_json::Number::from(1)),
        method: "ollama/chat".to_string(),
        params: serde_json::json!({
            "model": "llama3.2",
            "messages": [
                {
                    "role": "user",
                    "content": "Why is the sky blue?"
                }
            ]
        }),
    };

    let response = protocol.handle_ollama_chat(&request).await.unwrap();
    let response: Value = serde_json::from_str(&response).unwrap();

    assert_eq!(response["jsonrpc"], JSONRPC_VERSION);
    assert_eq!(response["id"], 1);
    assert!(response["result"]["message"].is_object());
    assert!(response["result"]["message"]["content"].is_string());
    assert!(!response["result"]["message"]["content"].as_str().unwrap().is_empty());
}