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
- [x] Set up test environment
- [x] Create mock client/server
- [x] Add basic protocol tests
- [x] Implement connection tests

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

The next task is to implement the MCP protocol types and handlers. Let's:

1. Create MCP types from schema
2. Implement initialization request/response handlers
3. Add client capability negotiation

Would you like me to proceed with implementing the MCP protocol types?