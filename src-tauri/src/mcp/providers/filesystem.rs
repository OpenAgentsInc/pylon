use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use async_trait::async_trait;
use url::Url;
use mime_guess::from_path;
use log::debug;

use super::{ResourceProvider, ResourceError};
use crate::mcp::types::{Resource, ResourceContents, TextResourceContents};

pub struct FileSystemProvider {
    root_path: PathBuf,
    watchers: Arc<RwLock<HashMap<String, RecommendedWatcher>>>,
}

impl FileSystemProvider {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            watchers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn validate_path(&self, path: &str) -> Result<PathBuf, ResourceError> {
        // If path starts with file://, strip it off
        let path_str = if path.starts_with("file://") {
            &path[7..]
        } else {
            path
        };

        let path = if path_str == "." {
            self.root_path.clone()
        } else {
            PathBuf::from(path_str)
        };
        
        debug!("Validating path: {:?}", path);
        
        // Canonicalize to resolve any .. or symlinks
        let canonical = path.canonicalize()
            .map_err(|e| ResourceError::InvalidPath(e.to_string()))?;
            
        debug!("Canonical path: {:?}", canonical);
        
        Ok(canonical)
    }
    
    fn path_to_uri(&self, path: &Path) -> Result<String, ResourceError> {
        let url = Url::from_file_path(path)
            .map_err(|_| ResourceError::InvalidPath("Invalid file path".into()))?;
        Ok(url.to_string())
    }
    
    async fn read_file_contents(&self, path: &Path) -> Result<ResourceContents, ResourceError> {
        debug!("Reading file contents from: {:?}", path);
        
        let metadata = path.metadata()
            .map_err(|e| ResourceError::IoError(e))?;
            
        if metadata.is_dir() {
            return Err(ResourceError::InvalidPath("Path is a directory".into()));
        }
        
        let uri = self.path_to_uri(path)?;
        let mime_type = from_path(path)
            .first_or_octet_stream()
            .to_string();
            
        // For now we'll treat everything as text
        // TODO: Handle binary files properly
        let contents = tokio::fs::read_to_string(path).await
            .map_err(|e| ResourceError::IoError(e))?;
            
        debug!("Read contents: {}", contents);
        
        Ok(ResourceContents::Text(TextResourceContents {
            uri,
            mime_type: Some(mime_type),
            text: contents,
        }))
    }
}

#[async_trait]
impl ResourceProvider for FileSystemProvider {
    fn name(&self) -> &'static str {
        "filesystem"
    }

    async fn list(&self, path: &str) -> Result<Vec<Resource>, ResourceError> {
        let path = self.validate_path(path)?;
        
        let mut entries = Vec::new();
        let mut dir = tokio::fs::read_dir(&path).await
            .map_err(|e| ResourceError::IoError(e))?;
            
        while let Some(entry) = dir.next_entry().await
            .map_err(|e| ResourceError::IoError(e))? 
        {
            let metadata = entry.metadata().await
                .map_err(|e| ResourceError::IoError(e))?;
                
            let name = entry.file_name().to_string_lossy().into_owned();
            let uri = self.path_to_uri(&entry.path())?;
            
            let mime_type = if metadata.is_file() {
                Some(from_path(&entry.path())
                    .first_or_octet_stream()
                    .to_string())
            } else {
                None
            };
            
            entries.push(Resource {
                name,
                uri,
                mime_type,
                description: None,
                annotations: None,
            });
        }
        
        Ok(entries)
    }
    
    async fn read(&self, path: &str) -> Result<Vec<ResourceContents>, ResourceError> {
        let path = self.validate_path(path)?;
        let contents = self.read_file_contents(&path).await?;
        Ok(vec![contents])
    }
    
    async fn watch(&self, path: &str) -> Result<(), ResourceError> {
        let path = self.validate_path(path)?;
        
        // Don't create duplicate watchers
        if self.watchers.read().await.contains_key(path.to_str().unwrap()) {
            return Ok(());
        }
        
        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(event) => {
                    // Handle file system events
                    debug!("Event: {:?}", event);
                },
                Err(e) => debug!("Watch error: {:?}", e),
            }
        }).map_err(ResourceError::WatchError)?;
        
        watcher.watch(&path, RecursiveMode::Recursive)
            .map_err(ResourceError::WatchError)?;
            
        self.watchers.write().await.insert(
            path.to_str().unwrap().to_string(),
            watcher
        );
        
        Ok(())
    }
    
    async fn unwatch(&self, path: &str) -> Result<(), ResourceError> {
        let path = self.validate_path(path)?;
        
        if let Some(_) = self.watchers.write().await.remove(path.to_str().unwrap()) {
            Ok(())
        } else {
            Err(ResourceError::NotFound("No watcher found for path".into()))
        }
    }
}