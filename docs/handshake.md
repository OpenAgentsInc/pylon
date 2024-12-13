# Onyx-Pylon Handshake Test Plan

## Overview

This document outlines the plan for testing the initial WebSocket handshake between Onyx (client) and Pylon (server) applications, with specific file locations for implementation.

## File Locations

### Pylon (Server)
- WebSocket Server: `src-tauri/src/mcp/server.rs`
- Frontend Status: `src/components/ConnectionStatus.tsx`
- Main App Integration: `src/App.tsx`

### Onyx (Client)
- WebSocket Client: `app/services/websocket/WebSocketService.ts` (exists)
- Connection Hook: `app/services/websocket/useWebSocket.ts` (exists)
- Types: `app/services/websocket/types.ts` (exists)
- Status Display: Replace DVMButton with NexusOverlay in `app/screens/OnyxScreen.tsx`

## Component Details

### Pylon Components

1. **WebSocket Server (`src-tauri/src/mcp/server.rs`)**
```rust
pub struct MCPServer {
    ws_server: WebSocketServer,
    clients: HashMap<ClientId, Client>,
}

impl MCPServer {
    pub fn new() -> Self {
        // Initialize server on localhost:3000
    }
}
```

2. **Frontend Status (`src/components/ConnectionStatus.tsx`)**
```tsx
export const ConnectionStatus = () => {
  // Display server status, client count, etc
}
```

### Onyx Components

1. **Status Display (`app/screens/OnyxScreen.tsx`)**
```typescript
// Replace:
{/* <DVMButton /> */}

// With:
<NexusOverlay 
  wsState={wsService.state}
  onRetry={() => wsService.connect()}
/>
```

## Protocol Flow

1. **Initial Connection**
   - Pylon server listens on `localhost:3000`
   - Onyx connects via existing `WebSocketService`
   - Both show connection status

2. **WebSocket Handshake**
   ```typescript
   // Already implemented in WebSocketService.ts
   this.ws = new WebSocket(this.config.url)
   ```

3. **MCP Protocol Initialization**
   ```json
   // Onyx -> Pylon (using existing AuthMessage type)
   {
     "type": "auth",
     "id": "random-id",
     "payload": {
       "apiKey": "test-key"
     }
   }
   ```

   ```json
   // Pylon -> Onyx (using existing AuthResponse type)
   {
     "type": "auth",
     "id": "random-id",
     "payload": {
       "status": "success"
     }
   }
   ```

## Implementation Steps

1. **Pylon Updates**
   ```bash
   # Create new files
   touch src/components/ConnectionStatus.tsx
   
   # Update existing
   # - src/App.tsx (add ConnectionStatus)
   # - src-tauri/src/mcp/server.rs (implement WebSocket)
   ```

2. **Onyx Updates**
   ```bash
   # Update OnyxScreen.tsx
   - Remove DVMButton
   - Add NexusOverlay with WebSocket state
   - Connect to Pylon WebSocket
   ```

## Testing

### Manual Test Flow
1. Start Pylon in dev mode
2. Start Onyx in dev mode
3. Verify connection status displays
4. Check console logs for protocol messages

### Automated Tests

1. **Pylon Tests**
   - Location: `src-tauri/src/tests/mcp/server_tests.rs`
   ```rust
   #[test]
   fn test_client_connection() {
     // Test connection handling
   }
   ```

2. **Onyx Tests**
   - Already has WebSocket tests
   - Add test for NexusOverlay in OnyxScreen

## Success Criteria

1. **Visual Indicators**
   - Pylon shows server status in `ConnectionStatus` component
   - Onyx shows client status in `NexusOverlay`
   - Both show matching connection IDs

2. **Protocol Success**
   - Auth message exchange logged in dev tools
   - Connection state properly managed in existing MobX store
   - Error states handled by existing error handlers

3. **Error Handling**
   - Using existing reconnection logic in WebSocketService
   - Error states shown in NexusOverlay
   - Retry button functional

## Implementation Order

1. **Pylon First**
   - Implement basic WebSocket server
   - Add ConnectionStatus component
   - Test server in isolation

2. **Onyx Second**
   - Update OnyxScreen to use NexusOverlay
   - Configure WebSocketService for Pylon
   - Test connection

3. **Integration**
   - Test full handshake flow
   - Verify status displays
   - Test error scenarios

## Next Steps

After successful handshake testing:
1. Implement full MCP protocol
2. Add resource provider testing
3. Implement tool provider testing
4. Add payment flow testing