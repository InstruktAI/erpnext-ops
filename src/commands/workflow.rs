use serde_json::{json, Map, Value};

use crate::_preview::{self, ReplayArgs};
use crate::output;
use crate::transport::{encode_segment, ApiError, Client};

/// Docstatus values ERPNext gives documents: draft, submitted, cancelled.
const DRAFT: i64 = 0;
const SUBMITTED: i64 = 1;
const CANCELLED: i64 = 2;

fn document_path(doctype: &str, name: &str) -> String {
    format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    )
}

fn collection_path(doctype: &str) -> String {
    format!("/api/v2/document/{}", encode_segment(doctype))
}

fn method_path(doctype: &str, name: &str, method: &str) -> String {
    format!(
        "/api/v2/document/{}/{}/method/{}",
        encode_segment(doctype),
        encode_segment(name),
        encode_segment(method)
    )
}

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
/// once the stored docstatus proves it. Both preview and apply read the current
/// document and its metadata first; an unreadable read stops the command before any
/// PUT.
async fn transition(client: &Client, doctype: &str, name: &str, target: i64, apply: bool) -> i32 {
    let path = document_path(doctype, name);
    let payload = json!({ "docstatus": target });

    let doc = match client.get(&path).await {
        Ok(v) => v,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("current document read failed: {e}"),
                output::EXIT_API,
            )
        }
    };
    let meta = match super::meta::fetch(client, doctype).await {
        Ok(m) => m,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("metadata read failed: {e}"),
                output::EXIT_API,
            )
        }
    };

    if !apply {
        let doc_body = doc.pointer("/data").cloned().unwrap_or(Value::Null);
        let args = if target == CANCELLED {
            ReplayArgs::WorkflowCancel { doctype, name }
        } else {
            ReplayArgs::WorkflowSubmit { doctype, name }
        };
        let apply_command = _preview::apply_command(client.context(), &args);
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "PUT", "path": path, "payload": payload }),
                target: Some(_preview::target_json(&doc_body)),
                schema: Some(_preview::schema_json(&meta)),
                preflight: _preview::no_fields_preflight_json(),
                apply_command,
                idempotency: None,
                intended_state: Some(describe(target)),
                persistence: None,
            }
            .into_json(),
        );
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
/// `amended_from`; the cancelled document itself never changes. The source is
/// read fresh on both preview and apply, so a source that changed status after a
/// preview is re-checked before any draft is created.
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

    let source_path = document_path(doctype, name);
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

    let meta = match super::meta::fetch(client, doctype).await {
        Ok(m) => m,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("metadata read failed: {e}"),
                output::EXIT_API,
            )
        }
    };

    let mut draft_map = source.as_object().cloned().unwrap_or_default();
    draft_map.remove("name");
    draft_map.remove("creation");
    draft_map.remove("modified");
    draft_map.insert("docstatus".into(), json!(DRAFT));
    draft_map.insert("amended_from".into(), json!(name));
    for (key, value) in overrides.clone() {
        draft_map.insert(key, value);
    }
    let draft = Value::Object(draft_map.clone());

    let create_path = collection_path(doctype);

    if !apply {
        let requested_data = Value::Object(overrides);
        let apply_command = _preview::apply_command(
            client.context(),
            &ReplayArgs::WorkflowAmend {
                doctype,
                name,
                data: &requested_data,
            },
        );
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "POST", "path": create_path, "payload": draft }),
                target: Some(_preview::target_json(&source)),
                schema: Some(_preview::schema_json(&meta)),
                preflight: _preview::preflight_json(doctype, &meta, &draft_map),
                apply_command,
                idempotency: None,
                intended_state: Some(describe(DRAFT)),
                persistence: None,
            }
            .into_json(),
        );
    }

    match client.post(&create_path, &draft).await {
        Ok(v) => output::ok_payload(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext workflow method <doctype> <name> <method> [--data <json>] [--apply]` — run a
/// whitelisted controller method through the native method envelope. Preview and
/// apply read the current document and its metadata first; apply then reports the
/// stored result without a secondary write when the controller persisted its effect,
/// and saves the returned document once only when the stored document stayed unchanged.
/// The method's `--data` is opaque method arguments, never checked as document fields.
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

    let path = method_path(doctype, name, method);

    let doc = match client.get(&document_path(doctype, name)).await {
        Ok(v) => v,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("current document read failed: {e}"),
                output::EXIT_API,
            )
        }
    };
    let meta = match super::meta::fetch(client, doctype).await {
        Ok(m) => m,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("metadata read failed: {e}"),
                output::EXIT_API,
            )
        }
    };

    if !apply {
        let doc_body = doc.pointer("/data").cloned().unwrap_or(Value::Null);
        let requested_data = if data.is_some() { Some(&payload) } else { None };
        let apply_command = _preview::apply_command(
            client.context(),
            &ReplayArgs::WorkflowMethod {
                doctype,
                name,
                method,
                data: requested_data,
            },
        );
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "POST", "path": path, "payload": payload }),
                target: Some(_preview::target_json(&doc_body)),
                schema: Some(_preview::schema_json(&meta)),
                preflight: _preview::opaque_preflight_json(),
                apply_command,
                idempotency: None,
                intended_state: None,
                persistence: Some(
                    "when the controller persisted its effect, apply sends no further write; when the effect exists only in the returned document, apply persists that document once, never over a newer stored version. The preview cannot show the returned document before the site runs the method",
                ),
            }
            .into_json(),
        );
    }

    let initial_document = doc.pointer("/data").cloned().unwrap_or(Value::Null);
    match apply_method(client, doctype, name, method, &payload, &initial_document).await {
        Ok(stored) => output::ok(stored),
        Err(error) => output::fail(error.kind(), &error.message(), output::EXIT_API),
    }
}

#[derive(Debug)]
pub enum MethodApplyError {
    Request(ApiError),
    Shape(String),
    ReadBack(ApiError),
    UndeterminedPersistence,
    RejectedPersistence(ApiError),
}

impl MethodApplyError {
    fn kind(&self) -> &'static str {
        match self {
            Self::Request(error) | Self::ReadBack(error) | Self::RejectedPersistence(error) => {
                error.kind()
            }
            Self::Shape(_) => "shape",
            Self::UndeterminedPersistence => "undetermined-persistence",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Request(error) => format!("controller method request failed: {error}"),
            Self::Shape(message) => message.clone(),
            Self::ReadBack(error) => format!("stored document read-back failed: {error}"),
            Self::UndeterminedPersistence => {
                "controller persistence could not be determined from stored state".into()
            }
            Self::RejectedPersistence(error) => {
                format!("controller result persistence was rejected: {error}")
            }
        }
    }
}

/// Apply a native document controller method and return the state the site actually
/// stored. The pre-call document, controller response, and fresh read-back determine
/// whether the controller already persisted its effect or the returned document still
/// needs one native save.
pub async fn apply_method(
    client: &Client,
    doctype: &str,
    name: &str,
    method: &str,
    payload: &Value,
    initial_document: &Value,
) -> Result<Value, MethodApplyError> {
    let method_result = client
        .post(&method_path(doctype, name, method), payload)
        .await
        .map_err(MethodApplyError::Request)?;

    let returned_doc = method_result.pointer("/docs/0").ok_or_else(|| {
        MethodApplyError::Shape("controller method response did not contain a document".into())
    })?;
    let returned_doctype = returned_doc
        .pointer("/doctype")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let returned_name = returned_doc
        .pointer("/name")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if returned_doctype != doctype || returned_name != name {
        return Err(MethodApplyError::Shape(format!(
            "controller method returned {returned_doctype} {returned_name}, expected {doctype} {name}"
        )));
    }

    let stored_result = client
        .get(&document_path(doctype, name))
        .await
        .map_err(MethodApplyError::ReadBack)?;
    let stored_document = match stored_result.get("data").and_then(Value::as_object) {
        Some(document) => Value::Object(document.clone()),
        None => {
            return Err(MethodApplyError::ReadBack(ApiError::Shape(
                "post-call read-back did not contain a document object".into(),
            )))
        }
    };

    if structurally_matches(returned_doc, &stored_document) {
        return Ok(stored_document);
    }

    if structurally_matches(&stored_document, initial_document) {
        return client
            .put(&document_path(doctype, name), returned_doc)
            .await
            .map(output::site_payload)
            .map_err(MethodApplyError::RejectedPersistence);
    }

    Err(MethodApplyError::UndeterminedPersistence)
}

fn structurally_matches(candidate: &Value, stored: &Value) -> bool {
    match (candidate, stored) {
        (Value::Object(candidate), Value::Object(stored)) => stored.iter().all(|(key, value)| {
            candidate
                .get(key)
                .is_some_and(|candidate_value| structurally_matches(candidate_value, value))
        }),
        (Value::Array(candidate), Value::Array(stored)) => {
            candidate.len() == stored.len()
                && candidate
                    .iter()
                    .zip(stored)
                    .all(|(candidate_value, stored_value)| {
                        structurally_matches(candidate_value, stored_value)
                    })
        }
        _ => candidate == stored,
    }
}

fn describe(docstatus: i64) -> &'static str {
    match docstatus {
        SUBMITTED => "submitted",
        CANCELLED => "cancelled",
        _ => "draft",
    }
}
