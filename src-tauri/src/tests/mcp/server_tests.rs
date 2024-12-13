use actix_web::{test, web, App};
use awc::ws;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;

use crate::mcp::{MCPServer, MCPProtocol};
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

    // Create test client
    let req = test::TestRequest::get()
        .uri("/mcp")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_websocket_echo() {
    // Create test server
    let protocol = Arc::new(MCPProtocol::new());
    let server = test::init_service(
        App::new()
            .app_data(web::Data::new(protocol))
            .route("/mcp", web::get().to(handle_connection))
    ).await;

    // Create test client
    let srv = test::TestServer::new(server.into_service());
    let url = srv.url("/mcp");

    let mut client = awc::Client::new()
        .ws(url)
        .connect()
        .await
        .unwrap();

    // Send test message
    let test_msg = "Hello, WebSocket!";
    client.1.send(ws::Message::Text(test_msg.into())).await.unwrap();

    // Receive response
    if let Some(Ok(ws::Frame::Text(bytes))) = client.1.next().await {
        let response = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(response.contains("error")); // Since we're not sending valid JSON-RPC
    } else {
        panic!("Expected text response");
    }
}

#[actix_web::test]
async fn test_multiple_clients() {
    // Create test server
    let protocol = Arc::new(MCPProtocol::new());
    let server = test::init_service(
        App::new()
            .app_data(web::Data::new(protocol))
            .route("/mcp", web::get().to(handle_connection))
    ).await;

    // Create test server instance
    let srv = test::TestServer::new(server.into_service());
    let url = srv.url("/mcp");

    // Create two clients
    let mut client1 = awc::Client::new()
        .ws(url.clone())
        .connect()
        .await
        .unwrap();

    let mut client2 = awc::Client::new()
        .ws(url)
        .connect()
        .await
        .unwrap();

    // Send messages from both clients
    let msg1 = "Hello from client 1";
    let msg2 = "Hello from client 2";

    client1.1.send(ws::Message::Text(msg1.into())).await.unwrap();
    client2.1.send(ws::Message::Text(msg2.into())).await.unwrap();

    // Verify both clients receive responses
    if let Some(Ok(ws::Frame::Text(_))) = client1.1.next().await {
        // Response received by client 1
    } else {
        panic!("Client 1 did not receive response");
    }

    if let Some(Ok(ws::Frame::Text(_))) = client2.1.next().await {
        // Response received by client 2
    } else {
        panic!("Client 2 did not receive response");
    }
}