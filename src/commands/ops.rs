use serde_json::{json, Value};

use crate::output;
use crate::transport::Client;

/// `erpnext ops doctor` — read-only checks of the CLI's own operating ground.
/// Verifies our functionality, not ERPNext's permission implementation.
pub async fn doctor(client: &Client) -> i32 {
    let mut checks: Vec<Value> = Vec::new();
    let mut failed = false;

    // 1. Endpoint answers.
    match client.ping().await {
        Ok(_) => checks.push(check("endpoint_answers", "pass", client.context().site())),
        Err(e) => {
            failed = true;
            checks.push(check("endpoint_answers", "fail", &e.to_string()));
        }
    }

    // 2. Credentials authenticate; report the identity the site sees.
    match client.whoami().await {
        Ok(user) => {
            checks.push(check("authenticated_identity", "pass", &user));
            // 3. Privilege separation: the operator identity must not be Administrator.
            let claims_operator = matches!(
                client.context().privilege,
                crate::config::PrivilegeMode::Operator
            );
            if claims_operator && user == "Administrator" {
                failed = true;
                checks.push(check(
                    "credential_separation",
                    "fail",
                    "context claims operator privilege but authenticates as Administrator",
                ));
            } else {
                checks.push(check(
                    "credential_separation",
                    "pass",
                    "privilege mode matches identity",
                ));
            }
        }
        Err(e) => {
            failed = true;
            checks.push(check("authenticated_identity", "fail", &e.to_string()));
            checks.push(check(
                "credential_separation",
                "indeterminate",
                "identity could not be resolved",
            ));
        }
    }

    // 4. Selected company exists on the site (when one is selected).
    match client.context().company.clone() {
        None => checks.push(check(
            "company_selected",
            "indeterminate",
            "no company selected in context",
        )),
        Some(company) => {
            let path = format!(
                "/api/v2/document/Company/{}",
                crate::transport::encode_segment(&company)
            );
            match client.get(&path).await {
                Ok(_) => checks.push(check("company_selected", "pass", &company)),
                Err(e) => {
                    failed = true;
                    checks.push(check(
                        "company_selected",
                        "fail",
                        &format!("{company}: {e}"),
                    ));
                }
            }
        }
    }

    let verdict = if failed { "fail" } else { "pass" };
    let exit = if failed {
        output::EXIT_API
    } else {
        output::EXIT_OK
    };
    output::ok(json!({ "verdict": verdict, "checks": checks }));
    exit
}

fn check(name: &str, state: &str, detail: &str) -> Value {
    json!({ "check": name, "state": state, "detail": detail })
}
