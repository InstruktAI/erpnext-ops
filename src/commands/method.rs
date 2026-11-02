use serde_json::{json, Value};

use crate::_preview::{self, ReplayArgs};
use crate::output;
use crate::transport::{encode_segment, ApiError, Client};

/// Invoke a whitelisted module-level method without assigning business meaning to
/// its opaque arguments or result.
pub async fn call(client: &Client, method: &str, data: Option<&str>, apply: bool) -> i32 {
    let payload = match data {
        None => json!({}),
        Some(raw) => match serde_json::from_str(raw) {
            Ok(value @ Value::Object(_)) => value,
            Ok(_) => {
                return output::fail(
                    "payload",
                    "--data must be a JSON object",
                    output::EXIT_CONFIG,
                )
            }
            Err(error) => {
                return output::fail(
                    "payload",
                    &format!("--data is not valid JSON: {error}"),
                    output::EXIT_CONFIG,
                )
            }
        },
    };
    let path = format!("/api/method/{}", encode_segment(method));

    if !apply {
        let requested_data = data.map(|_| &payload);
        let apply_command = _preview::apply_command(
            client.context(),
            &ReplayArgs::MethodCall {
                method,
                data: requested_data,
            },
        );
        return output::ok(
            _preview::Preview {
                context: _preview::context_json(client.context()),
                operation: json!({ "method": "POST", "path": path, "payload": payload }),
                target: None,
                schema: None,
                preflight: _preview::opaque_preflight_json(),
                apply_command,
                idempotency: None,
                intended_state: None,
                persistence: None,
            }
            .into_json(),
        );
    }

    match client.post(&path, &payload).await {
        Ok(response) => match method_payload(&response) {
            Ok(result) => output::ok(result),
            Err(error) => output::fail(error.kind(), &error.to_string(), output::EXIT_API),
        },
        Err(error) => output::fail(error.kind(), &error.to_string(), output::EXIT_API),
    }
}

fn method_payload(response: &Value) -> Result<Value, ApiError> {
    let object = response
        .as_object()
        .ok_or_else(|| ApiError::Shape("method response is not a JSON object".into()))?;
    Ok(object.get("message").cloned().unwrap_or(Value::Null))
}
