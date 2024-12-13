use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

use crate::mcp::providers::{ResourceProvider, ResourceError};
use crate::mcp::providers::filesystem::FileSystemProvider;

#[tokio::test]
async fn test_list_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let provider = FileSystemProvider::new(temp.path().to_path_buf());
    
    // Create some test files
    fs::write(temp.path().join("test1.txt"), "test1").await?;
    fs::write(temp.path().join("test2.txt"), "test2").await?;
    fs::create_dir(temp.path().join("subdir")).await?;
    
    let resources = provider.list(".").await?;
    
    assert_eq!(resources.len(), 3);
    assert!(resources.iter().any(|r| r.name == "test1.txt"));
    assert!(resources.iter().any(|r| r.name == "test2.txt"));
    assert!(resources.iter().any(|r| r.name == "subdir"));
    
    Ok(())
}

#[tokio::test]
async fn test_read_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let provider = FileSystemProvider::new(temp.path().to_path_buf());
    
    fs::write(temp.path().join("test.txt"), "test content").await?;
    
    let contents = provider.read("test.txt").await?;
    assert_eq!(contents.len(), 1);
    
    match &contents[0] {
        crate::mcp::types::ResourceContents::Text(text) => {
            assert_eq!(text.text, "test content");
            assert!(text.mime_type.as_ref().unwrap().contains("text/plain"));
        }
        _ => panic!("Expected text content"),
    }
    
    Ok(())
}

#[tokio::test]
async fn test_path_traversal() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let provider = FileSystemProvider::new(temp.path().to_path_buf());
    
    let result = provider.read("../outside.txt").await;
    assert!(matches!(result, Err(ResourceError::AccessDenied(_))));
    
    Ok(())
}

#[tokio::test]
async fn test_watch_unwatch() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let provider = FileSystemProvider::new(temp.path().to_path_buf());
    
    // Watch the directory
    provider.watch(".").await?;
    
    // Verify watcher is created
    fs::write(temp.path().join("watched.txt"), "test").await?;
    
    // Unwatch and verify
    provider.unwatch(".").await?;
    
    // Verify unwatch removes watcher
    let result = provider.unwatch(".").await;
    assert!(matches!(result, Err(ResourceError::NotFound(_))));
    
    Ok(())
}