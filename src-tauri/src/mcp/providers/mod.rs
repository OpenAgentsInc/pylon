pub mod filesystem;
pub mod ollama;

pub use filesystem::FileSystemProvider;
pub use ollama::{OllamaProvider, ChatMessage, ChatResponse, ModelInfo};
pub trait ResourceProvider {
    fn name(&self) -> &'static str;
}