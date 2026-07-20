use serde_json::{json, Map, Value};

use crate::output;
use crate::transport::{ApiError, Client};

fn encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

fn kind(e: &ApiError) -> &'static str {
    match e {
        ApiError::Auth { .. } => "auth",
        ApiError::Forbidden { .. } => "permission",
        ApiError::NotFound { .. } => "not_found",
        ApiError::Rejected { .. } => "rejected",
        ApiError::Transport(_) => "transport",
        ApiError::Shape(_) => "shape",
    }
}

/// `erpnext doc list <doctype> [--filter k=v]... [--limit n]`
pub async fn list(client: &Client, doctype: &str, filters: &[(String, String)], limit: u32) -> i32 {
    let filter_json: Vec<Value> = filters
        .iter()
        .map(|(k, v)| json!([doctype, k, "=", v]))
        .collect();
    let mut path = format!(
        "/api/v2/document/{}?limit={}&fields={}",
        encode(doctype),
        limit,
        encode("[\"name\"]")
    );
    if !filter_json.is_empty() {
        let rendered = serde_json::to_string(&filter_json).unwrap_or_default();
        path.push_str(&format!("&filters={}", encode(&rendered)));
    }
    match client.get(&path).await {
        Ok(v) => output::ok(v),
        Err(e) => output::fail(kind(&e), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc get <doctype> <name>`
pub async fn get(client: &Client, doctype: &str, name: &str) -> i32 {
    let path = format!("/api/v2/document/{}/{}", encode(doctype), encode(name));
    match client.get(&path).await {
        Ok(v) => output::ok(v),
        Err(e) => output::fail(kind(&e), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext doc create <doctype> --data <json> [--apply]`
///
/// Preview by default: shows the exact native operation that would be sent, without
/// sending it. `--apply` runs basic live-metadata preflight, sends the native request,
/// and reports the site's result.
pub async fn create(client: &Client, doctype: &str, data: &str, apply: bool) -> i32 {
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

    let path = format!("/api/v2/document/{}", encode(doctype));

    if !apply {
        return output::ok(json!({
            "preview": true,
            "context": {
                "site": client.context().site(),
                "company": client.context().company,
                "privilege": client.context().privilege,
            },
            "operation": { "method": "POST", "path": path, "payload": payload },
            "preflight": "runs against live DocType metadata on apply",
            "apply_command": format!("erpnext doc create {doctype} --data '<same payload>' --apply"),
        }));
    }

    // Basic preflight against live metadata: unknown fields and missing mandatory fields.
    let meta = match super::meta::fetch(client, doctype).await {
        Ok(m) => m,
        Err(e) => {
            return output::fail(
                kind(&e),
                &format!("metadata preflight failed: {e}"),
                output::EXIT_API,
            )
        }
    };
    if let Some(findings) = preflight(&meta, payload_map) {
        return output::fail("preflight", &findings, output::EXIT_API);
    }

    match client.post(&path, &payload).await {
        Ok(v) => output::ok(v),
        Err(e) => output::fail(kind(&e), &e.to_string(), output::EXIT_API),
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
