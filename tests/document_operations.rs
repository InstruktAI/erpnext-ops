use httpmock::prelude::*;
use serde_json::{json, Value};

#[path = "support/cli.rs"]
mod cli;

fn mapper_path() -> &'static str {
    "/api/method/erpnext.accounts.doctype.payment_entry.payment_entry.get_payment_entry"
}

fn mapper_result() -> Value {
    json!({
        "doctype": "Payment Entry",
        "references": [{ "reference_name": "ACC-SINV-2026-00001" }],
        "discount_eligible": true,
    })
}

fn mapper_request(extra: Value) -> Value {
    let mut body = extra.as_object().expect("object fixture").clone();
    body.insert("source_name".into(), json!("ACC-SINV-2026-00001"));
    body.insert("dt".into(), json!("Sales Invoice"));
    body.insert("dn".into(), json!("ACC-SINV-2026-00001"));
    Value::Object(body)
}

fn map_invocation<'a>(args: Option<&'a str>, data: Option<&'a str>, apply: bool) -> Vec<&'a str> {
    let mut invocation = vec![
        "doc",
        "map",
        "Sales Invoice",
        "ACC-SINV-2026-00001",
        "--via",
        "erpnext.accounts.doctype.payment_entry.payment_entry.get_payment_entry",
    ];
    if let Some(args) = args {
        invocation.extend(["--args", args]);
    }
    if let Some(data) = data {
        invocation.extend(["--data", data]);
    }
    if apply {
        invocation.push("--apply");
    }
    invocation
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-MAP1
fn mapper_inputs_reach_the_native_mapper_before_target_construction() {
    let server = MockServer::start();
    let mapper = server.mock(|when, then| {
        when.method(POST)
            .path(mapper_path())
            .json_body(mapper_request(json!({ "reference_date": "2026-09-01" })));
        then.status(200)
            .json_body(json!({ "message": mapper_result() }));
    });

    let invocation = map_invocation(Some(r#"{"reference_date":"2026-09-01"}"#), None, false);
    let output = cli::run_cli_against(&server, &invocation);

    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        cli::json_output(&output.stdout).pointer("/data/operation/payload/discount_eligible"),
        Some(&json!(true))
    );
    mapper.assert();
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-MAP2
fn target_overrides_apply_after_mapping_without_entering_mapper_inputs() {
    let server = MockServer::start();
    let mapper = server.mock(|when, then| {
        when.method(POST)
            .path(mapper_path())
            .json_body(mapper_request(json!({ "reference_date": "native" })));
        then.status(200).json_body(json!({ "message": {
            "doctype": "Payment Entry",
            "reference_date": "mapped",
        }}));
    });

    let override_data = r#"{"reference_date":"override","posting_date":"2026-09-03"}"#;
    let overrides: Value = serde_json::from_str(override_data).expect("target override fixture");
    let invocation = map_invocation(
        Some(r#"{"reference_date":"native"}"#),
        Some(override_data),
        false,
    );
    let output = cli::run_cli_against(&server, &invocation);

    assert!(output.status.success(), "{output:?}");
    let payload = cli::json_output(&output.stdout);
    assert_eq!(
        payload.pointer("/data/operation/payload/reference_date"),
        overrides.pointer("/reference_date")
    );
    assert_eq!(
        payload.pointer("/data/operation/payload/posting_date"),
        overrides.pointer("/posting_date")
    );
    mapper.assert();
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-MAP3
fn mapper_inputs_cannot_replace_the_selected_source_identity() {
    let server = MockServer::start();
    let mapper = server.mock(|when, then| {
        when.method(POST)
            .path(mapper_path())
            .json_body(mapper_request(json!({ "reference_date": "2026-09-01" })));
        then.status(200)
            .json_body(json!({ "message": mapper_result() }));
    });

    let invocation = map_invocation(
        Some(
            r#"{"reference_date":"2026-09-01","source_name":"other","dt":"Other","dn":"OTHER-1"}"#,
        ),
        None,
        false,
    );
    let output = cli::run_cli_against(&server, &invocation);

    assert!(output.status.success(), "{output:?}");
    mapper.assert();
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-MAP4
fn malformed_mapper_arguments_fail_before_any_http_request() {
    let server = MockServer::start();
    let requests = server.mock(|when, then| {
        when.path_matches(Regex::new(".*").expect("request matcher"));
        then.status(200).json_body(json!({}));
    });

    let invocation = map_invocation(Some("{"), None, false);
    let output = cli::run_cli_against(&server, &invocation);

    assert_eq!(output.status.code(), Some(2));
    let error = cli::json_output(&output.stderr);
    assert_eq!(error.pointer("/ok"), Some(&json!(false)));
    assert!(error
        .pointer("/error/kind")
        .and_then(Value::as_str)
        .is_some_and(|kind| !kind.is_empty()));
    assert!(error
        .pointer("/error/message")
        .and_then(Value::as_str)
        .is_some_and(|message| !message.is_empty()));
    assert_eq!(requests.hits(), 0);
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-MAP4
fn non_object_mapper_arguments_fail_before_any_http_request() {
    let server = MockServer::start();
    let requests = server.mock(|when, then| {
        when.path_matches(Regex::new(".*").expect("request matcher"));
        then.status(200).json_body(json!({}));
    });

    let invocation = map_invocation(Some("[]"), None, false);
    let output = cli::run_cli_against(&server, &invocation);

    assert_eq!(output.status.code(), Some(2));
    let error = cli::json_output(&output.stderr);
    assert_eq!(error.pointer("/ok"), Some(&json!(false)));
    assert!(error
        .pointer("/error/kind")
        .and_then(Value::as_str)
        .is_some_and(|kind| !kind.is_empty()));
    assert!(error
        .pointer("/error/message")
        .and_then(Value::as_str)
        .is_some_and(|message| !message.is_empty()));
    assert_eq!(requests.hits(), 0);
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-MAP5
fn preview_and_apply_use_the_same_post_override_target() {
    let server = MockServer::start();
    let expected_mapper_request = mapper_request(json!({ "reference_date": "2026-09-01" }));
    let mapper = server.mock(|when, then| {
        when.method(POST)
            .path(mapper_path())
            .json_body(expected_mapper_request);
        then.status(200)
            .json_body(json!({ "message": mapper_result() }));
    });
    let override_data = r#"{"posting_date":"2026-09-03"}"#;
    let overrides: serde_json::Map<String, Value> =
        serde_json::from_str(override_data).expect("target override fixture");
    let preview = map_invocation(
        Some(r#"{"reference_date":"2026-09-01"}"#),
        Some(override_data),
        false,
    );
    let preview_output = cli::run_cli_against(&server, &preview);
    assert!(preview_output.status.success(), "{preview_output:?}");
    let preview_envelope = cli::json_output(&preview_output.stdout);
    assert_eq!(
        preview_envelope.pointer("/data/preview"),
        Some(&json!(true))
    );
    let preview_target = preview_envelope
        .pointer("/data/operation/payload")
        .cloned()
        .expect("mapping preview supplies its target payload");
    let mut expected_target = mapper_result()
        .as_object()
        .expect("mapper fixture object")
        .clone();
    expected_target.extend(overrides.clone());
    assert_eq!(preview_target, Value::Object(expected_target));

    let create = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Payment%20Entry")
            .json_body(preview_target.clone());
        then.status(200).json_body(json!({ "data": {
            "name": "ACC-PAY-2026-00001",
            "references": [{ "reference_name": "ACC-SINV-2026-00001" }],
        }}));
    });
    assert_eq!(create.hits(), 0);

    let apply = map_invocation(
        Some(r#"{"reference_date":"2026-09-01"}"#),
        Some(override_data),
        true,
    );
    let apply_output = cli::run_cli_against(&server, &apply);
    assert!(apply_output.status.success(), "{apply_output:?}");
    assert_eq!(
        cli::json_output(&apply_output.stdout).pointer("/data/linked_to_source"),
        Some(&json!(true))
    );
    assert_eq!(mapper.hits(), 2);
    create.assert();
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-9
fn omitted_mapper_arguments_preserve_linked_and_unlinked_creation_outcomes() {
    let server = MockServer::start();
    let mapper = server.mock(|when, then| {
        when.method(POST)
            .path(mapper_path())
            .json_body(mapper_request(json!({})));
        then.status(200)
            .json_body(json!({ "message": mapper_result() }));
    });
    let linked_create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Payment%20Entry");
        then.status(200).json_body(json!({ "data": {
            "name": "ACC-PAY-2026-00001",
            "references": [{ "reference_name": "ACC-SINV-2026-00001" }],
        }}));
    });

    let linked = cli::run_cli_against(&server, &map_invocation(None, None, true));
    assert!(linked.status.success(), "{linked:?}");
    assert_eq!(
        cli::json_output(&linked.stdout).pointer("/data/linked_to_source"),
        Some(&json!(true))
    );
    mapper.assert();
    linked_create.assert();

    let unlinked_server = MockServer::start();
    unlinked_server.mock(|when, then| {
        when.method(POST)
            .path(mapper_path())
            .json_body(mapper_request(json!({})));
        then.status(200)
            .json_body(json!({ "message": mapper_result() }));
    });
    unlinked_server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Payment%20Entry");
        then.status(200)
            .json_body(json!({ "data": { "name": "ACC-PAY-UNLINKED" } }));
    });

    let unlinked = cli::run_cli_against(&unlinked_server, &map_invocation(None, None, true));
    assert_eq!(unlinked.status.code(), Some(1));
    let error = cli::json_output(&unlinked.stderr);
    assert_eq!(error.pointer("/ok"), Some(&json!(false)));
    assert!(error
        .pointer("/data/created")
        .and_then(Value::as_str)
        .is_some());
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-9
fn mapper_rejection_is_a_failure_without_target_creation() {
    let server = MockServer::start();
    let mapper = server.mock(|when, then| {
        when.method(POST)
            .path(mapper_path())
            .json_body(mapper_request(json!({})));
        then.status(417)
            .json_body(json!({ "message": "native mapper rejected input" }));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Payment%20Entry");
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(&server, &map_invocation(None, None, true));

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        cli::json_output(&output.stderr).pointer("/ok"),
        Some(&json!(false))
    );
    mapper.assert();
    assert_eq!(create.hits(), 0);
}

#[test]
// spec: project/spec/feature/erp-operations/document-operations UC-9
fn target_creation_rejection_is_a_failure_after_mapping() {
    let server = MockServer::start();
    let mapper = server.mock(|when, then| {
        when.method(POST)
            .path(mapper_path())
            .json_body(mapper_request(json!({})));
        then.status(200)
            .json_body(json!({ "message": mapper_result() }));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Payment%20Entry");
        then.status(417)
            .json_body(json!({ "message": "native creation rejected target" }));
    });

    let output = cli::run_cli_against(&server, &map_invocation(None, None, true));

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        cli::json_output(&output.stderr).pointer("/ok"),
        Some(&json!(false))
    );
    mapper.assert();
    create.assert();
}
