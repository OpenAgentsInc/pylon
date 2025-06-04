# Pylon Implementation Roadmap

This document outlines the step-by-step plan for evolving the current Todo application foundation into the full Pylon Nostr relay architecture.

## Current State Assessment

### ✅ What We Have
- **Monorepo Structure**: pnpm workspace with Effect services pattern
- **Contract-First Development**: Domain schemas driving implementation  
- **TypeScript Foundation**: Strict typing with branded types
- **Effect Services**: Composable service architecture
- **Build System**: Coordinated builds across packages
- **Development Tools**: CLI package for operations

### 🔄 What Needs Evolution
- **Protocol**: Todo schemas → Nostr event schemas
- **Transport**: HTTP REST API → WebSocket-based Nostr protocol
- **Storage**: In-memory HashMap → PostgreSQL with Nostr-optimized schema
- **Functionality**: CRUD operations → Event validation, subscription matching, real-time broadcast

## Phase 1: Foundation Transformation (Weeks 1-2)

### 1.1 Domain Package Evolution
**Goal**: Transform `@openagentsinc/domain` into `@openagentsinc/pylon-domain`

**Tasks**:
- [ ] **Replace Todo schemas with Nostr event schemas**
  - Basic event structure (id, pubkey, created_at, kind, tags, content, sig)
  - Event kinds (0, 1, 3, 5, 7, etc.)
  - Branded types for EventId, PubKey, Signature
- [ ] **Implement core NIPs**
  - NIP-01: Basic protocol flow and event structure
  - NIP-02: Contact List (kind 3)
  - NIP-09: Event Deletion (kind 5)
- [ ] **WebSocket message schemas**
  - CLIENT → RELAY: EVENT, REQ, CLOSE, AUTH
  - RELAY → CLIENT: EVENT, OK, EOSE, CLOSED, NOTICE
- [ ] **Filter schemas and validation**
  - ids, authors, kinds, tags, since, until, limit
  - Complex filter logic with AND/OR operations

### 1.2 Server Package Evolution  
**Goal**: Transform HTTP server into WebSocket Nostr relay

**Tasks**:
- [ ] **Replace HTTP server with WebSocket server**
  - WebSocket connection management
  - Message parsing and validation
  - Connection lifecycle (connect, authenticate, close)
- [ ] **Implement basic event handling**
  - Event validation (signature, structure, PoW)
  - Event storage and retrieval
  - Real-time event broadcast to subscribers
- [ ] **Basic subscription management**
  - Subscription creation and lifecycle
  - Event-to-subscription matching
  - EOSE (End of Stored Events) handling

### 1.3 CLI Package Enhancement
**Goal**: Create Nostr-specific development tools

**Tasks**:
- [ ] **Event testing tools**
  - Generate valid test events
  - Send events to relay for testing
  - Validate relay responses
- [ ] **Subscription testing**
  - Create test subscriptions
  - Verify event matching
  - Test filter logic
- [ ] **Relay management**
  - Start/stop relay instances
  - Monitor connection counts
  - View real-time event stream

## Phase 2: Core Relay Features (Weeks 3-4)

