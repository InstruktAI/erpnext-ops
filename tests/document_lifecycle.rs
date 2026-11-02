//! Compiled-boundary bindings for the controller lifecycle contract.
//! Every test enters through the real CLI; HTTP is the only fake.

use httpmock::prelude::*;
use serde_json::{json, Value};

#[path = "support/cli.rs"]
mod cli;

fn ordered_workflow_server(
    initial: Value,
    method_response: Value,
    readback_status: u16,
    readback: Value,
) -> cli::OrderedResponseServer {
    let document_path = "/api/v2/document/Bank%20Transaction/BT-TEST-1";
    cli::OrderedResponseServer::start(vec![
        cli::OrderedResponse::new("GET", document_path, 200, json!({ "data": initial })),
        cli::OrderedResponse::new(
            "GET",
            "/api/v2/doctype/Bank%20Transaction/meta",
            200,
            json!({ "data": { "fields": [] } }),
        ),
        cli::OrderedResponse::new(
            "POST",
            "/api/v2/document/Bank%20Transaction/BT-TEST-1/method/remove_payment_entries",
            200,
            method_response,
        ),
        cli::OrderedResponse::new("GET", document_path, readback_status, readback),
    ])
}

#[tokio::test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
async fn workflow_method_persists_the_native_controller_result() {
    let server = MockServer::start_async().await;
    let document = server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Sales%20Invoice/ACC-SINV-2026-00002");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1 } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Sales%20Invoice/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });
    let method = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Sales%20Invoice/ACC-SINV-2026-00002/method/add_timesheet_data")
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

    let output = cli::run_cli_against(
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
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(envelope.pointer("/ok"), Some(&json!(true)));
    assert_eq!(envelope.pointer("/data/stored_only"), Some(&json!(true)));
    assert_eq!(
        envelope.pointer("/data/controller_only"),
        None,
        "the command reports the stored PUT result, not the controller response"
    );
    assert_eq!(
        document.hits(),
        2,
        "initial and post-call reads are required"
    );
    method.assert();
    save.assert();
}

#[tokio::test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
async fn workflow_method_rejects_a_different_returned_document_without_saving() {
    let server = MockServer::start_async().await;
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Sales%20Invoice/INV-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1 } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Sales%20Invoice/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Sales%20Invoice/INV-1/method/add_timesheet_data");
        then.status(200).json_body(json!({ "docs": [{
            "doctype": "Sales Invoice", "name": "INV-OTHER"
        }] }));
    });
    let save = server.mock(|when, then| {
        when.method(PUT)
            .path_matches(Regex::new("^/api/v2/document/.*").expect("regex"));
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(
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
    let envelope = cli::json_output(&output.stderr);
    assert_eq!(save.hits(), 0, "a mismatched document is never saved");
    insta::assert_json_snapshot!(envelope, @r#"
    {
      "error": {
        "kind": "shape",
        "message": "controller method returned Sales Invoice INV-OTHER, expected Sales Invoice INV-1"
      },
      "ok": false
    }
    "#);
}

#[tokio::test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
async fn workflow_method_reports_rejected_persistence_without_controller_document() {
    let server = MockServer::start_async().await;
    let method_result = json!({ "docs": [{
        "doctype": "Sales Invoice",
        "name": "INV-1",
        "timesheets": [{ "time_sheet": "TS-1" }]
    }] });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Sales%20Invoice/INV-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1 } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Sales%20Invoice/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });
    let method = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Sales%20Invoice/INV-1/method/add_timesheet_data");
        then.status(200).json_body(method_result.clone());
    });
    let save = server.mock(|when, then| {
        when.method(PUT)
            .path("/api/v2/document/Sales%20Invoice/INV-1");
        then.status(417)
            .json_body(json!({ "message": "save rejected" }));
    });

    let output = cli::run_cli_against(
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
    let envelope = cli::json_output(&output.stderr);
    assert_eq!(
        envelope.get("data"),
        None,
        "failure carries no controller document"
    );
    method.assert();
    save.assert();
    insta::assert_json_snapshot!(envelope, @r#"
    {
      "error": {
        "kind": "rejected",
        "message": "controller result persistence was rejected: site rejected the request (HTTP 417): save rejected"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
fn workflow_method_skips_write_when_the_controller_effect_is_stored() {
    let stored = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Reconciled",
        "allocations": [{ "payment": "PAY-1" }]
    });
    let initial = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Unreconciled",
        "allocations": []
    });
    let returned = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Reconciled",
        "allocations": [{ "payment": "PAY-1" }],
        "controller_only": true
    });
    let server = ordered_workflow_server(
        initial,
        json!({ "docs": [returned] }),
        200,
        json!({ "data": stored }),
    );

    let output = cli::run_cli_at(
        &server.base_url(),
        &[
            "workflow",
            "method",
            "Bank Transaction",
            "BT-TEST-1",
            "remove_payment_entries",
            "--apply",
        ],
    );
    let requests = server.finish();

    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(envelope.pointer("/ok"), Some(&json!(true)));
    insta::assert_json_snapshot!(json!({
        "allocations": envelope.pointer("/data/allocations"),
    }), @r#"
    {
      "allocations": [
        {
          "payment": "PAY-1"
        }
      ]
    }
    "#);
    assert_eq!(
        requests.len(),
        4,
        "controller persistence must not be followed by PUT"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
fn workflow_method_skips_write_when_returned_stored_and_initial_documents_match() {
    let document = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Reconciled",
        "allocations": [{ "payment": "PAY-1" }]
    });
    let server = ordered_workflow_server(
        document.clone(),
        json!({ "docs": [document.clone()] }),
        200,
        json!({ "data": document }),
    );

    let output = cli::run_cli_at(
        &server.base_url(),
        &[
            "workflow",
            "method",
            "Bank Transaction",
            "BT-TEST-1",
            "remove_payment_entries",
            "--apply",
        ],
    );
    let requests = server.finish();

    assert!(output.status.success(), "{output:?}");
    insta::assert_json_snapshot!(cli::json_output(&output.stdout), @r#"
    {
      "data": {
        "allocations": [
          {
            "payment": "PAY-1"
          }
        ],
        "docstatus": 1,
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "status": "Reconciled"
      },
      "ok": true
    }
    "#);
    assert!(
        requests.iter().all(|request| request.method != "PUT"),
        "a controller result equal to the stored and pre-call documents needs no write"
    );
    assert_eq!(
        requests.len(),
        4,
        "pre-call reads, controller call, read-back"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
fn workflow_method_fails_when_stored_state_changed_but_result_is_not_present() {
    let initial = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Unreconciled",
        "allocations": [{ "payment": "PAY-1" }]
    });
    let returned = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "allocations": []
    });
    let stored = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Reconciled",
        "allocations": [{ "payment": "PAY-1" }],
        "note": "concurrent change"
    });
    let server = ordered_workflow_server(
        initial,
        json!({ "docs": [returned] }),
        200,
        json!({ "data": stored }),
    );

    let output = cli::run_cli_at(
        &server.base_url(),
        &[
            "workflow",
            "method",
            "Bank Transaction",
            "BT-TEST-1",
            "remove_payment_entries",
            "--apply",
        ],
    );
    let requests = server.finish();

    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stdout.is_empty(),
        "failure writes no success envelope"
    );
    assert_eq!(requests.len(), 4, "undetermined state must not be written");
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "undetermined-persistence",
        "message": "controller persistence could not be determined from stored state"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
