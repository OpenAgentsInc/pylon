# Prompt System

## Overview

The prompt system in Pylon provides a way to define reusable, templated interactions with language models. It uses YAML files to define prompts with arguments and message sequences, supporting both text and resource content.

## Architecture

```
pylon/
├── prompts/                    # Prompt definition files
│   ├── README.md              # Documentation for prompt authors
│   └── code_review.yaml       # Example prompt definition
├── src-tauri/src/mcp/prompts/ # Prompt system implementation
│   ├── mod.rs                 # Module exports
│   ├── types.rs               # Type definitions
│   ├── provider.rs            # Provider trait and utilities
│   └── providers/             # Provider implementations
│       └── filesystem.rs      # File-based prompt provider
```

## Protocol

### 1. Capabilities

The prompt system is exposed through the MCP protocol with these capabilities:

```json
{
  "prompts": {
    "list_changed": true
  }
}
```

### 2. Methods

#### prompts/list

Lists available prompts.

Request:
```json
{
  "jsonrpc": "2.0",
  "method": "prompts/list",
  "params": {
    "cursor": "optional-pagination-cursor"
  },
  "id": "1"
}
```

Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "prompts": [
      {
        "name": "code_review",
        "description": "Performs a code review on the given file",
        "arguments": [
          {
            "name": "file_path",
            "description": "Path to the file to review",
            "required": true
          },
          {
            "name": "style_guide",
            "description": "Optional style guide to follow",
            "required": false
          }
        ]
      }
    ],
    "cursor": null
  },
  "id": "1"
}
```

#### prompts/get

Gets a specific prompt and processes it with the given arguments.

Request:
```json
{
  "jsonrpc": "2.0",
  "method": "prompts/get",
  "params": {
    "name": "code_review",
    "arguments": {
      "file_path": "src/App.tsx",
      "style_guide": "React Native best practices"
    }
  },
  "id": "2"
}
```

Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "messages": [
      {
        "role": "assistant",
        "content": {
          "content_type": "text",
          "text": "I will review the following file..."
        }
      },
      {
        "role": "user",
        "content": {
          "content_type": "resource",
          "resource": {
            "type": "Text",
            "uri": "src/App.tsx",
            "mime_type": "text/typescript",
            "text": "..."
          }
        }
      },
      {
        "role": "assistant",
        "content": {
          "content_type": "text",
          "text": "Here is my review of the code..."
        }
      }
    ]
  },
  "id": "2"
}
```

## Prompt Definition

Prompts are defined in YAML files with the following structure:

```yaml
name: prompt_name
description: "Description of what the prompt does"
arguments:
  - name: arg1
    description: "Description of argument 1"
    required: true
  - name: arg2
    description: "Description of argument 2"
    required: false
messages:
  - role: assistant
    content:
      type: text
      text: "Initial message setting context"
  - role: user
    content:
      type: resource
      uri: "{arg1}"
  - role: assistant
    content:
      type: text
      text: "Response template"
```

### Message Types

1. Text Message
```yaml
role: assistant
content:
  type: text
  text: "Message text here"
```

2. Resource Message
```yaml
role: user
content:
  type: resource
  uri: "{file_path}"
```

3. Image Message
```yaml
role: user
content:
  type: image
  data: [binary data]
  mime_type: "image/png"
```

### Template Substitution

Arguments can be referenced in text and resource URIs using curly braces:
- Text: `"Reviewing file: {file_path}"`
- URI: `"{file_path}"`

## Implementation Details

### 1. Provider Interface

```rust
#[async_trait]
pub trait PromptProvider {
    async fn list_prompts(&self, cursor: Option<String>) 
        -> Result<(Vec<Prompt>, Option<String>)>;
    
    async fn get_prompt(&self, name: &str, arguments: Option<HashMap<String, String>>) 
        -> Result<Vec<PromptMessage>>;
    
    fn validate_arguments(&self, prompt: &Prompt, arguments: &HashMap<String, String>) 
        -> Result<()>;
}
```

