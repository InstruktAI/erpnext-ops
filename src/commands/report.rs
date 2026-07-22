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
    use httpmock::prelude::*;
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

    #[tokio::test]
    // spec: project/spec/feature/erp-operations/reporting UC-3
    async fn report_bound_overrides_the_ordinary_request_bound() {
        let server = MockServer::start_async().await;
        let runner = server.mock(|when, then| {
            when.method(POST)
                .path("/api/method/frappe.desk.query_report.run");
            then.status(200).json_body(json!({ "message": {
                "columns": [],
                "result": [],
            }}));
        });
        let client =
            Client::with_request_timeout(context(&server.base_url()), std::time::Duration::ZERO)
                .expect("client");

        let response = fetch(
            &client,
            &json!({ "report_name": "Profit and Loss" }),
            REPORT_TIMEOUT,
        )
        .await
        .expect("report-specific timeout overrides the immediate ordinary bound");

        runner.assert();
        assert!(report_payload(&response).is_ok());
    }

    #[tokio::test]
    // spec: project/spec/feature/erp-operations/reporting UC-3
    async fn finite_report_bound_surfaces_as_transport_failure() {
        let server = MockServer::start_async().await;
        let client = Client::new(context(&server.base_url())).expect("client");

        let error = fetch(
            &client,
            &json!({ "report_name": "Profit and Loss" }),
            std::time::Duration::ZERO,
        )
        .await
        .expect_err("an exhausted report bound must fail");

        assert!(matches!(error, ApiError::Transport(_)));
    }

    #[test]
    fn malformed_report_payload_is_a_shape_failure() {
        let error = report_payload(&json!({ "message": { "columns": [] } }))
            .expect_err("missing rows must never become an empty successful report");

        assert!(matches!(error, ApiError::Shape(_)));
    }
}
