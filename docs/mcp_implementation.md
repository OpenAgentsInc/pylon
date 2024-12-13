# MCP (Model Control Protocol) Implementation

## Overview
The MCP implementation in Pylon provides a client-server architecture for managing model interactions. The implementation is split across several key files in the `src-tauri/src/mcp/` directory.

## File Structure

### mod.rs
The main module file that exports public interfaces and types. It serves as the entry point for the MCP implementation.

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
Defines the MCP protocol specifics including message formats and protocol versions.

### server.rs
Implements the MCP server functionality that handles client connections and message routing.

### types.rs
Contains shared type definitions used across the MCP implementation.

### providers/
Directory containing various provider implementations for different resource types.

## Client Capabilities System

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

## Migration from Old Implementation

The original implementation in `capabilities.rs` was replaced because:
1. It used blocking RwLocks which could impact performance in an async context
2. It lacked important metadata tracking (connection time, last message)
3. The UUID-based system was less flexible than string-based IDs
4. The capability negotiation was overly complex for actual usage patterns

The new implementation in `clients.rs` provides a more streamlined, async-first approach that better matches the actual requirements of the application.