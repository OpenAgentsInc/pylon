use std::path::{Path, PathBuf};
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::fs;
use std::sync::Arc;
use log::debug;

use crate::mcp::prompts::{
    provider::{PromptProvider, utils},
    types::{Error, Prompt, PromptMessage, Result},
};
use crate::mcp::providers::FileSystemProvider;

pub struct FileSystemPromptProvider {
    root_path: PathBuf,
    resource_provider: Arc<FileSystemProvider>,
}

impl FileSystemPromptProvider {
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        let root_path = root_path.as_ref().to_path_buf();
        Self {
            resource_provider: Arc::new(FileSystemProvider::new(root_path.clone())),
            root_path,
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
        debug!("Loaded prompt: {:?}", prompt);
        utils::process_prompt_messages(&prompt, arguments.as_ref(), Some(&self.resource_provider)).await
    }
    
    fn validate_arguments(&self, prompt: &Prompt, arguments: &HashMap<String, String>) -> Result<()> {
        utils::validate_required_arguments(prompt, arguments)
    }
}