use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures_util::{Stream, StreamExt};
use bytes::Bytes;
use log::warn;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
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
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to list models: {}", error_text).into());
        }

        let response_text = response.text().await?;
        let response: ListModelsResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                warn!("Failed to parse models response: {}\nResponse text: {}", e, response_text);
                e
            })?;

        Ok(response.models)
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
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Chat failed: {}", error_text).into());
        }

        let response_text = response.text().await?;
        let response: ChatResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                warn!("Failed to parse chat response: {}\nResponse text: {}", e, response_text);
                e
            })?;

        Ok(response)
    }

    pub async fn chat_stream(&self, model: &str, messages: Vec<ChatMessage>) -> impl Stream<Item = Result<ChatResponse, Box<dyn Error>>> {
        let request = ChatRequest {
            model: model.to_string(),
            messages,
        };

        let response = match self.client
            .post(format!("{}/api/chat", self.endpoint))
            .json(&request)
            .send()
            .await {
                Ok(r) => r,
                Err(e) => {
                    warn!("Failed to start chat stream: {}", e);
                    return futures_util::stream::once(async move { Err(Box::new(e) as Box<dyn Error>) }).boxed();
                }
            };

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_else(|e| e.to_string());
            return futures_util::stream::once(async move {
                Err(format!("Chat stream failed: {}", error).into())
            }).boxed();
        }

        response
            .bytes_stream()
            .map(|result| {
                result
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
                    .and_then(|bytes| parse_stream_chunk(bytes))
            })
            .boxed()
    }
}

fn parse_stream_chunk(bytes: Bytes) -> Result<ChatResponse, Box<dyn Error>> {
    let response_text = String::from_utf8(bytes.to_vec())?;
    let response: ChatResponse = serde_json::from_str(&response_text)
        .map_err(|e| {
            warn!("Failed to parse stream chunk: {}\nChunk text: {}", e, response_text);
            e
        })?;
    Ok(response)
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new("http://localhost:11434".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use std::time::Duration;
    use crate::utils::ollama::is_ollama_running;

    async fn skip_if_ollama_not_running() {
        if !is_ollama_running().await {
            eprintln!("Skipping test: Ollama is not running");
            return;
        }
    }

    #[tokio::test]
    async fn test_list_models() {
        if !is_ollama_running().await {
            eprintln!("Skipping test: Ollama is not running");
            return;
        }

        let provider = OllamaProvider::default();
        let models = provider.list_models().await.unwrap();
        assert!(!models.is_empty());
        println!("Available models: {:?}", models);
    }

    #[tokio::test]
    async fn test_chat() {
        if !is_ollama_running().await {
            eprintln!("Skipping test: Ollama is not running");
            return;
        }

        let provider = OllamaProvider::default();
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Why is the sky blue?".to_string(),
            }
        ];

        let response = provider.chat("llama3.2", messages).await.unwrap();
        assert!(!response.message.content.is_empty());
        println!("Chat response: {}", response.message.content);
    }

    #[tokio::test]
    async fn test_chat_stream() {
        if !is_ollama_running().await {
            eprintln!("Skipping test: Ollama is not running");
            return;
        }

        let provider = OllamaProvider::default();
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Count from 1 to 5.".to_string(),
            }
        ];

        let mut stream = provider.chat_stream("llama3.2", messages).await;
        let mut response_parts = Vec::new();
        let mut full_response = String::new();

        while let Some(Ok(response)) = stream.next().await {
            response_parts.push(response.message.content.clone());
            full_response.push_str(&response.message.content);
            if response.done {
                break;
            }
            sleep(Duration::from_millis(10)).await;
        }

        assert!(!response_parts.is_empty());
        println!("Full streaming response: {}", full_response);
        assert!(full_response.contains("1") && full_response.contains("5"));
    }
}