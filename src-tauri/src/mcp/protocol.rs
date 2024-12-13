use crate::mcp::types::*;
use crate::mcp::providers::{filesystem::FileSystemProvider, ollama::{OllamaProvider, ChatMessage}, ResourceProvider};
use crate::mcp::clients::{ClientManager, ClientInfo};
use log::{error, info};
use serde_json::Value;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

// ... [previous code remains the same until create_error_response] ...

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
            "{{\"jsonrpc\":\"2.0\",\"id\":null,\"error\":{{\"code\":-32603,\"message\":\"Error creating error response: {}\"}}}}", 
            e
        )
    })
}