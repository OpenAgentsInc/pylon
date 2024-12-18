use super::*;
use tokio::time::sleep;
use std::time::Duration;
use crate::utils::ollama::is_ollama_running;
use futures_util::StreamExt;

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
        let content = response.message.content.clone();
        if !content.is_empty() {
            response_parts.push(content.clone());
            full_response.push_str(&content);
        }
        if response.done {
            break;
        }
        sleep(Duration::from_millis(10)).await;
    }

    assert!(!response_parts.is_empty());
    println!("Full streaming response: {}", full_response);
}