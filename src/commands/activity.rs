use serde_json::{json, Value};

use crate::output;
use crate::transport::{encode_segment, Client};

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
        // A follow-up is a `ToDo` carrying its own reference to the business document.
        // The assignment API refuses a second assignment of one user to one document and
        // reports that refusal as success, which caps a party at a single open commitment
        // and loses every later one silently. A business owes several at once, so the
        // record is created directly and read back.
        let owner = match client.whoami().await {
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
        let mut payload = json!({
            "description": follow_up,
            "reference_type": ref_doctype,
            "reference_name": ref_name,
            "allocated_to": owner,
            "status": "Open",
        });
        if let Some(due) = capture.due {
            payload["date"] = json!(due);
        }
        match client.post("/api/v2/document/ToDo", &payload).await {
            Ok(v) => {
                let name = v
                    .pointer("/data/name")
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                match name {
                    Some(name) => created.push(json!({ "kind": "ToDo", "name": name })),
                    None => {
                        return output::fail_with_data(
                            "follow-up-unrecorded",
                            "the site accepted the follow-up without naming a record; the \
                             commitment is not stored",
                            json!({ "captured": created }),
                            output::EXIT_API,
                        )
                    }
                }
            }
            Err(e) => {
                return output::fail_with_data(
                    e.kind(),
                    &format!("contact recorded but follow-up failed: {e}"),
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

    let records = timeline(&docinfo, since, until);

    output::ok(
        json!({ "reference": { "doctype": ref_doctype, "name": ref_name }, "activity": records }),
    )
}

/// Fold a document's native timeline into one ordered trail.
///
/// The site groups the timeline by kind: follow-ups arrive as assignments, and a
/// communication names its actor as the sender rather than the record's owner.
pub fn timeline(docinfo: &Value, since: Option<&str>, until: Option<&str>) -> Vec<Value> {
    let mut records: Vec<Value> = Vec::new();
    // The site groups a document's timeline by kind. Follow-ups reach it as assignments,
    // and a communication names its actor as the sender rather than the record's owner.
    for (key, kind, text_field) in [
        ("comments", "Comment", "content"),
        ("communications", "Communication", "content"),
        ("assignments", "Follow-up", "description"),
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
                .or_else(|| row.pointer("/communication_date"))
                .and_then(Value::as_str)
                .unwrap_or("");
            // A record the site timestamps can be placed in a window. One it does not —
            // an open assignment — is kept, because a bounded read that drops the
            // outstanding work defeats the reason for reading it.
            if !creation.is_empty() {
                if since.map(|s| creation < s).unwrap_or(false) {
                    continue;
                }
                if until.map(|u| creation > u).unwrap_or(false) {
                    continue;
                }
            }
            records.push(json!({
                "kind": kind,
                "name": row.pointer("/name"),
                "creation": creation,
                "text": row.pointer(&format!("/{text_field}")),
                "actor": row
                    .pointer("/owner")
                    .or_else(|| row.pointer("/sender"))
                    .cloned()
                    .unwrap_or(Value::Null),
                "status": row.pointer("/status").cloned().unwrap_or(Value::Null),
            }));
        }
    }

    records.sort_by(|a, b| {
        let ka = a.pointer("/creation").and_then(Value::as_str).unwrap_or("");
        let kb = b.pointer("/creation").and_then(Value::as_str).unwrap_or("");
        ka.cmp(kb)
    });

    records
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

/// `erpnext activity close-loop <name> [--note <text>]` — honour an open commitment.
///
/// An open loop that can never close turns the outstanding view into noise, so closing is
/// part of the same atom as opening. The stored status is read back: a site that accepts
/// the change while leaving the loop open is reported as a failure, never as success.
pub async fn close_loop(client: &Client, name: &str, note: Option<&str>) -> i32 {
    let path = format!("/api/v2/document/ToDo/{}", encode_segment(name));

    let before = match client.get(&path).await {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };
    let reference = (
        before
            .pointer("/data/reference_type")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned(),
        before
            .pointer("/data/reference_name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned(),
    );

    if let Err(e) = client.put(&path, &json!({ "status": "Closed" })).await {
        return output::fail(e.kind(), &e.to_string(), output::EXIT_API);
    }

    let stored = match client.get(&path).await {
        Ok(v) => v
            .pointer("/data/status")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned(),
        Err(e) => {
            return output::fail(
                e.kind(),
                &format!("close sent but the resulting state could not be read: {e}"),
                output::EXIT_API,
            )
        }
    };
    if stored != "Closed" {
        return output::fail(
            "loop-still-open",
            &format!(
                "site accepted the request but follow-up '{name}' is still {stored}; the \
                 commitment is not closed"
            ),
            output::EXIT_API,
        );
    }

    // What honoured the commitment belongs on the business document's trail, where the
    // next person reading that party's history will find it.
    let mut recorded_note = Value::Null;
    if let (Some(text), false) = (note, reference.0.is_empty()) {
        let params = json!({
            "reference_doctype": reference.0,
            "reference_name": reference.1,
            "content": text,
            "comment_email": "",
            "comment_by": "",
        });
        match client
            .post("/api/method/frappe.desk.form.utils.add_comment", &params)
            .await
        {
            Ok(v) => recorded_note = v.pointer("/message/name").cloned().unwrap_or(Value::Null),
            Err(e) => {
                return output::fail_with_data(
                    e.kind(),
                    &format!("loop closed but the note was not recorded: {e}"),
                    json!({ "closed": name }),
                    output::EXIT_API,
                )
            }
        }
    }

    output::ok(json!({
        "closed": name,
        "status": stored,
        "reference": { "doctype": reference.0, "name": reference.1 },
        "note": recorded_note,
    }))
}
