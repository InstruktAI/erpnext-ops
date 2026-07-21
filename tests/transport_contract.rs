use httpmock::prelude::*;
use serde_json::json;
use url::Url;

use erpnext::commands;
use erpnext::config::{Context, Credentials, PrivilegeMode};
use erpnext::transport::{ApiError, Client};

fn context_for(server: &MockServer) -> Context {
    Context {
        site_url: Url::parse(&server.base_url()).expect("mock server url"),
        company_slug: "test-co".into(),
        company: None,
        privilege: PrivilegeMode::Operator,
        credentials: Credentials {
            api_key: "test-key".into(),
            api_secret: "test-secret".into(),
        },
    }
}

#[tokio::test]
async fn preview_sends_no_request_at_all() {
    let server = MockServer::start_async().await;
    let catch_all = server.mock(|when, then| {
        when.path_matches(Regex::new(".*").expect("regex"));
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit =
        commands::doc::create(&client, "Customer", r#"{"customer_name":"Acme"}"#, false).await;

    assert_eq!(exit, 0);
    assert_eq!(catch_all.hits(), 0, "preview must not touch the site");
}

#[tokio::test]
async fn apply_sends_the_exact_native_request_with_auth() {
    let server = MockServer::start_async().await;

    let meta = server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200).json_body(json!({ "data": { "fields": [
            { "fieldname": "customer_name", "reqd": 1 },
        ]}}));
    });
    let create = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Customer")
            .header("Authorization", "token test-key:test-secret")
            .json_body(json!({ "customer_name": "Acme" }));
        then.status(200)
            .json_body(json!({ "data": { "name": "CUST-0001" } }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit =
        commands::doc::create(&client, "Customer", r#"{"customer_name":"Acme"}"#, true).await;

    assert_eq!(exit, 0);
    meta.assert();
    create.assert();
}

#[tokio::test]
async fn preflight_blocks_invalid_payload_before_any_write() {
    let server = MockServer::start_async().await;

    let meta = server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200).json_body(json!({ "data": { "fields": [
            { "fieldname": "customer_name", "reqd": 1 },
        ]}}));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Customer");
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::doc::create(&client, "Customer", r#"{"bogus":1}"#, true).await;

    assert_eq!(exit, 1);
    meta.assert();
    assert_eq!(
        create.hits(),
        0,
        "invalid payload must never reach the site"
    );
}

#[tokio::test]
async fn rejected_credentials_surface_as_typed_auth_error() {
    let server = MockServer::start_async().await;
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/method/frappe.auth.get_logged_user");
        then.status(401)
            .json_body(json!({ "message": "Invalid API Key" }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let err = client.whoami().await.expect_err("401 must be an error");

    assert!(matches!(err, ApiError::Auth { status: 401 }));
    let rendered = err.to_string();
    assert!(!rendered.contains("test-key"));
    assert!(!rendered.contains("test-secret"));
}

#[tokio::test]
async fn workflow_method_preview_sends_nothing_and_apply_hits_method_envelope() {
    let server = MockServer::start_async().await;
    let method = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Sales%20Invoice/INV-0001/method/submit/");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1 } }));
    });

    let client = Client::new(context_for(&server)).expect("client");

    let preview_exit =
        commands::workflow::run_method(&client, "Sales Invoice", "INV-0001", "submit", None, false)
            .await;
    assert_eq!(preview_exit, 0);
    assert_eq!(method.hits(), 0, "preview must not touch the site");

    let apply_exit =
        commands::workflow::run_method(&client, "Sales Invoice", "INV-0001", "submit", None, true)
            .await;
    assert_eq!(apply_exit, 0);
    method.assert();
}
