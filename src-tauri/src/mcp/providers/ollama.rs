use std::error::Error;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use log::{error, info};

use super::ResourceError;

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

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Debug)]
pub struct OllamaProvider {
    endpoint: String,
    client: Client,
}

impl OllamaProvider {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: Client::new(),
        }
    }

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, Box<dyn Error>> {
        let response = self.client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await?
            .json::<Vec<ModelInfo>>()
            .await?;

        Ok(response)
    }

    pub async fn chat(&self, model: &str, messages: Vec<ChatMessage>) -> Result<ChatResponse, Box<dyn Error>> {
        let request = ChatRequest {
            model: model.to_string(),
            messages,
        };

        let response = self.client
            .post(format!("{}/api/chat", self.endpoint))
            .json(&request)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        Ok(response)
    }

    pub async fn chat_stream(&self, model: &str, messages: Vec<ChatMessage>) -> impl futures::Stream<Item = Result<ChatResponse, Box<dyn Error>>> {
        let request = ChatRequest {
            model: model.to_string(),
            messages,
        };

        let response_stream = self.client
            .post(format!("{}/api/chat", self.endpoint))
            .json(&request)
            .send()
            .await
            .unwrap() // TODO: Better error handling
            .bytes_stream()
            .map(|result| {
                result.map_err(|e| Box::new(e) as Box<dyn Error>)
                    .and_then(|bytes| {
                        serde_json::from_slice::<ChatResponse>(&bytes)
                            .map_err(|e| Box::new(e) as Box<dyn Error>)
                    })
            });

        response_stream
    }
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new("http://localhost:11434".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_models() {
        let provider = OllamaProvider::default();
        let models = provider.list_models().await.unwrap();
        assert!(!models.is_empty());
    }

    #[tokio::test]
    async fn test_chat() {
        let provider = OllamaProvider::default();
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Why is the sky blue?".to_string(),
            }
        ];

        let response = provider.chat("llama3.2", messages).await.unwrap();
        assert!(!response.message.content.is_empty());
    }
}