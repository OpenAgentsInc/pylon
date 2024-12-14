use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mcp::types::{Role, TextContent, ImageContent, ResourceContents, Annotations};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub messages: Vec<PromptMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMessage {
    pub role: Role,
    pub content: MessageContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "text")]
    Text(TextContent),
    #[serde(rename = "image")]
    Image(ImageContent),
    #[serde(rename = "resource")]
    Resource(EmbeddedResource),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedResource {
    pub r#type: String,
    pub resource: ResourceContents,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

impl EmbeddedResource {
    pub fn with_uri(mut self, uri: String) -> Self {
        match &mut self.resource {
            ResourceContents::Text(text) => text.uri = uri,
            ResourceContents::Blob(blob) => blob.uri = uri,
        }
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Prompt not found: {0}")]
    PromptNotFound(String),
    
    #[error("Missing required argument: {0}")]
    MissingRequiredArgument(String),
    
    #[error("Invalid template: {0}")]
    InvalidTemplate(String),
    
    #[error("Resource error: {0}")]
    ResourceError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

// Template processing utilities
pub(crate) fn substitute_template(template: &str, arguments: &HashMap<String, String>) -> Result<String> {
    let mut result = template.to_string();
    for (key, value) in arguments {
        let placeholder = format!("{{{}}}", key);
        if template.contains(&placeholder) {
            result = result.replace(&placeholder, value);
        }
    }
    
    // Check if any unsubstituted placeholders remain
    if result.contains('{') && result.contains('}') {
        return Err(Error::InvalidTemplate(
            "Template contains unsubstituted placeholders".to_string(),
        ));
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_template_substitution() {
        let mut args = HashMap::new();
        args.insert("name".to_string(), "world".to_string());
        
        assert_eq!(
            substitute_template("Hello {name}!", &args).unwrap(),
            "Hello world!"
        );
        
        // Test missing argument
        assert!(substitute_template("Hello {missing}!", &args).is_err());
        
        // Test multiple substitutions
        args.insert("greeting".to_string(), "Hi".to_string());
        assert_eq!(
            substitute_template("{greeting} {name}!", &args).unwrap(),
            "Hi world!"
        );
    }
}