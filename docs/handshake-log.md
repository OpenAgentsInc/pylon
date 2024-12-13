# Handshake Implementation Log

## Overview

This log documents the implementation of the WebSocket handshake between Onyx (client) and Pylon (server).

## Changes Made

### 1. WebSocket Configuration
**File:** `app/config/websocket.ts`
```typescript
export const pylonConfig: WebSocketConfig = {
  url: 'ws://localhost:8080',
  maxReconnectAttempts: 5,
  reconnectInterval: 3000,
  reconnectBackoff: 'exponential',
  maxBackoffTime: 30000,
  pingInterval: 30000,
  pongTimeout: 5000,
  apiKey: 'test-key' // Temporary for testing
};
```

Added a new configuration specifically for Pylon connection, using the port from server logs:
```
[2024-12-13T01:37:09Z INFO  pylon_lib::mcp::server] Starting MCP server on 127.0.0.1:8080
```

### 2. Connection Status UI
**File:** `app/components/PylonOverlay.tsx`

Created a new overlay component based on NexusOverlay but with improvements:
- Persistent connection indicator (green dot when connected)
- Manual retry button for connection failures
- More detailed error state display
- Better visual styling

Key features:
```typescript
const PylonOverlay = observer(() => {
  const { state } = useWebSocket(pylonConfig);

  // Show different UI states based on connection
  if (!state.connected) {
    return (
      <View style={styles.container}>
        <Text style={styles.text}>
          {state.connecting ? 'Connecting to Pylon...' : 'Disconnected from Pylon'}
        </Text>
        {state.error && (
          <Text style={styles.errorText}>{state.error}</Text>
        )}
        {!state.connecting && (
          <TouchableOpacity 
            style={styles.retryButton}
            onPress={() => window.location.reload()}
          >
            <Text style={styles.retryText}>Retry Connection</Text>
          </TouchableOpacity>
        )}
      </View>
    );
  }

  // Connected state shows minimal indicator
  return (
    <View style={styles.connectedContainer}>
      <View style={styles.statusDot} />
      <Text style={styles.connectedText}>Connected to Pylon</Text>
    </View>
  );
});
```

### 3. Screen Integration
**File:** `app/screens/OnyxScreen.tsx`

Updated the main screen to use the new overlay:
```typescript
export const OnyxScreen = observer(function OnyxScreen({ visible = true }: OnyxScreenProps) {
  const $topInset = useSafeAreaInsetsStyle(["top"])

  if (!visible) return null

  return (
    <View style={[$container, $topInset]}>
      <Canvas />
      <PylonOverlay />
    </View>
  )
})
```

## Implementation Details

### WebSocket Connection Flow
1. OnyxScreen mounts
2. PylonOverlay initializes with pylonConfig
3. useWebSocket hook creates/reuses WebSocket connection
4. Connection status displayed in overlay
5. Auto-reconnection handled by existing WebSocketService

### Error Handling
- Connection failures shown in overlay
- Automatic reconnection attempts
- Manual retry button
- Error messages displayed to user

### Visual States
1. **Connecting**
   - Full-width overlay
   - "Connecting to Pylon..." message
   - No retry button

2. **Disconnected**
   - Full-width overlay
   - "Disconnected from Pylon" message
   - Error message if applicable
   - Retry button

3. **Connected**
   - Minimal overlay in top-right
   - Green status dot
   - "Connected to Pylon" text

## Testing Notes

### Manual Testing Steps
1. Start Pylon server (verified running on 8080)
2. Start Onyx in dev mode
3. Verify connection overlay appears
4. Verify successful connection shows green dot
5. Test disconnection handling:
   - Stop Pylon server
   - Verify disconnect state
   - Verify retry button
   - Restart server
   - Verify reconnection

### Known Issues
None currently identified.

## Next Steps

1. **Authentication**
   - Implement proper API key handling
   - Add secure key storage
   - Add auth failure handling

2. **Protocol Implementation**
   - Implement MCP message handling
   - Add capability negotiation
   - Add resource handling

3. **UI Enhancements**
   - Add connection details display
   - Add debug mode toggle
   - Add connection statistics

4. **Testing**
   - Add automated tests
   - Add connection failure scenarios
   - Add protocol compliance tests