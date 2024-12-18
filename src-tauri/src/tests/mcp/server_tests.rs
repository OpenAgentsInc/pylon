#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use awc::ws;
    use futures_util::{SinkExt, StreamExt};
    use std::sync::Arc;
    use serde_json::Value;

    use crate::mcp::protocol::MCPProtocol;
    use crate::mcp::server::handle_connection;
    use crate::mcp::types::{JSONRPC_VERSION, MCP_VERSION};

    #[actix_web::test]
    async fn test_websocket_connection() {
        // Create test server
        let protocol = Arc::new(MCPProtocol::new());
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(protocol))
                .route("/mcp", web::get().to(handle_connection))
        ).await;

        // Create test client with proper WebSocket headers
        let req = test::TestRequest::get()
            .uri("/mcp")
            .insert_header(("upgrade", "websocket"))
            .insert_header(("connection", "upgrade"))
            .insert_header(("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ=="))
            .insert_header(("sec-websocket-version", "13"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 101); // 101 Switching Protocols is the correct response for WebSocket upgrade
    }

    #[actix_web::test]
    async fn test_websocket_echo() {
        // Create test server with shared protocol instance
        let protocol = Arc::new(MCPProtocol::new());
        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(protocol.clone()))
                .route("/mcp", web::get().to(handle_connection))
        });

        // Create test client with proper WebSocket connection
        let client = awc::Client::new()
            .ws(srv.url("/mcp"))
            .set_header("upgrade", "websocket")
            .set_header("connection", "upgrade")
            .set_header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==")
            .set_header("sec-websocket-version", "13")
            .connect()
            .await;

        assert!(client.is_ok(), "Failed to establish WebSocket connection");
        let (_, mut connection) = client.unwrap();

        // Send test message
        let test_msg = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": 1,
            "method": "initialize",
            "params": {
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                },
                "protocolVersion": MCP_VERSION
            }
        });
        
        connection.send(ws::Message::Text(test_msg.to_string().into())).await.unwrap();

        // Receive and parse response
        if let Some(Ok(ws::Frame::Text(bytes))) = connection.next().await {
            let response: Value = serde_json::from_slice(&bytes).unwrap();
            println!("Received response: {}", serde_json::to_string_pretty(&response).unwrap());
            
            // Basic JSON-RPC response structure checks
            assert!(response.is_object(), "Response should be a JSON object");
            assert!(response.get("jsonrpc").is_some(), "Response should have jsonrpc field");
            assert!(response.get("id").is_some(), "Response should have id field");
            assert!(response.get("result").is_some(), "Response should have result field");
            
            // Verify specific fields
            assert_eq!(response["jsonrpc"].as_str().unwrap(), JSONRPC_VERSION);
            assert_eq!(response["id"].as_i64().unwrap(), 1);
            
            // Verify result structure
            let result = &response["result"];
            assert!(result.is_object(), "Result should be an object");
            assert!(result.get("capabilities").is_some(), "Result should have capabilities");
            assert!(result.get("protocol_version").is_some(), "Result should have protocol_version");
            assert!(result.get("server_info").is_some(), "Result should have server_info");
            
            assert_eq!(result["protocol_version"].as_str().unwrap(), MCP_VERSION);
        } else {
            panic!("Expected text response");
        }
    }

    #[actix_web::test]
    async fn test_multiple_clients() {
        // Create test server with shared protocol instance
        let protocol = Arc::new(MCPProtocol::new());
        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(protocol.clone()))
                .route("/mcp", web::get().to(handle_connection))
        });

        let url = srv.url("/mcp");

        // Create two clients with proper WebSocket connections
        let client1 = awc::Client::new()
            .ws(url.clone())
            .set_header("upgrade", "websocket")
            .set_header("connection", "upgrade")
            .set_header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==")
            .set_header("sec-websocket-version", "13")
            .connect()
            .await;

        let client2 = awc::Client::new()
            .ws(url)
            .set_header("upgrade", "websocket")
            .set_header("connection", "upgrade")
            .set_header("sec-websocket-key", "SGVsbG8sIHdvcmxkIQ==") // Different key for second client
            .set_header("sec-websocket-version", "13")
            .connect()
            .await;

        assert!(client1.is_ok(), "Failed to establish WebSocket connection for client 1");
        assert!(client2.is_ok(), "Failed to establish WebSocket connection for client 2");

        let (_, mut connection1) = client1.unwrap();
        let (_, mut connection2) = client2.unwrap();

        // Send messages from both clients
        let msg1 = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": 1,
            "method": "initialize",
            "params": {
                "capabilities": {},
                "clientInfo": {
                    "name": "test1-client",
                    "version": "1.0.0"
                },
                "protocolVersion": MCP_VERSION
            }
        });

        let msg2 = serde_json::json!({
            "jsonrpc": JSONRPC_VERSION,
            "id": 2,
            "method": "initialize",
            "params": {
                "capabilities": {},
                "clientInfo": {
                    "name": "test2-client",
                    "version": "1.0.0"
                },
                "protocolVersion": MCP_VERSION
            }
        });

        connection1.send(ws::Message::Text(msg1.to_string().into())).await.unwrap();
        connection2.send(ws::Message::Text(msg2.to_string().into())).await.unwrap();

        // Verify both clients receive responses
        if let Some(Ok(ws::Frame::Text(bytes))) = connection1.next().await {
            let response: Value = serde_json::from_slice(&bytes).unwrap();
            println!("Client 1 received response: {}", serde_json::to_string_pretty(&response).unwrap());
            
            assert!(response.is_object(), "Response should be a JSON object");
            assert!(response.get("jsonrpc").is_some(), "Response should have jsonrpc field");
            assert!(response.get("id").is_some(), "Response should have id field");
            assert!(response.get("result").is_some(), "Response should have result field");
            
            assert_eq!(response["jsonrpc"].as_str().unwrap(), JSONRPC_VERSION);
            assert_eq!(response["id"].as_i64().unwrap(), 1);
            
            let result = &response["result"];
            assert!(result.is_object(), "Result should be an object");
            assert!(result.get("capabilities").is_some(), "Result should have capabilities");
            assert!(result.get("protocol_version").is_some(), "Result should have protocol_version");
            assert!(result.get("server_info").is_some(), "Result should have server_info");
            
            assert_eq!(result["protocol_version"].as_str().unwrap(), MCP_VERSION);
        } else {
            panic!("Client 1 did not receive response");
        }

        if let Some(Ok(ws::Frame::Text(bytes))) = connection2.next().await {
            let response: Value = serde_json::from_slice(&bytes).unwrap();
            println!("Client 2 received response: {}", serde_json::to_string_pretty(&response).unwrap());
            
            assert!(response.is_object(), "Response should be a JSON object");
            assert!(response.get("jsonrpc").is_some(), "Response should have jsonrpc field");
            assert!(response.get("id").is_some(), "Response should have id field");
            assert!(response.get("result").is_some(), "Response should have result field");
            
            assert_eq!(response["jsonrpc"].as_str().unwrap(), JSONRPC_VERSION);
            assert_eq!(response["id"].as_i64().unwrap(), 2);
            
            let result = &response["result"];
            assert!(result.is_object(), "Result should be an object");
            assert!(result.get("capabilities").is_some(), "Result should have capabilities");
            assert!(result.get("protocol_version").is_some(), "Result should have protocol_version");
            assert!(result.get("server_info").is_some(), "Result should have server_info");
            
            assert_eq!(result["protocol_version"].as_str().unwrap(), MCP_VERSION);
        } else {
            panic!("Client 2 did not receive response");
        }
    }
}