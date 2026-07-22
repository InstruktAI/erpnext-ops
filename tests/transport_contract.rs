use httpmock::prelude::*;
use serde_json::{json, Value};
use std::fs;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};
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
            user: None,
            api_key: "test-key".into(),
            api_secret: "test-secret".into(),
        },
    }
}

fn run_cli_against(server: &MockServer, args: &[&str]) -> Output {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let home = std::env::temp_dir().join(format!(
        "erpnext-cli-contract-{}-{unique}",
        std::process::id()
    ));
    let company_home = home.join(".teleclaude/companies/test-co");
    fs::create_dir_all(&company_home).expect("test credential home");
    let site_url = server.base_url().replace("127.0.0.1", "localhost");
    fs::write(
        company_home.join("erpnext.json"),
        serde_json::to_vec(&json!({
            "site_url": site_url,
            "company": "Test Company",
            "operator": {
                "api_key": "test-key",
                "api_secret": "test-secret"
            }
        }))
        .expect("credential json"),
    )
    .expect("write credentials");

    let output = Command::new(env!("CARGO_BIN_EXE_erpnext"))
        .env("HOME", &home)
        .args(["--company", "test-co"])
        .args(args)
        .output()
        .expect("run erpnext CLI");
    fs::remove_dir_all(home).expect("remove test home");
    output
}

fn json_output(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("machine-readable JSON envelope")
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
async fn workflow_submit_drives_docstatus_and_verifies_the_stored_state() {
    let server = MockServer::start_async().await;
    let put = server.mock(|when, then| {
        when.method(PUT)
            .path("/api/v2/document/Sales%20Invoice/INV-0001")
            .json_body(json!({ "docstatus": 1 }));
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1 } }));
    });
    let read_back = server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Sales%20Invoice/INV-0001");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1 } }));
    });

    let client = Client::new(context_for(&server)).expect("client");

    let preview = commands::workflow::submit(&client, "Sales Invoice", "INV-0001", false).await;
    assert_eq!(preview, 0);
    assert_eq!(put.hits(), 0, "preview must not touch the site");

    let applied = commands::workflow::submit(&client, "Sales Invoice", "INV-0001", true).await;
    assert_eq!(applied, 0);
    put.assert();
    read_back.assert();
}

#[tokio::test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
async fn workflow_method_persists_the_native_controller_result() {
    let server = MockServer::start_async().await;
    let method = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Sales%20Invoice/ACC-SINV-2026-00002/method/add_timesheet_data/")
            .header("Authorization", "token test-key:test-secret")
            .json_body(json!({}));
        then.status(200).json_body(json!({ "docs": [{
                "doctype": "Sales Invoice",
                "name": "ACC-SINV-2026-00002",
                "controller_only": true,
                "timesheets": [{ "time_sheet": "TS-0001" }]
            }] }));
    });
    let save = server.mock(|when, then| {
        when.method(PUT)
            .path("/api/v2/document/Sales%20Invoice/ACC-SINV-2026-00002")
            .json_body(json!({
                "doctype": "Sales Invoice",
                "name": "ACC-SINV-2026-00002",
                "controller_only": true,
                "timesheets": [{ "time_sheet": "TS-0001" }]
            }));
        then.status(200).json_body(json!({ "data": {
            "doctype": "Sales Invoice",
            "name": "ACC-SINV-2026-00002",
            "stored_only": true,
            "timesheets": [{ "time_sheet": "TS-0001" }]
        } }));
    });

    let output = run_cli_against(
        &server,
        &[
            "workflow",
            "method",
            "Sales Invoice",
            "ACC-SINV-2026-00002",
            "add_timesheet_data",
            "--apply",
        ],
    );
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty(), "success writes no error envelope");
    let envelope = json_output(&output.stdout);
    assert_eq!(envelope.pointer("/ok"), Some(&json!(true)));
    assert_eq!(envelope.pointer("/data/stored_only"), Some(&json!(true)));
    assert_eq!(
        envelope.pointer("/data/controller_only"),
        None,
        "the command reports the stored PUT result, not the controller response"
    );
    method.assert();
    save.assert();
}