### 2. Message Processing

The system processes messages in this order:

1. Load prompt definition
2. Validate required arguments
3. For each message:
   - For text content: substitute argument values
   - For resource content: 
     - Substitute URI
     - Load resource content
   - For image content: pass through unchanged
4. Return processed messages

### 3. Resource Integration

Resource messages are processed using the resource provider system:

1. URI is processed with argument substitution
2. Resource is loaded via resource provider
3. Content is embedded in the message
4. Message is returned with the loaded content

## Example: Code Review

### 1. Prompt Definition

```yaml
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
  - role: assistant
    content:
      type: text
      text: |
        I will review the following file following best practices and suggest improvements for:
        1. Performance
        2. Code organization
        3. React/TypeScript usage
        4. Error handling
        5. UI/UX patterns

        I will format my response with clear sections and provide specific examples for any suggested improvements.

        Let me examine the file:

  - role: user
    content:
      type: resource
      uri: "{file_path}"

  - role: assistant
    content:
      type: text
      text: |
        Here is my review of the code:

        ## Code Organization
        {Let me analyze the code structure and organization...}

        ## Performance Considerations
        {I'll look for performance optimizations...}

        ## React/TypeScript Usage
        {I'll review React patterns and TypeScript types...}

        ## Error Handling
        {I'll examine error handling patterns...}

        ## UI/UX Patterns
        {I'll review UI/UX best practices...}

        ## Style Guide Compliance
        {If a style guide was provided, I'll check compliance...}

        ## Summary
        {I'll summarize the key findings and recommendations...}
```

### 2. Frontend Integration

```typescript
const handleCodeReviewPrompt = useCallback(async () => {
  if (chatLoading || loading || !selectedFile) return;

  setLoading(true);
  try {
    // Get the prompt result
    const result = await sendWsMessage({
      jsonrpc: '2.0',
      method: 'prompts/get',
      params: {
        name: 'code_review',
        arguments: {
          file_path: selectedFile,
          style_guide: 'React Native best practices'
        }
      }
    });

    // Send each message to the chat
    if (result.messages) {
      for (const msg of result.messages) {
        await sendMessage(msg.content);
      }
    }
    
    scrollViewRef.current?.scrollToEnd({ animated: true });
  } catch (err) {
    console.error('Failed to send code review prompt:', err);
  } finally {
    setLoading(false);
  }
}, [chatLoading, loading, selectedFile, sendWsMessage, sendMessage]);
```

## Security Considerations

1. **Resource Access**
   - Resource URIs are validated before loading
   - Resource provider enforces access controls
   - File paths are sanitized

2. **Argument Validation**
   - Required arguments are enforced
   - Argument values are sanitized
   - Template substitution is validated

3. **Content Safety**
   - Message content is validated
   - Resource content is validated
   - MIME types are checked

## Error Handling

1. **Prompt Errors**
   - PromptNotFound: Prompt file doesn't exist
   - MissingRequiredArgument: Required argument not provided
   - InvalidTemplate: Template substitution failed

2. **Resource Errors**
   - ResourceError: Resource loading failed
   - IoError: File system error
   - YamlError: YAML parsing error

## Best Practices

1. **Prompt Design**
   - Keep prompts focused and single-purpose
   - Document arguments clearly
   - Use descriptive names
   - Include example responses
   - Consider error cases

2. **Resource Usage**
   - Use resource messages for file content
   - Keep resource paths relative
   - Handle resource loading errors

3. **Message Flow**
   - Start with context-setting message
   - Include resource content
   - End with structured response

4. **Frontend Integration**
   - Handle loading states
   - Show error messages
   - Provide progress feedback
   - Allow cancellation

## Future Enhancements

1. **Prompt Features**
   - Prompt versioning
   - Template inheritance
   - Conditional sections
   - Multi-file prompts

2. **Resource Types**
   - Binary file support
   - Remote resources
   - Database resources

3. **UI Integration**
   - Prompt discovery
   - Argument forms
   - Response formatting
   - Progress indicators