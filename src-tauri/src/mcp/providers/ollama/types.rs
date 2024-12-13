use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

pub type DynError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub done: bool,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub digest: String,
    pub size: u64,
    pub modified_at: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ModelsResponse {
    pub models: Vec<ModelInfo>
}

#[derive(Debug, Deserialize)]
pub(crate) struct OllamaResponse {
    pub model: String,
    pub message: ChatMessage,
    pub done: bool,
    pub created_at: Option<String>,
}