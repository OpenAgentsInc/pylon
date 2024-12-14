use std::collections::HashMap;
use async_trait::async_trait;

use super::types::{Prompt, PromptMessage, Result};

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
    use crate::mcp::prompts::types::{Error, MessageContent, substitute_template};
    
    /// Process a message template with the given arguments
    pub fn process_message_template(
        message: &PromptMessage,
        arguments: &HashMap<String, String>,
    ) -> Result<PromptMessage> {
        let content = match &message.content {
            MessageContent::Text(text) => {
                let processed_text = substitute_template(&text.text, arguments)?;
                MessageContent::Text(text.clone().with_text(processed_text))
            }
            MessageContent::Resource(resource) => {
                let processed_uri = substitute_template(&resource.resource.uri(), arguments)?;
                MessageContent::Resource(resource.clone().with_uri(processed_uri))
            }
            MessageContent::Image(image) => MessageContent::Image(image.clone()),
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
    ) -> Result<Vec<PromptMessage>> {
        let arguments = arguments.unwrap_or(&HashMap::new());
        
        // First validate required arguments
        validate_required_arguments(prompt, arguments)?;
        
        // Then process each message
        let mut processed_messages = Vec::new();
        for message in &prompt.messages {
            let processed = process_message_template(message, arguments)?;
            processed_messages.push(processed);
        }
        
        Ok(processed_messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::protocol::types::{Role, TextContent};
    
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
                    content: MessageContent::Text(TextContent {
                        text: "Hello {name}!".to_string(),
                        r#type: "text".to_string(),
                        annotations: None,
                    }),
                },
            ],
        };
        
        let mut args = HashMap::new();
        args.insert("name".to_string(), "world".to_string());
        
        let processed = utils::process_prompt_messages(&prompt, Some(&args)).await.unwrap();
        assert_eq!(processed.len(), 1);
        
        match &processed[0].content {
            MessageContent::Text(text) => {
                assert_eq!(text.text, "Hello world!");
            }
            _ => panic!("Expected TextContent"),
        }
    }
}