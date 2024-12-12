# Pylon Development Todo

## Phase 1: MCP Handshake Implementation

### 1. Basic Tauri Setup
- [x] Initialize Tauri project with Rust backend and React frontend
- [x] Set up development environment
- [x] Configure build system
- [x] Add required dependencies:
  - tokio
  - tokio-tungstenite
  - jsonrpc-core
  - serde
  - serde_json

### 2. WebSocket Server
- [ ] Implement basic WebSocket server in Rust
- [ ] Add connection handling
- [ ] Set up JSON-RPC message parsing
- [ ] Create connection management system

### 3. MCP Protocol Implementation
- [ ] Implement core MCP types from schema
- [ ] Create initialization request/response handlers
- [ ] Add client capability negotiation
- [ ] Implement basic resource handling

### 4. Basic Client (for testing)
- [ ] Create test client implementation
- [ ] Add initialization request sending
- [ ] Implement capability negotiation
- [ ] Add basic resource requests

### 5. Testing Infrastructure
- [ ] Set up test environment
- [ ] Create mock client/server
- [ ] Add basic protocol tests
- [ ] Implement connection tests

### 6. Frontend Status Display
- [ ] Add connection status display
- [ ] Create basic metrics view
- [ ] Implement log display
- [ ] Add configuration interface

## Phase 2: Resource Implementation

### 1. File System Provider
- [ ] Implement basic file system access
- [ ] Add directory listing
- [ ] Create file reading functionality
- [ ] Implement change notifications

### 2. Resource Management
- [ ] Create resource provider interface
- [ ] Implement resource discovery
- [ ] Add resource caching
- [ ] Create update notification system

### 3. Testing
- [ ] Add resource provider tests
- [ ] Create file system tests
- [ ] Implement change notification tests
- [ ] Add integration tests

## Phase 3: Tool Implementation

### 1. Basic Tools
- [ ] Create tool provider interface
- [ ] Implement basic tool registration
- [ ] Add tool discovery
- [ ] Create tool execution framework

### 2. Testing
- [ ] Add tool provider tests
- [ ] Create tool execution tests
- [ ] Implement tool discovery tests
- [ ] Add integration tests

## Phase 4: NIP-90 Integration

### 1. Basic Setup
- [ ] Add Nostr client
- [ ] Implement basic event handling
- [ ] Create job management system
- [ ] Add result delivery

### 2. Testing
- [ ] Create Nostr client tests
- [ ] Add event handling tests
- [ ] Implement job management tests
- [ ] Create integration tests

## Phase 5: Payment Integration

### 1. Basic Setup
- [ ] Add Breez SDK
- [ ] Implement invoice generation
- [ ] Create payment verification
- [ ] Add payment timeout handling

### 2. Testing
- [ ] Create payment generation tests
- [ ] Add verification tests
- [ ] Implement timeout tests
- [ ] Create integration tests

## Immediate Next Steps

1. **Initialize Project**
```bash
# Create new Tauri project
cargo create-tauri-app pylon
cd pylon

# Add dependencies to Cargo.toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
tokio-tungstenite = "0.20"
jsonrpc-core = "18.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

2. **Create Basic Server Structure**
```rust
// src-tauri/src/mcp/mod.rs
pub mod server;
pub mod types;
pub mod handlers;

// src-tauri/src/mcp/server.rs
pub struct MCPServer {
    ws_server: WebSocketServer,
    clients: HashMap<ClientId, Client>,
}

// src-tauri/src/mcp/types.rs
#[derive(Serialize, Deserialize)]
pub struct ClientCapabilities {
    // Add from schema
}

#[derive(Serialize, Deserialize)]
pub struct ServerCapabilities {
    // Add from schema
}
```

3. **Implement Basic Handler**
```rust
// src-tauri/src/mcp/handlers/init.rs
pub async fn handle_initialize(
    request: InitializeRequest,
    client: &mut Client,
) -> Result<InitializeResult, Error> {
    // Implement initialization
}
```

4. **Create Test Infrastructure**
```rust
// src-tauri/src/tests/mcp/mod.rs
mod server_tests;
mod client_tests;
mod protocol_tests;

// src-tauri/src/tests/mcp/server_tests.rs
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_server_initialization() {
        // Implement test
    }
}
```

## Progress Tracking

- [ ] Project initialized
- [ ] Basic server implemented
- [ ] MCP types defined
- [ ] Initialization handler working
- [ ] Basic tests passing
- [ ] Client connection established
- [ ] Protocol handshake completed

## Notes

- Focus on getting basic handshake working first
- Use test-driven development approach
- Keep implementation minimal but extensible
- Document all protocol interactions
- Add logging for debugging
- Consider error handling carefully
