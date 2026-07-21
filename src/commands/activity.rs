use serde_json::{json, Value};

use crate::output;
use crate::transport::Client;

/// Frappe's native communication mediums; validated client-side so a typo surfaces
/// before any request.
const MEDIUMS: [&str; 8] = [
    "Email", "Chat", "Phone", "SMS", "Event", "Meeting", "Visit", "Other",
];

/// The activity trail rides Frappe's permission-aware timeline APIs — never generic
/// CRUD on `Comment`/`Communication`, which would demand widened DocType permissions.
/// Access is checked against the referenced business document by the site itself.
pub struct Capture<'a> {
    pub summary: &'a str,
    pub verbatim: Option<&'a str>,
    pub medium: Option<&'a str>,
    pub reference: Option<(&'a str, &'a str)>,
    pub follow_up: Option<&'a str>,
    pub due: Option<&'a str>,
}

/// `erpnext activity capture` — record business input; optionally open a loop.
pub async fn capture(client: &Client, capture: Capture<'_>) -> i32 {
    let Some((ref_doctype, ref_name)) = capture.reference else {
        return output::fail(
            "reference-required",
            "activity capture records against a business document; pass --reference <doctype> <name>",
            output::EXIT_CONFIG,
        );
    };

    if let Some(medium) = capture.medium {
        if !MEDIUMS.contains(&medium) {
            return output::fail(
                "medium",
                &format!(
                    "medium '{medium}' is not a native value; use one of: {}",
                    MEDIUMS.join(", ")
                ),
                output::EXIT_CONFIG,
            );
        }
    }

    let content = match capture.verbatim {
        Some(verbatim) => format!("{}\n\n> {}", capture.summary, verbatim),
        None => capture.summary.to_owned(),
    };

    let mut created: Vec<Value> = Vec::new();

    let outcome = match capture.medium {
        Some(medium) => {
            // Permission-aware Communication creation; no email is sent.
            let params = json!({
                "doctype": ref_doctype,
                "name": ref_name,
                "content": content,
                "subject": capture.summary,
                "sent_or_received": "Received",
                "send_email": 0,
                "communication_medium": medium,
            });
            client
                .post(
                    "/api/method/frappe.core.doctype.communication.email.make",
                    &params,
                )
                .await
                .map(|v| json!({ "kind": "Communication", "result": v.pointer("/message") }))
        }
        None => {
            // Permission-aware Comment creation on the referenced document's timeline.
            let params = json!({
                "reference_doctype": ref_doctype,
                "reference_name": ref_name,
                "content": content,
                "comment_email": "",
                "comment_by": "",
            });
            client
                .post("/api/method/frappe.desk.form.utils.add_comment", &params)
                .await
                .map(|v| json!({ "kind": "Comment", "result": v.pointer("/message/name") }))
        }
    };

    match outcome {
        Ok(v) => created.push(v),
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }

    if let Some(follow_up) = capture.follow_up {
        // Open loops ride the native assignment API: the ToDo is created internally
        // after a permission check on the referenced document — no widened ToDo CRUD.
        let assignee = match client.whoami().await {
            Ok(user) => user,
            Err(e) => {
                return output::fail_with_data(
                    e.kind(),
                    &format!("follow-up not recorded (identity lookup failed): {e}"),
                    json!({ "captured": created }),
                    output::EXIT_API,
                )
            }
        };
        let mut args = json!({
            "assign_to": [assignee],
            "doctype": ref_doctype,
            "name": ref_name,
            "description": follow_up,
        });
        if let Some(due) = capture.due {
            args["date"] = json!(due);
        }
        match client
            .post("/api/method/frappe.desk.form.assign_to.add", &args)
            .await
        {
            Ok(_) => created.push(json!({ "kind": "ToDo", "result": "assigned" })),
            Err(e) => {
                return output::fail_with_data(
                    e.kind(),
                    &format!("comment recorded but follow-up failed: {e}"),
                    json!({ "captured": created }),
                    output::EXIT_API,
                )
            }
        }
    }

    output::ok(json!({ "captured": created }))
}

/// `erpnext activity list` — the referenced document's timeline, permission-checked
/// by the site through the native docinfo loader.
pub async fn list(
    client: &Client,
    reference: Option<(&str, &str)>,
    since: Option<&str>,
    until: Option<&str>,
) -> i32 {
    let Some((ref_doctype, ref_name)) = reference else {
        return output::fail(
            "reference-required",
            "activity list reads a business document's timeline; pass --reference <doctype> <name>",
            output::EXIT_CONFIG,
        );
    };

    let params = json!({ "doctype": ref_doctype, "name": ref_name });
    let docinfo = match client
        .post("/api/method/frappe.desk.form.load.get_docinfo", &params)
        .await
    {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let mut records: Vec<Value> = Vec::new();
    for (key, kind, text_field) in [
        ("comments", "Comment", "content"),
        ("communications", "Communication", "content"),
        ("todos", "ToDo", "description"),
    ] {
        let rows = docinfo
            .pointer(&format!("/docinfo/{key}"))
            .or_else(|| docinfo.pointer(&format!("/message/docinfo/{key}")))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for row in rows {
            let creation = row
                .pointer("/creation")
                .and_then(Value::as_str)
                .unwrap_or("");
            if since.map(|s| creation < s).unwrap_or(false) {
                continue;
            }
            if until.map(|u| creation > u).unwrap_or(false) {
                continue;
            }
            records.push(json!({
                "kind": kind,
                "name": row.pointer("/name"),
                "creation": creation,
                "text": row.pointer(&format!("/{text_field}")),
                "owner": row.pointer("/owner"),
            }));
        }
    }

    records.sort_by(|a, b| {
        let ka = a.pointer("/creation").and_then(Value::as_str).unwrap_or("");
        let kb = b.pointer("/creation").and_then(Value::as_str).unwrap_or("");
        ka.cmp(kb)
    });

    output::ok(
        json!({ "reference": { "doctype": ref_doctype, "name": ref_name }, "activity": records }),
    )
}

/// `erpnext activity open-loops` — open `ToDo` records, optionally due-bounded.
pub async fn open_loops(
    client: &Client,
    reference: Option<(&str, &str)>,
    due: Option<&str>,
) -> i32 {
    let doctype = "ToDo";
    let mut filters = vec![json!([doctype, "status", "=", "Open"])];
    if let Some((ref_dt, ref_name)) = reference {
        filters.push(json!([doctype, "reference_type", "=", ref_dt]));
        filters.push(json!([doctype, "reference_name", "=", ref_name]));
    }
    if let Some(due) = due {
        filters.push(json!([doctype, "date", "<=", due]));
    }

    let rendered = serde_json::to_string(&filters).unwrap_or_default();
    let path = format!(
        "/api/v2/document/{}?fields={}&filters={}",
        crate::transport::encode_segment(doctype),
        crate::transport::encode_segment(
            r#"["name","description","date","reference_type","reference_name"]"#
        ),
        crate::transport::encode_segment(&rendered),
    );
    match client.get(&path).await {
        Ok(v) => {
            let rows = v.pointer("/data").cloned().unwrap_or_else(|| json!([]));
            output::ok(json!({ "open_loops": rows }))
        }
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}
