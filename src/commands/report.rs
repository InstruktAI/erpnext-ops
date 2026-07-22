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

    let response = match fetch(client, &params, REPORT_TIMEOUT).await {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let (columns, rows) = match report_payload(&response) {
        Ok(payload) => payload,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Context, Credentials, PrivilegeMode};
    use httpmock::Method::POST;
    use httpmock::MockServer;
    use url::Url;

    fn context(site_url: &str) -> Context {
        Context {
            site_url: Url::parse(site_url).expect("url"),
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

    #[tokio::test(flavor = "current_thread")]
    // spec: project/spec/feature/erp-operations/reporting UC-3
    async fn public_report_run_uses_a_distinct_finite_bound() {
        let server = MockServer::start_async().await;
        let report = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/api/method/frappe.desk.query_report.run");
                then.status(200)
                    .delay(std::time::Duration::from_millis(25))
                    .json_body(json!({ "message": {
                        "columns": [],
                        "result": [],
                    }}));
            })
            .await;
        let client = Client::with_request_timeout(
            context(&server.base_url()),
            std::time::Duration::from_millis(1),
        )
        .expect("client");

        let completed = run(&client, "Profit and Loss", &[]).await;

        assert_eq!(completed, output::EXIT_OK);
        report.assert_async().await;
    }

    #[tokio::test(flavor = "current_thread")]
    // spec: project/spec/feature/erp-operations/reporting UC-3
    async fn public_report_run_surfaces_transport_failure() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("available local port");
        let site_url = format!("http://{}", listener.local_addr().expect("local address"));
        drop(listener);
        let client = Client::new(context(&site_url)).expect("client");

        let stalled = run(&client, "Profit and Loss", &[]).await;

        assert_eq!(stalled, output::EXIT_API);
    }

    #[test]
    fn malformed_report_payload_is_a_shape_failure() {
        let error = report_payload(&json!({ "message": { "columns": [] } }))
            .expect_err("missing rows must never become an empty successful report");

        assert!(matches!(error, ApiError::Shape(_)));
    }
}
