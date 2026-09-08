use serde_json::{json, Map, Value};

use crate::_preview::{self, ReplayArgs};
use crate::output;
use crate::transport::{encode_segment, Client};

mod _links;

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

/// `erpnext doc list <doctype> [--filter k=v]... [--limit n]`
pub async fn list(client: &Client, doctype: &str, filters: &[(String, String)], limit: u32) -> i32 {
    let filter_json: Vec<Value> = filters
        .iter()
        .map(|(k, v)| json!([doctype, k, "=", v]))
        .collect();
    let mut path = format!(
        "/api/v2/document/{}?limit={}&fields={}",
        encode_segment(doctype),
        limit,
        encode_segment("[\"name\"]")
    );
    if !filter_json.is_empty() {
        let rendered = serde_json::to_string(&filter_json).unwrap_or_default();
        path.push_str(&format!("&filters={}", encode_segment(&rendered)));
    }
    match client.get(&path).await {
        Ok(v) => output::ok_payload(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc get <doctype> <name>`
pub async fn get(client: &Client, doctype: &str, name: &str) -> i32 {
    match client.get(&document_path(doctype, name)).await {
        Ok(v) => output::ok_payload(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc create <doctype> --data <json> [--source <token>] [--apply]`
///
/// Preview by default: reads live DocType metadata and shows the exact native
/// operation that would be sent, without sending it. `--apply` re-reads that
/// metadata, sends the native request, and reports the site's result.
///
/// With `--source <token>` the create is exactly idempotent: the document name is
/// derived deterministically from site + doctype + token and sent atomically in the
/// payload, so a retry after a lost response finds the existing document instead of
/// creating a second one. An idempotency-named document opts out of native series
/// naming — use it where replay safety outweighs series numbers.
pub async fn create(
    client: &Client,
    doctype: &str,
    data: &str,
    source: Option<&str>,
    apply: bool,
) -> i32 {
    let requested_payload: Value = match serde_json::from_str(data) {
        Ok(v) => v,
        Err(e) => {
            return output::fail(
                "payload",
                &format!("--data is not valid JSON: {e}"),
                output::EXIT_CONFIG,
            )
        }
    };
    let Some(payload_map) = requested_payload.as_object() else {
        return output::fail(
            "payload",
            "--data must be a JSON object",
            output::EXIT_CONFIG,
        );
    };
    let mut payload_map = payload_map.clone();

    let derived_name =
        source.map(|token| derive_source_name(client.context().site(), doctype, token));
    if let Some(name) = &derived_name {
        payload_map.insert("name".into(), Value::String(name.clone()));
    }
    let payload = Value::Object(payload_map.clone());

    let path = collection_path(doctype);

    if !apply {
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
        let apply_cmd = _preview::apply_command(
            client.context(),
            &ReplayArgs::DocCreate {
                doctype,
                data: &requested_payload,
                source,
            },
        );
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "POST", "path": path, "payload": payload }),
                target: None,
                schema: Some(_preview::schema_json(&meta)),
                preflight: _preview::preflight_json(doctype, &meta, &payload_map),
                apply_command: apply_cmd,
                idempotency: derived_name
                    .as_ref()
                    .map(|n| json!({ "source_derived_name": n })),
                intended_state: None,
                persistence: None,
            }
            .into_json(),
        );
    }

    // Destination metadata is re-read unconditionally, whether this call turns out
    // to be a replay or a fresh write: it permits a truthful inspection either way,
    // and a faithful replay must never become a new write merely because metadata
    // drifted since the earlier create.
    let meta = match super::meta::fetch(client, doctype).await {
        Ok(m) => m,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("metadata preflight failed: {e}"),
                output::EXIT_API,
            )
        }
    };

    // Idempotent replay: the deterministic name is looked up before any write.
    if let Some(name) = &derived_name {
        match replay_check(client, doctype, name, &payload_map).await {
            ReplayCheck::Missing => {}
            ReplayCheck::Replayed(existing) => return output::ok(create_result(true, existing)),
            ReplayCheck::Conflict(diff) => {
                return output::fail(
                    "idempotency-conflict",
                    &format!(
                        "document '{name}' exists from the same source token but differs: {diff}"
                    ),
                    output::EXIT_API,
                )
            }
            ReplayCheck::Failed(e) => {
                return output::fail(e.kind(), &e.to_string(), output::EXIT_API)
            }
        }
    }

    // Unknown fields against the exact native default-field set and the DocType's
    // own declared fields gate only a fresh write. Missing mandatory fields are not
    // checked here; ERPNext derives many of them during validation.
    if let Err((kind, message)) = _preview::check_fields(&meta, &payload_map).gate() {
        return output::fail(kind, &message, output::EXIT_API);
    }

    match client.post(&path, &payload).await {
        Ok(v) => output::ok(create_result(false, v)),
        Err(e) => {
            // A concurrent create with the same derived name loses the primary-key race;
            // resolve it as a replay instead of surfacing the duplicate error.
            if let Some(name) = &derived_name {
                if matches!(e, crate::transport::ApiError::Rejected { .. }) {
                    match replay_check(client, doctype, name, &payload_map).await {
                        ReplayCheck::Replayed(existing) => {
                            return output::ok(create_result(true, existing))
                        }
                        ReplayCheck::Conflict(diff) => {
                            return output::fail(
                                "idempotency-conflict",
                                &format!(
                                    "document '{name}' exists from the same source token but differs: {diff}"
                                ),
                                output::EXIT_API,
                            )
                        }
                        _ => {}
                    }
                }
            }
            output::fail(e.kind(), &e.to_string(), output::EXIT_API)
        }
    }
}

