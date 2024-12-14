use std::path::{Path, PathBuf};
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::fs;

use crate::mcp::prompts::{
    provider::{PromptProvider, utils},
    types::{Error, Prompt, PromptMessage, Result},
};

pub struct FileSystemPromptProvider {
    root_path: PathBuf,
}

impl FileSystemPromptProvider {
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        Self {
            root_path: root_path.as_ref().to_path_buf(),
        }
    }
    
    async fn load_prompt(&self, name: &str) -> Result<Prompt> {
        let path = self.root_path.join(format!("{}.yaml", name));
        let content = fs::read_to_string(&path).await?;
        let prompt: Prompt = serde_yaml::from_str(&content)?;
        
        // Validate the prompt has the correct name
        if prompt.name != name {
            return Err(Error::InvalidTemplate(format!(
                "Prompt name mismatch: expected {}, found {}",
                name, prompt.name
            )));
        }
        
        Ok(prompt)
    }
    
    async fn list_prompt_files(&self) -> Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        let mut read_dir = fs::read_dir(&self.root_path).await?;
        
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                entries.push(path);
            }
        }
        
        Ok(entries)
    }
    
    async fn load_resource(&self, uri: &str) -> Result<String> {
        // For now, only support file:// URIs
        if !uri.starts_with("file://") {
            return Err(Error::ResourceError(format!(
                "Unsupported URI scheme: {}",
                uri
            )));
        }
        
        let path = uri.trim_start_matches("file://");
        fs::read_to_string(path)
            .await
            .map_err(|e| Error::ResourceError(format!("Failed to read resource: {}", e)))
    }
}

#[async_trait]
impl PromptProvider for FileSystemPromptProvider {
    async fn list_prompts(&self, _cursor: Option<String>) -> Result<(Vec<Prompt>, Option<String>)> {
        let mut prompts = Vec::new();
        
        for path in self.list_prompt_files().await? {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(prompt) = self.load_prompt(stem).await {
                    prompts.push(prompt);
                }
            }
        }
        
        // Sort prompts by name for consistent ordering
        prompts.sort_by(|a, b| a.name.cmp(&b.name));
        
        Ok((prompts, None)) // No pagination for now
    }
    
    async fn get_prompt(&self, name: &str, arguments: Option<HashMap<String, String>>) -> Result<Vec<PromptMessage>> {
        let prompt = self.load_prompt(name).await?;
        utils::process_prompt_messages(&prompt, arguments.as_ref()).await
    }
    
    fn validate_arguments(&self, prompt: &Prompt, arguments: &HashMap<String, String>) -> Result<()> {
        utils::validate_required_arguments(prompt, arguments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::mcp::types::{Role, TextContent};
    use crate::mcp::prompts::MessageContent;
    
    async fn setup_test_prompt(dir: &Path, name: &str, content: &str) -> Result<()> {
        fs::write(
            dir.join(format!("{}.yaml", name)),
            content,
        ).await?;
        Ok(())
    }
    
    #[tokio::test]
    async fn test_load_prompt() {
        let temp_dir = TempDir::new().unwrap();
        let provider = FileSystemPromptProvider::new(temp_dir.path());
        
        setup_test_prompt(
            temp_dir.path(),
            "test",
            r#"
name: test
description: Test prompt
arguments:
  - name: arg1
    required: true
messages:
  - role: user
    content:
      type: text
      text: "Hello {arg1}!"
"#,
        )
        .await
        .unwrap();
        
        let prompt = provider.load_prompt("test").await.unwrap();
        assert_eq!(prompt.name, "test");
        assert_eq!(prompt.description, Some("Test prompt".to_string()));
        assert_eq!(prompt.arguments.len(), 1);
        assert_eq!(prompt.messages.len(), 1);
    }
    
    #[tokio::test]
    async fn test_list_prompts() {
        let temp_dir = TempDir::new().unwrap();
        let provider = FileSystemPromptProvider::new(temp_dir.path());
        
        // Create two test prompts
        setup_test_prompt(
            temp_dir.path(),
            "test1",
            "name: test1\ndescription: First test prompt",
        )
        .await
        .unwrap();
        
        setup_test_prompt(
            temp_dir.path(),
            "test2",
            "name: test2\ndescription: Second test prompt",
        )
        .await
        .unwrap();
        
        let (prompts, cursor) = provider.list_prompts(None).await.unwrap();
        assert_eq!(prompts.len(), 2);
        assert!(cursor.is_none());
        
        assert_eq!(prompts[0].name, "test1");
        assert_eq!(prompts[1].name, "test2");
    }
    
    #[tokio::test]
    async fn test_get_prompt_with_arguments() {
        let temp_dir = TempDir::new().unwrap();
        let provider = FileSystemPromptProvider::new(temp_dir.path());
        
        setup_test_prompt(
            temp_dir.path(),
            "greeting",
            r#"
name: greeting
arguments:
  - name: name
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
        
        let mut args = HashMap::new();
        args.insert("name".to_string(), "world".to_string());
        
        let messages = provider.get_prompt("greeting", Some(args)).await.unwrap();
        assert_eq!(messages.len(), 1);
        
        match &messages[0].content {
            MessageContent::Text(text) => {
                assert_eq!(text.text, "Hello world!");
            }
            _ => panic!("Expected TextContent"),
        }
    }
    
    #[tokio::test]
    async fn test_invalid_prompt() {
        let temp_dir = TempDir::new().unwrap();
        let provider = FileSystemPromptProvider::new(temp_dir.path());
        
        // Create an invalid YAML file
        setup_test_prompt(
            temp_dir.path(),
            "invalid",
            "invalid: - yaml: content",
        )
        .await
        .unwrap();
        
        assert!(provider.get_prompt("invalid", None).await.is_err());
    }
}