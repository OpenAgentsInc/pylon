# Onyx-Pylon Handshake Test Plan

## Overview

This document outlines the plan for testing the initial WebSocket handshake between Onyx (client) and Pylon (server) applications.

## Components

### Pylon (Server)
- WebSocket server running on localhost:3000
- MCP protocol implementation
- Connection status display in frontend

### Onyx (Client)
- WebSocket client implementation
- Connection status display in frontend
- Capability negotiation handling

## Test Flow

1. **Initial Connection**
   - Pylon starts WebSocket server
   - Onyx attempts connection to ws://localhost:3000
   - Both frontends show "Connecting..." status

2. **WebSocket Handshake**
   - Connection established
   - Both frontends update to "Connected" status
   - Connection IDs displayed on both ends

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

4. **Frontend Display**
   - Pylon shows:
     - WebSocket server status
     - Number of connected clients
     - Client capabilities
   
   - Onyx shows:
     - Connection status
     - Server capabilities
     - Handshake completion status

## Implementation Steps

1. **Pylon Updates**
   - Add connection status component to frontend
   - Display WebSocket server info
   - Show connected client details

2. **Onyx Updates**
   - Add connection status component
   - Implement basic WebSocket client
   - Display server connection info

3. **Testing**
   - Manual testing of connection flow
   - Verify status displays on both ends
   - Test connection error scenarios

## Success Criteria

1. **Connection Success**
   - WebSocket connection established
   - Status correctly displayed on both ends
   - Connection IDs match

2. **Protocol Success**
   - MCP initialization completed
   - Capabilities exchanged
   - Both applications show negotiated capabilities

3. **Error Handling**
   - Connection failures gracefully handled
   - Reconnection attempts shown
   - Error states clearly displayed

## Next Steps

After successful handshake testing:
1. Implement full MCP protocol
2. Add resource provider testing
3. Implement tool provider testing
4. Add payment flow testing