pub mod types;
pub mod providers;
pub mod protocol;
pub mod server;
pub mod clients;

pub use protocol::MCPProtocol;
pub use server::MCPServer;
pub use types::{
    Implementation, ServerCapabilities, ResourcesCapability,
    PromptsCapability, ToolsCapability, InitializeRequest,
    InitializeParams, InitializeResult, Role, TextContent,
    Annotations, Resource, ResourceContents, TextResourceContents,
    BlobResourceContents, JsonRpcRequest, JsonRpcResponse,
    JsonRpcError, JsonRpcErrorDetail, JSONRPC_VERSION, MCP_VERSION,
};
pub use providers::ResourceProvider;
pub use clients::{ClientManager, ClientInfo};