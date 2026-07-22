use serde_json::{json, Map, Value};

use crate::output;
use crate::summary;
use crate::transport::{encode_segment, Client};

/// Docstatus values ERPNext gives documents: draft, submitted, cancelled.
const DRAFT: i64 = 0;
const SUBMITTED: i64 = 1;
const CANCELLED: i64 = 2;

/// `erpnext workflow submit <doctype> <name> [--apply]`
pub async fn submit(client: &Client, doctype: &str, name: &str, apply: bool) -> i32 {
    transition(client, doctype, name, SUBMITTED, apply).await
}

/// `erpnext workflow cancel <doctype> <name> [--apply]`
pub async fn cancel(client: &Client, doctype: &str, name: &str, apply: bool) -> i32 {
    transition(client, doctype, name, CANCELLED, apply).await
}

/// Drive a document's docstatus through the native document envelope, then read the
/// document back. The site answers a lifecycle request with the document body and an
/// unchanged docstatus when it declines the transition, so success is only claimed
/// once the stored docstatus proves it.
async fn transition(client: &Client, doctype: &str, name: &str, target: i64, apply: bool) -> i32 {
    let path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    );
    let payload = json!({ "docstatus": target });

    if !apply {
        let action = if target == CANCELLED {
            summary::Action::Cancel { doctype, name }
        } else {
            summary::Action::Submit { doctype, name }
        };
        return output::ok(preview_envelope(
            client, &action, "PUT", &path, &payload, target,
        ));
    }

    if let Err(e) = client.put(&path, &payload).await {
        return output::fail(e.kind(), &e.to_string(), output::EXIT_API);
    }

    match client.get(&path).await {
        Ok(doc) => {
            let reached = doc
                .pointer("/data/docstatus")
                .and_then(Value::as_i64)
                .unwrap_or(DRAFT);
            if reached == target {
                output::ok(json!({
                    "reference": { "doctype": doctype, "name": name },
                    "docstatus": reached,
                    "state": describe(reached),
                }))
            } else {
                output::fail(
                    "lifecycle-unchanged",
                    &format!(
                        "site accepted the request but '{name}' remains {} (docstatus {reached}); \
                         the transition to {} did not take effect",
                        describe(reached),
                        describe(target)
                    ),
                    output::EXIT_API,
                )
            }
        }
        Err(e) => output::fail(
            e.kind(),
            &format!("transition sent but the resulting state could not be read: {e}"),
            output::EXIT_API,
        ),
    }
}

/// `erpnext workflow amend <doctype> <name> --data <json> [--apply]`
///
/// ERPNext amends a cancelled document by creating a new draft that carries
/// `amended_from`; the cancelled document itself never changes.
pub async fn amend(client: &Client, doctype: &str, name: &str, data: &str, apply: bool) -> i32 {
    let overrides: Map<String, Value> = match serde_json::from_str::<Value>(data) {
        Ok(Value::Object(m)) => m,
        Ok(_) => {
            return output::fail(
                "payload",
                "--data must be a JSON object",
                output::EXIT_CONFIG,
            )
        }
        Err(e) => {
            return output::fail(
                "payload",
                &format!("--data is not valid JSON: {e}"),
                output::EXIT_CONFIG,
            )
        }
    };

    let source_path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    );
    let source = match client.get(&source_path).await {
        Ok(v) => v.pointer("/data").cloned().unwrap_or(Value::Null),
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let cancelled = source
        .pointer("/docstatus")
        .and_then(Value::as_i64)
        .unwrap_or(DRAFT);
    if cancelled != CANCELLED {
        return output::fail(
            "lifecycle-conflict",
            &format!(
                "'{name}' is {} (docstatus {cancelled}); ERPNext amends only cancelled documents",
                describe(cancelled)
            ),
            output::EXIT_API,
        );
    }

    let mut payload = source.as_object().cloned().unwrap_or_default();
    payload.remove("name");
    payload.remove("creation");
    payload.remove("modified");
    payload.insert("docstatus".into(), json!(DRAFT));
    payload.insert("amended_from".into(), json!(name));
    for (key, value) in overrides {
        payload.insert(key, value);
    }
    let payload = Value::Object(payload);

    let create_path = format!("/api/v2/document/{}", encode_segment(doctype));

    if !apply {
        return output::ok(preview_envelope(
            client,
            &summary::Action::Amend { doctype, name },
            "POST",
            &create_path,
            &payload,
            DRAFT,
        ));
    }

    match client.post(&create_path, &payload).await {
        Ok(v) => output::ok_payload(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext workflow method <doctype> <name> <method> [--data <json>] [--apply]` — run a
/// whitelisted controller method through the native method envelope.
pub async fn run_method(
    client: &Client,
    doctype: &str,
    name: &str,
    method: &str,
    data: Option<&str>,
    apply: bool,
) -> i32 {
    let payload: Value = match data {
        None => json!({}),
        Some(raw) => match serde_json::from_str(raw) {
            Ok(v @ Value::Object(_)) => v,
            Ok(_) => {
                return output::fail(
                    "payload",
                    "--data must be a JSON object",
                    output::EXIT_CONFIG,
                )
            }
            Err(e) => {
                return output::fail(
                    "payload",
                    &format!("--data is not valid JSON: {e}"),
                    output::EXIT_CONFIG,
                )
            }
        },
    };

    let path = format!(
        "/api/v2/document/{}/{}/method/{}/",
        encode_segment(doctype),
        encode_segment(name),
        encode_segment(method)
    );

    if !apply {
        return output::ok(json!({
            "preview": true,
            "summary": summary::describe(
                &summary::Action::Method { doctype, name, method },
                client.context().company.as_deref(),
            ),
            "context": {
                "site": client.context().site(),
                "company": client.context().company,
                "privilege": client.context().privilege,
            },
            "operation": { "method": "POST", "path": path, "payload": payload },
            "persistence": "when the method returns the changed document, apply saves that exact document and reports the stored result",
            "apply_command": "re-run the same command with --apply",
        }));
    }

    let method_result = match client.post(&path, &payload).await {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let Some(returned_doc) = method_result.pointer("/docs/0") else {
        return output::ok_payload(method_result);
    };
    let returned_doctype = returned_doc
        .pointer("/doctype")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let returned_name = returned_doc
        .pointer("/name")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if returned_doctype != doctype || returned_name != name {
        return output::fail(
            "shape",
            &format!(
                "controller method returned {returned_doctype} {returned_name}, expected {doctype} {name}"
            ),
            output::EXIT_API,
        );
    }

    let document_path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    );
    match client.put(&document_path, returned_doc).await {
        Ok(saved) => output::ok_payload(saved),
        Err(e) => output::fail_with_data(
            e.kind(),
            &format!("controller method ran but its returned document was not saved: {e}"),
            json!({ "method_result": method_result }),
            output::EXIT_API,
        ),
    }
}

fn preview_envelope(
    client: &Client,
    action: &summary::Action,
    method: &str,
    path: &str,
    payload: &Value,
    target: i64,
) -> Value {
    json!({
        "preview": true,
        "summary": summary::describe(action, client.context().company.as_deref()),
        "context": {
            "site": client.context().site(),
            "company": client.context().company,
            "privilege": client.context().privilege,
        },
        "operation": { "method": method, "path": path, "payload": payload },
        "intended_state": describe(target),
        "verification": "the stored docstatus is read back on apply; an unchanged docstatus fails",
        "apply_command": "re-run the same command with --apply",
    })
}

fn describe(docstatus: i64) -> &'static str {
    match docstatus {
        SUBMITTED => "submitted",
        CANCELLED => "cancelled",
        _ => "draft",
    }
}
