use serde_json::{json, Value};

use crate::output;
use crate::transport::{encode_segment, Client};

/// `erpnext workflow submit|cancel <doctype> <name> [--apply]` and
/// `erpnext workflow method <doctype> <name> <method> [--data <json>] [--apply]`.
///
/// All run a whitelisted controller method on the document through the native
/// `method/{method}` envelope: preview shows the exact native request; `--apply`
/// sends it and reports the site's result — the site owns docstatus semantics.
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
            "context": {
                "site": client.context().site(),
                "company": client.context().company,
                "privilege": client.context().privilege,
            },
            "operation": { "method": "POST", "path": path, "payload": payload },
            "apply_command": "re-run the same command with --apply",
        }));
    }

    match client.post(&path, &payload).await {
        Ok(v) => output::ok(v),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}
