use crate::mcp::providers::ollama::{OllamaProvider, ChatMessage};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header, body_json};
use serde_json::json;

#[tokio::test]
async fn test_ollama_integration() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock the models endpoint
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!([
                {
                    "name": "llama3.2",
                    "modified_at": "2024-02-20T12:00:00Z",
                    "size": 1000
                }
            ])))
        .mount(&mock_server)
        .await;

    // Mock the chat endpoint
    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({
            "model": "llama3.2",
            "messages": [{
                "role": "user",
                "content": "Why is the sky blue?"
            }]
        })))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "message": {
                    "role": "assistant",
                    "content": "The sky appears blue due to Rayleigh scattering of sunlight in the atmosphere."
                },
                "done": true
            })))
        .mount(&mock_server)
        .await;

    // Create provider with mock server URL
    let provider = OllamaProvider::new(mock_server.uri());

    // Test list_models
    let models = provider.list_models().await.unwrap();
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].name, "llama3.2");

    // Test chat
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Why is the sky blue?".to_string(),
        }
    ];

    let response = provider.chat("llama3.2", messages).await.unwrap();
    assert!(response.message.content.contains("Rayleigh scattering"));
    assert!(response.done);
}

#[tokio::test]
async fn test_ollama_error_handling() {
    let mock_server = MockServer::start().await;

    // Mock server error
    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(500)
            .set_body_json(json!({
                "error": "Internal server error"
            })))
        .mount(&mock_server)
        .await;

    let provider = OllamaProvider::new(mock_server.uri());
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Why is the sky blue?".to_string(),
        }
    ];

    let result = provider.chat("llama3.2", messages).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_ollama_streaming() {
    use futures_util::StreamExt;
    use tokio::time::sleep;
    use std::time::Duration;

    let mock_server = MockServer::start().await;

    // Mock streaming response
    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(concat!(
                "{\"message\":{\"role\":\"assistant\",\"content\":\"The\"},\"done\":false}\n",
                "{\"message\":{\"role\":\"assistant\",\"content\":\" sky\"},\"done\":false}\n",
                "{\"message\":{\"role\":\"assistant\",\"content\":\" is\"},\"done\":false}\n",
                "{\"message\":{\"role\":\"assistant\",\"content\":\" blue\"},\"done\":true}\n"
            )))
        .mount(&mock_server)
        .await;

    let provider = OllamaProvider::new(mock_server.uri());
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Why is the sky blue?".to_string(),
        }
    ];

    let mut stream = provider.chat_stream("llama3.2", messages).await;
    let mut response_parts = Vec::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response_parts.push(response.message.content);
                if response.done {
                    break;
                }
            }
            Err(e) => panic!("Stream error: {}", e),
        }
        sleep(Duration::from_millis(10)).await;
    }

    assert_eq!(response_parts, vec!["The", " sky", " is", " blue"]);
}