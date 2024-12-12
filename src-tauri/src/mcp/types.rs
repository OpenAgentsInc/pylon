use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceContents {
    Text(TextResourceContents),
    Blob(BlobResourceContents),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextResourceContents {
    pub uri: String,
    pub mime_type: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobResourceContents {
    pub uri: String,
    pub mime_type: Option<String>,
    pub blob: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub uri: String,
    pub mime_type: Option<String>,
    pub description: Option<String>,
    pub annotations: Option<ResourceAnnotations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAnnotations {
    pub audience: Option<Vec<Role>>,
    pub priority: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    User,
    Assistant,
}