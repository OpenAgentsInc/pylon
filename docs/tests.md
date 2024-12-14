# Pylon Test Plan

## Currently passing tests

test mcp::capabilities::tests::test_capability_negotiation_with_experimental ... ok
test mcp::capabilities::tests::test_capability_negotiation_with_roots ... ok
test mcp::protocol::tests::test_unknown_method ... ok
test mcp::protocol::tests::test_initialize_request ... ok
test mcp::capabilities::tests::test_capability_update ... ok
test mcp::capabilities::tests::test_client_registration ... ok
test mcp::capabilities::tests::test_client_removal ... ok
test tests::mcp::server_tests::tests::test_websocket_connection ... ok
test tests::mcp::server_tests::tests::test_websocket_echo ... ok
test tests::mcp::server_tests::tests::test_multiple_clients ... ok
test mcp::prompts::provider::tests::test_validate_required_arguments ... ok
test mcp::prompts::provider::tests::test_process_prompt_messages ... ok

## Test Structure

```
pylon/
├── src-tauri/
│   ├── src/
│   │   ├── mcp/
│   │   │   ├── prompts/
│   │   │   │   ├── provider.rs          # Prompt provider tests
│   │   │   │   │   ├── test_validate_required_arguments
│   │   │   │   │   └── test_process_prompt_messages
│   │   │   │   └── providers/
│   │   │   │       └── filesystem.rs    # Filesystem provider tests
│   │   │   ├── capabilities/
│   │   │   │   └── tests/              # Capability system tests
│   │   │   │       ├── test_capability_negotiation_with_experimental
│   │   │   │       ├── test_capability_negotiation_with_roots
│   │   │   │       ├── test_capability_update
│   │   │   │       ├── test_client_registration
│   │   │   │       └── test_client_removal
│   │   │   └── protocol/
│   │   │       └── tests/              # Protocol handling tests
│   │   │           ├── test_unknown_method
│   │   │           └── test_initialize_request
│   │   └── tests/
│   │       ├── mod.rs                  # Test module configuration
│   │       ├── mcp/
│   │       │   ├── mod.rs              # MCP test module
│   │       │   ├── server_tests.rs     # Server tests
│   │       │   │   ├── test_websocket_connection
│   │       │   │   ├── test_websocket_echo
│   │       │   │   └── test_multiple_clients
│   │       │   ├── client_tests.rs     # Client connection tests
│   │       │   ├── protocol_tests.rs   # Protocol message tests
│   │       │   ├── resource_tests.rs   # Resource provider tests
│   │       │   ├── tool_tests.rs       # Tool provider tests
│   │       │   └── integration_tests.rs # MCP integration tests
│   │       ├── nostr/
│   │       │   ├── mod.rs              # Nostr test module
│   │       │   ├── event_tests.rs      # Event handling tests
│   │       │   ├── job_tests.rs        # Job management tests
│   │       │   └── integration_tests.rs # Nostr integration tests
│   │       └── breez/
│   │           ├── mod.rs              # Breez test module
│   │           ├── payment_tests.rs     # Payment processing tests
│   │           └── integration_tests.rs # Payment integration tests
├── src/
│   └── __tests__/                      # Frontend tests
│       ├── components/                  # Component tests
│       ├── stores/                     # State management tests
│       └── integration/                # Frontend integration tests
└── e2e/                               # End-to-end tests
    ├── setup.ts
    └── scenarios/
```

## Backend Tests (Rust)

### 1. MCP Prompt Tests

**Provider Tests (provider.rs)**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_validate_required_arguments() {
        // Test argument validation
    }

    #[tokio::test]
    async fn test_process_prompt_messages() {
        // Test message processing
    }
}
```

### 2. MCP Server Tests

**Server Core (server_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_websocket_connection() {
        // Test server connection handling
    }

    #[tokio::test]
    async fn test_websocket_echo() {
        // Test message echo functionality
    }

    #[tokio::test]
    async fn test_multiple_clients() {
        // Test multiple client handling
    }
}
```

### 3. MCP Protocol Tests

**Protocol Handling (protocol_tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_initialize_request() {
        // Test initialization request/response
    }

    #[tokio::test]
    async fn test_unknown_method() {
        // Test unknown method handling
    }
}
```

### 4. MCP Capabilities Tests

**Capability System (capabilities/tests.rs)**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_capability_negotiation_with_experimental() {
        // Test experimental capability negotiation
    }

    #[tokio::test]
    async fn test_capability_negotiation_with_roots() {
        // Test roots capability negotiation
    }

    #[tokio::test]
    async fn test_capability_update() {
        // Test capability updates
    }

    #[tokio::test]
    async fn test_client_registration() {
        // Test client registration
    }

    #[tokio::test]
    async fn test_client_removal() {
        // Test client removal
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