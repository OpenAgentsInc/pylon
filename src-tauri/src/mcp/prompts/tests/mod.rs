use super::*;
use std::collections::HashMap;

mod types;
mod provider;
mod filesystem;

// Common test utilities and mock implementations
pub(crate) struct MockPromptProvider {
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
            },
        );

        prompt_messages.insert(
            "test_prompt".to_string(),
            vec![PromptMessage {
                role: Role::User,
                content: MessageContent::Text(TextContent {
                    text: "Test message".to_string(),
                    r#type: "text".to_string(),
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

impl PromptProvider for MockPromptProvider {
    async fn list_prompts(&self, _cursor: Option<String>) -> Result<(Vec<Prompt>, Option<String>)> {
        Ok((self.prompts.values().cloned().collect(), None))
    }

    async fn get_prompt(&self, name: &str, _arguments: Option<HashMap<String, String>>) -> Result<Vec<PromptMessage>> {
        self.prompt_messages
            .get(name)
            .cloned()
            .ok_or_else(|| Error::PromptNotFound(name.to_string()))
    }

    fn validate_arguments(&self, prompt: &Prompt, arguments: &HashMap<String, String>) -> Result<()> {
        for arg in &prompt.arguments {
            if arg.required && !arguments.contains_key(&arg.name) {
                return Err(Error::MissingRequiredArgument(arg.name.clone()));
            }
        }
        Ok(())
    }
}