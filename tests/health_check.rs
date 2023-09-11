use std::mem::forget;
use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // let port = listener.local_addr().unwrap().port();
    let address = listener.local_addr().unwrap().to_string();
    let server = newsletter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://{address}")
}

#[tokio::test]
async fn health_check_works() {
    // Given
    let address = spawn_app();

    // When
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

