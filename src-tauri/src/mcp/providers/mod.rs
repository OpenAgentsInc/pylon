use std::path::PathBuf;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::mcp::types::{Resource, ResourceContents};

pub mod filesystem;

#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Watch error: {0}")]
    WatchError(#[from] notify::Error),
}

/// A provider that can access resources
#[async_trait]
pub trait ResourceProvider: Send + Sync {
    /// List contents at the given path
    async fn list(&self, path: &str) -> Result<Vec<Resource>, ResourceError>;
    
    /// Read contents at the given path
    async fn read(&self, path: &str) -> Result<Vec<ResourceContents>, ResourceError>;
    
    /// Watch for changes at the given path
    async fn watch(&self, path: &str) -> Result<(), ResourceError>;
    
    /// Stop watching the given path
    async fn unwatch(&self, path: &str) -> Result<(), ResourceError>;
}

/// A resource path, which can be either a file or directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePath {
    pub path: PathBuf,
    pub is_dir: bool,
}

impl ResourcePath {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        Self { path, is_dir }
    }
    
    pub fn from_path(path: PathBuf) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(&path)?;
        Ok(Self::new(path, metadata.is_dir()))
    }
}