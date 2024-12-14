# WebSocket Cleanup Issues

## Current Problems

1. Client Management
   - Clients not being properly removed on disconnect
   - Multiple client instances accumulating
   - Last message updates failing for non-existent clients
   - File listing errors with root path resolution

2. Connection Lifecycle
   - Initialize message handling needs review
   - Client cleanup on disconnect is unreliable
   - WebSocket close events not properly propagating
   - Root path resolution inconsistent between dev and production

## Investigation Points

### Client Tracking
```rust
// Current flow:
1. Client connects -> new UUID assigned
2. Initialize message -> client added to manager
3. Client disconnects -> cleanup sometimes fails
4. New connection -> new UUID, old client still in manager
```

### Root Path Issues
```rust
// Current implementation
let root_path = env::current_dir()
    .unwrap_or_else(|_| PathBuf::from("."));

// Problems:
1. Different paths in dev vs production
2. src-tauri vs project root confusion
3. Relative path resolution failing
```

## Proposed Solutions

### 1. Client Management
```rust
// Add explicit cleanup on connection close
pub async fn handle_connection(
    client_id: String,
    protocol: Arc<MCPProtocol>,
) {
    // Set up connection cleanup
    let cleanup = async {
        protocol.get_client_manager().remove_client(&client_id).await;
        debug!("Cleaned up client {}", client_id);
    };

    // Ensure cleanup runs on any exit path
    tokio::spawn(async move {
        tokio::select! {
            _ = connection_loop() => cleanup().await,
            _ = tokio::signal::ctrl_c() => cleanup().await,
        }
    });
}
```

### 2. Root Path Resolution
```rust
// Proposed fix
pub fn resolve_root_path() -> PathBuf {
    env::current_dir()
        .map(|mut d| {
            // Handle development case
            if d.ends_with("src-tauri") {
                d.pop();
            }
            
            // Verify it's a valid project root
            if d.join(".git").exists() || 
               d.join("src-tauri").exists() {
                d
            } else {
                warn!("Could not verify project root at {:?}", d);
                PathBuf::from(".")
            }
        })
        .unwrap_or_else(|e| {
            error!("Failed to get current directory: {}", e);
            PathBuf::from(".")
        })
}
```

### 3. Connection State Machine
```rust
#[derive(Debug)]
enum ConnectionState {
    New,
    Initializing,
    Connected,
    Disconnecting,
    Disconnected,
}

struct Connection {
    id: String,
    state: ConnectionState,
    connected_at: DateTime<Utc>,
    last_message: DateTime<Utc>,
}

// Track connection state explicitly
impl Connection {
    async fn transition(&mut self, to: ConnectionState) {
        debug!("Connection {} transitioning from {:?} to {:?}", 
               self.id, self.state, to);
        self.state = to;
        
        match to {
            ConnectionState::Disconnected => {
                // Ensure cleanup happens exactly once
                self.cleanup().await;
            }
            _ => {}
        }
    }
}
```

## Testing Plan

1. Connection Lifecycle Tests
```rust
#[tokio::test]
async fn test_connection_lifecycle() {
    // Test full lifecycle:
    // connect -> initialize -> messages -> disconnect
}

#[tokio::test]
async fn test_abnormal_disconnection() {
    // Test cleanup on abnormal disconnection:
    // - Process termination
    // - Network failure
    // - Client crash
}
```

2. Root Path Tests
```rust
#[test]
fn test_root_path_resolution() {
    // Test different working directory scenarios
    // Test project root detection
    // Test fallback behavior
}
```

3. Client Management Tests
```rust
#[tokio::test]
async fn test_client_cleanup() {
    // Test client removal
    // Verify no orphaned clients
    // Check message routing after cleanup
}
```

## Next Steps

1. Immediate Fixes
   - Add connection state tracking
   - Implement proper cleanup on disconnect
   - Fix root path resolution
   - Add more logging around client lifecycle

2. Medium Term
   - Add client heartbeat mechanism
   - Implement reconnection handling
   - Add client session persistence
   - Improve error reporting

3. Long Term
   - Consider client authentication
   - Add connection monitoring dashboard
   - Implement connection pooling
   - Add rate limiting

## Notes

- The WebSocket cleanup issues appear to be a combination of:
  1. Async timing issues in cleanup
  2. Missing state tracking
  3. Incomplete cleanup procedures
  4. Root path resolution bugs

- The fixes need to be coordinated between:
  1. Pylon server (Rust)
  2. Onyx client (TypeScript)
  3. Protocol handlers
  4. File system provider

- Testing should focus on:
  1. Connection edge cases
  2. Cleanup reliability
  3. State consistency
  4. Path resolution