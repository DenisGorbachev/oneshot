// use std::sync::Arc;
//
// use derive_more::{Error, From};
// use reqwest::Client as HttpClient;
// use serde_json::Value;
// use tokio::sync::Mutex;
//
// #[derive(Debug, Error, From)]
// pub enum LspClientError {
//     #[error("HTTP error: {0}")]
//     HttpError(#[from] reqwest::Error),
//     #[error("JSON error: {0}")]
//     JsonError(#[from] serde_json::Error),
//     #[error("IO error: {0}")]
//     IoError(#[from] std::io::Error),
// }
//
// pub struct LspClient {
//     http_client: HttpClient,
//     server_url: String,
//     request_id: Arc<Mutex<i64>>,
// }
//
// impl LspClient {
//     pub fn new(server_url: String) -> Self {
//         LspClient {
//             http_client: HttpClient::new(),
//             server_url,
//             request_id: Arc::new(Mutex::new(0)),
//         }
//     }
//
//     async fn send_request(&self, method: &str, params: Value) -> Result<Value, LspClientError> {
//         let mut id = self.request_id.lock().await;
//         *id += 1;
//         let request_id = *id;
//
//         let request_body = serde_json::json!({
//             "jsonrpc": "2.0",
//             "id": request_id,
//             "method": method,
//             "params": params,
//         });
//
//         let response = self.http_client
//             .post(&self.server_url)
//             .json(&request_body)
//             .send()
//             .await?
//             .json::<Value>()
//             .await?;
//
//         Ok(response)
//     }
//
//     pub async fn initialize(&self, root_uri: &str) -> Result<Value, LspClientError> {
//         let params = serde_json::json!({
//             "processId": std::process::id(),
//             "rootUri": root_uri,
//             "capabilities": {}
//         });
//
//         self.send_request("initialize", params).await
//     }
//
//     pub async fn shutdown(&self) -> Result<Value, LspClientError> {
//         self.send_request("shutdown", Value::Null).await
//     }
//
//     pub async fn text_document_did_open(&self, uri: &str, text: &str) -> Result<Value, LspClientError> {
//         let params = serde_json::json!({
//             "textDocument": {
//                 "uri": uri,
//                 "languageId": "rust",
//                 "version": 1,
//                 "text": text
//             }
//         });
//
//         self.send_request("textDocument/didOpen", params).await
//     }
//
//     // Add more methods for other LSP requests as needed
// }
