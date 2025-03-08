use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub async fn mock_node_response_str(mock_server: &MockServer, method_name: &str, body: &String) {
    Mock::given(method("POST"))
        .and(path("/"))
        .and(wiremock::matchers::body_string_contains(method_name))
        .respond_with(ResponseTemplate::new(200).set_body_string(body).append_header("content-type", "application/json"))
        .mount(mock_server)
        .await;
}

pub async fn mock_node_response_json(mock_server: &MockServer, method_name: &str, result: Value) {
    let mut body = json!(
        {"jsonrpc":"2.0","id":1}
    );
    body["result"] = result;

    mock_node_response_str(mock_server, method_name, &body.to_string()).await;
}

pub async fn mock_node_response_file(mock_server: &MockServer, method_name: &str, file_path: &str) {
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    full_path.push("tests");
    full_path.push(file_path);
    let mut body = String::new();
    fs::File::open(full_path).unwrap().read_to_string(&mut body).unwrap();
    mock_node_response_str(mock_server, method_name, &body).await;
}
