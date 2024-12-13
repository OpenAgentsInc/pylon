#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use awc::ws;
    use futures_util::{SinkExt, StreamExt};
    use std::sync::Arc;

    use crate::mcp::MCPProtocol;
    use crate::mcp::server::handle_connection;

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
        // Create test server
        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(MCPProtocol::new()))
                .route("/mcp", web::get().to(handle_connection))
        });

        // Create test client with proper WebSocket connection
        let mut client = awc::Client::new()
            .ws(srv.url("/mcp"))
            .set_header("upgrade", "websocket")
            .set_header("connection", "upgrade")
            .set_header("sec-websocket-version", "13")
            .connect()
            .await
            .unwrap();

        // Send test message
        let test_msg = r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{},"clientInfo":{"name":"test","version":"1.0"},"protocolVersion":"0.1"},"id":1}"#;
        client.1.send(ws::Message::Text(test_msg.into())).await.unwrap();

        // Receive response
        if let Some(Ok(ws::Frame::Text(bytes))) = client.1.next().await {
            let response = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(response.contains("\"jsonrpc\":\"2.0\"")); // Should be valid JSON-RPC response
        } else {
            panic!("Expected text response");
        }
    }

    #[actix_web::test]
    async fn test_multiple_clients() {
        // Create test server
        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(MCPProtocol::new()))
                .route("/mcp", web::get().to(handle_connection))
        });

        let url = srv.url("/mcp");

        // Create two clients with proper WebSocket connections
        let mut client1 = awc::Client::new()
            .ws(url.clone())
            .set_header("upgrade", "websocket")
            .set_header("connection", "upgrade")
            .set_header("sec-websocket-version", "13")
            .connect()
            .await
            .unwrap();

        let mut client2 = awc::Client::new()
            .ws(url)
            .set_header("upgrade", "websocket")
            .set_header("connection", "upgrade")
            .set_header("sec-websocket-version", "13")
            .connect()
            .await
            .unwrap();

        // Send messages from both clients
        let msg1 = r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{},"clientInfo":{"name":"test1","version":"1.0"},"protocolVersion":"0.1"},"id":1}"#;
        let msg2 = r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{},"clientInfo":{"name":"test2","version":"1.0"},"protocolVersion":"0.1"},"id":2}"#;

        client1.1.send(ws::Message::Text(msg1.into())).await.unwrap();
        client2.1.send(ws::Message::Text(msg2.into())).await.unwrap();

        // Verify both clients receive responses
        if let Some(Ok(ws::Frame::Text(bytes))) = client1.1.next().await {
            let response = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(response.contains("\"jsonrpc\":\"2.0\"")); // Should be valid JSON-RPC response
        } else {
            panic!("Client 1 did not receive response");
        }

        if let Some(Ok(ws::Frame::Text(bytes))) = client2.1.next().await {
            let response = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(response.contains("\"jsonrpc\":\"2.0\"")); // Should be valid JSON-RPC response
        } else {
            panic!("Client 2 did not receive response");
        }
    }
}