use std::collections::HashMap;
use async_trait::async_trait;
use log::debug;

use super::types::{Prompt, PromptMessage, Result, Error};

/// Provider trait for prompt management
#[async_trait]
pub trait PromptProvider {
    /// List available prompts with optional pagination
    async fn list_prompts(&self, cursor: Option<String>) -> Result<(Vec<Prompt>, Option<String>)>;
    
    /// Get a specific prompt with optional argument values
    async fn get_prompt(&self, name: &str, arguments: Option<HashMap<String, String>>) -> Result<Vec<PromptMessage>>;
    
    /// Validate prompt arguments
    fn validate_arguments(&self, prompt: &Prompt, arguments: &HashMap<String, String>) -> Result<()>;
}

/// Default implementations for common provider functionality
pub(crate) mod utils {
    use super::*;
    use crate::mcp::prompts::types::{MessageContent, substitute_template};
    use crate::mcp::providers::ResourceProvider;
    use std::sync::Arc;
    
    /// Process a message template with the given arguments
    pub async fn process_message_template(
        message: &PromptMessage,
        arguments: &HashMap<String, String>,
        resource_provider: Option<&Arc<dyn ResourceProvider>>,
    ) -> Result<PromptMessage> {
        let content = match &message.content {
            MessageContent::Text { text, annotations } => {
                let processed_text = substitute_template(text, arguments)?;
                MessageContent::Text {
                    text: processed_text,
                    annotations: annotations.clone(),
                }
            }
            MessageContent::Resource { r#type, resource, annotations } => {
                let processed_uri = substitute_template(&resource.uri(), arguments)?;
                debug!("Processed resource URI: {}", processed_uri);

                // If we have a resource provider, try to read the resource
                if let Some(provider) = resource_provider {
                    debug!("Reading resource from provider");
                    match provider.read(&processed_uri).await {
                        Ok(contents) => {
                            debug!("Got resource contents: {:?}", contents);
                            if let Some(content) = contents.first() {
                                MessageContent::Resource {
                                    r#type: r#type.clone(),
                                    resource: content.clone(),
                                    annotations: annotations.clone(),
                                }
                            } else {
                                debug!("No resource contents returned");
                                MessageContent::Resource {
                                    r#type: r#type.clone(),
                                    resource: resource.clone(),
                                    annotations: annotations.clone(),
                                }
                            }
                        }
                        Err(e) => {
                            debug!("Error reading resource: {:?}", e);
                            MessageContent::Resource {
                                r#type: r#type.clone(),
                                resource: resource.clone(),
                                annotations: annotations.clone(),
                            }
                        }
                    }
                } else {
                    debug!("No resource provider available");
                    let mut new_resource = resource.clone();
                    new_resource.set_uri(processed_uri);
                    MessageContent::Resource {
                        r#type: r#type.clone(),
                        resource: new_resource,
                        annotations: annotations.clone(),
                    }
                }
            }
            MessageContent::Image { data, mime_type, annotations } => MessageContent::Image {
                data: data.clone(),
                mime_type: mime_type.clone(),
                annotations: annotations.clone(),
            },
        };
        
        Ok(PromptMessage {
            role: message.role.clone(),
            content,
        })
    }
    
    /// Validate required arguments are present
    pub fn validate_required_arguments(prompt: &Prompt, arguments: &HashMap<String, String>) -> Result<()> {
        for arg in prompt.arguments.iter().filter(|a| a.required) {
            if !arguments.contains_key(&arg.name) {
                return Err(Error::MissingRequiredArgument(arg.name.clone()));
            }
        }
        Ok(())
    }
    
    /// Process all messages in a prompt with the given arguments
    pub async fn process_prompt_messages(
        prompt: &Prompt,
        arguments: Option<&HashMap<String, String>>,
        resource_provider: Option<&Arc<dyn ResourceProvider>>,
    ) -> Result<Vec<PromptMessage>> {
        let arguments = arguments.cloned().unwrap_or_default();
        
        // First validate required arguments
        validate_required_arguments(prompt, &arguments)?;
        
        // Then process each message
        let mut processed_messages = Vec::new();
        for message in &prompt.messages {
            let processed = process_message_template(message, &arguments, resource_provider).await?;
            processed_messages.push(processed);
        }
        
        Ok(processed_messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::types::Role;
    use crate::mcp::prompts::MessageContent;
    
    #[test]
    fn test_validate_required_arguments() {
        let prompt = Prompt {
            name: "test".to_string(),
            description: None,
            arguments: vec![
                super::super::types::PromptArgument {
                    name: "required".to_string(),
                    description: None,
                    required: true,
                },
                super::super::types::PromptArgument {
                    name: "optional".to_string(),
                    description: None,
                    required: false,
                },
            ],
            messages: vec![],
        };
        
        // Test with all arguments
        let mut args = HashMap::new();
        args.insert("required".to_string(), "value".to_string());
        args.insert("optional".to_string(), "value".to_string());
        assert!(utils::validate_required_arguments(&prompt, &args).is_ok());
        
        // Test with only required argument
        let mut args = HashMap::new();
        args.insert("required".to_string(), "value".to_string());
        assert!(utils::validate_required_arguments(&prompt, &args).is_ok());
        
        // Test missing required argument
        let args = HashMap::new();
        assert!(utils::validate_required_arguments(&prompt, &args).is_err());
    }
    
    #[tokio::test]
    async fn test_process_prompt_messages() {
        let prompt = Prompt {
            name: "test".to_string(),
            description: None,
            arguments: vec![
                super::super::types::PromptArgument {
                    name: "name".to_string(),
                    description: None,
                    required: true,
                },
            ],
            messages: vec![
                PromptMessage {
                    role: Role::User,
                    content: MessageContent::Text {
                        text: "Hello {name}!".to_string(),
                        annotations: None,
                    },
                },
            ],
        };
        
        let mut args = HashMap::new();
        args.insert("name".to_string(), "world".to_string());
        
        let processed = utils::process_prompt_messages(&prompt, Some(&args), None).await.unwrap();
        assert_eq!(processed.len(), 1);
        
        match &processed[0].content {
            MessageContent::Text { text, .. } => {
                assert_eq!(text, "Hello world!");
            }
            _ => panic!("Expected TextContent"),
        }
    }
}