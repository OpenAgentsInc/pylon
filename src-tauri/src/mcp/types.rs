use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implementation {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ollama: Option<OllamaCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootsCapability {
    pub list_changed: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ollama: Option<OllamaCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaCapability {
    pub available_models: Vec<String>,
    pub endpoint: String,
    pub streaming: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptsCapability {
    pub list_changed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesCapability {
    pub list_changed: bool,
    pub subscribe: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCapability {
    pub list_changed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub method: String,
    pub params: InitializeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "serverInfo")]
    pub server_info: Implementation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Assistant,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

// Resource content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceContents {
    Text(TextResourceContents),
    Blob(BlobResourceContents),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextResourceContents {
    pub uri: String,
    pub mime_type: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobResourceContents {
    pub uri: String,
    pub mime_type: Option<String>,
    pub blob: Vec<u8>,
}

// JSON-RPC types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub error: JsonRpcErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcErrorDetail {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

// Constants
pub const JSONRPC_VERSION: &str = "2.0";
pub const MCP_VERSION: &str = "0.1.0";

impl Default for Implementation {
    fn default() -> Self {
        Self {
            name: "pylon".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}