/// A create and its idempotent replay return the same shape. The vendor's response
/// wrapper is removed before the document enters the CLI result envelope.
fn create_result(replayed: bool, response: Value) -> Value {
    json!({
        "replayed": replayed,
        "document": output::site_payload(response),
    })
}

/// Deterministic, length-bounded document name for a source token.
fn derive_source_name(site: &str, doctype: &str, token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(site.as_bytes());
    hasher.update(b"|");
    hasher.update(doctype.as_bytes());
    hasher.update(b"|");
    hasher.update(token.as_bytes());
    let digest = hasher.finalize();
    let hex: String = digest.iter().take(10).map(|b| format!("{b:02x}")).collect();
    format!("SRC-{hex}")
}

enum ReplayCheck {
    Missing,
    Replayed(Value),
    Conflict(String),
    Failed(crate::transport::ApiError),
}

/// Fetch the deterministic name and compare the requested fields against the
/// existing document. Equal fields mean the earlier create succeeded and this is
/// a replay; different fields mean the same token was reused for different data.
async fn replay_check(
    client: &Client,
    doctype: &str,
    name: &str,
    requested: &Map<String, Value>,
) -> ReplayCheck {
    let existing = match client.get(&document_path(doctype, name)).await {
        Ok(doc) => doc.pointer("/data").cloned().unwrap_or(Value::Null),
        Err(crate::transport::ApiError::NotFound { .. }) => return ReplayCheck::Missing,
        Err(e) => return ReplayCheck::Failed(e),
    };

    let mut diffs: Vec<String> = Vec::new();
    for (key, requested_value) in requested {
        if key == "doctype" || key == "name" {
            continue;
        }
        let existing_value = existing.pointer(&format!("/{key}")).unwrap_or(&Value::Null);
        if !json_loosely_equal(requested_value, existing_value) {
            diffs.push(format!("field '{key}'"));
        }
    }

    if diffs.is_empty() {
        ReplayCheck::Replayed(existing)
    } else {
        ReplayCheck::Conflict(diffs.join(", "))
    }
}

/// ERPNext coerces scalar types (numbers arrive back as floats or strings); compare
/// loosely enough to recognize a faithful replay without accepting real differences.
fn json_loosely_equal(a: &Value, b: &Value) -> bool {
    if a == b {
        return true;
    }
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x
            .as_f64()
            .zip(y.as_f64())
            .map(|(x, y)| (x - y).abs() < 1e-9)
            .unwrap_or(false),
        (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => s
            .parse::<f64>()
            .ok()
            .zip(n.as_f64())
            .map(|(x, y)| (x - y).abs() < 1e-9)
            .unwrap_or(false),
        (Value::Array(xs), Value::Array(ys)) => {
            xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| json_loosely_equal(x, y))
        }
        (Value::Object(requested), Value::Object(existing)) => {
            requested.iter().all(|(key, value)| {
                existing
                    .get(key)
                    .is_some_and(|stored| json_loosely_equal(value, stored))
            })
        }
        _ => false,
    }
}

