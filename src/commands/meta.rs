use crate::output;
use crate::transport::{encode_segment, ApiError, Client};

/// `erpnext meta show <doctype>` — live DocType schema from the running site.
pub async fn show(client: &Client, doctype: &str) -> i32 {
    match fetch(client, doctype).await {
        Ok(meta) => output::ok_payload(meta),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

pub async fn fetch(client: &Client, doctype: &str) -> Result<serde_json::Value, ApiError> {
    let path = format!("/api/v2/doctype/{}/meta", encode_segment(doctype));
    client.get(&path).await
}
