use crate::output;
use crate::transport::{ApiError, Client};

/// `erpnext meta show <doctype>` — live DocType schema from the running site.
pub async fn show(client: &Client, doctype: &str) -> i32 {
    match fetch(client, doctype).await {
        Ok(meta) => output::ok(meta),
        Err(e) => output::fail(kind(&e), &e.to_string(), output::EXIT_API),
    }
}

pub async fn fetch(client: &Client, doctype: &str) -> Result<serde_json::Value, ApiError> {
    let path = format!("/api/v2/doctype/{}/meta", encode(doctype));
    client.get(&path).await
}

fn encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

fn kind(e: &ApiError) -> &'static str {
    match e {
        ApiError::Auth { .. } => "auth",
        ApiError::Forbidden { .. } => "permission",
        ApiError::NotFound { .. } => "not_found",
        ApiError::Rejected { .. } => "rejected",
        ApiError::Transport(_) => "transport",
        ApiError::Shape(_) => "shape",
    }
}
