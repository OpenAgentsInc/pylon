use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use bytes::Bytes;
use futures_util::stream::{self, BoxStream};
use std::error::Error as StdError;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::utils::ollama::is_ollama_running;

type DynError = Box<dyn StdError + Send + Sync + 'static>;

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
struct ModelsResponse {
    models: Vec<ModelInfo>
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

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, DynError> {
        let response = self.client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to list models: {}", error_text).into());
        }

        let response_text = response.text().await?;
        let models_response: ModelsResponse = serde_json::from_str(&response_text)?;
        Ok(models_response.models)
    }

    pub async fn chat(&self, model: &str, messages: Vec<ChatMessage>) -> Result<ChatResponse, DynError> {
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
        let mut response: ChatResponse = serde_json::from_str(&response_text)?;
        response.done = true; // Single response is always done
        Ok(response)
    }

    pub async fn chat_stream(&self, model: &str, messages: Vec<ChatMessage>) -> BoxStream<'static, Result<ChatResponse, DynError>> {
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
                    return stream::once(async move { 
                        Err(Box::new(e) as DynError) 
                    }).boxed();
                }
            };

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_else(|e| e.to_string());
            return stream::once(async move {
                Err(format!("Chat stream failed: {}", error).into())
            }).boxed();
        }

        let last_response = Arc::new(Mutex::new(None));
        let last_response_clone = last_response.clone();

        let stream = response
            .bytes_stream()
            .map(|result| -> Result<ChatResponse, DynError> {
                match result {
                    Ok(bytes) => parse_stream_chunk(bytes),
                    Err(e) => Err(Box::new(e) as DynError),
                }
            })
            .boxed();

        let filtered = stream.filter_map(move |result| {
            let last_response = last_response.clone();
            async move {
                match result {
                    Ok(response) => {
                        if response.done {
                            let mut guard = last_response.lock().await;
                            *guard = Some(response.clone());
                            None
                        } else {
                            Some(Ok(response))
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            }
        });

        filtered
            .chain(stream::once(async move {
                let guard = last_response_clone.lock().await;
                Ok(guard.clone().unwrap_or(ChatResponse {
                    message: ChatMessage {
                        role: "assistant".to_string(),
                        content: String::new(),
                    },
                    done: true,
                    model: String::new(),
                    created_at: String::new(),
                }))
            }))
            .boxed()
    }
}

fn parse_stream_chunk(bytes: Bytes) -> Result<ChatResponse, DynError> {
    let response_text = String::from_utf8(bytes.to_vec())?;
    let response: ChatResponse = serde_json::from_str(&response_text)?;
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
    }
}