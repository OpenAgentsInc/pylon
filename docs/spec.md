# Pylon MCP Server Specification

## Overview

Pylon is a desktop application built with Tauri that implements an MCP (Model Context Protocol) server and NIP-90 service provider. It enables users to run an OpenAgents node that can process requests from Onyx mobile app users and earn bitcoin via Lightning Network payments.

## Core Components

### 1. Tauri Application Structure

```
pylon/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # Main Tauri application entry
│   │   ├── mcp/         # MCP server implementation
│   │   │   ├── mod.rs
│   │   │   ├── server.rs
│   │   │   ├── handlers/
│   │   │   └── types.rs
│   │   ├── nostr/       # Nostr/NIP-90 implementation
│   │   └── breez/       # Breez SDK integration
│   └── Cargo.toml
├── src/                 # Frontend React/TypeScript
│   ├── App.tsx
│   ├── components/
│   └── stores/
└── package.json
```

### 2. MCP Server Implementation

The MCP server will be implemented in Rust using:
- Tokio for async runtime
- WebSocket server (tokio-tungstenite)
- JSON-RPC handling (jsonrpc-core)
- Serde for JSON serialization

Key components:

1. **Server Core**
```rust
pub struct MCPServer {
    // WebSocket server instance
    ws_server: WebSocketServer,
    // Active client connections
    clients: HashMap<ClientId, Client>,
    // Resource providers
    resources: Vec<Box<dyn ResourceProvider>>,
    // Tool providers
    tools: Vec<Box<dyn ToolProvider>>,
    // Prompt templates
    prompts: Vec<PromptTemplate>,
}
```

2. **Client Management**
```rust
pub struct Client {
    // Client capabilities from initialization
    capabilities: ClientCapabilities,
    // Active subscriptions
    subscriptions: Vec<ResourceSubscription>,
    // WebSocket sender
    tx: WebSocketSender,
}
```

3. **Resource Providers**
- File system access
- Git repository access
- Database connections
- External API integrations

4. **Tool Providers**
- Code analysis
- Data processing
- External service integrations

### 3. NIP-90 Integration

Implement NIP-90 service provider functionality:

1. **Event Handling**
```rust
pub struct DvmProvider {
    // Nostr client
    client: NostrClient,
    // Active job handlers
    jobs: HashMap<JobId, Job>,
    // Available computation resources
    resources: ComputeResources,
}
```

2. **Job Types**
- Text generation/completion
- Code analysis
- Data processing
- Custom computation tasks

3. **Payment Flow**
- Lightning invoice generation via Breez SDK
- Payment verification
- Job result delivery

### 4. Frontend Interface

React/TypeScript frontend with:

1. **Dashboard**
- Node status
- Active connections
- Resource usage
- Earnings overview

2. **Configuration**
- Resource allocation
- Pricing settings
- Network parameters
- Tool enablement

3. **Monitoring**
- Active jobs
- Client connections
- Resource usage
- Payment history

## Implementation Plan

### Phase 1: Core Infrastructure

1. **Basic Tauri Setup**
- Initialize Tauri project
- Configure build system
- Set up development environment

2. **MCP Server Core**
- Implement WebSocket server
- Add JSON-RPC handling
- Create basic resource providers
- Implement client management

3. **Frontend Foundation**
- Create basic UI layout
- Add configuration interface
- Implement monitoring views

### Phase 2: NIP-90 Integration

1. **Nostr Implementation**
- Add Nostr client
- Implement NIP-90 event handling
- Create job management system

2. **Payment Processing**
- Integrate Breez SDK
- Implement invoice generation
- Add payment verification

3. **Resource Management**
- Add compute resource tracking
- Implement resource allocation
- Create pricing mechanisms

### Phase 3: Tools & Resources

1. **Tool Providers**
- Implement core tools
- Add external integrations
- Create tool discovery system

2. **Resource Providers**
- Add file system provider
- Implement Git provider
- Create database connectors

3. **Frontend Enhancement**
- Add detailed monitoring
- Improve configuration UI
- Create resource management interface

## Technical Requirements

### Backend (Rust)

- tokio = "1.0"
- tokio-tungstenite = "0.20"
- jsonrpc-core = "18.0"
- serde = "1.0"
- serde_json = "1.0"
- nostr = "0.24"
- breez-sdk-core = "0.3"
- tauri = "1.5"

### Frontend (TypeScript)

- React 18+
- TypeScript 5+
- TailwindCSS
- React Query
- Zustand for state management

## Security Considerations

1. **Client Authentication**
- Implement secure WebSocket connections
- Add client capability verification
- Manage resource access permissions

2. **Resource Protection**
- Sandbox file system access
- Rate limit resource usage
- Monitor compute resources

3. **Payment Security**
- Secure Lightning payment handling
- Verify payment before job execution
- Implement refund mechanisms

## Testing Strategy

1. **Unit Tests**
- Test MCP message handling
- Verify resource provider logic
- Check payment processing

2. **Integration Tests**
- Test client connections
- Verify job execution flow
- Check payment integration

3. **End-to-End Tests**
- Test complete job flows
- Verify payment cycles
- Check resource management

## Deployment

1. **Application Packaging**
- Create installers for major platforms
- Sign applications
- Implement auto-updates

2. **Resource Configuration**
- Default resource limits
- Configuration templates
- Network settings

3. **Monitoring & Logging**
- Error tracking
- Performance monitoring
- Usage statistics

## Future Enhancements

1. **Advanced Features**
- Custom tool development
- Enhanced resource providers
- Advanced pricing models

2. **Performance Optimization**
- Resource usage optimization
- Connection pooling
- Caching mechanisms

3. **UI Improvements**
- Advanced monitoring
- Detailed analytics
- Enhanced configuration

## Documentation

1. **User Documentation**
- Installation guide
- Configuration manual
- Troubleshooting guide

2. **Developer Documentation**
- API documentation
- Integration guide
- Plugin development

3. **Maintenance Guide**
- Update procedures
- Backup strategies
- Security practices