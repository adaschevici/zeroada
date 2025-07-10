use zr2prd::run;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8091/health_check")
        .send()
        .await
        .expect("Failed to send request");
    assert!(
        response.status().is_success(),
        "Response was not successful"
    );
    // assert_eq!(
    //     response.text().await.expect("Failed to read response text"),
    //     "Ok",
    //     "Response text did not match expected value"
    // );
}

fn spawn_app() {
    let server = run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
