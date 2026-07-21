use serde_json::{json, Value};

use crate::output;
use crate::transport::{encode_segment, ApiError, Client};

/// A resolution query never guesses: it returns exactly one reference, a structured
/// candidate list, or a structured miss. Replayable procedures resolve natural keys
/// here before any create, so a rerun finds the record instead of duplicating it.
struct Candidate {
    doctype: &'static str,
    name: String,
    title: String,
}

/// `erpnext resolve party <query>` — customers and suppliers by exact name or title.
pub async fn party(client: &Client, query: &str) -> i32 {
    let mut candidates: Vec<Candidate> = Vec::new();
    for (doctype, title_field) in [("Customer", "customer_name"), ("Supplier", "supplier_name")] {
        match collect(client, doctype, title_field, query).await {
            Ok(mut found) => candidates.append(&mut found),
            Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
        }
    }
    render(candidates)
}

/// `erpnext resolve item <query>` — items by exact code or item name.
pub async fn item(client: &Client, query: &str) -> i32 {
    match collect(client, "Item", "item_name", query).await {
        Ok(candidates) => render(candidates),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// `erpnext resolve document <doctype> <query>` — a document by its exact name.
pub async fn document(client: &Client, doctype: &str, query: &str) -> i32 {
    let path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(query)
    );
    match client.get(&path).await {
        Ok(doc) => {
            let name = doc
                .pointer("/data/name")
                .and_then(Value::as_str)
                .unwrap_or(query)
                .to_owned();
            output::ok(json!({
                "resolution": "resolved",
                "reference": { "doctype": doctype, "name": name },
            }))
        }
        Err(ApiError::NotFound { .. }) => output::ok(json!({ "resolution": "none" })),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

/// Exact-match candidates for one doctype: by document name and by its title field.
async fn collect(
    client: &Client,
    doctype: &'static str,
    title_field: &str,
    query: &str,
) -> Result<Vec<Candidate>, ApiError> {
    let mut candidates: Vec<Candidate> = Vec::new();

    for field in ["name", title_field] {
        let filters =
            serde_json::to_string(&json!([[doctype, field, "=", query]])).unwrap_or_default();
        let path = format!(
            "/api/v2/document/{}?fields={}&filters={}",
            encode_segment(doctype),
            encode_segment(&format!("[\"name\",\"{title_field}\"]")),
            encode_segment(&filters),
        );
        let listing = client.get(&path).await?;
        let rows = listing
            .pointer("/data")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for row in rows {
            let Some(name) = row.pointer("/name").and_then(Value::as_str) else {
                continue;
            };
            if candidates
                .iter()
                .any(|c| c.doctype == doctype && c.name == name)
            {
                continue;
            }
            let title = row
                .pointer(&format!("/{title_field}"))
                .and_then(Value::as_str)
                .unwrap_or(name)
                .to_owned();
            candidates.push(Candidate {
                doctype,
                name: name.to_owned(),
                title,
            });
        }
    }

    Ok(candidates)
}

fn render(candidates: Vec<Candidate>) -> i32 {
    match candidates.len() {
        0 => output::ok(json!({ "resolution": "none" })),
        1 => {
            let c = &candidates[0];
            output::ok(json!({
                "resolution": "resolved",
                "reference": { "doctype": c.doctype, "name": c.name, "title": c.title },
            }))
        }
        _ => output::ok(json!({
            "resolution": "ambiguous",
            "candidates": candidates
                .iter()
                .map(|c| json!({ "doctype": c.doctype, "name": c.name, "title": c.title }))
                .collect::<Vec<_>>(),
        })),
    }
}