#[tokio::test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
async fn workflow_method_rejects_a_different_returned_document_without_saving() {
    let server = MockServer::start_async().await;
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Sales%20Invoice/INV-1/method/add_timesheet_data/");
        then.status(200).json_body(json!({ "docs": [{
            "doctype": "Sales Invoice", "name": "INV-OTHER"
        }] }));
    });
    let save = server.mock(|when, then| {
        when.method(PUT)
            .path_matches(Regex::new("^/api/v2/document/.*").expect("regex"));
        then.status(200).json_body(json!({}));
    });

    let output = run_cli_against(
        &server,
        &[
            "workflow",
            "method",
            "Sales Invoice",
            "INV-1",
            "add_timesheet_data",
            "--apply",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stdout.is_empty(),
        "failure writes no success envelope"
    );
    let envelope = json_output(&output.stderr);
    assert_eq!(envelope.pointer("/ok"), Some(&json!(false)));
    assert_eq!(envelope.pointer("/error/kind"), Some(&json!("shape")));
    assert!(envelope
        .pointer("/error/message")
        .and_then(Value::as_str)
        .is_some());
    assert_eq!(save.hits(), 0, "a mismatched document is never saved");
}

#[tokio::test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
async fn workflow_method_preserves_the_controller_result_when_save_fails() {
    let server = MockServer::start_async().await;
    let method_result = json!({ "docs": [{
        "doctype": "Sales Invoice",
        "name": "INV-1",
        "timesheets": [{ "time_sheet": "TS-1" }]
    }] });
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Sales%20Invoice/INV-1/method/add_timesheet_data/");
        then.status(200).json_body(method_result.clone());
    });
    server.mock(|when, then| {
        when.method(PUT)
            .path("/api/v2/document/Sales%20Invoice/INV-1");
        then.status(417)
            .json_body(json!({ "message": "save rejected" }));
    });

    let output = run_cli_against(
        &server,
        &[
            "workflow",
            "method",
            "Sales Invoice",
            "INV-1",
            "add_timesheet_data",
            "--apply",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stdout.is_empty(),
        "failure writes no success envelope"
    );
    let envelope = json_output(&output.stderr);
    assert_eq!(envelope.pointer("/ok"), Some(&json!(false)));
    assert_eq!(
        envelope.pointer("/data/method_result"),
        Some(&method_result),
        "partial success remains observable for recovery"
    );
}

