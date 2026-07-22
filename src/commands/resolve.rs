use serde_json::{json, Value};

use crate::output;
use crate::transport::{encode_segment, ApiError, Client};

/// A resolution query never guesses: it returns exactly one reference, a structured
/// candidate list, or a structured miss. Replayable procedures resolve natural keys
/// here before any create, so a rerun finds the record instead of duplicating it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Reference {
    pub doctype: String,
    pub name: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    Resolved(Reference),
    Ambiguous(Vec<Reference>),
    None,
}

impl Resolution {
    fn envelope(&self) -> Value {
        match self {
            Resolution::Resolved(r) => json!({ "resolution": "resolved", "reference": r }),
            Resolution::Ambiguous(candidates) => {
                json!({ "resolution": "ambiguous", "candidates": candidates })
            }
            Resolution::None => json!({ "resolution": "none" }),
        }
    }
}

/// `erpnext resolve party <query>` — customers and suppliers by exact name or title.
pub async fn party(client: &Client, query: &str) -> i32 {
    render(party_resolution(client, query).await)
}

pub async fn party_resolution(client: &Client, query: &str) -> Result<Resolution, ApiError> {
    let mut candidates: Vec<Reference> = Vec::new();
    for (doctype, title_field) in [("Customer", "customer_name"), ("Supplier", "supplier_name")] {
        candidates.append(&mut collect(client, doctype, title_field, query).await?);
    }
    Ok(classify(candidates))
}

/// `erpnext resolve item <query>` — items by exact code or item name.
pub async fn item(client: &Client, query: &str) -> i32 {
    render(item_resolution(client, query).await)
}

pub async fn item_resolution(client: &Client, query: &str) -> Result<Resolution, ApiError> {
    Ok(classify(collect(client, "Item", "item_name", query).await?))
}

/// `erpnext resolve document <doctype> <query>` — a document by its exact name.
pub async fn document(client: &Client, doctype: &str, query: &str) -> i32 {
    render(document_resolution(client, doctype, query).await)
}

pub async fn document_resolution(
    client: &Client,
    doctype: &str,
    query: &str,
) -> Result<Resolution, ApiError> {
    let meta = crate::commands::meta::fetch(client, doctype).await?;
    if let Some(title_field) = meta
        .pointer("/data/title_field")
        .and_then(Value::as_str)
        .filter(|field| !field.is_empty() && *field != "name")
    {
        return Ok(classify(
            collect(client, doctype, title_field, query).await?,
        ));
    }

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
            Ok(Resolution::Resolved(Reference {
                doctype: doctype.to_owned(),
                name: name.clone(),
                title: name,
            }))
        }
        Err(ApiError::NotFound { .. }) => Ok(Resolution::None),
        Err(e) => Err(e),
    }
}

/// Exact-match candidates for one doctype: by document name and by its title field.
async fn collect(
    client: &Client,
    doctype: &str,
    title_field: &str,
    query: &str,
) -> Result<Vec<Reference>, ApiError> {
    let mut candidates: Vec<Reference> = Vec::new();

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
            candidates.push(Reference {
                doctype: doctype.to_owned(),
                name: name.to_owned(),
                title,
            });
        }
    }

    Ok(candidates)
}

fn classify(candidates: Vec<Reference>) -> Resolution {
    match candidates.len() {
        0 => Resolution::None,
        1 => Resolution::Resolved(
            candidates
                .into_iter()
                .next()
                .unwrap_or_else(|| unreachable!("length checked")),
        ),
        _ => Resolution::Ambiguous(candidates),
    }
}

fn render(outcome: Result<Resolution, ApiError>) -> i32 {
    match outcome {
        Ok(resolution) => output::ok(resolution.envelope()),
        Err(e) => output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::json;
    use url::Url;

    use super::*;
    use crate::config::{Context, Credentials, PrivilegeMode};

    fn client_for(server: &MockServer) -> Client {
        Client::new(Context {
            site_url: Url::parse(&server.base_url()).expect("mock server url"),
            company_slug: "test-co".into(),
            company: None,
            privilege: PrivilegeMode::Operator,
            credentials: Credentials {
                user: None,
                api_key: "test-key".into(),
                api_secret: "test-secret".into(),
            },
        })
        .expect("client")
    }

    #[tokio::test]
    async fn document_resolution_uses_the_live_title_field() {
        let server = MockServer::start_async().await;
        let meta = server.mock(|when, then| {
            when.method(GET).path("/api/v2/doctype/Project/meta");
            then.status(200).json_body(json!({
                "data": { "title_field": "project_name" }
            }));
        });
        let by_name = server.mock(|when, then| {
            when.method(GET)
                .path("/api/v2/document/Project")
                .query_param(
                    "filters",
                    r#"[["Project","name","=","Sandbox Solar Installation"]]"#,
                );
            then.status(200).json_body(json!({ "data": [] }));
        });
        let by_title = server.mock(|when, then| {
            when.method(GET)
                .path("/api/v2/document/Project")
                .query_param(
                    "filters",
                    r#"[["Project","project_name","=","Sandbox Solar Installation"]]"#,
                );
            then.status(200).json_body(json!({
                "data": [{
                    "name": "PROJ-0001",
                    "project_name": "Sandbox Solar Installation"
                }]
            }));
        });

        let result = document_resolution(
            &client_for(&server),
            "Project",
            "Sandbox Solar Installation",
        )
        .await
        .expect("resolution");

        assert_eq!(
            result,
            Resolution::Resolved(Reference {
                doctype: "Project".into(),
                name: "PROJ-0001".into(),
                title: "Sandbox Solar Installation".into(),
            })
        );
        meta.assert();
        by_name.assert();
        by_title.assert();
    }
}
