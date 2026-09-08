//! Compiled-boundary bindings for `project/spec/feature/erp-operations/mutation-preview`.
//! Every test spawns the real binary from an isolated credential home through
//! `tests/support/cli.rs`; HTTP is the only fake.

use httpmock::prelude::*;
use serde_json::{json, Value};
use std::fs;

#[path = "support/cli.rs"]
mod cli;

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE1
fn update_preview_reports_observed_target() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Customer/CUST-0001");
        then.status(200).json_body(json!({ "data": {
            "docstatus": 1,
            "company": "Other Co",
            "modified": "2026-01-01 10:00:00",
        }}));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [ { "fieldname": "customer_name" } ] } }));
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "doc",
            "update",
            "Customer",
            "CUST-0001",
            "--data",
            r#"{"customer_name":"Acme"}"#,
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    let data = envelope.pointer("/data").expect("preview data").clone();

    insta::assert_json_snapshot!(json!({
        "target": data.pointer("/target"),
        "context_company": data.pointer("/context/company"),
    }), @r#"
    {
      "context_company": "Test Company",
      "target": {
        "company": {
          "observed": true,
          "value": "Other Co"
        },
        "docstatus": {
          "observed": true,
          "value": 1
        },
        "modified": {
          "observed": true,
          "value": "2026-01-01 10:00:00"
        }
      }
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE1
fn delete_preview_observes_without_deleting() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Customer/CUST-0001");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 0, "company": "Test Company", "modified": "2026-01-01" } }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });
    let delete = server.mock(|when, then| {
        when.method(DELETE)
            .path("/api/v2/document/Customer/CUST-0001");
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(&server, &["doc", "delete", "Customer", "CUST-0001"]);
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(envelope.pointer("/data/preview"), Some(&json!(true)));
    assert_eq!(
        envelope.pointer("/data/target/docstatus"),
        Some(&json!({ "observed": true, "value": 0 }))
    );
    assert_eq!(delete.hits(), 0, "preview must not delete");
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE1
fn copy_preview_observes_source_and_template() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Quotation/Q-1");
        then.status(200).json_body(json!({ "data": {
            "docstatus": 1, "company": "Test Company", "modified": "2026-01-01",
            "party_name": "Acme",
        }}));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Quotation/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [ { "fieldname": "party_name" } ] } }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Quotation/Q-1/copy");
        then.status(200)
            .json_body(json!({ "data": { "party_name": "Acme", "docstatus": 0 } }));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Quotation");
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(&server, &["doc", "copy", "Quotation", "Q-1"]);
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(
        envelope.pointer("/data/target/docstatus"),
        Some(&json!({ "observed": true, "value": 1 })),
        "target facts reflect the stored source, not the unborn copy"
    );
    assert_eq!(
        envelope.pointer("/data/operation/payload/docstatus"),
        Some(&json!(0)),
        "the copy payload is the template, kept separate from the source's own facts"
    );
    assert_eq!(create.hits(), 0, "preview must not create");
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE1
fn map_preview_observes_without_creating_target() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Sales%20Invoice/ACC-SINV-1");
        then.status(200).json_body(json!({ "data": {
            "docstatus": 1, "company": "Test Company", "modified": "2026-01-01",
        }}));
    });
    let mapper = server.mock(|when, then| {
        when.method(POST).path(
            "/api/method/erpnext.accounts.doctype.payment_entry.payment_entry.get_payment_entry",
        );
        then.status(200).json_body(
            json!({ "message": { "doctype": "Payment Entry", "reference_date": "2026-01-01" } }),
        );
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Payment%20Entry/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [ { "fieldname": "reference_date" } ] } }));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Payment%20Entry");
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "doc",
            "map",
            "Sales Invoice",
            "ACC-SINV-1",
            "--via",
            "erpnext.accounts.doctype.payment_entry.payment_entry.get_payment_entry",
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(
        envelope.pointer("/data/target/docstatus"),
        Some(&json!({ "observed": true, "value": 1 }))
    );
    assert_eq!(
        envelope.pointer("/data/schema/is_submittable"),
        Some(&json!({ "observed": false, "value": null })),
        "destination metadata carries no is_submittable in this fixture, so it stays unknown"
    );
    mapper.assert();
    assert_eq!(
        create.hits(),
        0,
        "preview must not create the mapped target"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE1
fn submit_preview_reads_without_transition() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Sales%20Invoice/INV-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 0, "company": "Test Company", "modified": "2026-01-01" } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Sales%20Invoice/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });
    let put = server.mock(|when, then| {
        when.method(PUT)
            .path("/api/v2/document/Sales%20Invoice/INV-1");
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(&server, &["workflow", "submit", "Sales Invoice", "INV-1"]);
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(
        envelope.pointer("/data/target/docstatus"),
        Some(&json!({ "observed": true, "value": 0 }))
    );
    assert_eq!(
        envelope.pointer("/data/intended_state"),
        Some(&json!("submitted"))
    );
    assert_eq!(put.hits(), 0, "preview must not transition");
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE1
fn cancel_preview_reads_without_transition() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Sales%20Invoice/INV-2");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1, "company": "Test Company", "modified": "2026-01-01" } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Sales%20Invoice/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });
    let put = server.mock(|when, then| {
        when.method(PUT)
            .path("/api/v2/document/Sales%20Invoice/INV-2");
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(&server, &["workflow", "cancel", "Sales Invoice", "INV-2"]);
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(
        envelope.pointer("/data/target/docstatus"),
        Some(&json!({ "observed": true, "value": 1 }))
    );
    assert_eq!(
        envelope.pointer("/data/intended_state"),
        Some(&json!("cancelled"))
    );
    assert_eq!(put.hits(), 0, "preview must not transition");
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE1
fn amend_preview_observes_cancelled_source() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Quotation/Q-1");
        then.status(200).json_body(json!({ "data": {
            "docstatus": 2, "company": "Test Company", "modified": "2026-01-01",
            "party_name": "Acme",
        }}));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Quotation/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [ { "fieldname": "party_name" } ] } }));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Quotation");
        then.status(200).json_body(json!({}));
    });

    let output = cli::run_cli_against(&server, &["workflow", "amend", "Quotation", "Q-1"]);
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(
        envelope.pointer("/data/target/docstatus"),
        Some(&json!({ "observed": true, "value": 2 }))
    );
    assert_eq!(
        envelope.pointer("/data/intended_state"),
        Some(&json!("draft"))
    );
    assert_eq!(
        create.hits(),
        0,
        "preview must not create the amended draft"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE1
fn controller_preview_observes_without_execution() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Sales%20Invoice/INV-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1, "company": "Test Company", "modified": "2026-01-01" } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Sales%20Invoice/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });
    let method = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Sales%20Invoice/INV-1/method/add_timesheet_data/");
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
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(
        envelope.pointer("/data/target/docstatus"),
        Some(&json!({ "observed": true, "value": 1 }))
    );
    assert_eq!(
        method.hits(),
        0,
        "preview must not execute the controller method"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE2
fn document_preview_reports_schema_and_unknown_keys() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Sales%20Invoice/INV-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 0, "company": "Test Company", "modified": "2026-01-01" } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Sales%20Invoice/meta");
        then.status(200).json_body(json!({ "data": {
            "is_submittable": 1,
            "track_changes": 1,
            "fields": [
                { "fieldname": "customer", "allow_on_submit": 0 },
                { "fieldname": "remarks", "allow_on_submit": 1 },
            ],
        }}));
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "doc",
            "update",
            "Sales Invoice",
            "INV-1",
            "--data",
            r#"{"remarks":"noted","owner":"agent@instrukt.ai","bogus_field":1}"#,
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    let data = envelope.pointer("/data").expect("preview data").clone();

    insta::assert_json_snapshot!(json!({
        "schema": data.pointer("/schema"),
        "preflight": data.pointer("/preflight"),
    }), @r#"
    {
      "preflight": {
        "checked": true,
        "checked_keys": [
          "bogus_field",
          "owner",
          "remarks"
        ],
        "doctype": "Sales Invoice",
        "site_validation": "mandatory, link-target, accounting, stock, workflow, and permission validation remain the site's decision",
        "unknown_fields": [
          "bogus_field"
        ]
      },
      "schema": {
        "allow_on_submit_fields": {
          "observed": true,
          "value": [
            "remarks"
          ]
        },
        "is_submittable": {
          "observed": true,
          "value": 1
        },
        "track_changes": {
          "observed": true,
          "value": 1
        }
      }
    }
    "#);
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE2
fn controller_arguments_remain_opaque() {
    let server = MockServer::start();
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
            .json_body(json!({ "data": { "fields": [ { "fieldname": "customer" } ] } }));
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "workflow",
            "method",
            "Sales Invoice",
            "INV-1",
            "add_timesheet_data",
            "--data",
            r#"{"time_sheet":"TS-1"}"#,
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    assert_eq!(
        envelope.pointer("/data/preflight/checked"),
        Some(&json!(false)),
        "method arguments are never checked as document fields"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE3
fn preview_labels_configured_identity_without_credentials() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Customer/CUST-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 0 } }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });

    let output = cli::run_cli_against(&server, &["doc", "delete", "Customer", "CUST-1"]);
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    let context = envelope
        .pointer("/data/context")
        .cloned()
        .expect("preview context");

    insta::assert_json_snapshot!(context, @r#"
    {
      "company": "Test Company",
      "identity": {
        "authenticated_user": null,
        "basis": "configuration",
        "configured_user": "operator@instrukt.ai"
      },
      "privilege": "operator",
      "site": "localhost"
    }
    "#);

    assert!(
        !output
            .stdout
            .windows(b"test-secret".len())
            .any(|window| window == b"test-secret"),
        "preview context never carries credential material"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE4
fn emitted_create_replays_payload_source_and_context() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [ { "fieldname": "customer_name" } ] } }));
    });
    server.mock(|when, then| {
        when.method(GET).path_matches(
            Regex::new("^/api/v2/document/Customer/SRC-[0-9a-f]{20}$").expect("regex"),
        );
        then.status(404)
            .json_body(json!({ "message": "not found" }));
    });
    let tricky_name = "Acme's \"Shop\" $(pwd) `id`\nSecond line";
    let create = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Customer")
            .json_body_partial(json!({ "customer_name": tricky_name }).to_string());
        then.status(200)
            .json_body(json!({ "data": { "name": "SRC-new" } }));
    });

    let home = cli::test_home(&server.base_url().replace("127.0.0.1", "localhost"), false);
    let data = json!({ "customer_name": tricky_name }).to_string();
    let preview = cli::run_cli_from_home(
        &home,
        &[
            "doc",
            "create",
            "Customer",
            "--data",
            &data,
            "--source",
            "customer/acme/tricky",
        ],
    );
    assert!(preview.status.success(), "{preview:?}");
    let apply_command = cli::json_output(&preview.stdout)
        .pointer("/data/apply_command")
        .and_then(Value::as_str)
        .expect("apply_command")
        .to_string();

    let replayed = cli::run_replay(&home, &apply_command);
    fs::remove_dir_all(&home).ok();

    assert!(
        replayed.status.success(),
        "replay: {replayed:?}\ncommand: {apply_command}"
    );
    create.assert();
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE4
fn emitted_map_preserves_arguments_and_overrides() {
    let server = MockServer::start();
    let source = server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Sales%20Invoice/ACC-SINV-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1 } }));
    });
    let mapper = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/erpnext.accounts.doctype.payment_entry.payment_entry.get_payment_entry")
            .json_body_partial(json!({ "reference_date": "2026-01-01" }).to_string());
        then.status(200)
            .json_body(json!({ "message": { "doctype": "Payment Entry", "reference_date": "native" } }));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Payment%20Entry/meta");
        then.status(200).json_body(json!({ "data": { "fields": [
            { "fieldname": "reference_date" }, { "fieldname": "posting_date" },
            { "fieldname": "references", "fieldtype": "Table", "options": "Payment Entry Reference" },
        ]}}));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Payment%20Entry%20Reference/meta");
        then.status(200).json_body(json!({ "data": { "fields": [
            { "fieldname": "reference_doctype", "fieldtype": "Link", "options": "DocType" },
            { "fieldname": "reference_name", "fieldtype": "Dynamic Link", "options": "reference_doctype" },
        ]}}));
    });
    let create = server.mock(|when, then| {
        when.method(POST)
            .path("/api/v2/document/Payment%20Entry")
            .json_body_partial(
                json!({ "reference_date": "native", "posting_date": "2026-09-03" }).to_string(),
            );
        then.status(200)
            .json_body(json!({ "data": { "name": "ACC-PAY-1" } }));
    });
    let read_back = server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/document/Payment%20Entry/ACC-PAY-1");
        then.status(200).json_body(json!({ "data": {
            "doctype": "Payment Entry",
            "name": "ACC-PAY-1",
            "references": [{
                "reference_doctype": "Sales Invoice",
                "reference_name": "ACC-SINV-1",
            }],
        }}));
    });

    let home = cli::test_home(&server.base_url().replace("127.0.0.1", "localhost"), false);
    let preview = cli::run_cli_from_home(
        &home,
        &[
            "doc",
            "map",
            "Sales Invoice",
            "ACC-SINV-1",
            "--via",
            "erpnext.accounts.doctype.payment_entry.payment_entry.get_payment_entry",
            "--args",
            r#"{"reference_date":"2026-01-01"}"#,
            "--data",
            r#"{"posting_date":"2026-09-03"}"#,
        ],
    );
    assert!(preview.status.success(), "{preview:?}");
    let apply_command = cli::json_output(&preview.stdout)
        .pointer("/data/apply_command")
        .and_then(Value::as_str)
        .expect("apply_command")
        .to_string();

    let replayed = cli::run_replay(&home, &apply_command);
    fs::remove_dir_all(&home).ok();

    assert!(
        replayed.status.success(),
        "replay: {replayed:?}\ncommand: {apply_command}"
    );
    assert_eq!(
        source.hits(),
        2,
        "source is re-observed on preview and apply"
    );
    assert_eq!(
        mapper.hits(),
        2,
        "mapper is re-invoked on preview and apply"
    );
    create.assert();
    read_back.assert();
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE5
fn apply_rechecks_metadata_before_fresh_write() {
    let server = MockServer::start();
    let mut meta = server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200).json_body(json!({ "data": { "fields": [
            { "fieldname": "customer_name" }, { "fieldname": "territory" },
        ]}}));
    });

    let home = cli::test_home(&server.base_url().replace("127.0.0.1", "localhost"), false);
    let preview = cli::run_cli_from_home(
        &home,
        &[
            "doc",
            "create",
            "Customer",
            "--data",
            r#"{"customer_name":"Acme","territory":"All"}"#,
        ],
    );
    assert!(preview.status.success(), "{preview:?}");

    meta.delete();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [ { "fieldname": "customer_name" } ] } }));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Customer");
        then.status(200).json_body(json!({}));
    });

    let apply = cli::run_cli_from_home(
        &home,
        &[
            "doc",
            "create",
            "Customer",
            "--data",
            r#"{"customer_name":"Acme","territory":"All"}"#,
            "--apply",
        ],
    );
    fs::remove_dir_all(&home).ok();

    assert_eq!(apply.status.code(), Some(1));
    assert_eq!(
        create.hits(),
        0,
        "a newly unknown field must block the fresh write"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE5
fn apply_stops_when_current_observation_fails() {
    let server = MockServer::start();
    let mut get_doc = server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Customer/CUST-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 0 } }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Customer/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });

    let home = cli::test_home(&server.base_url().replace("127.0.0.1", "localhost"), false);
    let preview = cli::run_cli_from_home(&home, &["doc", "delete", "Customer", "CUST-1"]);
    assert!(preview.status.success(), "{preview:?}");

    get_doc.delete();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Customer/CUST-1");
        then.status(404).json_body(json!({ "message": "gone" }));
    });
    let delete = server.mock(|when, then| {
        when.method(DELETE).path("/api/v2/document/Customer/CUST-1");
        then.status(200).json_body(json!({}));
    });

    let apply = cli::run_cli_from_home(&home, &["doc", "delete", "Customer", "CUST-1", "--apply"]);
    fs::remove_dir_all(&home).ok();

    assert_eq!(apply.status.code(), Some(1));
    assert_eq!(
        delete.hits(),
        0,
        "an unreadable current observation must block the DELETE"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE5
fn amend_rechecks_cancellation_before_creation() {
    let server = MockServer::start();
    let mut get_source = server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Quotation/Q-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 2, "name": "Q-1" } }));
    });
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/doctype/Quotation/meta");
        then.status(200)
            .json_body(json!({ "data": { "fields": [] } }));
    });

    let home = cli::test_home(&server.base_url().replace("127.0.0.1", "localhost"), false);
    let preview = cli::run_cli_from_home(&home, &["workflow", "amend", "Quotation", "Q-1"]);
    assert!(preview.status.success(), "{preview:?}");

    get_source.delete();
    server.mock(|when, then| {
        when.method(GET).path("/api/v2/document/Quotation/Q-1");
        then.status(200)
            .json_body(json!({ "data": { "docstatus": 1, "name": "Q-1" } }));
    });
    let create = server.mock(|when, then| {
        when.method(POST).path("/api/v2/document/Quotation");
        then.status(200).json_body(json!({}));
    });

    let apply =
        cli::run_cli_from_home(&home, &["workflow", "amend", "Quotation", "Q-1", "--apply"]);
    fs::remove_dir_all(&home).ok();

    assert_eq!(apply.status.code(), Some(1));
    assert_eq!(
        create.hits(),
        0,
        "a source that changed status after preview must not be amended"
    );
}

#[test]
// spec: project/spec/feature/erp-operations/mutation-preview UC-PRE6
fn create_preview_emits_no_inferred_consequences() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/doctype/Sales%20Invoice/meta");
        then.status(200).json_body(json!({ "data": { "fields": [
            { "fieldname": "customer" }, { "fieldname": "grand_total" },
        ]}}));
    });

    let output = cli::run_cli_against(
        &server,
        &[
            "doc",
            "create",
            "Sales Invoice",
            "--data",
            r#"{"customer":"Acme","grand_total":2500.0}"#,
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let envelope = cli::json_output(&output.stdout);
    let data = envelope.pointer("/data").expect("preview data");

    for absent in ["summary", "consequence", "reversibility"] {
        assert!(
            data.get(absent).is_none(),
            "no generated narrative field '{absent}' is emitted: {data}"
        );
    }
}