/// The site answers a declined lifecycle request with the document body and an
/// unchanged docstatus. Reporting that as success is the defect this pins.
#[tokio::test]
async fn workflow_submit_fails_when_the_docstatus_never_changes() {
    let server = MockServer::start_async().await;
    server.mock(|when, then| {
        when.method(PUT).path("/api/v2/document/Quotation/Q-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 0 } }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Quotation/Q-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 0 } }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::workflow::submit(&client, "Quotation", "Q-1", true).await;

    assert_eq!(exit, 1, "a silent no-op must never be reported as success");
}

#[tokio::test]
async fn workflow_amend_refuses_a_document_that_is_not_cancelled() {
    let server = MockServer::start_async().await;
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Quotation/Q-2");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1 } }));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Quotation");
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::workflow::amend(&client, "Quotation", "Q-2", "{}", true).await;

    assert_eq!(exit, 1);
    assert_eq!(create.hits(), 0, "only cancelled documents are amended");
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
    let meta = server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Quotation/meta");
        then.status(200)
            .json_body(json!({ "data": { "title_field": null } }));
    });
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

    meta.assert();
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

/// ERPNext enriches child rows with names, indexes, accounts, and derived amounts. A
/// replay compares the caller-owned fields recursively and must not mistake those
/// server-owned additions for divergent input.
#[tokio::test]
// spec: project/spec/feature/erp-operations/document-operations UC-8
async fn source_replay_accepts_server_enriched_child_rows() {
    let server = MockServer::start_async().await;

    let get_existing = server.mock(|when, then| {
        when.method(GET).path_matches(
            Regex::new("^/api/v2/document/Sales%20Invoice/SRC-[0-9a-f]{20}$").expect("regex"),
        );
        then.status(200).json_body(json!({ "data": {
            "name": "SRC-known",
            "customer": "CUSTOMER-1",
            "items": [{
                "item_code": "SERVICE-1",
                "qty": 1.0,
                "rate": 100.0,
                "amount": 100.0,
                "income_account": "Services - CO",
                "idx": 1,
                "name": "row-1",
            }],
        }}));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Sales%20Invoice");
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::doc::create(
        &client,
        "Sales Invoice",
        r#"{"customer":"CUSTOMER-1","items":[{"item_code":"SERVICE-1","qty":1,"rate":100}]}"#,
        Some("sales-invoice/customer-1/service-1"),
        true,
    )
    .await;

    assert_eq!(exit, 0, "server-owned child fields do not break replay");
    get_existing.assert();
    assert_eq!(
        create.hits(),
        0,
        "a replay must not create a second invoice"
    );
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

#[tokio::test]
async fn activity_capture_uses_permission_aware_timeline_apis() {
    let server = MockServer::start_async().await;
    let comment = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.desk.form.utils.add_comment")
            .body_matches(Regex::new(r#""reference_name":"SAL-QTN-2026-00001""#).expect("regex"));
        then.status(200)
            .json_body(json!({ "message": { "name": "cmt-1" } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/method/frappe.auth.get_logged_user");
        then.status(200)
            .json_body(json!({ "message": "agent@instrukt.ai" }));
    });
    // A follow-up is stored as its own referencing record. The assignment API allows one
    // open assignment per user per document and answers a refused second one with success,
    // which would cap a party at a single commitment and lose the rest without a trace.
    let follow_up = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/ToDo")
            .body_matches(Regex::new(r#""reference_name":"SAL-QTN-2026-00001""#).expect("regex"));
        then.status(200)
            .json_body(json!({ "data": { "name": "TODO-1", "status": "Open" } }));
    });
    let assign = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.desk.form.assign_to.add");
        then.status(200).json_body(json!({ "message": [] }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::activity::capture(
        &client,
        commands::activity::Capture {
            summary: "note",
            verbatim: None,
            medium: None,
            reference: Some(("Quotation", "SAL-QTN-2026-00001")),
            follow_up: Some("call them"),
            due: Some("2026-07-28"),
        },
    )
    .await;

    assert_eq!(exit, 0);
    comment.assert();
    follow_up.assert();
    assert_eq!(
        assign.hits(),
        0,
        "the capped assignment API is never the store for a commitment"
    );
}

#[tokio::test]
async fn activity_capture_without_reference_is_a_structured_blocker_before_any_request() {
    let server = MockServer::start_async().await;
    let catch_all = server.mock(|when, then| {
        when.path_matches(Regex::new(".*").expect("regex"));
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::activity::capture(
        &client,
        commands::activity::Capture {
            summary: "note",
            verbatim: None,
            medium: None,
            reference: None,
            follow_up: None,
            due: None,
        },
    )
    .await;

    assert_eq!(
        exit, 2,
        "missing reference is a config-class structured failure"
    );
    assert_eq!(catch_all.hits(), 0);
}

#[tokio::test]
async fn activity_capture_rejects_non_native_medium_client_side() {
    let server = MockServer::start_async().await;
    let catch_all = server.mock(|when, then| {
        when.path_matches(Regex::new(".*").expect("regex"));
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::activity::capture(
        &client,
        commands::activity::Capture {
            summary: "note",
            verbatim: None,
            medium: Some("Carrier Pigeon"),
            reference: Some(("Quotation", "Q-1")),
            follow_up: None,
            due: None,
        },
    )
    .await;

    assert_eq!(exit, 2);
    assert_eq!(catch_all.hits(), 0);
}

#[tokio::test]
async fn evidence_attach_uploads_via_native_endpoint_and_notes_the_timeline() {
    let server = MockServer::start_async().await;
    let upload = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/upload_file")
            .body_contains("SAL-QTN-2026-00001");
        then.status(200)
            .json_body(json!({ "message": { "file_url": "/private/files/x.txt" } }));
    });
    let comment = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.desk.form.utils.add_comment")
            .body_matches(Regex::new("attachment: /private/files/x.txt").expect("regex"));
        then.status(200)
            .json_body(json!({ "message": { "name": "cmt-2" } }));
    });

    let dir = std::env::temp_dir().join(format!("erpnext-evidence-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let file = dir.join("x.txt");
    std::fs::write(&file, b"evidence").expect("write");

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::evidence::attach(
        &client,
        ("Quotation", "SAL-QTN-2026-00001"),
        file.to_str().expect("utf8 path"),
        Some("noted"),
    )
    .await;

    assert_eq!(exit, 0);
    upload.assert();
    comment.assert();
    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn evidence_attach_missing_file_fails_closed_before_any_request() {
    let server = MockServer::start_async().await;
    let catch_all = server.mock(|when, then| {
        when.path_matches(Regex::new(".*").expect("regex"));
        then.status(200).json_body(json!({}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::evidence::attach(
        &client,
        ("Quotation", "SAL-QTN-2026-00001"),
        "/nonexistent/evidence.txt",
        None,
    )
    .await;

    assert_eq!(exit, 2);
    assert_eq!(catch_all.hits(), 0);
}

#[tokio::test]
async fn report_run_uses_the_native_runner_with_filters() {
    let server = MockServer::start_async().await;
    let runner = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.desk.query_report.run")
            .body_matches(Regex::new(r#""report_name":"Quotation Trends""#).expect("regex"))
            .body_matches(Regex::new(r#""company":"Instrukt AI""#).expect("regex"));
        then.status(200).json_body(json!({ "message": {
            "columns": [ { "fieldname": "item" } ],
            "result": [ { "item": "BIB-ONBOARDING" } ],
        }}));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::report::run(
        &client,
        "Quotation Trends",
        &[("company".into(), "Instrukt AI".into())],
    )
    .await;

    assert_eq!(exit, 0);
    runner.assert();
}

#[test]
// spec: project/spec/feature/erp-operations/reporting UC-2
fn malformed_report_response_is_a_structured_shape_failure() {
    let server = MockServer::start();
    let runner = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.desk.query_report.run");
        then.status(200)
            .json_body(json!({ "message": { "columns": [] } }));
    });

    let output = run_cli_against(&server, &["report", "run", "Malformed Report"]);
    let failure = json_output(&output.stderr);

    assert!(!output.status.success());
    assert!(
        output.stdout.is_empty(),
        "a malformed report has no success payload"
    );
    let shape_kind = ApiError::Shape(String::new()).kind();
    assert_eq!(failure.pointer("/error/kind"), Some(&json!(shape_kind)));
    runner.assert();
}

#[tokio::test]
async fn import_preview_builds_the_native_batch_chain() {
    let server = MockServer::start_async().await;
    let create = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Data%20Import")
            .body_matches(Regex::new(r#""reference_doctype":"Customer""#).expect("regex"));
        then.status(200)
            .json_body(json!({ "data": { "name": "IMP-0001" } }));
    });
    let upload = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/upload_file")
            .body_contains("IMP-0001");
        then.status(200)
            .json_body(json!({ "message": { "file_url": "/private/files/batch.csv" } }));
    });
    let bind = server.mock(|when, then| {
        when.method(PUT)
            .path("/api/v2/document/Data%20Import/IMP-0001")
            .body_matches(
                Regex::new(r#""import_file":"/private/files/batch.csv""#).expect("regex"),
            );
        then.status(200)
            .json_body(json!({ "data": { "name": "IMP-0001" } }));
    });
    let preview = server.mock(|when, then| {
        when.method(POST)
            .path(
                "/api/method/frappe.core.doctype.data_import.data_import.get_preview_from_template",
            )
            .body_matches(Regex::new(r#""data_import":"IMP-0001""#).expect("regex"));
        then.status(200)
            .json_body(json!({ "message": { "columns": [], "data": [["Import Proof BV"]] } }));
    });

    let dir = std::env::temp_dir().join(format!("erpnext-import-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let file = dir.join("batch.csv");
    std::fs::write(&file, b"customer_name\nImport Proof BV\n").expect("write");

    let client = Client::new(context_for(&server)).expect("client");
    let exit =
        commands::import::preview(&client, file.to_str().expect("utf8 path"), "Customer").await;

    assert_eq!(exit, 0);
    create.assert();
    upload.assert();
    bind.assert();
    preview.assert();
    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn import_apply_starts_exactly_the_named_batch() {
    let server = MockServer::start_async().await;
    let start = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.core.doctype.data_import.data_import.form_start_import")
            .body_matches(Regex::new(r#""data_import":"IMP-0001""#).expect("regex"));
        then.status(200).json_body(json!({ "message": true }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Data%20Import/IMP-0001");
        then.status(200)
            .json_body(json!({ "data": { "status": "Success" } }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::import::apply(&client, "IMP-0001").await;

    assert_eq!(exit, 0);
    start.assert();
}

/// Two agents share one home directory, so a persisted selection is shared state. An
/// explicit company makes a single invocation self-contained.
#[test]
fn an_explicit_company_resolves_without_the_persisted_selection() {
    use erpnext::config::{resolve_override, ConfigError, Paths, PrivilegeMode};
    use std::fs;

    let base = std::env::temp_dir().join(format!("erpnext-override-{}", std::process::id()));
    let registry = base.join("companies");
    fs::create_dir_all(registry.join("other-co")).expect("registry");
    fs::write(
        registry.join("other-co").join("erpnext.json"),
        r#"{"site_url":"https://other.erpnext.example","company":"Other","operator":{"api_key":"k","api_secret":"s"}}"#,
    )
    .expect("credential file");
    let paths = Paths {
        selection_file: base.join("config").join("context.json"),
        registry_root: registry,
    };

    let ctx = resolve_override(&paths, None, Some("other-co"), PrivilegeMode::Operator)
        .expect("explicit company resolves");
    assert_eq!(ctx.company_slug, "other-co");
    assert_eq!(ctx.site(), "other.erpnext.example");
    assert_eq!(ctx.privilege, PrivilegeMode::Operator);
    assert!(
        !paths.selection_file.exists(),
        "an override must not write the shared selection"
    );

    let err = resolve_override(&paths, None, None, PrivilegeMode::Operator)
        .expect_err("no selector is a blocker");
    assert!(matches!(err, ConfigError::NoSelection));

    fs::remove_dir_all(&base).ok();
}

/// Commands that hand a site response straight to the caller emit the site's payload,
/// not its envelope, so every command's `data` carries the result at one depth.
#[test]
fn site_responses_are_emitted_without_the_vendor_envelope() {
    let document = json!({ "name": "ACC-SINV-2026-00001", "status": "Paid" });

    let unwrapped = erpnext::output::site_payload(json!({ "data": document.clone() }));
    assert_eq!(
        unwrapped, document,
        "the site's data wrapper is removed before the CLI envelope is applied"
    );

    let method_answer = json!({ "message": "queued" });
    assert_eq!(
        erpnext::output::site_payload(method_answer.clone()),
        method_answer,
        "a response carrying no data wrapper is emitted as it stands"
    );
}

/// A preview is read by the person who asked for the work, not only by the agent that
/// composed it. It states the record in business language and the consequence that
/// outlives the action, so a misread instruction is caught before anything is sent.
#[test]
fn preview_restates_the_action_in_business_language() {
    use erpnext::summary::{describe, Action};

    let payload = json!({ "customer_name": "Sunfield BV", "grand_total": 2500.0 });
    let create = describe(
        &Action::Create {
            doctype: "Sales Invoice",
            payload: &payload,
        },
        Some("Instrukt AI Sandbox"),
    );
    assert!(create.contains("Sunfield BV"), "names the record: {create}");
    assert!(create.contains("2500.00"), "names the money: {create}");
    assert!(
        create.contains("Instrukt AI Sandbox"),
        "names whose books: {create}"
    );
    assert!(
        !create.contains("approve"),
        "routine previews do not manufacture an approval boundary: {create}"
    );

    let submit = describe(
        &Action::Submit {
            doctype: "Sales Invoice",
            name: "ACC-SINV-2026-00001",
        },
        None,
    );
    assert!(
        submit.contains("cancelled") && submit.contains("never edited"),
        "submitting states its lasting consequence: {submit}"
    );

    let delete = describe(
        &Action::Delete {
            doctype: "Customer",
            name: "Acme Pilot BV",
        },
        None,
    );
    assert!(
        delete.contains("not a cancellation"),
        "deleting is distinguished from cancelling: {delete}"
    );

    let convert = describe(
        &Action::Convert {
            source_doctype: "Sales Order",
            source_name: "SAL-ORD-2026-00001",
            target_doctype: "Sales Invoice",
        },
        None,
    );
    assert!(
        convert.contains("second document"),
        "converting warns about double conversion: {convert}"
    );
}

/// One read of a business document answers both what happened and what is still owed.
/// The site names those differently from the CLI: follow-ups arrive as assignments,
/// a communication names its actor as the sender, and an open assignment carries no
/// timestamp — so a bounded read must not drop it.
#[test]
fn timeline_folds_contacts_and_open_follow_ups_into_one_trail() {
    use erpnext::commands::activity::timeline;

    let docinfo = json!({ "docinfo": {
        "comments": [],
        "communications": [ {
            "name": "COMM-1",
            "content": "Called about the solar install quote.",
            "sender": "agent@instrukt.ai",
            "creation": "2026-07-22 23:13:29",
        } ],
        "assignments": [ {
            "name": "TODO-1",
            "description": "Book the site survey",
            "owner": "agent@instrukt.ai",
            "status": "Open",
        } ],
    }});

    let all = timeline(&docinfo, None, None);
    assert_eq!(all.len(), 2, "contact and follow-up both appear: {all:?}");

    let follow_up = all
        .iter()
        .find(|r| r.pointer("/kind").and_then(Value::as_str) == Some("Follow-up"))
        .expect("the assignment is folded in as a follow-up");
    assert_eq!(
        follow_up.pointer("/status").and_then(Value::as_str),
        Some("Open"),
        "an outstanding commitment reads as open"
    );

    let contact = all
        .iter()
        .find(|r| r.pointer("/kind").and_then(Value::as_str) == Some("Communication"))
        .expect("the communication is folded in");
    assert_eq!(
        contact.pointer("/actor").and_then(Value::as_str),
        Some("agent@instrukt.ai"),
        "a communication names its actor from the sender"
    );

    // A window that excludes the contact must still surface the untimestamped commitment.
    let bounded = timeline(&docinfo, Some("2027-01-01"), None);
    assert_eq!(
        bounded.len(),
        1,
        "the open follow-up survives a bounded read: {bounded:?}"
    );
    assert_eq!(
        bounded[0].pointer("/kind").and_then(Value::as_str),
        Some("Follow-up")
    );
}

/// Closing a commitment is only closed once the site says so. A site that accepts the
/// change while leaving the loop open is a silent decline, and reporting it as success
/// drops the very commitment the outstanding view exists to keep.
#[tokio::test]
async fn closing_a_loop_fails_when_the_stored_status_never_changes() {
    let server = MockServer::start_async().await;
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/ToDo/TODO-1");
        then.status(200).json_body(json!({ "data": {
            "name": "TODO-1",
            "status": "Open",
            "reference_type": "Customer",
            "reference_name": "CUST-1",
        }}));
    });
    let put = server.mock(|when, then| {
        when.method(PUT).path("/api/v2/document/ToDo/TODO-1");
        then.status(200)
            .json_body(json!({ "data": { "status": "Open" } }));
    });
    let note = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.desk.form.utils.add_comment");
        then.status(200)
            .json_body(json!({ "message": { "name": "C-1" } }));
    });

    let client = Client::new(context_for(&server)).expect("client");
    let exit = commands::activity::close_loop(&client, "TODO-1", Some("done")).await;

    assert_eq!(exit, 1, "an unchanged status is never reported as closed");
    put.assert();
    assert_eq!(
        note.hits(),
        0,
        "no note is recorded for a commitment that did not close"
    );
}
