# Onyx-Pylon Handshake Test Plan

## Overview

This document outlines the plan for testing the initial WebSocket handshake between Onyx (client) and Pylon (server) applications, with specific file locations for implementation.

## File Locations

### Pylon (Server)
- WebSocket Server: `src-tauri/src/mcp/server.rs`
- Frontend Status: `src/components/ConnectionStatus.tsx`
- Main App Integration: `src/App.tsx`

### Onyx (Client)
- WebSocket Client: `app/services/websocket/WebSocketService.ts`
- Connection Hook: `app/services/websocket/useWebSocket.ts`
- Types: `app/services/websocket/types.ts`
- Status Display: `app/screens/OnyxScreen.tsx` (replace DVMButton with connection status)

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

1. **WebSocket Service (`app/services/websocket/WebSocketService.ts`)**
```typescript
export class WebSocketService {
  connect(url: string = 'ws://localhost:3000') {
    // Connect to Pylon
  }
}
```

2. **Connection Hook (`app/services/websocket/useWebSocket.ts`)**
```typescript
export const useWebSocket = () => {
  // Manage connection state
}
```

3. **Status Display (`app/screens/OnyxScreen.tsx`)**
```typescript
// Replace:
{/* <DVMButton /> */}

// With:
<ConnectionStatus />
```

## Protocol Flow

1. **Initial Connection**
   - Pylon server listens on `localhost:3000`
   - Onyx connects via `WebSocketService`
   - Both show connection status

2. **WebSocket Handshake**
   ```typescript
   // In WebSocketService.ts
   this.ws = new WebSocket('ws://localhost:3000')
   this.ws.onopen = () => {
     this.sendInitialize()
   }
   ```

3. **MCP Protocol Initialization**
   ```json
   // Onyx -> Pylon
   {
     "jsonrpc": "2.0",
     "method": "initialize",
     "params": {
       "capabilities": {
         "experimental": false,
         "roots": []
       }
     },
     "id": 1
   }
   ```

   ```json
   // Pylon -> Onyx
   {
     "jsonrpc": "2.0",
     "result": {
       "capabilities": {
         "experimental": false,
         "roots": []
       }
     },
     "id": 1
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
   # Update existing
   # - app/screens/OnyxScreen.tsx (replace DVMButton)
   # - app/services/websocket/* (implement client)
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
   - Location: `app/services/websocket/__tests__/WebSocketService.test.ts`
   ```typescript
   describe('WebSocketService', () => {
     test('connects successfully', () => {
       // Test connection
     })
   })
   ```

## Success Criteria

1. **Visual Indicators**
   - Pylon shows server status in `ConnectionStatus` component
   - Onyx shows client status in `OnyxScreen`
   - Both show matching connection IDs

2. **Protocol Success**
   - MCP initialization logged in dev tools
   - Capabilities exchanged and displayed
   - Connection state properly managed in `useWebSocket` hook

3. **Error Handling**
   - Connection failures shown in UI
   - Automatic reconnection attempts visible
   - Error states clearly indicated in status components

## Next Steps

After successful handshake testing:
1. Implement full MCP protocol in respective service files
2. Add resource provider testing
3. Implement tool provider testing
4. Add payment flow testing