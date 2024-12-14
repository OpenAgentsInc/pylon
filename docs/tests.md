# Pylon Test Plan

## Currently passing tests

running 22 tests
test mcp::prompts::types::tests::test_message_content_serialization ... ok
test mcp::prompts::types::tests::test_template_substitution ... ok
test mcp::prompts::provider::tests::test_validate_required_arguments ... ok
test mcp::prompts::provider::tests::test_process_prompt_messages ... ok
test mcp::protocol::tests::test_unknown_method ... ok
test mcp::prompts::types::tests::test_yaml_serialization ... ok
test tests::mcp::prompts::test_list_prompts ... ok
test tests::mcp::prompts::test_get_prompt ... ok
test mcp::clients::tests::test_client_management ... ok
test tests::mcp::prompts::test_filesystem_provider ... ok
test tests::mcp::prompts::test_prompt_with_resource ... ok
test mcp::protocol::tests::test_initialize_request ... ok
test tests::mcp::server_tests::tests::test_websocket_connection ... ok
test mcp::providers::ollama::tests::test_chat_stream ... ok
test tests::ollama_tests::tests::test_ollama_error_handling ... ok
test mcp::providers::ollama::tests::test_list_models ... ok
test mcp::providers::ollama::tests::test_chat ... ok
test tests::ollama_tests::tests::test_ollama_streaming ... ok
test mcp::protocol::tests::test_ollama_chat ... ok
test tests::ollama_tests::tests::test_ollama_integration ... ok
test tests::mcp::server_tests::tests::test_websocket_echo ... ok
test tests::mcp::server_tests::tests::test_multiple_clients ... ok

test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

## Test Structure

```
pylon/
├── src-tauri/
│   ├── src/
│   │   └── tests/
│   │       ├── mod.rs                    # Test module configuration
│   │       ├── mcp/
│   │       │   ├── mod.rs                # MCP test module
│   │       │   ├── server_tests.rs       # Server initialization and core functionality
│   │       │   ├── client_tests.rs       # Client connection and management
│   │       │   ├── protocol_tests.rs     # Protocol message handling
│   │       │   ├── resource_tests.rs     # Resource provider functionality
│   │       │   ├── tool_tests.rs         # Tool provider functionality
│   │       │   └── integration_tests.rs  # MCP integration scenarios
│   │       ├── nostr/
│   │       │   ├── mod.rs                # Nostr test module
│   │       │   ├── event_tests.rs        # Event handling and validation
│   │       │   ├── job_tests.rs          # Job management and execution
│   │       │   └── integration_tests.rs  # Nostr integration scenarios
│   │       └── breez/
│   │           ├── mod.rs                # Breez test module
│   │           ├── payment_tests.rs      # Payment processing
│   │           └── integration_tests.rs  # Payment integration scenarios
├── src/
│   └── __tests__/                        # Frontend tests
│       ├── components/                    # Component tests
│       ├── stores/                       # State management tests
│       └── integration/                  # Frontend integration tests
└── e2e/                                  # End-to-end tests
    ├── setup.ts
    └── scenarios/
```

## Backend Tests (Rust)

### 1. MCP Server Tests

**Server Core (server_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_initialization() {
        // Test server creation with default config
    }

    #[tokio::test]
    async fn test_server_shutdown() {
        // Test clean server shutdown
    }

    #[tokio::test]
    async fn test_multiple_client_connections() {
        // Test handling multiple simultaneous clients
    }
}
```

**Client Management (client_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_client_connection() {
        // Test new client connection handling
    }

    #[tokio::test]
    async fn test_client_capabilities() {
        // Test client capability negotiation
    }

    #[tokio::test]
    async fn test_client_disconnection() {
        // Test client cleanup on disconnect
    }
}
```

**Protocol Handling (protocol_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_initialize_request() {
        // Test initialization request/response
    }

    #[tokio::test]
    async fn test_resource_request() {
        // Test resource request handling
    }

    #[tokio::test]
    async fn test_tool_request() {
        // Test tool invocation
    }
}
```

### 2. Resource Provider Tests

**File System Provider (resource_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_file_read() {
        // Test file reading functionality
    }

    #[tokio::test]
    async fn test_directory_listing() {
        // Test directory content listing
    }

    #[tokio::test]
    async fn test_resource_updates() {
        // Test resource change notifications
    }
}
```