enum JsonObjectInputError {
    InvalidJson {
        option: &'static str,
        source: serde_json::Error,
    },
    NotObject {
        option: &'static str,
    },
}

fn parse_json_object(
    option: &'static str,
    raw: Option<&str>,
) -> Result<Map<String, Value>, JsonObjectInputError> {
    let Some(raw) = raw else {
        return Ok(Map::new());
    };

    match serde_json::from_str(raw) {
        Ok(Value::Object(object)) => Ok(object),
        Ok(_) => Err(JsonObjectInputError::NotObject { option }),
        Err(source) => Err(JsonObjectInputError::InvalidJson { option, source }),
    }
}

fn render_json_object_error(error: JsonObjectInputError) -> i32 {
    match error {
        JsonObjectInputError::InvalidJson { option, source } => output::fail(
            "payload",
            &format!("{option} is not valid JSON: {source}"),
            output::EXIT_CONFIG,
        ),
        JsonObjectInputError::NotObject { option } => output::fail(
            "payload",
            &format!("{option} must be a JSON object"),
            output::EXIT_CONFIG,
        ),
    }
}

/// `erpnext doc map <doctype> <name> --via <native.mapper> [--args <json>] [--data <json>] [--apply]`
///
/// ERPNext converts documents along its own chains — quotation to sales order, sales
/// order to invoice — through mapper methods that copy items, taxes and terms and link
/// the result back to its source. The mapper is named by the caller, so no ERP chain
/// knowledge lives here: the site builds the target document and this sends it.
pub async fn map(
    client: &Client,
    doctype: &str,
    name: &str,
    via: &str,
    args: Option<&str>,
    data: Option<&str>,
    apply: bool,
) -> i32 {
    let mut mapper_args = match parse_json_object("--args", args) {
        Ok(arguments) => arguments,
        Err(error) => return render_json_object_error(error),
    };
    let requested_args = args.map(|_| Value::Object(mapper_args.clone()));
    let overrides = match parse_json_object("--data", data) {
        Ok(overrides) => overrides,
        Err(error) => return render_json_object_error(error),
    };
    let requested_data = data.map(|_| Value::Object(overrides.clone()));

    // Selling-chain mappers name the source `source_name`; accounts-chain mappers
    // name it `dt` and `dn`. Insert these after optional arguments so the caller
    // cannot redirect the mapper to a different source document.
    mapper_args.insert("source_name".into(), Value::String(name.into()));
    mapper_args.insert("dt".into(), Value::String(doctype.into()));
    mapper_args.insert("dn".into(), Value::String(name.into()));

    let source = match client.get(&document_path(doctype, name)).await {
        Ok(v) => v,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("source document read failed: {e}"),
                output::EXIT_API,
            )
        }
    };
    let source_body = source.pointer("/data").cloned().unwrap_or(Value::Null);

    let mapped = match client
        .post(&format!("/api/method/{via}"), &Value::Object(mapper_args))
        .await
    {
        Ok(v) => v.pointer("/message").cloned().unwrap_or(Value::Null),
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let mapped_map = {
        let mut m = mapped.as_object().cloned().unwrap_or_default();
        for (k, v) in overrides {
            m.insert(k, v);
        }
        m
    };
    let mapped = Value::Object(mapped_map.clone());

    let Some(target_doctype) = mapped
        .pointer("/doctype")
        .and_then(Value::as_str)
        .map(str::to_owned)
    else {
        return output::fail(
            "shape",
            &format!("'{via}' returned no target document for '{name}'"),
            output::EXIT_API,
        );
    };

    let target_meta = match super::meta::fetch(client, &target_doctype).await {
        Ok(m) => m,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("destination metadata read failed: {e}"),
                output::EXIT_API,
            )
        }
    };

    let path = collection_path(&target_doctype);

    if !apply {
        let apply_cmd = _preview::apply_command(
            client.context(),
            &ReplayArgs::DocMap {
                doctype,
                name,
                via,
                args: requested_args.as_ref(),
                data: requested_data.as_ref(),
            },
        );
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "POST", "path": path, "payload": mapped }),
                target: Some(_preview::target_json(&source_body)),
                schema: Some(_preview::schema_json(&target_meta)),
                preflight: _preview::preflight_json(&target_doctype, &target_meta, &mapped_map),
                apply_command: apply_cmd,
                idempotency: None,
                intended_state: None,
                persistence: None,
            }
            .into_json(),
        );
    }

    let created = match client.post(&path, &mapped).await {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    // The create response and the mapper payload are text the site returned about the
    // write, never evidence of what it persisted. Only a stored-target read-back proves
    // the relationship, so the one thing taken from this response is a usable identifier.
    let Some(created_name) = created
        .pointer("/data/name")
        .and_then(Value::as_str)
        .filter(|created_name| !created_name.is_empty())
    else {
        return output::fail_with_data(
            "shape",
            &format!(
                "'{via}' created a '{target_doctype}' document but the create response carries no usable name"
            ),
            json!({ "target_doctype": target_doctype, "created": true }),
            output::EXIT_API,
        );
    };
    let created_name = created_name.to_owned();

    match _links::verify(client, doctype, name, &target_doctype, &created_name).await {
        Ok(_links::LinkProof::Verified) => output::ok(json!({
            "source": { "doctype": doctype, "name": name },
            "created": { "doctype": target_doctype, "name": created_name },
            "linked_to_source": true,
        })),
        Ok(_links::LinkProof::Missing) => output::fail_with_data(
            "linkage-missing",
            &format!("'{created_name}' was created but references no source '{name}'"),
            json!({ "created": created_name, "target_doctype": target_doctype }),
            output::EXIT_API,
        ),
        Ok(_links::LinkProof::Unavailable(reason)) => output::fail_with_data(
            "linkage-unavailable",
            &format!(
                "'{created_name}' was created but its relationship to '{name}' could not be verified: {reason}"
            ),
            json!({ "created": created_name, "target_doctype": target_doctype }),
            output::EXIT_API,
        ),
        Err(e) => output::fail_with_data(
            e.kind(),
            &format!("'{created_name}' was created but could not be read back to verify its source: {e}"),
            json!({ "created": created_name, "target_doctype": target_doctype }),
            output::EXIT_API,
        ),
    }
}

