use crate::mcp::types::{Role, TextContent, ResourceContents};
use std::collections::HashMap;
use tempfile::TempDir;
use async_trait::async_trait;

// Import the prompts module
use crate::mcp::prompts::{
    Prompt, PromptArgument, PromptMessage, MessageContent, Error,
    PromptProvider, FileSystemPromptProvider,
};

mod mock {
    use super::*;

    pub struct MockPromptProvider {
        prompts: HashMap<String, Prompt>,
        prompt_messages: HashMap<String, Vec<PromptMessage>>,
    }

    impl MockPromptProvider {
        pub fn new() -> Self {
            let mut prompts = HashMap::new();
            let mut prompt_messages = HashMap::new();

            // Add some test prompts
            prompts.insert(
                "test_prompt".to_string(),
                Prompt {
                    name: "test_prompt".to_string(),
                    description: Some("A test prompt".to_string()),
                    arguments: vec![PromptArgument {
                        name: "test_arg".to_string(),
                        description: Some("A test argument".to_string()),
                        required: true,
                    }],
                    messages: vec![],
                },
            );

            prompt_messages.insert(
                "test_prompt".to_string(),
                vec![PromptMessage {
                    role: Role::User,
                    content: MessageContent::Text(TextContent {
                        text: "Test message".to_string(),
                        content_type: "text".to_string(),
                        annotations: None,
                    }),
                }],
            );

            Self {
                prompts,
                prompt_messages,
            }
        }
    }

    #[async_trait]
    impl PromptProvider for MockPromptProvider {
        async fn list_prompts(&self, _cursor: Option<String>) -> Result<(Vec<Prompt>, Option<String>), Error> {
            Ok((self.prompts.values().cloned().collect(), None))
        }

        async fn get_prompt(&self, name: &str, _arguments: Option<HashMap<String, String>>) -> Result<Vec<PromptMessage>, Error> {
            self.prompt_messages
                .get(name)
                .cloned()
                .ok_or_else(|| Error::PromptNotFound(name.to_string()))
        }

        fn validate_arguments(&self, prompt: &Prompt, arguments: &HashMap<String, String>) -> Result<(), Error> {
            for arg in &prompt.arguments {
                if arg.required && !arguments.contains_key(&arg.name) {
                    return Err(Error::MissingRequiredArgument(arg.name.clone()));
                }
            }
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_list_prompts() {
    let provider = mock::MockPromptProvider::new();
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

#[tokio::test]
async fn test_get_prompt() {
    let provider = mock::MockPromptProvider::new();
    let messages = provider.get_prompt("test_prompt", None).await.unwrap();

    assert_eq!(messages.len(), 1);
    let message = &messages[0];
    assert_eq!(message.role, Role::User);

    match &message.content {
        MessageContent::Text(text) => {
            assert_eq!(text.text, "Test message");
            assert_eq!(text.content_type, "text");
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn test_filesystem_provider() {
    let temp_dir = TempDir::new().unwrap();
    let provider = FileSystemPromptProvider::new(temp_dir.path());

    // Create a test prompt file
    tokio::fs::write(
        temp_dir.path().join("test.yaml"),
        r#"
name: test
description: Test prompt
arguments:
  - name: name
    description: Test argument
    required: true
messages:
  - role: user
    content:
      type: text
      text: "Hello {name}!"
"#,
    )
    .await
    .unwrap();

    // Test listing prompts
    let (prompts, _) = provider.list_prompts(None).await.unwrap();
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0].name, "test");

    // Test getting prompt with arguments
    let mut args = HashMap::new();
    args.insert("name".to_string(), "world".to_string());
    let messages = provider.get_prompt("test", Some(args)).await.unwrap();

    assert_eq!(messages.len(), 1);
    match &messages[0].content {
        MessageContent::Text(text) => {
            assert_eq!(text.text, "Hello world!");
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn test_prompt_with_resource() {
    let temp_dir = TempDir::new().unwrap();
    let provider = FileSystemPromptProvider::new(temp_dir.path());

    // Create a test resource file
    tokio::fs::write(
        temp_dir.path().join("resource.txt"),
        "Test resource content",
    )
    .await
    .unwrap();

    // Create a prompt that references the resource
    tokio::fs::write(
        temp_dir.path().join("test.yaml"),
        format!(
            r#"
name: test
arguments:
  - name: resource_path
    required: true
messages:
  - role: user
    content:
      type: resource
      resource:
        uri: "{{resource_path}}"
"#
        ),
    )
    .await
    .unwrap();

    let mut args = HashMap::new();
    args.insert(
        "resource_path".to_string(),
        format!("file://{}", temp_dir.path().join("resource.txt").display()),
    );

    let messages = provider.get_prompt("test", Some(args)).await.unwrap();
    assert_eq!(messages.len(), 1);

    match &messages[0].content {
        MessageContent::Resource(resource) => match &resource.resource {
            ResourceContents::Text(text) => {
                assert_eq!(text.text, "Test resource content");
            }
            _ => panic!("Expected TextResourceContents"),
        },
        _ => panic!("Expected ResourceContent"),
    }
}