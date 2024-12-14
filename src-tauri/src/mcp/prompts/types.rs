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
    #[serde(flatten)]
    pub content: MessageContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "content_type")]
pub enum MessageContent {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
    },
    #[serde(rename = "image")]
    Image {
        data: Vec<u8>,
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
    },
    #[serde(rename = "resource")]
    Resource {
        r#type: String,
        resource: ResourceContents,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
    },
}

impl MessageContent {
    pub fn with_text(text: String) -> Self {
        MessageContent::Text {
            text,
            annotations: None,
        }
    }

    pub fn with_resource(resource: ResourceContents) -> Self {
        MessageContent::Resource {
            r#type: "resource".to_string(),
            resource,
            annotations: None,
        }
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

    #[test]
    fn test_message_content_serialization() {
        let text_content = MessageContent::Text {
            text: "Hello".to_string(),
            annotations: None,
        };

        let json = serde_json::to_string(&text_content).unwrap();
        assert_eq!(
            json,
            r#"{"content_type":"text","text":"Hello"}"#
        );

        let resource_content = MessageContent::Resource {
            r#type: "resource".to_string(),
            resource: ResourceContents::Text(crate::mcp::types::TextResourceContents {
                uri: "test.txt".to_string(),
                mime_type: Some("text/plain".to_string()),
                text: "Hello".to_string(),
            }),
            annotations: None,
        };

        let json = serde_json::to_string(&resource_content).unwrap();
        assert!(json.contains(r#""content_type":"resource""#));
        assert!(json.contains(r#""resource":{"type":"Text""#));
    }

    #[test]
    fn test_yaml_serialization() {
        let prompt = Prompt {
            name: "test".to_string(),
            description: Some("Test prompt".to_string()),
            arguments: vec![PromptArgument {
                name: "arg1".to_string(),
                description: Some("Test arg".to_string()),
                required: true,
            }],
            messages: vec![PromptMessage {
                role: Role::User,
                content: MessageContent::Text {
                    text: "Hello {arg1}!".to_string(),
                    annotations: None,
                },
            }],
        };

        let yaml = serde_yaml::to_string(&prompt).unwrap();
        let parsed: Prompt = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(parsed.name, prompt.name);
        assert_eq!(parsed.description, prompt.description);
        assert_eq!(parsed.arguments.len(), prompt.arguments.len());
        assert_eq!(parsed.messages.len(), prompt.messages.len());
    }
}