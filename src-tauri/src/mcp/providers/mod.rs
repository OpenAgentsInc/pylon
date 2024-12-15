pub mod filesystem;
pub mod ollama;
pub mod roots;

use async_trait::async_trait;
use thiserror::Error;

use crate::mcp::types::{Resource, ResourceContents};

#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Access denied: {0}")]
    AccessDenied(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Watch error: {0}")]
    WatchError(#[from] notify::Error),
}

#[async_trait]
pub trait ResourceProvider {
    fn name(&self) -> &'static str;
    
    async fn list(&self, path: &str) -> Result<Vec<Resource>, ResourceError>;
    async fn read(&self, path: &str) -> Result<Vec<ResourceContents>, ResourceError>;
    async fn watch(&self, path: &str) -> Result<(), ResourceError>;
    async fn unwatch(&self, path: &str) -> Result<(), ResourceError>;
}