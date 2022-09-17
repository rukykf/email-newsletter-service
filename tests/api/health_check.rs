use crate::helpers::spawn_app;
use crate::helpers::TestApp;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let TestApp { address, .. } = spawn_app().await;

    let client = reqwest::Client::new();

    // Act

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
