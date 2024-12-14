use super::*;
use std::fs;
use tempfile::TempDir;
use tokio::test;

struct TestContext {
    _temp_dir: TempDir,
    provider: FileSystemPromptProvider,
}

impl TestContext {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let provider = FileSystemPromptProvider::new(temp_dir.path().to_path_buf());
        Self {
            _temp_dir: temp_dir,
            provider,
        }
    }

    fn create_prompt_file(&self, name: &str, content: &str) {
        let path = self._temp_dir.path().join(format!("{}.yaml", name));
        fs::write(path, content).unwrap();
    }
}

#[test]
async fn test_load_prompt_from_file() {
    let ctx = TestContext::new();
    ctx.create_prompt_file(
        "test_prompt",
        r#"
name: test_prompt
description: A test prompt
arguments:
  - name: test_arg
    description: A test argument
    required: true
messages:
  - role: user
    content:
      type: text
      text: Test message with {test_arg}
"#,
    );

    let mut args = HashMap::new();
    args.insert("test_arg".to_string(), "test_value".to_string());
    
    let messages = ctx.provider.get_prompt("test_prompt", Some(args)).await.unwrap();
    assert_eq!(messages.len(), 1);
    
    match &messages[0].content {
        MessageContent::Text(text) => {
            assert_eq!(text.text, "Test message with test_value");
        }
        _ => panic!("Expected TextContent"),
    }
}

#[test]
async fn test_list_prompts_from_directory() {
    let ctx = TestContext::new();
    
    // Create multiple prompt files
    ctx.create_prompt_file(
        "prompt1",
        r#"
name: prompt1
description: First test prompt
"#,
    );
    
    ctx.create_prompt_file(
        "prompt2",
        r#"
name: prompt2
description: Second test prompt
"#,
    );

    let (prompts, cursor) = ctx.provider.list_prompts(None).await.unwrap();
    assert_eq!(prompts.len(), 2);
    assert!(cursor.is_none());

    let names: Vec<_> = prompts.iter().map(|p| &p.name).collect();
    assert!(names.contains(&&"prompt1".to_string()));
    assert!(names.contains(&&"prompt2".to_string()));
}

#[test]
async fn test_prompt_with_resource() {
    let ctx = TestContext::new();
    ctx.create_prompt_file(
        "resource_prompt",
        r#"
name: resource_prompt
arguments:
  - name: file_path
    required: true
messages:
  - role: user
    content:
      type: resource
      resource:
        uri: "{file_path}"
"#,
    );

    // Create a test file
    let test_file = ctx._temp_dir.path().join("test.txt");
    fs::write(&test_file, "Test content").unwrap();

    let mut args = HashMap::new();
    args.insert("file_path".to_string(), test_file.to_str().unwrap().to_string());

    let messages = ctx.provider.get_prompt("resource_prompt", Some(args)).await.unwrap();
    assert_eq!(messages.len(), 1);

    match &messages[0].content {
        MessageContent::Resource(resource) => {
            match &resource.resource {
                ResourceContents::Text(text) => {
                    assert_eq!(text.text, "Test content");
                }
                _ => panic!("Expected TextResourceContents"),
            }
        }
        _ => panic!("Expected ResourceContent"),
    }
}

#[test]
async fn test_invalid_prompt_file() {
    let ctx = TestContext::new();
    ctx.create_prompt_file(
        "invalid",
        r#"
invalid: yaml: content
"#,
    );

    let result = ctx.provider.get_prompt("invalid", None).await;
    assert!(result.is_err());
}

#[test]
async fn test_missing_required_fields() {
    let ctx = TestContext::new();
    ctx.create_prompt_file(
        "missing_name",
        r#"
description: A prompt without a name
"#,
    );

    let result = ctx.provider.get_prompt("missing_name", None).await;
    assert!(result.is_err());
}

#[test]
async fn test_template_substitution() {
    let ctx = TestContext::new();
    ctx.create_prompt_file(
        "template_prompt",
        r#"
name: template_prompt
arguments:
  - name: var1
    required: true
  - name: var2
    required: true
messages:
  - role: user
    content:
      type: text
      text: "First value: {var1}, Second value: {var2}"
"#,
    );

    let mut args = HashMap::new();
    args.insert("var1".to_string(), "hello".to_string());
    args.insert("var2".to_string(), "world".to_string());

    let messages = ctx.provider.get_prompt("template_prompt", Some(args)).await.unwrap();
    match &messages[0].content {
        MessageContent::Text(text) => {
            assert_eq!(text.text, "First value: hello, Second value: world");
        }
        _ => panic!("Expected TextContent"),
    }
}

#[test]
async fn test_prompt_with_multiple_messages() {
    let ctx = TestContext::new();
    ctx.create_prompt_file(
        "multi_message",
        r#"
name: multi_message
messages:
  - role: system
    content:
      type: text
      text: "System message"
  - role: user
    content:
      type: text
      text: "User message"
  - role: assistant
    content:
      type: text
      text: "Assistant message"
"#,
    );

    let messages = ctx.provider.get_prompt("multi_message", None).await.unwrap();
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[0].role, Role::System);
    assert_eq!(messages[1].role, Role::User);
    assert_eq!(messages[2].role, Role::Assistant);
}

#[test]
async fn test_prompt_with_annotations() {
    let ctx = TestContext::new();
    ctx.create_prompt_file(
        "annotated_prompt",
        r#"
name: annotated_prompt
messages:
  - role: user
    content:
      type: text
      text: "Test message"
      annotations:
        audience: ["user", "assistant"]
        priority: 0.8
"#,
    );

    let messages = ctx.provider.get_prompt("annotated_prompt", None).await.unwrap();
    match &messages[0].content {
        MessageContent::Text(text) => {
            let annotations = text.annotations.as_ref().unwrap();
            assert_eq!(annotations.audience.as_ref().unwrap().len(), 2);
            assert_eq!(annotations.priority.unwrap(), 0.8);
        }
        _ => panic!("Expected TextContent"),
    }
}