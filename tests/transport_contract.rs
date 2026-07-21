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
    let exit = commands::doc::create(
        &client,
        "Customer",
        r#"{"customer_name":"Acme"}"#,
        None,
        false,
    )
    .await;

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
    let exit = commands::doc::create(
        &client,
        "Customer",
        r#"{"customer_name":"Acme"}"#,
        None,
        true,
    )
    .await;

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
    let exit = commands::doc::create(&client, "Customer", r#"{"bogus":1}"#, None, true).await;

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

#[tokio::test]
async fn resolve_party_queries_exact_filters_and_reports_ambiguity() {
    let server = MockServer::start_async().await;
    let customer = server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Customer")
            .query_param("filters", r#"[["Customer","name","=","Acme"]]"#);
        then.status(200)
            .json_body(json!({ "data": [ { "name": "Acme", "customer_name": "Acme BV" } ] }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Customer");
        then.status(200).json_body(json!({ "data": [] }));
    });
    let supplier = server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Supplier")
            .query_param("filters", r#"[["Supplier","name","=","Acme"]]"#);
        then.status(200)
            .json_body(json!({ "data": [ { "name": "Acme", "supplier_name": "Acme BV" } ] }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Supplier");
        then.status(200).json_body(json!({ "data": [] }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let resolution = commands::resolve::party_resolution(&client, "Acme")
        .await
        .expect("query succeeds");

    customer.assert();
    supplier.assert();
    match resolution {
        commands::resolve::Resolution::Ambiguous(candidates) => {
            assert_eq!(candidates.len(), 2);
            assert_eq!(candidates[0].doctype, "Customer");
            assert_eq!(candidates[0].name, "Acme");
            assert_eq!(candidates[0].title, "Acme BV");
            assert_eq!(candidates[1].doctype, "Supplier");
        }
        other => panic!("expected ambiguity, got {other:?}"),
    }
}

#[tokio::test]
async fn resolve_item_returns_the_single_reference() {
    let server = MockServer::start_async().await;
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Item")
            .query_param("filters", r#"[["Item","item_name","=","Business in a Box Onboarding"]]"#);
        then.status(200).json_body(
            json!({ "data": [ { "name": "BIB-ONBOARDING", "item_name": "Business in a Box Onboarding" } ] }),
        );
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Item");
        then.status(200).json_body(json!({ "data": [] }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let resolution = commands::resolve::item_resolution(&client, "Business in a Box Onboarding")
        .await
        .expect("query succeeds");

    match resolution {
        commands::resolve::Resolution::Resolved(reference) => {
            assert_eq!(reference.doctype, "Item");
            assert_eq!(reference.name, "BIB-ONBOARDING");
            assert_eq!(reference.title, "Business in a Box Onboarding");
        }
        other => panic!("expected resolved, got {other:?}"),
    }
}

#[tokio::test]
async fn resolve_document_miss_is_structured_none() {
    let server = MockServer::start_async().await;
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Quotation/MISSING-1");
        then.status(404)
            .json_body(json!({ "message": "not found" }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let resolution = commands::resolve::document_resolution(&client, "Quotation", "MISSING-1")
        .await
        .expect("query succeeds");

    assert_eq!(resolution, commands::resolve::Resolution::None);
}

/// The peer-specified crash-window proof: the first create reached ERPNext but its
/// response was lost. The retry with the same source token resolves the existing
/// document by its deterministic name and sends zero further create requests.
#[tokio::test]
async fn source_retry_after_lost_response_replays_without_second_create() {
    let server = MockServer::start_async().await;

    // The document already exists on the site (created by the "lost" first attempt).
    let get_existing = server.mock(|when, then| {
        when.method(GET).path_matches(
            Regex::new("^/api/v2/document/Quotation/SRC-[0-9a-f]{20}$").expect("regex"),
        );
        then.status(200).json_body(json!({ "data": {
            "name": "SRC-known", "party_name": "Acme Pilot BV", "quotation_to": "Customer",
        }}));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Quotation");
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::doc::create(
        &client,
        "Quotation",
        r#"{"party_name":"Acme Pilot BV","quotation_to":"Customer"}"#,
        Some("quotation/acme-pilot/2026-07-onboarding"),
        true,
    )
    .await;

    assert_eq!(exit, 0, "a faithful replay succeeds");
    get_existing.assert();
    assert_eq!(create.hits(), 0, "the retry must send zero create requests");
}

/// Same token, different data: a structured idempotency conflict, never a write.
#[tokio::test]
async fn source_token_reuse_with_different_data_is_a_conflict() {
    let server = MockServer::start_async().await;

    server.mock(|when, then| {
        when.method(GET).path_matches(
            Regex::new("^/api/v2/document/Quotation/SRC-[0-9a-f]{20}$").expect("regex"),
        );
        then.status(200).json_body(json!({ "data": {
            "name": "SRC-known", "party_name": "Someone Else BV",
        }}));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Quotation");
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::doc::create(
        &client,
        "Quotation",
        r#"{"party_name":"Acme Pilot BV"}"#,
        Some("quotation/acme-pilot/2026-07-onboarding"),
        true,
    )
    .await;

    assert_eq!(exit, 1, "reused token with different data must fail");
    assert_eq!(
        create.hits(),
        0,
        "a conflict must never reach the site as a write"
    );
}

/// Fresh token: the derived name travels inside the create payload atomically.
#[tokio::test]
async fn source_create_sends_the_derived_name_in_the_payload() {
    let server = MockServer::start_async().await;

    server.mock(|when, then| {
        when.method(GET).path_matches(
            Regex::new("^/api/v2/document/Quotation/SRC-[0-9a-f]{20}$").expect("regex"),
        );
        then.status(404)
            .json_body(json!({ "message": "not found" }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Quotation/meta");
        then.status(200).json_body(json!({ "data": { "fields": [
            { "fieldname": "party_name", "reqd": 0 },
        ]}}));
    });
    let create = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Quotation")
            .body_matches(Regex::new(r#""name":"SRC-[0-9a-f]{20}""#).expect("regex"));
        then.status(200)
            .json_body(json!({ "data": { "name": "SRC-new" } }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::doc::create(
        &client,
        "Quotation",
        r#"{"party_name":"Acme Pilot BV"}"#,
        Some("quotation/acme-pilot/fresh"),
        true,
    )
    .await;

    assert_eq!(exit, 0);
    create.assert();
}
