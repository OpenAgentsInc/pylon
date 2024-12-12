#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use actix_ws::Message;
    use futures_util::StreamExt;

    #[actix_web::test]
    async fn test_websocket_connection() {
        // Create test app
        let app = test::init_service(
            App::new().route("/mcp", web::get().to(crate::mcp::server::handle_connection))
        ).await;

        // Create test request
        let req = test::TestRequest::with_uri("/mcp")
            .insert_header(("upgrade", "websocket"))
            .insert_header(("connection", "upgrade"))
            .insert_header(("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ=="))
            .insert_header(("sec-websocket-version", "13"))
            .to_request();

        // Send request and get response
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_websocket_echo() {
        // Create test app with server
        let server = crate::mcp::MCPServer::new(8080);
        actix_web::rt::spawn(server.run());

        // Connect to websocket
        let mut client = awc::Client::new()
            .ws("ws://127.0.0.1:8080/mcp")
            .connect()
            .await
            .unwrap();

        // Send test message
        let text = "Hello, WebSocket!";
        client.send(Message::Text(text.into())).await.unwrap();

        // Receive echo
        if let Some(msg) = client.next().await {
            match msg.unwrap() {
                Message::Text(txt) => assert_eq!(txt, text),
                _ => panic!("Expected text message"),
            }
        }
    }

    #[actix_web::test]
    async fn test_websocket_ping_pong() {
        // Create test app with server
        let server = crate::mcp::MCPServer::new(8081);
        actix_web::rt::spawn(server.run());

        // Connect to websocket
        let mut client = awc::Client::new()
            .ws("ws://127.0.0.1:8081/mcp")
            .connect()
            .await
            .unwrap();

        // Send ping
        client.send(Message::Ping("ping".into())).await.unwrap();

        // Expect pong
        if let Some(msg) = client.next().await {
            match msg.unwrap() {
                Message::Pong(_) => (),
                _ => panic!("Expected pong message"),
            }
        }
    }

    #[actix_web::test]
    async fn test_multiple_clients() {
        // Create test app with server
        let server = crate::mcp::MCPServer::new(8082);
        actix_web::rt::spawn(server.run());

        // Connect first client
        let mut client1 = awc::Client::new()
            .ws("ws://127.0.0.1:8082/mcp")
            .connect()
            .await
            .unwrap();

        // Connect second client
        let mut client2 = awc::Client::new()
            .ws("ws://127.0.0.1:8082/mcp")
            .connect()
            .await
            .unwrap();

        // Test messages from both clients
        let text1 = "Message from client 1";
        let text2 = "Message from client 2";

        client1.send(Message::Text(text1.into())).await.unwrap();
        client2.send(Message::Text(text2.into())).await.unwrap();

        // Check responses
        if let Some(msg) = client1.next().await {
            match msg.unwrap() {
                Message::Text(txt) => assert_eq!(txt, text1),
                _ => panic!("Expected text message"),
            }
        }

        if let Some(msg) = client2.next().await {
            match msg.unwrap() {
                Message::Text(txt) => assert_eq!(txt, text2),
                _ => panic!("Expected text message"),
            }
        }
    }
}