/// `erpnext doc update <doctype> <name> --data <json> [--apply]`
///
/// Preview by default. On apply, the current document and live metadata are read
/// again; unknown payload fields are rejected against that metadata (mandatory
/// checks do not apply to partial updates), then the native PUT is sent and the
/// site's result reported.
pub async fn update(client: &Client, doctype: &str, name: &str, data: &str, apply: bool) -> i32 {
    let payload: Value = match serde_json::from_str(data) {
        Ok(v) => v,
        Err(e) => {
            return output::fail(
                "payload",
                &format!("--data is not valid JSON: {e}"),
                output::EXIT_CONFIG,
            )
        }
    };
    let Some(payload_map) = payload.as_object() else {
        return output::fail(
            "payload",
            "--data must be a JSON object",
            output::EXIT_CONFIG,
        );
    };

    let path = document_path(doctype, name);

    if !apply {
        let doc = match client.get(&path).await {
            Ok(v) => v,
            Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
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
        let doc_body = doc.pointer("/data").cloned().unwrap_or(Value::Null);
        let apply_cmd = _preview::apply_command(
            client.context(),
            &ReplayArgs::DocUpdate {
                doctype,
                name,
                data: &payload,
            },
        );
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "PUT", "path": path, "payload": payload }),
                target: Some(_preview::target_json(&doc_body)),
                schema: Some(_preview::schema_json(&meta)),
                preflight: _preview::preflight_json(doctype, &meta, payload_map),
                apply_command: apply_cmd,
                idempotency: None,
                intended_state: None,
                persistence: None,
            }
            .into_json(),
        );
    }

    if let Err(e) = client.get(&path).await {
        return output::fail(
            e.kind(),
            &format!("current document read failed: {e}"),
            output::EXIT_API,
        );
    }
    let meta = match super::meta::fetch(client, doctype).await {
        Ok(m) => m,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("metadata preflight failed: {e}"),
                output::EXIT_API,
            )
        }
    };
    if let Err((kind, message)) = _preview::check_fields(&meta, payload_map).gate() {
        return output::fail(kind, &message, output::EXIT_API);
    }

    match client.put(&path, &payload).await {
        Ok(v) => output::ok_payload(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc delete <doctype> <name> [--apply]` — preview by default. Both
/// preview and apply read the current document and its metadata; an unreadable
/// document or metadata response stops the command before any DELETE.
pub async fn delete(client: &Client, doctype: &str, name: &str, apply: bool) -> i32 {
    let path = document_path(doctype, name);

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
        let apply_cmd =
            _preview::apply_command(client.context(), &ReplayArgs::DocDelete { doctype, name });
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "DELETE", "path": path, "payload": Value::Null }),
                target: Some(_preview::target_json(&doc_body)),
                schema: Some(_preview::schema_json(&meta)),
                preflight: _preview::no_fields_preflight_json(),
                apply_command: apply_cmd,
                idempotency: None,
                intended_state: None,
                persistence: None,
            }
            .into_json(),
        );
    }

    match client.delete(&path).await {
        Ok(v) => output::ok_payload(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc copy <doctype> <name> [--data <json>] [--apply]`
///
/// Reads the source document, its metadata, and the site's native copy template,
/// merges optional overrides onto the template, then follows the create contract:
/// preview by default, re-observed reads plus POST on apply.
pub async fn copy(
    client: &Client,
    doctype: &str,
    name: &str,
    data: Option<&str>,
    apply: bool,
) -> i32 {
    let overrides: Map<String, Value> = match data {
        None => Map::new(),
        Some(raw) => match serde_json::from_str::<Value>(raw) {
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
        },
    };
    let requested_data = data.map(|_| Value::Object(overrides.clone()));

    let source = match client.get(&document_path(doctype, name)).await {
        Ok(v) => v,
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("source document read failed: {e}"),
                output::EXIT_API,
            )
        }
    };
    let source_body = source.pointer("/data").cloned().unwrap_or(Value::Null);
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

    let copy_path = format!(
        "/api/v2/document/{}/{}/copy",
        encode_segment(doctype),
        encode_segment(name)
    );
    let template = match client.get(&copy_path).await {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };
    let mut payload_map = template
        .pointer("/data")
        .or_else(|| template.pointer("/message"))
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for (k, v) in overrides {
        payload_map.insert(k, v);
    }
    let payload = Value::Object(payload_map.clone());

    let path = collection_path(doctype);

    if !apply {
        let apply_cmd = _preview::apply_command(
            client.context(),
            &ReplayArgs::DocCopy {
                doctype,
                name,
                data: requested_data.as_ref(),
            },
        );
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "POST", "path": path, "payload": payload }),
                target: Some(_preview::target_json(&source_body)),
                schema: Some(_preview::schema_json(&meta)),
                preflight: _preview::preflight_json(doctype, &meta, &payload_map),
                apply_command: apply_cmd,
                idempotency: None,
                intended_state: None,
                persistence: None,
            }
            .into_json(),
        );
    }

    match client.post(&path, &payload).await {
        Ok(v) => output::ok_payload(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta_fixture() -> Value {
        json!({ "data": { "fields": [
            { "fieldname": "customer_name", "reqd": 1 },
            { "fieldname": "customer_type", "reqd": 1, "default": "Company" },
            { "fieldname": "territory", "reqd": 0 },
        ]}})
    }

    #[test]
    fn create_and_replay_share_one_result_shape() {
        let response = json!({ "data": { "name": "SRC-known" } });
        let created = create_result(false, response.clone());
        let replayed = create_result(true, response);

        assert_eq!(created.pointer("/document"), replayed.pointer("/document"));
        assert_eq!(created.pointer("/replayed"), Some(&Value::Bool(false)));
        assert_eq!(replayed.pointer("/replayed"), Some(&Value::Bool(true)));
    }

    #[test]
    fn meta_fixture_has_no_unknown_fields_for_a_valid_payload() {
        let payload = json!({ "customer_name": "Acme" });
        assert!(matches!(
            _preview::check_fields(&meta_fixture(), payload.as_object().unwrap()),
            _preview::FieldCheck::Clean
        ));
    }
}
