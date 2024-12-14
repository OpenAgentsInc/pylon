use super::*;
use tokio::test;

#[test]
async fn test_list_prompts() {
    let provider = MockPromptProvider::new();
    let (prompts, cursor) = provider.list_prompts(None).await.unwrap();

    assert_eq!(prompts.len(), 1);
    assert!(cursor.is_none());

    let prompt = &prompts[0];
    assert_eq!(prompt.name, "test_prompt");
    assert_eq!(prompt.description, Some("A test prompt".to_string()));
    assert_eq!(prompt.arguments.len(), 1);

    let arg = &prompt.arguments[0];
    assert_eq!(arg.name, "test_arg");
    assert_eq!(arg.description, Some("A test argument".to_string()));
    assert!(arg.required);
}

#[test]
async fn test_get_prompt() {
    let provider = MockPromptProvider::new();
    let messages = provider.get_prompt("test_prompt", None).await.unwrap();

    assert_eq!(messages.len(), 1);
    let message = &messages[0];
    assert_eq!(message.role, Role::User);

    match &message.content {
        MessageContent::Text(text) => {
            assert_eq!(text.text, "Test message");
            assert_eq!(text.r#type, "text");
        }
        _ => panic!("Expected TextContent"),
    }
}

#[test]
async fn test_get_nonexistent_prompt() {
    let provider = MockPromptProvider::new();
    let result = provider.get_prompt("nonexistent", None).await;
    assert!(matches!(result, Err(Error::PromptNotFound(_))));
}

#[test]
async fn test_prompt_with_arguments() {
    let provider = MockPromptProvider::new();
    let mut args = HashMap::new();
    args.insert("test_arg".to_string(), "test_value".to_string());

    let messages = provider.get_prompt("test_prompt", Some(args)).await.unwrap();
    assert!(!messages.is_empty());
}

#[test]
async fn test_prompt_pagination() {
    let provider = MockPromptProvider::new();
    
    // First page
    let (prompts, cursor) = provider.list_prompts(None).await.unwrap();
    assert!(!prompts.is_empty());
    
    // No more pages expected in mock
    if let Some(cursor) = cursor {
        let (next_prompts, next_cursor) = provider.list_prompts(Some(cursor)).await.unwrap();
        assert!(next_prompts.is_empty());
        assert!(next_cursor.is_none());
    }
}

#[test]
fn test_validate_arguments() {
    let provider = MockPromptProvider::new();
    let prompt = Prompt {
        name: "test".to_string(),
        description: None,
        arguments: vec![
            PromptArgument {
                name: "required_arg".to_string(),
                description: None,
                required: true,
            },
            PromptArgument {
                name: "optional_arg".to_string(),
                description: None,
                required: false,
            },
        ],
    };

    // Test with all arguments
    let mut args = HashMap::new();
    args.insert("required_arg".to_string(), "value".to_string());
    args.insert("optional_arg".to_string(), "value".to_string());
    assert!(provider.validate_arguments(&prompt, &args).is_ok());

    // Test with only required argument
    let mut args = HashMap::new();
    args.insert("required_arg".to_string(), "value".to_string());
    assert!(provider.validate_arguments(&prompt, &args).is_ok());

    // Test missing required argument
    let args = HashMap::new();
    assert!(matches!(
        provider.validate_arguments(&prompt, &args),
        Err(Error::MissingRequiredArgument(_))
    ));
}

#[test]
async fn test_prompt_error_handling() {
    let provider = MockPromptProvider::new();

    // Test invalid argument
    let mut args = HashMap::new();
    args.insert("invalid_arg".to_string(), "value".to_string());
    let prompt = Prompt {
        name: "test".to_string(),
        description: None,
        arguments: vec![PromptArgument {
            name: "required_arg".to_string(),
            description: None,
            required: true,
        }],
    };
    assert!(provider.validate_arguments(&prompt, &args).is_err());

    // Test nonexistent prompt
    assert!(matches!(
        provider.get_prompt("nonexistent", None).await,
        Err(Error::PromptNotFound(_))
    ));
}