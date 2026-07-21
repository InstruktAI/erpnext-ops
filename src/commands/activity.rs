use serde_json::{json, Value};

use crate::output;
use crate::transport::{encode_segment, ApiError, Client};

/// The activity trail is thin operations over native records: `Comment` for notes,
/// `Communication` for medium-carrying messages, `ToDo` for open loops. Capture is a
/// routine internal write and proceeds directly per the operating disciplines.
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
    let mut created: Vec<Value> = Vec::new();

    let content = match capture.verbatim {
        Some(verbatim) => format!("{}\n\n> {}", capture.summary, verbatim),
        None => capture.summary.to_owned(),
    };

    let record = match capture.medium {
        Some(medium) => {
            let mut payload = json!({
                "communication_type": "Communication",
                "communication_medium": medium,
                "subject": capture.summary,
                "content": content,
                "sent_or_received": "Received",
            });
            if let Some((doctype, name)) = capture.reference {
                payload["reference_doctype"] = json!(doctype);
                payload["reference_name"] = json!(name);
            }
            ("Communication", payload)
        }
        None => {
            let mut payload = json!({
                "comment_type": "Comment",
                "content": content,
            });
            if let Some((doctype, name)) = capture.reference {
                payload["reference_doctype"] = json!(doctype);
                payload["reference_name"] = json!(name);
            }
            ("Comment", payload)
        }
    };

    match client
        .post(
            &format!("/api/v2/document/{}", encode_segment(record.0)),
            &record.1,
        )
        .await
    {
        Ok(v) => created.push(json!({ "doctype": record.0, "result": v.pointer("/data/name") })),
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }

    if let Some(follow_up) = capture.follow_up {
        let mut payload = json!({
            "description": follow_up,
            "status": "Open",
        });
        if let Some(due) = capture.due {
            payload["date"] = json!(due);
        }
        if let Some((doctype, name)) = capture.reference {
            payload["reference_type"] = json!(doctype);
            payload["reference_name"] = json!(name);
        }
        match client.post("/api/v2/document/ToDo", &payload).await {
            Ok(v) => created.push(json!({ "doctype": "ToDo", "result": v.pointer("/data/name") })),
            Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
        }
    }

    output::ok(json!({ "captured": created }))
}

/// `erpnext activity list` — time-ordered native records for a reference and window.
pub async fn list(
    client: &Client,
    reference: Option<(&str, &str)>,
    since: Option<&str>,
    until: Option<&str>,
) -> i32 {
    let mut records: Vec<Value> = Vec::new();

    for (doctype, ref_dt_field, ref_name_field, text_field) in [
        ("Comment", "reference_doctype", "reference_name", "content"),
        (
            "Communication",
            "reference_doctype",
            "reference_name",
            "content",
        ),
    ] {
        match query(
            client,
            doctype,
            ref_dt_field,
            ref_name_field,
            text_field,
            reference,
            since,
            until,
        )
        .await
        {
            Ok(mut rows) => records.append(&mut rows),
            Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
        }
    }

    records.sort_by(|a, b| {
        let ka = a.pointer("/creation").and_then(Value::as_str).unwrap_or("");
        let kb = b.pointer("/creation").and_then(Value::as_str).unwrap_or("");
        ka.cmp(kb)
    });

    output::ok(json!({ "activity": records }))
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
        encode_segment(doctype),
        encode_segment(r#"["name","description","date","reference_type","reference_name"]"#),
        encode_segment(&rendered),
    );
    match client.get(&path).await {
        Ok(v) => {
            let rows = v.pointer("/data").cloned().unwrap_or_else(|| json!([]));
            output::ok(json!({ "open_loops": rows }))
        }
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

#[allow(clippy::too_many_arguments)]
async fn query(
    client: &Client,
    doctype: &str,
    ref_dt_field: &str,
    ref_name_field: &str,
    text_field: &str,
    reference: Option<(&str, &str)>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<Vec<Value>, ApiError> {
    let mut filters: Vec<Value> = Vec::new();
    if let Some((ref_dt, ref_name)) = reference {
        filters.push(json!([doctype, ref_dt_field, "=", ref_dt]));
        filters.push(json!([doctype, ref_name_field, "=", ref_name]));
    }
    if let Some(since) = since {
        filters.push(json!([doctype, "creation", ">=", since]));
    }
    if let Some(until) = until {
        filters.push(json!([doctype, "creation", "<=", until]));
    }

    let rendered = serde_json::to_string(&filters).unwrap_or_default();
    let path = format!(
        "/api/v2/document/{}?fields={}&filters={}",
        encode_segment(doctype),
        encode_segment(&format!(r#"["name","creation","{text_field}"]"#)),
        encode_segment(&rendered),
    );
    let listing = client.get(&path).await?;
    let rows = listing
        .pointer("/data")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    Ok(rows
        .into_iter()
        .map(|mut row| {
            if let Some(obj) = row.as_object_mut() {
                obj.insert("doctype".into(), json!(doctype));
            }
            row
        })
        .collect())
}
