pub mod types;
pub mod providers;
pub mod protocol;
pub mod server;
pub mod clients;

pub use protocol::MCPProtocol;
pub use server::MCPServer;
pub use types::{
    JSONRPC_VERSION,
    MCP_VERSION,
    Implementation,
    ServerCapabilities,
    InitializeParams,
    InitializeResult,
    ClientCapabilities,
    RootsCapability,
    OllamaCapability,
};