use reqwest::Client;

pub async fn is_ollama_running() -> bool {
    let client = Client::new();
    match client.get("http://localhost:11434/api/tags").send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}