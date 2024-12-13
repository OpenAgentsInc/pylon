use serde_json::Value;

#[derive(serde::Deserialize)]
pub struct ChatParams {
    pub model: String,
    pub messages: Vec<crate::mcp::providers::ollama::ChatMessage>,
}

#[derive(serde::Deserialize)]
pub struct ListParams {
    pub path: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ReadParams {
    pub path: String,
}

#[derive(serde::Deserialize)]
pub struct WatchParams {
    pub path: String,
}

#[derive(serde::Deserialize)]
pub struct UnwatchParams {
    pub path: String,
}

pub fn create_error_response(id: Value, code: i32, message: String) -> String {
    let error = serde_json::json!({
        "jsonrpc": crate::mcp::types::JSONRPC_VERSION,
        "id": id,
        "error": {
            "code": code,
            "message": message,
            "data": null
        }
    });

    serde_json::to_string(&error).unwrap_or_else(|e| {
        format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":null,\"error\":{{\"code\":-32603,\"message\":\"Error creating error response: {}\"}}}}", 
            e
        )
    })
}