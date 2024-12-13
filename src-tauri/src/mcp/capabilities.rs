use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use crate::mcp::types::*;

#[derive(Debug)]
pub struct ClientState {
    pub id: Uuid,
    pub capabilities: ClientCapabilities,
    pub negotiated_capabilities: ServerCapabilities,
}

pub struct CapabilityManager {
    clients: RwLock<HashMap<Uuid, ClientState>>,
    server_capabilities: ServerCapabilities,
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            server_capabilities: ServerCapabilities {
                resources: Some(ResourcesCapability {
                    list_changed: true,
                    subscribe: true,
                }),
                tools: Some(ToolsCapability {
                    list_changed: true,
                }),
                prompts: Some(PromptsCapability {
                    list_changed: true,
                }),
                ..Default::default()
            },
        }
    }

    pub fn register_client(&self, capabilities: ClientCapabilities) -> Uuid {
        let client_id = Uuid::new_v4();
        let negotiated = self.negotiate_capabilities(&capabilities);
        
        let state = ClientState {
            id: client_id,
            capabilities,
            negotiated_capabilities: negotiated.clone(),
        };

        self.clients.write().unwrap().insert(client_id, state);
        client_id
    }

    pub fn negotiate_capabilities(&self, client_caps: &ClientCapabilities) -> ServerCapabilities {
        let mut negotiated = self.server_capabilities.clone();

        // Negotiate file system capabilities if client supports roots
        if client_caps.roots.is_some() {
            negotiated.resources = Some(ResourcesCapability {
                list_changed: true,
                subscribe: true,
            });
        }

        // Only enable tools if client has experimental capabilities
        if client_caps.experimental.is_some() {
            negotiated.tools = Some(ToolsCapability {
                list_changed: true,
            });
        }

        negotiated
    }

    pub fn get_client_capabilities(&self, client_id: &Uuid) -> Option<ServerCapabilities> {
        self.clients.read()
            .unwrap()
            .get(client_id)
            .map(|state| state.negotiated_capabilities.clone())
    }

    pub fn update_client_capabilities(&self, client_id: &Uuid, capabilities: ClientCapabilities) -> Option<ServerCapabilities> {
        let mut clients = self.clients.write().unwrap();
        
        if let Some(state) = clients.get_mut(client_id) {
            let negotiated = self.negotiate_capabilities(&capabilities);
            state.capabilities = capabilities;
            state.negotiated_capabilities = negotiated.clone();
            Some(negotiated)
        } else {
            None
        }
    }

    pub fn remove_client(&self, client_id: &Uuid) -> bool {
        self.clients.write().unwrap().remove(client_id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_registration() {
        let manager = CapabilityManager::new();
        let caps = ClientCapabilities::default();
        let client_id = manager.register_client(caps);
        
        assert!(manager.get_client_capabilities(&client_id).is_some());
    }

    #[test]
    fn test_capability_negotiation_with_roots() {
        let manager = CapabilityManager::new();
        let mut caps = ClientCapabilities::default();
        caps.roots = Some(RootsCapability { list_changed: true });
        
        let negotiated = manager.negotiate_capabilities(&caps);
        assert!(negotiated.resources.is_some());
        assert!(negotiated.resources.unwrap().subscribe);
    }

    #[test]
    fn test_capability_negotiation_with_experimental() {
        let manager = CapabilityManager::new();
        let mut caps = ClientCapabilities::default();
        caps.experimental = Some(HashMap::new());
        
        let negotiated = manager.negotiate_capabilities(&caps);
        assert!(negotiated.tools.is_some());
    }

    #[test]
    fn test_capability_update() {
        let manager = CapabilityManager::new();
        let caps = ClientCapabilities::default();
        let client_id = manager.register_client(caps);
        
        let mut new_caps = ClientCapabilities::default();
        new_caps.roots = Some(RootsCapability { list_changed: true });
        
        let updated = manager.update_client_capabilities(&client_id, new_caps);
        assert!(updated.is_some());
        assert!(updated.unwrap().resources.is_some());
    }

    #[test]
    fn test_client_removal() {
        let manager = CapabilityManager::new();
        let caps = ClientCapabilities::default();
        let client_id = manager.register_client(caps);
        
        assert!(manager.remove_client(&client_id));
        assert!(manager.get_client_capabilities(&client_id).is_none());
    }
}