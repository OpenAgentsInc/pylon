use reqwest::Client;
use futures_util::StreamExt;
use bytes::Bytes;
use futures_util::stream::{self, BoxStream};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json;

use super::types::*;

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
        println!("Raw response: {}", response_text);
        
        let response: OllamaResponse = serde_json::from_str(&response_text)?;
        Ok(ChatResponse {
            message: ChatMessage {
                role: "assistant".to_string(),
                content: response.response,
            },
            done: true,
            model: response.model,
            created_at: response.created_at.unwrap_or_default(),
        })
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
    println!("Raw stream chunk: {}", response_text);
    
    let response: OllamaResponse = serde_json::from_str(&response_text)?;
    Ok(ChatResponse {
        message: ChatMessage {
            role: "assistant".to_string(),
            content: response.response,
        },
        done: response.done,
        model: response.model,
        created_at: response.created_at.unwrap_or_default(),
    })
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new("http://localhost:11434".to_string())
    }
}