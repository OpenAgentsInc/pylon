# Architecture Overview

This monorepo is designed to implement Pylon, a comprehensive Nostr relay for the OpenAgents AI ecosystem. The current packages serve as a foundation and will be evolved into the full Pylon architecture.

> **📖 For detailed package architecture, see [Packages Architecture](./packages-architecture.md)**

## Current State: MVP Foundation

The monorepo currently contains three foundational packages that demonstrate the architectural patterns Pylon will use:

### `@openagentsinc/domain` - Protocol Schemas & Contracts
- **Current**: Todo application schemas and API contracts
- **Future**: Nostr event schemas, NIP implementations, WebSocket message types
- **Purpose**: Single source of truth for protocol compliance and shared contracts

### `@openagentsinc/server` - Server Implementation  
- **Current**: HTTP server with in-memory Todo storage
- **Future**: WebSocket-based Nostr relay with PostgreSQL/pglite backends
- **Purpose**: High-performance relay server implementation

### `@openagentsinc/cli` - Development Tools
- **Current**: Todo management CLI client
- **Future**: Relay management, testing, and operational tools
- **Purpose**: Developer experience and operational tooling

## Planned Evolution to Full Pylon

The current packages will evolve into a comprehensive Nostr relay architecture:

### Core Foundation
- **`@openagentsinc/pylon-domain`** - Nostr protocol schemas and NIP implementations
- **`@openagentsinc/pylon-core`** - Core relay services (Event, Subscription, Auth, etc.)

### Deployment Variants
- **`@openagentsinc/pylon-server`** - Full-scale PostgreSQL-backed relay
- **`@openagentsinc/pylon-edge`** - Lightweight pglite-based edge/embedded relay
- **`@openagentsinc/pylon-actors`** - Rivet actor implementations for distributed management

### Specialized Features
- **`@openagentsinc/pylon-ai`** - AI agent integration and NIP-90 DVM support
- **`@openagentsinc/pylon-admin`** - Administrative interface and monitoring
- **`@openagentsinc/pylon-cli`** - Enhanced development and operational tools

## Key Architectural Benefits

- **Contract-First Development**: Protocol schemas defined before implementations ensure compliance
- **Multi-Deployment Support**: Same codebase supports cloud, edge, and embedded scenarios  
- **Effect Services Pattern**: Composable, testable services with dependency injection
- **Actor-Based Concurrency**: Rivet actors enable massive scale and distributed management
- **Provider-Agnostic AI**: Effect AI abstractions allow flexible LLM integrations
- **Type Safety**: End-to-end TypeScript with branded types prevents runtime errors
- **Monorepo Benefits**: Coordinated development and shared contracts across all packages

## Implementation Strategy

### Phase 1: Foundation (Current)
- ✅ Basic monorepo structure with Effect services
- ✅ Contract-first development patterns
- 🔄 Convert from Todo app to basic Nostr relay

### Phase 2: Core Relay
- 📋 Implement NIP-01 basic protocol in domain package
- 📋 WebSocket server with event storage in server package  
- 📋 Basic CLI tools for testing and management

### Phase 3: Scale & Performance
- 📋 Rivet actor integration for distributed connection management
- 📋 pglite-based edge relay for embedded deployment
- 📋 Enhanced NIP support and performance optimization

### Phase 4: AI & Advanced Features
- 📋 AI agent coordination and NIP-90 DVM support
- 📋 Administrative interface and monitoring
- 📋 Advanced privacy and security features

This architecture ensures Pylon can scale from personal embedded relays to global infrastructure serving 100 million users while maintaining focus on AI agent coordination and privacy-first communication.