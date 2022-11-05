use std::net::TcpListener;
use health_check::run;

fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let server = run(listener).expect("Failed to bind to address");
  tokio::spawn(server);

  format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_ok(){
  let address = spawn_app();
  let client = reqwest::Client::new();

  let response = client
  .get(&format!("{}/health_check", &address))
  .send()
  .await
  .expect("Failed to get response");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}