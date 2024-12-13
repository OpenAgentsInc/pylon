use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures_util::Stream;
use tokio_stream::StreamExt;
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

    pub async fn chat_stream(&self, model: &str, messages: Vec<ChatMessage>) -> impl Stream<Item = Result<ChatResponse, Box<dyn Error>>> {
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
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_list_models() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/tags"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(vec![
                    ModelInfo {
                        name: "llama3.2".to_string(),
                        modified_at: "2024-02-20T12:00:00Z".to_string(),
                        size: 1000,
                    }
                ]))
            .mount(&mock_server)
            .await;

        let provider = OllamaProvider::new(mock_server.uri());
        let models = provider.list_models().await.unwrap();
        
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].name, "llama3.2");
    }

    #[tokio::test]
    async fn test_chat() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/chat"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(ChatResponse {
                    message: ChatMessage {
                        role: "assistant".to_string(),
                        content: "The sky appears blue due to Rayleigh scattering.".to_string(),
                    },
                    done: true,
                }))
            .mount(&mock_server)
            .await;

        let provider = OllamaProvider::new(mock_server.uri());
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Why is the sky blue?".to_string(),
            }
        ];

        let response = provider.chat("llama3.2", messages).await.unwrap();
        assert!(response.message.content.contains("sky"));
        assert!(response.done);
    }
}