use super::*;
use serde_json::json;

#[test]
fn test_prompt_serialization() {
    let prompt = Prompt {
        name: "test_prompt".to_string(),
        description: Some("A test prompt".to_string()),
        arguments: vec![PromptArgument {
            name: "test_arg".to_string(),
            description: Some("A test argument".to_string()),
            required: true,
        }],
    };

    let json = serde_json::to_value(&prompt).unwrap();
    assert_eq!(
        json,
        json!({
            "name": "test_prompt",
            "description": "A test prompt",
            "arguments": [{
                "name": "test_arg",
                "description": "A test argument",
                "required": true
            }]
        })
    );
}

#[test]
fn test_prompt_message_serialization() {
    let message = PromptMessage {
        role: Role::User,
        content: MessageContent::Text(TextContent {
            text: "Test message".to_string(),
            r#type: "text".to_string(),
            annotations: None,
        }),
    };

    let json = serde_json::to_value(&message).unwrap();
    assert_eq!(
        json,
        json!({
            "role": "user",
            "content": {
                "type": "text",
                "text": "Test message"
            }
        })
    );
}

#[test]
fn test_prompt_message_with_resource() {
    let message = PromptMessage {
        role: Role::User,
        content: MessageContent::Resource(EmbeddedResource {
            r#type: "resource".to_string(),
            resource: ResourceContents::Text(TextResourceContents {
                text: "Resource content".to_string(),
                uri: "file:///test.txt".to_string(),
                mime_type: Some("text/plain".to_string()),
            }),
            annotations: None,
        }),
    };

    let json = serde_json::to_value(&message).unwrap();
    assert_eq!(
        json,
        json!({
            "role": "user",
            "content": {
                "type": "resource",
                "resource": {
                    "text": "Resource content",
                    "uri": "file:///test.txt",
                    "mimeType": "text/plain"
                }
            }
        })
    );
}

#[test]
fn test_prompt_argument_validation() {
    let arg = PromptArgument {
        name: "test_arg".to_string(),
        description: Some("A test argument".to_string()),
        required: true,
    };

    let mut args = HashMap::new();
    args.insert("test_arg".to_string(), "test_value".to_string());

    let prompt = Prompt {
        name: "test_prompt".to_string(),
        description: None,
        arguments: vec![arg],
    };

    let provider = MockPromptProvider::new();
    assert!(provider.validate_arguments(&prompt, &args).is_ok());

    // Test missing required argument
    let empty_args = HashMap::new();
    assert!(matches!(
        provider.validate_arguments(&prompt, &empty_args),
        Err(Error::MissingRequiredArgument(_))
    ));
}

#[test]
fn test_prompt_with_annotations() {
    let message = PromptMessage {
        role: Role::Assistant,
        content: MessageContent::Text(TextContent {
            text: "Test message".to_string(),
            r#type: "text".to_string(),
            annotations: Some(Annotations {
                audience: Some(vec![Role::User, Role::Assistant]),
                priority: Some(0.5),
            }),
        }),
    };

    let json = serde_json::to_value(&message).unwrap();
    assert_eq!(
        json,
        json!({
            "role": "assistant",
            "content": {
                "type": "text",
                "text": "Test message",
                "annotations": {
                    "audience": ["user", "assistant"],
                    "priority": 0.5
                }
            }
        })
    );
}