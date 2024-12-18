# Pylon Development Todo

## Phase 1: MCP Handshake Implementation

### 1. Basic Tauri Setup
- [x] Initialize Tauri project with Rust backend and React frontend
- [x] Set up development environment
- [x] Configure build system
- [x] Add required dependencies:
  - [x] tokio
  - [x] tokio-tungstenite
  - [x] jsonrpc-core
  - [x] serde
  - [x] serde_json

### 2. WebSocket Server
- [x] Implement basic WebSocket server in Rust
- [x] Add connection handling
- [x] Set up JSON-RPC message parsing
- [x] Create connection management system

### 3. MCP Protocol Implementation
- [x] Implement core MCP types from schema
- [x] Create initialization request/response handlers
- [x] Add client capability negotiation
- [x] Implement basic resource handling

### 4. Basic Client (for testing)
- [ ] Create test client implementation
- [ ] Add initialization request sending
- [ ] Implement capability negotiation
- [ ] Add basic resource requests

### 5. Testing Infrastructure
- [x] Set up test environment
- [x] Create mock client/server
- [x] Add basic protocol tests
- [x] Implement connection tests

### 6. Frontend Status Display
- [ ] Add connection status display
- [ ] Create basic metrics view
- [ ] Implement log display
- [ ] Add configuration interface

## Next Steps

1. **Client Capability Negotiation**
   - [x] Implement client capability storage
   - [x] Add capability matching logic
   - [x] Create capability update handlers
   - [x] Add tests for capability negotiation

2. **Resource Handling**
   - [x] Implement resource provider interface
   - [x] Add file system resource provider
   - [x] Create resource update notifications
   - [x] Add tests for resource handling

3. **Frontend Integration**
   - [ ] Create WebSocket connection component
   - [ ] Add protocol state management
   - [ ] Implement status display
   - [ ] Add configuration interface

## Notes

- Focus on completing capability negotiation next
- Keep implementation modular and testable
- Document all protocol interactions
- Consider error handling and recovery
- Add logging for debugging