use std::error::Error;
use reqwest::Client;
use tokio::time::Duration;

pub async fn is_ollama_running() -> bool {
    let client = Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap_or_default();

    match client.get("http://localhost:11434/api/version").send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}