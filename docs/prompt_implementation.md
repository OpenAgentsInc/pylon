# Prompt Implementation Plan

## Overview

This document outlines the plan for implementing prompt handling capabilities according to the Model Context Protocol (MCP) specification. The implementation will support prompt templates, arguments, and prompt-related notifications.

Prompts in MCP are designed to be **user-controlled**, meaning they are exposed from servers to clients with the intention of the user being able to explicitly select them for use.

## Core Features

Prompts in MCP are predefined templates that can:
- Accept dynamic arguments
- Include context from resources
- Chain multiple interactions
- Guide specific workflows
- Surface as UI elements (like slash commands)

## Core Components

### 1. Types and Structures

```rust
// src-tauri/src/mcp/prompts/types.rs

pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<PromptArgument>,
}

pub struct PromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
}

pub struct PromptMessage {
    pub role: Role,
    pub content: MessageContent,
}

pub enum MessageContent {
    Text(TextContent),
    Image(ImageContent),
    Resource(EmbeddedResource),
}
```

### 2. Provider Interface

```rust
// src-tauri/src/mcp/prompts/provider.rs

pub trait PromptProvider {
    // List available prompts with pagination support
    async fn list_prompts(&self, cursor: Option<String>) -> Result<(Vec<Prompt>, Option<String>)>;
    
    // Get a specific prompt with optional argument values
    async fn get_prompt(&self, name: &str, arguments: Option<HashMap<String, String>>) -> Result<Vec<PromptMessage>>;
    
    // Validate prompt arguments
    fn validate_arguments(&self, prompt: &Prompt, arguments: &HashMap<String, String>) -> Result<()>;
}
```

### 3. File-based Provider

Implement a file-based prompt provider that loads prompts from YAML/JSON files:

```rust
// src-tauri/src/mcp/prompts/providers/filesystem.rs

pub struct FileSystemPromptProvider {
    root_path: PathBuf,
}

impl PromptProvider for FileSystemPromptProvider {
    // Implementation details...
}
```

## Implementation Phases

### Phase 1: Core Types and Provider Interface

1. Create prompt-related type definitions
2. Implement the PromptProvider trait
3. Add error types and handling
4. Set up unit tests for core functionality

### Phase 2: File-based Provider

1. Implement FileSystemPromptProvider
2. Add prompt file loading and parsing
3. Support argument validation
4. Add template substitution
5. Implement resource embedding
6. Add tests for file operations

### Phase 3: Protocol Integration

1. Add prompt-related message handlers:
   - prompts/list
   - prompts/get
2. Implement prompt list change notifications
3. Add prompt capability negotiation
4. Update server capabilities
5. Add integration tests

### Phase 4: Frontend Integration

1. Add prompt-related WebSocket message handling
2. Create prompt management UI components:
   - Slash commands
   - Quick actions
   - Context menu items
   - Command palette entries
   - Guided workflows
   - Interactive forms
3. Implement prompt template visualization
4. Add argument input forms
5. Support prompt list updates

## File Structure

```
src-tauri/src/mcp/
├── prompts/
│   ├── mod.rs              # Module definition and exports
│   ├── types.rs            # Type definitions
│   ├── provider.rs         # Provider trait definition
│   ├── error.rs            # Error types
│   ├── handlers.rs         # Protocol message handlers
│   └── providers/          # Provider implementations
│       ├── mod.rs          # Provider module definition
│       └── filesystem.rs   # File-based provider
```

## Testing Strategy

1. **Unit Tests**
   - Type validation
   - Argument handling
   - Template substitution
   - Provider operations

2. **Integration Tests**
   - Protocol message flow
   - Provider integration
   - Resource embedding
   - Error handling

3. **End-to-End Tests**
   - Complete prompt workflows
   - UI interaction
   - WebSocket communication

## Prompt File Format

```yaml
# prompts/example.yaml
name: code_review
description: "Performs a code review on the given file"
arguments:
  - name: file_path
    description: "Path to the file to review"
    required: true
  - name: style_guide
    description: "Optional style guide to follow"
    required: false
messages:
  - role: system
    content:
      type: text
      text: "You are a code reviewer examining the following file:"
  - role: user
    content:
      type: resource
      uri: "{file_path}"
```

## Dynamic Prompts

### 1. Resource Context

Support embedding resource context in prompts:
```rust
pub struct ResourceContext {
    pub uri: String,
    pub content: String,
    pub mime_type: Option<String>,
}
```

### 2. Multi-step Workflows

Support for chained interactions:
```rust
pub struct WorkflowStep {
    pub messages: Vec<PromptMessage>,
    pub next_step: Option<Box<WorkflowStep>>,
}
```

## Security Considerations

1. **Input Validation**
   - Validate all arguments
   - Sanitize user input
   - Implement strict type checking

2. **Access Control**
   - Implement access controls for prompts
   - Control resource access
   - Audit prompt usage

3. **Data Protection**
   - Handle sensitive data appropriately
   - Implement secure storage
   - Validate generated content

4. **Rate Limiting**
   - Implement request rate limiting
   - Add timeout mechanisms
   - Monitor usage patterns

5. **Prompt Injection Prevention**
   - Validate template syntax
   - Escape special characters
   - Implement content filtering

## Best Practices

1. **Naming and Documentation**
   - Use clear, descriptive prompt names
   - Provide detailed descriptions
   - Document expected argument formats

2. **Error Handling**
   - Validate all required arguments
   - Handle missing arguments gracefully
   - Provide clear error messages

3. **Performance**
   - Cache dynamic content when appropriate
   - Implement lazy loading
   - Optimize template processing

4. **Maintainability**
   - Consider versioning for prompt templates
   - Support template inheritance
   - Enable prompt composability

5. **Testing**
   - Test with various inputs
   - Verify error cases
   - Validate template rendering

## Updates and Changes

1. **Server Capabilities**
   - Implement prompts.listChanged capability
   - Support notifications/prompts/list_changed
   - Handle client re-fetching

2. **Change Management**
   - Track prompt versions
   - Notify clients of updates
   - Support backwards compatibility

## Next Steps

1. Begin with core type definitions and provider trait
2. Implement file-based provider with basic functionality
3. Add protocol message handlers
4. Create basic UI components
5. Implement end-to-end testing

## Future Enhancements

1. **Dynamic Prompts**
   - Runtime prompt generation
   - Context-aware templates
   - Chain of thought prompts

2. **Advanced Features**
   - Prompt versioning
   - Template inheritance
   - Conditional sections
   - Multi-file prompts

3. **Performance Optimizations**
   - Prompt caching
   - Lazy loading
   - Template precompilation

4. **Security**
   - Argument sanitization
   - Resource access control
   - Template validation