fn workflow_method_fails_without_write_when_read_back_is_unavailable() {
    let initial = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1
    });
    let returned = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Reconciled"
    });
    let server = ordered_workflow_server(
        initial,
        json!({ "docs": [returned] }),
        403,
        json!({ "message": "denied" }),
    );

    let output = cli::run_cli_at(
        &server.base_url(),
        &[
            "workflow",
            "method",
            "Bank Transaction",
            "BT-TEST-1",
            "remove_payment_entries",
            "--apply",
        ],
    );
    let requests = server.finish();

    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stdout.is_empty(),
        "failure writes no success envelope"
    );
    assert_eq!(requests.len(), 4, "unreadable state must not be written");
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "permission",
        "message": "stored document read-back failed: permission denied by site (HTTP 403): denied"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
fn workflow_method_without_returned_document_fails_without_reporting_stored_state() {
    let stored = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Reconciled"
    });
    let initial = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1,
        "status": "Unreconciled"
    });
    let server = ordered_workflow_server(
        initial,
        json!({ "message": null }),
        200,
        json!({ "data": stored }),
    );

    let output = cli::run_cli_at(
        &server.base_url(),
        &[
            "workflow",
            "method",
            "Bank Transaction",
            "BT-TEST-1",
            "remove_payment_entries",
            "--apply",
        ],
    );
    let requests = server.finish();

    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let envelope = cli::json_output(&output.stderr);
    insta::assert_json_snapshot!(envelope, @r#"
    {
      "error": {
        "kind": "shape",
        "message": "controller method response did not contain a document"
      },
      "ok": false
    }
    "#);
    assert_eq!(
        requests.len(),
        3,
        "the response shape fails before read-back"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-7
fn workflow_method_fails_when_successful_read_back_has_no_document_payload() {
    let initial = json!({
        "doctype": "Bank Transaction",
        "name": "BT-TEST-1",
        "docstatus": 1
    });
    let server = ordered_workflow_server(
        initial,
        json!({ "docs": [{
            "doctype": "Bank Transaction", "name": "BT-TEST-1"
        }]}),
        200,
        json!({ "message": "unexpected response shape" }),
    );

    let output = cli::run_cli_at(
        &server.base_url(),
        &[
            "workflow",
            "method",
            "Bank Transaction",
            "BT-TEST-1",
            "remove_payment_entries",
            "--apply",
        ],
    );
    let requests = server.finish();

    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stdout.is_empty(),
        "failure writes no success envelope"
    );
    assert_eq!(
        requests.len(),
        4,
        "an unreadable result must not be written"
    );
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "shape",
        "message": "stored document read-back failed: unexpected response shape: post-call read-back did not contain a document object"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/document-lifecycle UC-8
fn workflow_method_redirect_fails_without_readback_or_save() {
    let server = MockServer::start();
    let target = server.mock(|when, then| {
        when.method(GET).path("/redirect-target");
        then.status(200).json_body(json!({ "message": "followed" }));
    });
    let method = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Bank%20Transaction/BT-1/method/remove_payment_entries");
        then.status(301)
            .header("Location", "/redirect-target?api_secret=test-secret");
    });
    let read = server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Bank%20Transaction/BT-1");
        then.status(200).json_body(json!({ "data": {
            "doctype": "Bank Transaction", "name": "BT-1", "docstatus": 1
        }}));
    });
    let save = server.mock(|when, then| {
        when.method(PUT)
            .path("/api/v2/document/Bank%20Transaction/BT-1");
        then.status(200).json_body(json!({}));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Bank%20Transaction/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] }}));
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "workflow",
            "method",
            "Bank Transaction",
            "BT-1",
            "remove_payment_entries",
            "--apply",
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let envelope = cli::json_output(&output.stderr);
    insta::assert_json_snapshot!(envelope, @r#"
    {
      "error": {
        "kind": "redirect",
        "message": "controller method request failed: site redirected mutating request (HTTP 301) to /redirect-target"
      },
      "ok": false
    }
    "#);
    assert!(!String::from_utf8_lossy(&output.stderr).contains("test-secret"));
    assert_eq!(method.hits(), 1);
    assert_eq!(target.hits(), 0);
    assert_eq!(read.hits(), 1, "only the pre-call read may occur");
    assert_eq!(save.hits(), 0);
}
