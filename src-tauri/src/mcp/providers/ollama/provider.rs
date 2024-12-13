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
        let mut stream = self.chat_stream(model, messages).await;
        let mut final_content = String::new();
        let mut last_response: Option<ChatResponse> = None;

        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    final_content.push_str(&response.message.content);
                    last_response = Some(response);
                    
                    if response.done {
                        return Ok(ChatResponse {
                            message: ChatMessage {
                                role: "assistant".to_string(),
                                content: final_content,
                            },
                            done: true,
                            model: response.model,
                            created_at: response.created_at,
                        });
                    }
                },
                Err(e) => return Err(e),
            }
        }

        match last_response {
            Some(last) => Ok(ChatResponse {
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: final_content,
                },
                done: true,
                model: last.model,
                created_at: last.created_at,
            }),
            None => Err("No response received from stream".into())
        }
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

        let stream = response
            .bytes_stream()
            .map(|result| -> Result<ChatResponse, DynError> {
                match result {
                    Ok(bytes) => parse_stream_chunk(bytes),
                    Err(e) => Err(Box::new(e) as DynError),
                }
            })
            .boxed();

        stream.boxed()
    }
}

fn parse_stream_chunk(bytes: Bytes) -> Result<ChatResponse, DynError> {
    let response_text = String::from_utf8(bytes.to_vec())?;
    
    let response: OllamaResponse = serde_json::from_str(&response_text)?;
    Ok(ChatResponse {
        message: response.message,
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