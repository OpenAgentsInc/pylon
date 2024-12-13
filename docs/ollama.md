# Ollama Chat Integration Plan

## Overview
We will implement a new capability to interact with a local Ollama instance through its chat API endpoint. This will allow users to have conversations with locally running LLMs.

## Implementation Steps

1. Add New Capability Types
```rust
// In src/mcp/types.rs
pub struct OllamaCapability {
    pub available_models: Vec<String>,
    pub endpoint: String,
}

// Update ServerCapabilities
pub struct ServerCapabilities {
    // ... existing fields ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ollama: Option<OllamaCapability>,
}
```

2. Create Ollama Provider
```rust
// In src/mcp/providers/ollama.rs
pub struct OllamaProvider {
    endpoint: String,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub async fn chat(&self, model: &str, messages: Vec<ChatMessage>) -> Result<String, Error> {
        let response = self.client
            .post(format!("{}/api/chat", self.endpoint))
            .json(&ChatRequest {
                model: model.to_string(),
                messages,
            })
            .send()
            .await?;
            
        // Handle response
    }
    
    pub async fn list_models(&self) -> Result<Vec<String>, Error> {
        // Implement model listing
    }
}
```

3. Add Chat Message Types
```rust
// In src/mcp/types.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
}
```

4. Update Protocol Handler
```rust
// In src/mcp/protocol.rs
impl MCPProtocol {
    // Add new handler
    async fn handle_ollama_chat(&self, request: &JsonRpcRequest) -> Result<String, Box<dyn Error>> {
        let params: ChatRequest = serde_json::from_value(request.params.clone())?;
        
        match self.ollama_provider.chat(&params.model, params.messages).await {
            Ok(response) => {
                let json_response = serde_json::json!({
                    "jsonrpc": JSONRPC_VERSION,
                    "id": request.id,
                    "result": response
                });
                Ok(serde_json::to_string(&json_response)?)
            }
            Err(e) => Ok(self.create_error_response(
                request.id.clone(),
                -32000,
                format!("Chat error: {}", e),
            )),
        }
    }
}
```

5. Add Configuration
```rust
// In src/config.rs or similar
pub struct OllamaConfig {
    pub endpoint: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:11434".to_string(),
        }
    }
}
```

## API Usage Example

```rust
// Client code example
let request = JsonRpcRequest {
    jsonrpc: "2.0".to_string(),
    id: 1,
    method: "ollama/chat".to_string(),
    params: serde_json::json!({
        "model": "llama3.2",
        "messages": [
            {
                "role": "user",
                "content": "Why is the sky blue?"
            }
        ]
    }),
};

let response = client.send_request(request).await?;
```

## Testing Plan

1. Unit Tests
- Test message serialization/deserialization
- Test error handling
- Test capability negotiation

2. Integration Tests
- Test connection to Ollama server
- Test chat functionality with different models
- Test error cases (server down, invalid model, etc.)

3. End-to-End Tests
- Test full chat conversation flow
- Test model switching
- Test concurrent chats

## Implementation Order

1. Add new types and capabilities
2. Implement OllamaProvider
3. Add configuration handling
4. Update protocol handler
5. Add tests
6. Document API usage

## Security Considerations

1. Input Validation
- Validate model names
- Validate message content
- Sanitize inputs

2. Error Handling
- Graceful handling of server errors
- Clear error messages
- Rate limiting support

3. Configuration
- Configurable endpoint
- Optional authentication
- Connection timeouts

## Future Enhancements

1. Streaming Support
- Implement streaming responses
- Progress indicators
- Cancellation support

2. Model Management
- List available models
- Model status checking
- Model loading/unloading

3. Advanced Features
- Temperature control
- Context window management
- System prompt customization