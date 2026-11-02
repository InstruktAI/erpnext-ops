//! Compiled-boundary bindings for `project/spec/feature/erp-operations/module-method-call`.
//! Every test starts the real binary from an isolated credential home; HTTP is the
//! only faked boundary.

use httpmock::prelude::*;
use serde_json::{json, Value};
use std::fs;

#[path = "support/cli.rs"]
mod cli;

const METHOD_PATH: &str = "/api/method/erpnext.accounts.reconcile";
const AUTHORIZATION: &str = "token test-key:test-secret";

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-1
fn preview_sends_nothing_and_replays_the_native_request() {
    let server = MockServer::start();
    let payload = json!({ "transaction": "BANK-0001", "vouchers": [] });
    let request = server.mock(|when, then| {
        when.method(POST)
            .path(METHOD_PATH)
            .header("Authorization", AUTHORIZATION)
            .json_body(payload.clone());
        then.status(200)
            .json_body(json!({ "message": { "accepted": true } }));
    });
    let home = cli::test_home(&server.base_url().replace("127.0.0.1", "localhost"), false);
    let preview = cli::run_cli_from_home(
        &home,
        &[
            "method",
            "call",
            "erpnext.accounts.reconcile",
            "--data",
            &payload.to_string(),
        ],
    );

    assert!(preview.status.success(), "{preview:?}");
    assert_eq!(request.hits(), 0, "preview must not send a native request");
    let envelope = cli::json_output(&preview.stdout);
    let data = envelope.pointer("/data").cloned().expect("preview data");
    insta::assert_json_snapshot!(data, @r#"
    {
      "apply_command": "'erpnext' '--company' 'test-co' '--as' 'operator' 'method' 'call' 'erpnext.accounts.reconcile' '--data' '{\"transaction\":\"BANK-0001\",\"vouchers\":[]}' '--apply'",
      "context": {
        "company": "Test Company",
        "identity": {
          "authenticated_user": null,
          "basis": "configuration",
          "configured_user": "operator@instrukt.ai"
        },
        "privilege": "operator",
        "site": "localhost"
      },
      "operation": {
        "method": "POST",
        "path": "/api/method/erpnext.accounts.reconcile",
        "payload": {
          "transaction": "BANK-0001",
          "vouchers": []
        }
      },
      "preflight": {
        "checked": false,
        "reason": "method arguments are opaque; no document-field claim is made"
      },
      "preview": true
    }
    "#);

    let apply_command = data
        .pointer("/apply_command")
        .and_then(Value::as_str)
        .expect("apply command")
        .to_owned();
    let replay = cli::run_replay(&home, &apply_command);
    fs::remove_dir_all(&home).expect("remove test home");

    assert!(replay.status.success(), "{replay:?}");
    assert_eq!(request.hits(), 1, "replay sends exactly one native request");
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-2
fn apply_forwards_the_opaque_object_and_unwraps_the_message() {
    let server = MockServer::start();
    let payload = json!({ "transaction": "BANK-0001", "vouchers": [] });
    let request = server.mock(|when, then| {
        when.method(POST)
            .path(METHOD_PATH)
            .header("Authorization", AUTHORIZATION)
            .json_body(payload.clone());
        then.status(200)
            .json_body(json!({ "message": { "accepted": true } }));
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "method",
            "call",
            "erpnext.accounts.reconcile",
            "--data",
            &payload.to_string(),
            "--apply",
        ],
    );

    assert!(output.status.success(), "{output:?}");
    assert_eq!(request.hits(), 1, "apply sends exactly one native request");
    insta::assert_json_snapshot!(cli::json_output(&output.stdout), @r#"
    {
      "data": {
        "accepted": true
      },
      "ok": true
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-2
fn apply_reports_null_when_the_method_returns_nothing() {
    let server = MockServer::start();
    let request = server.mock(|when, then| {
        when.method(POST).path(METHOD_PATH);
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(
        &server,
        &["method", "call", "erpnext.accounts.reconcile", "--apply"],
    );

    assert!(output.status.success(), "{output:?}");
    assert_eq!(request.hits(), 1, "apply sends exactly one native request");
    insta::assert_json_snapshot!(cli::json_output(&output.stdout), @r#"
    {
      "data": null,
      "ok": true
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-3
fn malformed_data_fails_before_any_request() {
    let server = MockServer::start();
    let request = server.mock(|when, then| {
        when.method(POST).path(METHOD_PATH);
        then.status(200).json_body(json!({}));
    });
    let output = cli::run_cli_against(
        &server,
        &[
            "method",
            "call",
            "erpnext.accounts.reconcile",
            "--data",
            "{",
            "--apply",
        ],
    );

    assert!(!output.status.success());
    assert_eq!(request.hits(), 0, "malformed input must not reach the site");
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "payload",
        "message": "--data is not valid JSON: EOF while parsing an object at line 1 column 1"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-3
fn non_object_data_fails_before_any_request() {
    let server = MockServer::start();
    let request = server.mock(|when, then| {
        when.method(POST).path(METHOD_PATH);
        then.status(200).json_body(json!({}));
    });
    let output = cli::run_cli_against(
        &server,
        &[
            "method",
            "call",
            "erpnext.accounts.reconcile",
            "--data",
            "[]",
            "--apply",
        ],
    );

    assert!(!output.status.success());
    assert_eq!(
        request.hits(),
        0,
        "non-object input must not reach the site"
    );
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "payload",
        "message": "--data must be a JSON object"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-4
fn unavailable_method_is_a_typed_failure() {
    let server = MockServer::start();
    let request = server.mock(|when, then| {
        when.method(POST).path(METHOD_PATH);
        then.status(404).json_body(json!({ "message": "missing" }));
    });
    let output = cli::run_cli_against(
        &server,
        &["method", "call", "erpnext.accounts.reconcile", "--apply"],
    );

    assert!(!output.status.success());
    assert!(output.stdout.is_empty(), "failure has no success payload");
    assert_eq!(request.hits(), 1, "the native refusal follows one request");
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "not_found",
        "message": "not found: missing"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-4
fn permission_denial_is_a_typed_failure() {
    let server = MockServer::start();
    let request = server.mock(|when, then| {
        when.method(POST).path(METHOD_PATH);
        then.status(403).json_body(json!({ "message": "denied" }));
    });
    let output = cli::run_cli_against(
        &server,
        &["method", "call", "erpnext.accounts.reconcile", "--apply"],
    );

    assert!(!output.status.success());
    assert!(output.stdout.is_empty(), "failure has no success payload");
    assert_eq!(request.hits(), 1, "the native refusal follows one request");
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "permission",
        "message": "permission denied by site (HTTP 403): denied"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-4
fn site_rejection_is_a_typed_failure() {
    let server = MockServer::start();
    let request = server.mock(|when, then| {
        when.method(POST).path(METHOD_PATH);
        then.status(422).json_body(json!({ "message": "rejected" }));
    });
    let output = cli::run_cli_against(
        &server,
        &["method", "call", "erpnext.accounts.reconcile", "--apply"],
    );

    assert!(!output.status.success());
    assert!(output.stdout.is_empty(), "failure has no success payload");
    assert_eq!(request.hits(), 1, "the native refusal follows one request");
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "rejected",
        "message": "site rejected the request (HTTP 422): rejected"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-5
fn non_object_method_response_is_a_shape_failure() {
    let server = MockServer::start();
    let request = server.mock(|when, then| {
        when.method(POST).path(METHOD_PATH);
        then.status(200).body("not-json");
    });
    let output = cli::run_cli_against(
        &server,
        &["method", "call", "erpnext.accounts.reconcile", "--apply"],
    );

    assert!(!output.status.success());
    assert!(output.stdout.is_empty(), "failure has no success payload");
    assert_eq!(
        request.hits(),
        1,
        "the malformed response follows one request"
    );
    insta::assert_json_snapshot!(cli::json_output(&output.stderr), @r#"
    {
      "error": {
        "kind": "shape",
        "message": "unexpected response shape: method response is not a JSON object"
      },
      "ok": false
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/module-method-call UC-5
fn transport_failure_is_typed_and_secret_safe() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("local listener");
    let address = listener.local_addr().expect("listener address");
    let peer = std::thread::spawn(move || {
        let (stream, _) = listener.accept().expect("accept method request");
        drop(stream);
    });
    let output = cli::run_cli_at(
        &format!("http://localhost:{}", address.port()),
        &["method", "call", "erpnext.accounts.reconcile", "--apply"],
    );
    peer.join().expect("peer closes the accepted connection");

    assert!(!output.status.success());
    assert!(output.stdout.is_empty(), "failure has no success payload");
    assert!(
        !output
            .stderr
            .windows(b"test-key".len())
            .any(|window| window == b"test-key")
            && !output
                .stderr
                .windows(b"test-secret".len())
                .any(|window| window == b"test-secret"),
        "failure output must not expose credentials"
    );
    let failure = cli::json_output(&output.stderr);
    insta::assert_json_snapshot!(json!({
        "ok": failure.get("ok"),
        "kind": failure.pointer("/error/kind"),
    }), @r#"
    {
      "kind": "transport",
      "ok": false
    }
    "#);
}
