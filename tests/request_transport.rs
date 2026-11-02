use httpmock::prelude::*;
use httpmock::{Mock, Then};
use serde_json::{json, Value};
use std::fs;

#[path = "support/cli.rs"]
mod cli;

fn redirect_failure_envelope(output: std::process::Output) -> Value {
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(
        !String::from_utf8_lossy(&output.stderr).contains("test-secret"),
        "redirect errors must not expose credentials"
    );
    cli::json_output(&output.stderr)
}

fn redirect_target(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(GET).path("/redirect-target");
        then.status(200).json_body(json!({ "message": "followed" }));
    })
}

fn redirect_response(then: Then) -> Then {
    then.status(301)
        .header("Location", "/redirect-target?api_secret=test-secret")
}

fn mock_metadata(server: &MockServer) {
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200).json_body(json!({ "data": { "fields": [
            { "fieldname": "customer_name", "fieldtype": "Data" }
        ]}}));
    });
}

fn mock_customer(server: &MockServer) {
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Customer/C-1");
        then.status(200).json_body(json!({ "data": {
            "doctype": "Customer", "name": "C-1", "customer_name": "Existing"
        }}));
    });
    mock_metadata(server);
}

#[test]
// spec: project/spec/feature/erp-operations/request-transport UC-TR1
fn post_does_not_follow_a_redirect_from_document_create() {
    let server = MockServer::start();
    let target = redirect_target(&server);
    mock_metadata(&server);
    server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Customer");
        redirect_response(then);
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "doc",
            "create",
            "Customer",
            "--data",
            "{\"customer_name\":\"A\"}",
            "--apply",
        ],
    );

    insta::assert_json_snapshot!(redirect_failure_envelope(output), @r#"
    {
      "error": {
        "kind": "redirect",
        "message": "site redirected mutating request (HTTP 301) to /redirect-target"
      },
      "ok": false
    }
    "#);
    assert_eq!(target.hits(), 0);
}

#[test]
// spec: project/spec/feature/erp-operations/request-transport UC-TR1
fn put_does_not_follow_a_redirect_from_document_update() {
    let server = MockServer::start();
    let target = redirect_target(&server);
    mock_customer(&server);
    server.mock(|when, then| {
        when.method(PUT).path("/api/v2/document/Customer/C-1");
        redirect_response(then);
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "doc",
            "update",
            "Customer",
            "C-1",
            "--data",
            "{\"customer_name\":\"New\"}",
            "--apply",
        ],
    );

    insta::assert_json_snapshot!(redirect_failure_envelope(output), @r#"
    {
      "error": {
        "kind": "redirect",
        "message": "site redirected mutating request (HTTP 301) to /redirect-target"
      },
      "ok": false
    }
    "#);
    assert_eq!(target.hits(), 0);
}

#[test]
// spec: project/spec/feature/erp-operations/request-transport UC-TR1
fn delete_does_not_follow_a_redirect_from_document_delete() {
    let server = MockServer::start();
    let target = redirect_target(&server);
    mock_customer(&server);
    server.mock(|when, then| {
        when.method(DELETE).path("/api/v2/document/Customer/C-1");
        redirect_response(then);
    });

    let output = cli::run_cli_against(&server, &["doc", "delete", "Customer", "C-1", "--apply"]);

    insta::assert_json_snapshot!(redirect_failure_envelope(output), @r#"
    {
      "error": {
        "kind": "redirect",
        "message": "site redirected mutating request (HTTP 301) to /redirect-target"
      },
      "ok": false
    }
    "#);
    assert_eq!(target.hits(), 0);
}

#[test]
// spec: project/spec/feature/erp-operations/request-transport UC-TR1
fn upload_does_not_follow_a_redirect_from_evidence_attach() {
    let server = MockServer::start();
    let target = redirect_target(&server);
    let file_path = std::env::temp_dir().join(format!(
        "erpnext-redirect-evidence-{}.txt",
        std::process::id()
    ));
    fs::write(&file_path, "evidence").expect("write evidence fixture");
    server.mock(|when, then| {
        when.method(POST).path("/api/method/upload_file");
        redirect_response(then);
    });

    let file = file_path.to_string_lossy().into_owned();
    let output = cli::run_cli_against(
        &server,
        &[
            "evidence",
            "attach",
            "--reference",
            "Customer",
            "C-1",
            "--file",
            &file,
        ],
    );
    fs::remove_file(file_path).expect("remove evidence fixture");

    insta::assert_json_snapshot!(redirect_failure_envelope(output), @r#"
    {
      "error": {
        "kind": "redirect",
        "message": "site redirected mutating request (HTTP 301) to /redirect-target"
      },
      "ok": false
    }
    "#);
    assert_eq!(target.hits(), 0);
}

#[test]
// spec: project/spec/feature/erp-operations/request-transport UC-TR1
fn timeout_bounded_post_does_not_follow_a_redirect_from_report_run() {
    let server = MockServer::start();
    let target = redirect_target(&server);
    server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.desk.query_report.run");
        redirect_response(then);
    });

    let output = cli::run_cli_against(&server, &["report", "run", "General Ledger"]);

    insta::assert_json_snapshot!(redirect_failure_envelope(output), @r#"
    {
      "error": {
        "kind": "redirect",
        "message": "site redirected mutating request (HTTP 301) to /redirect-target"
      },
      "ok": false
    }
    "#);
    assert_eq!(target.hits(), 0);
}
