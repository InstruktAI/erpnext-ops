use serde_json::{json, Map, Value};

use crate::output;
use crate::transport::{encode_segment, Client};

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
        Ok(v) => output::ok(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc get <doctype> <name>`
pub async fn get(client: &Client, doctype: &str, name: &str) -> i32 {
    let path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    );
    match client.get(&path).await {
        Ok(v) => output::ok(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc create <doctype> --data <json> [--source <token>] [--apply]`
///
/// Preview by default: shows the exact native operation that would be sent, without
/// sending it. `--apply` runs basic live-metadata preflight, sends the native request,
/// and reports the site's result.
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
    let mut payload_map = payload_map.clone();

    let derived_name =
        source.map(|token| derive_source_name(client.context().site(), doctype, token));
    if let Some(name) = &derived_name {
        payload_map.insert("name".into(), Value::String(name.clone()));
    }
    let payload = Value::Object(payload_map.clone());

    let path = format!("/api/v2/document/{}", encode_segment(doctype));

    if !apply {
        return output::ok(json!({
            "preview": true,
            "context": {
                "site": client.context().site(),
                "company": client.context().company,
                "privilege": client.context().privilege,
            },
            "operation": { "method": "POST", "path": path, "payload": payload },
            "idempotency": derived_name.as_ref().map(|n| json!({ "source_derived_name": n })),
            "preflight": "runs against live DocType metadata on apply",
            "apply_command": format!("erpnext doc create {doctype} --data '<same payload>' --apply"),
        }));
    }

    // Idempotent replay: the deterministic name is looked up before any write.
    if let Some(name) = &derived_name {
        match replay_check(client, doctype, name, &payload_map).await {
            ReplayCheck::Missing => {}
            ReplayCheck::Replayed(existing) => {
                return output::ok(json!({ "replayed": true, "data": existing }))
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
            ReplayCheck::Failed(e) => {
                return output::fail(e.kind(), &e.to_string(), output::EXIT_API)
            }
        }
    }

    // Basic preflight against live metadata: unknown fields and missing mandatory fields.
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
    if let Some(findings) = preflight(&meta, &payload_map) {
        return output::fail("preflight", &findings, output::EXIT_API);
    }

    match client.post(&path, &payload).await {
        Ok(v) => output::ok(v),
        Err(e) => {
            // A concurrent create with the same derived name loses the primary-key race;
            // resolve it as a replay instead of surfacing the duplicate error.
            if let Some(name) = &derived_name {
                if matches!(e, crate::transport::ApiError::Rejected { .. }) {
                    match replay_check(client, doctype, name, &payload_map).await {
                        ReplayCheck::Replayed(existing) => {
                            return output::ok(json!({ "replayed": true, "data": existing }))
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
    let path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    );
    let existing = match client.get(&path).await {
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
        _ => false,
    }
}

/// `erpnext doc map <doctype> <name> --via <native.mapper> [--apply]`
///
/// ERPNext converts documents along its own chains — quotation to sales order, sales
/// order to invoice — through mapper methods that copy items, taxes and terms and link
/// the result back to its source. The mapper is named by the caller, so no ERP chain
/// knowledge lives here: the site builds the target document and this sends it.
pub async fn map(client: &Client, doctype: &str, name: &str, via: &str, apply: bool) -> i32 {
    let mapped = match client
        .post(
            &format!("/api/method/{via}"),
            &json!({ "source_name": name }),
        )
        .await
    {
        Ok(v) => v.pointer("/message").cloned().unwrap_or(Value::Null),
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

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

    let path = format!("/api/v2/document/{}", encode_segment(&target_doctype));

    if !apply {
        return output::ok(json!({
            "preview": true,
            "context": {
                "site": client.context().site(),
                "company": client.context().company,
                "privilege": client.context().privilege,
            },
            "source": { "doctype": doctype, "name": name },
            "target_doctype": target_doctype,
            "operation": { "method": "POST", "path": path, "payload": mapped },
            "verification": "the created document is read back and must reference its source",
            "apply_command": "re-run the same command with --apply",
        }));
    }

    let created = match client.post(&path, &mapped).await {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let created_name = created
        .pointer("/data/name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();

    // A converted document that lost its link to the source is a broken chain, not a
    // success: the source would stay open and bill twice.
    let linked = created
        .pointer("/data")
        .map(|d| d.to_string().contains(name))
        .unwrap_or(false);
    if !linked {
        return output::fail_with_data(
            "linkage-missing",
            &format!("'{created_name}' was created but references no source '{name}'"),
            json!({ "created": created_name, "target_doctype": target_doctype }),
            output::EXIT_API,
        );
    }

    output::ok(json!({
        "source": { "doctype": doctype, "name": name },
        "created": { "doctype": target_doctype, "name": created_name },
        "linked_to_source": true,
    }))
}

/// `erpnext doc update <doctype> <name> --data <json> [--apply]`
///
/// Preview by default. On apply, unknown payload fields are rejected against live
/// metadata (mandatory checks do not apply to partial updates), then the native PUT
/// is sent and the site's result reported.
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

    let path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    );

    if !apply {
        return output::ok(preview_envelope(client, "PUT", &path, &payload));
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
    if let Some(findings) = unknown_fields(&meta, payload_map) {
        return output::fail("preflight", &findings, output::EXIT_API);
    }

    match client.put(&path, &payload).await {
        Ok(v) => output::ok(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc delete <doctype> <name> [--apply]` — preview by default.
pub async fn delete(client: &Client, doctype: &str, name: &str, apply: bool) -> i32 {
    let path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    );

    if !apply {
        return output::ok(preview_envelope(client, "DELETE", &path, &Value::Null));
    }

    match client.delete(&path).await {
        Ok(v) => output::ok(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc copy <doctype> <name> [--data <json>] [--apply]`
///
/// Fetches the site's native copy template, merges optional overrides, then follows
/// the create contract: preview by default, preflight + POST on apply.
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
    let payload = Value::Object(payload_map);

    let path = format!("/api/v2/document/{}", encode_segment(doctype));

    if !apply {
        return output::ok(preview_envelope(client, "POST", &path, &payload));
    }

    match client.post(&path, &payload).await {
        Ok(v) => output::ok(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// The documented preview shape: context, exact native operation, apply pointer.
fn preview_envelope(client: &Client, method: &str, path: &str, payload: &Value) -> Value {
    json!({
        "preview": true,
        "context": {
            "site": client.context().site(),
            "company": client.context().company,
            "privilege": client.context().privilege,
        },
        "operation": { "method": method, "path": path, "payload": payload },
        "preflight": "runs against live DocType metadata on apply",
        "apply_command": "re-run the same command with --apply",
    })
}

/// Unknown-field check for partial payloads (updates), against live metadata.
fn unknown_fields(meta: &Value, payload: &Map<String, Value>) -> Option<String> {
    let fields = meta
        .pointer("/data/fields")
        .or_else(|| meta.pointer("/message/fields"))?
        .as_array()?;
    let known: Vec<&str> = fields
        .iter()
        .filter_map(|f| f.pointer("/fieldname").and_then(Value::as_str))
        .collect();
    let findings: Vec<String> = payload
        .keys()
        .filter(|k| k.as_str() != "doctype" && k.as_str() != "name" && !known.contains(&k.as_str()))
        .map(|k| format!("unknown field '{k}'"))
        .collect();
    if findings.is_empty() {
        None
    } else {
        Some(findings.join("; "))
    }
}

/// Returns a human-readable findings string when the payload violates the live schema.
fn preflight(meta: &Value, payload: &Map<String, Value>) -> Option<String> {
    let fields = meta
        .pointer("/data/fields")
        .or_else(|| meta.pointer("/message/fields"))?
        .as_array()?;

    let known: Vec<&str> = fields
        .iter()
        .filter_map(|f| f.pointer("/fieldname").and_then(Value::as_str))
        .collect();
    let mut findings: Vec<String> = Vec::new();

    for key in payload.keys() {
        if key == "doctype" || key == "name" {
            continue;
        }
        if !known.contains(&key.as_str()) {
            findings.push(format!("unknown field '{key}'"));
        }
    }

    for f in fields {
        let mandatory = f.pointer("/reqd").and_then(Value::as_i64).unwrap_or(0) == 1;
        let has_default = f.pointer("/default").map(|d| !d.is_null()).unwrap_or(false);
        if mandatory && !has_default {
            if let Some(fieldname) = f.pointer("/fieldname").and_then(Value::as_str) {
                if !payload.contains_key(fieldname) {
                    findings.push(format!("missing mandatory field '{fieldname}'"));
                }
            }
        }
    }

    if findings.is_empty() {
        None
    } else {
        Some(findings.join("; "))
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
    fn preflight_rejects_unknown_field() {
        let payload = json!({ "customer_name": "Acme", "bogus": 1 });
        let findings = preflight(&meta_fixture(), payload.as_object().unwrap());
        assert!(findings.unwrap().contains("unknown field 'bogus'"));
    }

    #[test]
    fn preflight_rejects_missing_mandatory_without_default() {
        let payload = json!({ "territory": "All Territories" });
        let findings = preflight(&meta_fixture(), payload.as_object().unwrap());
        assert!(findings
            .unwrap()
            .contains("missing mandatory field 'customer_name'"));
    }

    #[test]
    fn preflight_accepts_valid_payload_and_defaults() {
        let payload = json!({ "customer_name": "Acme" });
        assert!(preflight(&meta_fixture(), payload.as_object().unwrap()).is_none());
    }
}