### 3. Tool Provider Tests

**Tool Management (tool_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_tool_registration() {
        // Test tool provider registration
    }

    #[tokio::test]
    async fn test_tool_execution() {
        // Test tool execution flow
    }

    #[tokio::test]
    async fn test_tool_error_handling() {
        // Test error scenarios in tools
    }
}
```

### 4. Nostr/NIP-90 Tests

**Event Handling (event_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_event_validation() {
        // Test NIP-90 event validation
    }

    #[tokio::test]
    async fn test_event_processing() {
        // Test event processing pipeline
    }
}
```

**Job Management (job_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_job_creation() {
        // Test job creation from events
    }

    #[tokio::test]
    async fn test_job_execution() {
        // Test job execution flow
    }

    #[tokio::test]
    async fn test_job_result_delivery() {
        // Test result delivery
    }
}
```

### 5. Payment Tests

**Payment Processing (payment_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_invoice_generation() {
        // Test Lightning invoice creation
    }

    #[tokio::test]
    async fn test_payment_verification() {
        // Test payment verification
    }

    #[tokio::test]
    async fn test_payment_timeout() {
        // Test payment timeout handling
    }
}
```

## Frontend Tests (TypeScript/Jest)

### 1. Component Tests

**Dashboard Components**
```typescript
// __tests__/components/Dashboard.test.tsx
describe('Dashboard', () => {
    test('renders node status correctly', () => {
        // Test status display
    });

    test('updates metrics in real-time', () => {
        // Test metric updates
    });
});
```

**Configuration Components**
```typescript
// __tests__/components/Config.test.tsx
describe('Configuration', () => {
    test('saves settings correctly', () => {
        // Test settings persistence
    });

    test('validates input properly', () => {
        // Test input validation
    });
});
```

### 2. Store Tests

**State Management**
```typescript
// __tests__/stores/nodeStore.test.ts
describe('Node Store', () => {
    test('updates node state correctly', () => {
        // Test state updates
    });

    test('handles errors properly', () => {
        // Test error handling
    });
});
```

## Integration Tests

### 1. Backend Integration

**MCP-Nostr Integration**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_job_flow_with_payment() {
        // Test complete job flow including payment
    }
}
```

### 2. Frontend Integration

```typescript
// __tests__/integration/jobFlow.test.tsx
describe('Job Flow', () => {
    test('complete job cycle', async () => {
        // Test job creation to completion
    });
});
```

## End-to-End Tests

```typescript
// e2e/scenarios/fullJobCycle.test.ts
describe('Full Job Cycle', () => {
    test('job from request to payment', async () => {
        // Test complete system flow
    });
});
```

## Test Coverage Requirements

1. **Unit Test Coverage**
   - Minimum 90% line coverage
   - Minimum 85% branch coverage
   - Critical paths must have 100% coverage

2. **Integration Test Coverage**
   - All major workflows covered
   - Error scenarios tested
   - Edge cases handled

3. **E2E Test Coverage**
   - All user-facing workflows tested
   - Payment flows verified
   - Network error scenarios covered

## Test Utilities

```rust
// src-tauri/src/tests/utils/mod.rs
pub mod test_utils {
    pub struct TestServer {
        // Test server configuration
    }

    pub struct MockClient {
        // Mock client for testing
    }

    pub struct MockPaymentProvider {
        // Mock payment processing
    }
}
```

## Continuous Integration

### 1. GitHub Actions Workflow
```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run unit tests
        run: cargo test
      - name: Run integration tests
        run: cargo test --test '*'
      - name: Run frontend tests
        run: npm test
      - name: Run E2E tests
        run: npm run test:e2e
```

### 2. Coverage Reporting
- Use cargo-tarpaulin for Rust coverage
- Use Jest coverage for TypeScript
- Generate combined coverage reports

## Test Documentation

### 1. Test Plan Documentation
- Test strategy overview
- Coverage requirements
- Test environment setup

### 2. Test Case Documentation
- Detailed test case descriptions
- Input/output specifications
- Edge case considerations
