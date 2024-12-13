use crate::mcp::providers::ollama::{OllamaProvider, ChatMessage};
use tokio_stream::StreamExt;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_ollama_integration() {
    let provider = OllamaProvider::default();

    // Test list_models
    let models = provider.list_models().await.unwrap();
    assert!(!models.is_empty());
    println!("Available models: {:?}", models);

    // Test chat
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Why is the sky blue?".to_string(),
        }
    ];

    let response = provider.chat("llama3.2", messages.clone()).await.unwrap();
    assert!(!response.message.content.is_empty());
    println!("Chat response: {}", response.message.content);

    // Test streaming
    let mut stream = provider.chat_stream("llama3.2", messages).await;
    let mut response_parts = Vec::new();

    while let Some(Ok(response)) = stream.next().await {
        response_parts.push(response.message.content);
        if response.done {
            break;
        }
        sleep(Duration::from_millis(10)).await;
    }

    assert!(!response_parts.is_empty());
    println!("Streaming response parts: {:?}", response_parts);
}

#[tokio::test]
async fn test_ollama_error_handling() {
    let provider = OllamaProvider::new("http://localhost:11434".to_string());
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Why is the sky blue?".to_string(),
        }
    ];

    // Test with non-existent model
    let result = provider.chat("non-existent-model", messages).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_ollama_streaming() {
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