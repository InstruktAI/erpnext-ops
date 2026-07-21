use serde_json::{json, Map, Value};

use crate::output;
use crate::transport::Client;

/// `erpnext report run <report-name> [--filter k=v]...` — run a native report through
/// Frappe's permission-aware report runner and return machine-readable columns/rows.
pub async fn run(client: &Client, report_name: &str, filters: &[(String, String)]) -> i32 {
    let mut filter_map = Map::new();
    for (key, value) in filters {
        filter_map.insert(key.clone(), Value::String(value.clone()));
    }

    let params = json!({
        "report_name": report_name,
        "filters": filter_map,
        "ignore_prepared_report": 1,
    });

    let response = match client
        .post("/api/method/frappe.desk.query_report.run", &params)
        .await
    {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let message = response.pointer("/message").cloned().unwrap_or(Value::Null);
    let columns = message
        .pointer("/columns")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let rows = message
        .pointer("/result")
        .cloned()
        .unwrap_or_else(|| json!([]));

    output::ok(json!({
        "report": report_name,
        "columns": columns,
        "rows": rows,
    }))
}
