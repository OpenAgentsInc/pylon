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
    use crate::mcp::prompts::types::substitute_template;
    use crate::mcp::providers::ResourceProvider;
    use std::sync::Arc;
    
    /// Process a message template with the given arguments
    pub async fn process_message_template(
        message: &PromptMessage,
        arguments: &HashMap<String, String>,
        resource_provider: Option<&Arc<dyn ResourceProvider>>,
    ) -> Result<PromptMessage> {
        match message.content_type.as_str() {
            "text" => {
                if let Some(text) = &message.text {
                    let processed_text = substitute_template(text, arguments)?;
                    Ok(PromptMessage {
                        role: message.role.clone(),
                        content_type: "text".to_string(),
                        text: Some(processed_text),
                        resource: None,
                        annotations: message.annotations.clone(),
                    })
                } else {
                    Err(Error::InvalidTemplate("Text message missing text field".to_string()))
                }
            }
            "resource" => {
                if let Some(resource) = &message.resource {
                    let processed_uri = substitute_template(&resource.uri(), arguments)?;
                    debug!("Processed resource URI: {}", processed_uri);

                    // If we have a resource provider, try to read the resource
                    if let Some(provider) = resource_provider {
                        debug!("Reading resource from provider");
                        match provider.read(&processed_uri).await {
                            Ok(contents) => {
                                debug!("Got resource contents: {:?}", contents);
                                if let Some(content) = contents.first() {
                                    Ok(PromptMessage {
                                        role: message.role.clone(),
                                        content_type: "resource".to_string(),
                                        text: None,
                                        resource: Some(content.clone()),
                                        annotations: message.annotations.clone(),
                                    })
                                } else {
                                    debug!("No resource contents returned");
                                    let mut new_resource = resource.clone();
                                    new_resource.set_uri(processed_uri);
                                    Ok(PromptMessage {
                                        role: message.role.clone(),
                                        content_type: "resource".to_string(),
                                        text: None,
                                        resource: Some(new_resource),
                                        annotations: message.annotations.clone(),
                                    })
                                }
                            }
                            Err(e) => {
                                debug!("Error reading resource: {:?}", e);
                                let mut new_resource = resource.clone();
                                new_resource.set_uri(processed_uri);
                                Ok(PromptMessage {
                                    role: message.role.clone(),
                                    content_type: "resource".to_string(),
                                    text: None,
                                    resource: Some(new_resource),
                                    annotations: message.annotations.clone(),
                                })
                            }
                        }
                    } else {
                        debug!("No resource provider available");
                        let mut new_resource = resource.clone();
                        new_resource.set_uri(processed_uri);
                        Ok(PromptMessage {
                            role: message.role.clone(),
                            content_type: "resource".to_string(),
                            text: None,
                            resource: Some(new_resource),
                            annotations: message.annotations.clone(),
                        })
                    }
                } else {
                    Err(Error::InvalidTemplate("Resource message missing resource field".to_string()))
                }
            }
            _ => Err(Error::InvalidTemplate(format!("Unknown content type: {}", message.content_type))),
        }
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
    use crate::mcp::types::{Role, ResourceContents, TextResourceContents};
    
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
                    content_type: "text".to_string(),
                    text: Some("Hello {name}!".to_string()),
                    resource: None,
                    annotations: None,
                },
            ],
        };
        
        let mut args = HashMap::new();
        args.insert("name".to_string(), "world".to_string());
        
        let processed = utils::process_prompt_messages(&prompt, Some(&args), None).await.unwrap();
        assert_eq!(processed.len(), 1);
        
        assert_eq!(processed[0].content_type, "text");
        assert_eq!(processed[0].text.as_ref().unwrap(), "Hello world!");
    }

    #[tokio::test]
    async fn test_process_resource_message() {
        let message = PromptMessage {
            role: Role::User,
            content_type: "resource".to_string(),
            text: None,
            resource: Some(ResourceContents::Text(TextResourceContents {
                uri: "test/{name}.txt".to_string(),
                mime_type: Some("text/plain".to_string()),
                text: "Hello".to_string(),
            })),
            annotations: None,
        };

        let mut args = HashMap::new();
        args.insert("name".to_string(), "world".to_string());

        let processed = utils::process_message_template(&message, &args, None).await.unwrap();
        assert_eq!(processed.content_type, "resource");
        assert!(processed.resource.is_some());
        assert_eq!(processed.resource.unwrap().uri(), "test/world.txt");
    }
}