### 2.1 Enhanced Event Processing
- [ ] **Event types and handling**
  - Regular events (store all)
  - Replaceable events (store latest only)
  - Ephemeral events (forward only, don't store)
  - Addressable events (parameterized replaceable)
- [ ] **Advanced validation**
  - Schnorr signature verification
  - Event ID validation (SHA256 hash)
  - Content size limits and rate limiting
- [ ] **Storage optimization**
  - Database indexes for common query patterns
  - Event expiration (NIP-40)
  - Efficient tag querying

### 2.2 PostgreSQL Integration
- [ ] **Database schema design**
  - Optimized events table with proper indexes
  - Event tags table for efficient filtering
  - Authentication and configuration tables
- [ ] **Query optimization**
  - Prepared statements for common operations
  - Connection pooling
  - Read replica support planning
- [ ] **Migration system**
  - Schema versioning
  - Safe migration procedures
  - Rollback capabilities

### 2.3 Authentication & Security
- [ ] **NIP-42 Authentication**
  - Challenge-response authentication
  - JWT token management
  - Authenticated user permissions
- [ ] **Basic security features**
  - Rate limiting per connection
  - IP-based restrictions
  - Event size limits

## Phase 3: Scale & Performance (Weeks 5-8)

### 3.1 Rivet Actor Integration
**Goal**: Create `@openagentsinc/pylon-actors` package

**Tasks**:
- [ ] **ClientConnectionActor**
  - Manage individual WebSocket connections
  - Handle rate limiting per connection
  - Maintain connection state and subscriptions
- [ ] **SubscriptionMatcherActor**
  - Efficient event-to-subscription matching
  - Optimized data structures for large subscription sets
  - Fan-out logic for event distribution
- [ ] **EventProcessorActor**
  - Parallel event validation
  - Distributed event storage
  - Event deduplication logic

### 3.2 Edge Relay Implementation
**Goal**: Create `@openagentsinc/pylon-edge` package

**Tasks**:
- [ ] **pglite integration**
  - WASM PostgreSQL setup
  - Local storage persistence (IndexedDB/filesystem)
  - Simplified schema for edge deployment
- [ ] **Sync protocols**
  - Relay-to-relay synchronization
  - Delta sync for bandwidth efficiency
  - Conflict resolution strategies
- [ ] **Offline capabilities**
  - Event queuing during network outages
  - Background synchronization
  - Bandwidth-aware sync

### 3.3 Performance Optimization
- [ ] **Subscription tree optimization**
  - Efficient data structures for filter matching
  - Bloom filters for quick rejection
  - Lazy evaluation for complex filters
- [ ] **Caching layers**
  - Hot event caching
  - Subscription result caching
  - Connection metadata caching
- [ ] **Metrics and monitoring**
  - Prometheus metrics export
  - Connection and event rate tracking
  - Performance bottleneck identification

## Phase 4: AI & Advanced Features (Weeks 9-12)

### 4.1 AI Integration
**Goal**: Create `@openagentsinc/pylon-ai` package

**Tasks**:
- [ ] **NIP-90 DVM implementation**
  - Data Vending Machine job routing
  - Job request/response handling
  - Provider discovery and selection
- [ ] **Effect AI integration**
  - Provider-agnostic LLM interfaces
  - AI-powered content analysis
  - Intelligent spam detection
- [ ] **Agent coordination**
  - Custom event kinds for agent communication
  - Swarm coordination protocols
  - Agent state management

### 4.2 Administrative Interface
**Goal**: Create `@openagentsinc/pylon-admin` package

**Tasks**:
- [ ] **Monitoring dashboard**
  - Real-time connection metrics
  - Event throughput visualization
  - Health status indicators
- [ ] **Configuration management**
  - Dynamic relay configuration
  - Feature flag management
  - Performance tuning interface
- [ ] **Moderation tools**
  - Content filtering and review
  - User management and banning
  - Spam detection and prevention

### 4.3 Advanced NIP Support
- [ ] **Privacy-focused NIPs**
  - NIP-17: Private Direct Messages
  - NIP-59: Gift Wrap
  - NIP-44: Versioned Encryption
- [ ] **Discovery and metadata**
  - NIP-11: Relay Information Document
  - NIP-65: Relay List Metadata
  - NIP-89: Recommended Application Handlers
- [ ] **Content and media**
  - NIP-94: File Metadata
  - NIP-50: Search Capability
  - NIP-28: Public Chat

## Success Criteria

### Phase 1 Success
- [ ] Basic Nostr relay that handles REQ/EVENT/CLOSE messages
- [ ] Event validation and storage working
- [ ] Simple subscription matching functional
- [ ] CLI tools can test basic relay functionality

### Phase 2 Success  
- [ ] Full NIP-01 compliance
- [ ] PostgreSQL storage with optimized queries
- [ ] NIP-42 authentication working
- [ ] Basic rate limiting and security

### Phase 3 Success
- [ ] Rivet actors handling 1000+ concurrent connections
- [ ] Edge relay running in embedded scenarios
- [ ] Performance targets met (sub-100ms event processing)
- [ ] Comprehensive monitoring and metrics

### Phase 4 Success
- [ ] AI agent coordination working
- [ ] NIP-90 DVM implementation complete
- [ ] Administrative interface operational
- [ ] Advanced NIP support enabling rich applications

## Development Practices

- **Test-Driven Development**: Write tests before implementation
- **Contract-First**: Always update domain schemas first
- **Incremental Deployment**: Each phase should be deployable
- **Performance Monitoring**: Benchmark every major change
- **Documentation**: Keep docs updated with each phase
- **Community Feedback**: Regular testing with Nostr ecosystem

This roadmap ensures systematic evolution from the current Todo foundation to a production-ready Nostr relay capable of serving the OpenAgents AI ecosystem at scale.