# Prompt Flow Between Onyx and Pylon

## Overview

This document describes the implementation of prompt-based interactions between the Onyx frontend and Pylon backend, specifically focusing on how predefined prompts are triggered and processed.

## Architecture

```
Onyx (Frontend)                    Pylon (Backend)
+----------------+                +----------------+
|                |                |                |
| InboxScreen    |  WebSocket    | MCP Server     |
| - UI Elements  | ----------->  | - Prompt       |
| - Prompt       |   JSON-RPC    |   Handler      |
|   Triggers     |               | - Resource     |
|                |               |   Provider     |
+----------------+                +----------------+
```

## Implementation Details

### 1. Frontend Components (Onyx)

#### InboxScreen.tsx
The main chat interface includes:
- Regular chat input
- Special prompt buttons (e.g., "Code Review This File")
- Message display area

#### Prompt Trigger Implementation
```typescript
const handleCodeReviewPrompt = useCallback(async () => {
  if (isLoading) return

  try {
    await sendMessage(JSON.stringify({
      jsonrpc: '2.0',
      method: 'prompts/get',
      params: {
        name: 'code_review',
        arguments: {
          file_path: 'app/screens/InboxScreen.tsx',
          style_guide: 'React Native best practices'
        }
      }
    }))
    scrollViewRef.current?.scrollToEnd({ animated: true })
  } catch (err) {
    console.error('Failed to send code review prompt:', err)
  }
}, [isLoading, sendMessage])
```

### 2. Backend Processing (Pylon)

#### Prompt Definition (YAML)
```yaml
# prompts/code_review.yaml
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

#### Message Flow

1. **Frontend Trigger**
   - User clicks "Code Review This File" button
   - Frontend constructs JSON-RPC message
   - Message sent via WebSocket

2. **Backend Processing**
   - MCP Server receives message
   - Validates prompt name and arguments
   - Loads prompt template
   - Resolves file content via Resource Provider
   - Processes prompt with arguments

3. **Response Flow**
   - Backend generates response
   - Sends back via WebSocket
   - Frontend displays in chat interface

## WebSocket Communication

### Request Format
```json
{
  "jsonrpc": "2.0",
  "method": "prompts/get",
  "params": {
    "name": "code_review",
    "arguments": {
      "file_path": "app/screens/InboxScreen.tsx",
      "style_guide": "React Native best practices"
    }
  }
}
```

### Response Format
```json
{
  "jsonrpc": "2.0",
  "result": {
    "message": {
      "role": "assistant",
      "content": "Here's my review of InboxScreen.tsx..."
    }
  }
}
```

## Error Handling

1. **Frontend Errors**
   - WebSocket connection issues
   - Loading state management
   - Error message display

2. **Backend Errors**
   - Invalid prompt names
   - Missing required arguments
   - Resource access failures
   - Template processing errors

## UI/UX Considerations

1. **Button States**
   - Normal: Ready to trigger prompt
   - Disabled: During processing
   - Error: When prompt fails

2. **Visual Feedback**
   - Loading indicators
   - Error messages
   - Auto-scroll to responses

3. **Message Threading**
   - Prompt requests and responses are part of the chat history
   - Maintains conversation context

## Security Considerations

1. **Input Validation**
   - Validate file paths
   - Sanitize arguments
   - Prevent path traversal

2. **Resource Access**
   - Verify file access permissions
   - Limit accessible paths
   - Validate resource types

## Future Enhancements

1. **Additional Prompts**
   - Add more predefined prompts
   - Support custom prompt creation
   - Enable prompt chaining

2. **UI Improvements**
   - Prompt selection menu
   - Argument input forms
   - Response formatting

3. **Integration Features**
   - Version control integration
   - CI/CD workflow triggers
   - Team collaboration features

## Testing

1. **Unit Tests**
   - Button functionality
   - Message formatting
   - Error handling

2. **Integration Tests**
   - WebSocket communication
   - Prompt processing
   - Resource access

3. **End-to-End Tests**
   - Complete prompt workflows
   - Error scenarios
   - UI interactions