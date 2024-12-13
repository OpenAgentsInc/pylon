# MCP (Model Control Protocol) Implementation

## Overview
The MCP implementation in Pylon provides a client-server architecture for managing model interactions through WebSocket connections. The implementation follows a modular design with clear separation of concerns across multiple components.

## File Structure

### mod.rs
The main module file that exports public interfaces and types. It serves as the entry point for the MCP implementation and re-exports the most commonly used types and traits.

### clients.rs
Contains the active implementation of client management using async/await patterns with Tokio. This is the current production code that handles:
- Client registration and management
- Client capabilities tracking
- Connection state management
- Async client operations

### capabilities.rs (Deprecated)
This file appears unused because it was replaced by the async implementation in `clients.rs`. The key differences:
- `capabilities.rs` uses std::sync::RwLock (blocking)
- `clients.rs` uses tokio::sync::RwLock (async)
- The newer implementation in `clients.rs` has better integration with the async runtime

The capabilities system was reimplemented in `clients.rs` to:
1. Better support async operations
2. Provide more robust client state management
3. Include additional client metadata (connected_at, last_message)
4. Use string-based IDs instead of UUIDs for better interop

### protocol.rs
Implements the core MCP protocol functionality:
- Message handling
- JSON-RPC request/response processing
- Client initialization
- Resource operations (list, read, watch, unwatch)
- Error handling and response formatting

### server.rs
Implements the WebSocket server using Actix-Web:
- WebSocket connection handling
- Client message routing
- Connection lifecycle management
- Error handling and logging
- Ping/pong handling for connection health

### types.rs
Defines the complete type system for the MCP implementation:
- Protocol types (Implementation, ClientCapabilities, ServerCapabilities)
- Resource types (Resource, ResourceContents)
- JSON-RPC message types
- Role and content types
- Protocol constants

### providers/
Directory containing resource provider implementations:

#### providers/mod.rs
Defines the core provider traits and types:
- ResourceProvider trait for implementing providers
- ResourceError enum for error handling
- ResourcePath struct for path management

#### providers/filesystem.rs
Implements filesystem access provider:
- File and directory listing
- File content reading
- File system watching
- Path validation and security
- MIME type detection
- URI generation

## Key Components

### Client Capabilities System
The client capabilities system is implemented in `clients.rs` with three main components:

1. `ClientCapabilities`: Defines what features a client supports
   - experimental features
   - roots capability
   - sampling capability

2. `ClientInfo`: Basic client metadata
   - name
   - version

3. `ConnectedClient`: Full client state
   - client ID
   - capabilities
   - connection timestamp
   - last message

### Resource Provider System
The resource provider system allows extensible access to different types of resources:

1. `ResourceProvider` trait defines the interface:
   - list: List resources at a path
   - read: Read resource contents
   - watch: Watch for resource changes
   - unwatch: Stop watching resources

2. `FileSystemProvider` implementation:
   - Secure file system access
   - Path validation
   - File watching
   - MIME type detection

### WebSocket Server
The WebSocket server implementation in `server.rs`:
- Handles client connections
- Routes messages to protocol handler
- Manages connection lifecycle
- Provides error handling
- Implements ping/pong for connection health

## Protocol Features

1. Client Initialization
   - Capability negotiation
   - Version checking
   - Client registration

2. Resource Operations
   - List resources
   - Read resource contents
   - Watch for changes
   - Unwatch resources

3. Error Handling
   - Structured error responses
   - Error categorization
   - Client-friendly error messages

## Migration from Old Implementation

The original implementation in `capabilities.rs` was replaced because:
1. It used blocking RwLocks which could impact performance in an async context
2. It lacked important metadata tracking (connection time, last message)
3. The UUID-based system was less flexible than string-based IDs
4. The capability negotiation was overly complex for actual usage patterns

The new implementation in `clients.rs` provides a more streamlined, async-first approach that better matches the actual requirements of the application.

## Future Improvements

1. Binary File Handling
   - The FileSystemProvider currently treats all files as text
   - Need to implement proper binary file handling

2. Event Handling
   - File system events are currently just logged
   - Need to implement proper event propagation to clients

3. Resource Provider Extensions
   - Add more resource provider implementations
   - Support for remote resources
   - Database resource provider