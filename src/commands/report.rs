use serde_json::{json, Map, Value};

use crate::output;
use crate::transport::{ApiError, Client};

const REPORT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(120);

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

    let request = match build_request(client, &params) {
        Ok(request) => request,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };
    let response = match client.execute(request).await {
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

fn build_request(client: &Client, params: &Value) -> Result<reqwest::Request, ApiError> {
    client.build_post_with_timeout(
        "/api/method/frappe.desk.query_report.run",
        params,
        REPORT_TIMEOUT,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Context, Credentials, PrivilegeMode};
    use url::Url;

    fn context() -> Context {
        Context {
            site_url: Url::parse("https://erpnext.example").expect("url"),
            company_slug: "test-co".into(),
            company: Some("Test Company".into()),
            privilege: PrivilegeMode::Operator,
            credentials: Credentials {
                user: None,
                api_key: "key".into(),
                api_secret: "secret".into(),
            },
        }
    }

    #[test]
    // spec: project/spec/feature/erp-operations/reporting UC-3
    fn report_request_has_the_exact_finite_long_running_bound() {
        let client = Client::new(context()).expect("client");
        let request = build_request(&client, &json!({ "report_name": "Profit and Loss" }))
            .expect("report request");

        assert_eq!(request.timeout(), Some(&REPORT_TIMEOUT));
        assert_eq!(REPORT_TIMEOUT, std::time::Duration::from_secs(120));
    }
}
