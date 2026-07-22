use serde_json::{json, Map, Value};

use crate::output;
use crate::transport::{ApiError, Client};

const REPORT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(120);

/// `erpnext report run <report-name> [--filter k=v]...` — run a native report through
/// Frappe's permission-aware report runner and return machine-readable columns/rows.
pub async fn run(client: &Client, report_name: &str, filters: &[(String, String)]) -> i32 {
    run_with_failure_emitter(client, report_name, filters, output::fail).await
}

pub async fn run_with_failure_emitter<F>(
    client: &Client,
    report_name: &str,
    filters: &[(String, String)],
    mut emit_failure: F,
) -> i32
where
    F: FnMut(&str, &str, i32) -> i32,
{
    let mut filter_map = Map::new();
    for (key, value) in filters {
        filter_map.insert(key.clone(), Value::String(value.clone()));
    }

    let params = json!({
        "report_name": report_name,
        "filters": filter_map,
        "ignore_prepared_report": 1,
    });

    let response = match fetch(client, &params, REPORT_TIMEOUT).await {
        Ok(v) => v,
        Err(e) => return emit_failure(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let (columns, rows) = match report_payload(&response) {
        Ok(payload) => payload,
        Err(e) => return emit_failure(e.kind(), &e.to_string(), output::EXIT_API),
    };

    output::ok(json!({
        "report": report_name,
        "columns": columns,
        "rows": rows,
    }))
}

async fn fetch(
    client: &Client,
    params: &Value,
    timeout: std::time::Duration,
) -> Result<Value, ApiError> {
    client
        .post_with_timeout("/api/method/frappe.desk.query_report.run", params, timeout)
        .await
}

fn report_payload(response: &Value) -> Result<(Value, Value), ApiError> {
    let message = response
        .get("message")
        .and_then(Value::as_object)
        .ok_or_else(|| ApiError::Shape("report response has no object 'message'".into()))?;
    let columns = message
        .get("columns")
        .filter(|value| value.is_array())
        .cloned()
        .ok_or_else(|| ApiError::Shape("report response has no array 'message.columns'".into()))?;
    let rows = message
        .get("result")
        .filter(|value| value.is_array())
        .cloned()
        .ok_or_else(|| ApiError::Shape("report response has no array 'message.result'".into()))?;

    Ok((columns, rows))
}
