//! Stored native relationship proof for a converted document.
//!
//! `doc::map` creates the target document, then this module reads it back and
//! inspects its own Link and Dynamic Link fields — including nested child
//! tables — against the document's selected conversion source. Only an exact
//! match on a recognized native field proves the relationship; incidental text
//! elsewhere in the document proves nothing.

use std::collections::{HashMap, HashSet};

use serde_json::{Map, Value};

use crate::commands::meta;
use crate::transport::{encode_segment, ApiError, Client};

/// Field types whose stored value can legitimately name a doctype without a
/// closed vocabulary constraining it, so they are trusted as a Dynamic Link
/// discriminator on their text alone.
const FREE_TEXT_DISCRIMINATOR_TYPES: &[&str] = &["Data", "Text", "Small Text", "Long Text"];

/// Outcome of inspecting a created document's stored native relationships
/// against the selected conversion source.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum LinkProof {
    /// An exact Link or Dynamic Link field matched the selected source.
    Verified,
    /// A complete inspection found at least one source-expressible field and
    /// none of them matched.
    Missing,
    /// Inspection completed but could not settle verified-or-missing; `reason`
    /// names what the metadata could not resolve — an unrecognized or
    /// permission-hidden candidate, or no source-expressible field at all.
    Unavailable(String),
}

/// The parts of a relationship walk that are fixed for its whole recursion —
/// the collected schema and the selected conversion source's identity — kept
/// out of every function's per-call arguments (field, record, doctype).
struct WalkContext<'a> {
    schema: &'a HashMap<String, Value>,
    source_doctype: &'a str,
    source_name: &'a str,
}

/// Read the created target back and classify its native relationship to the
/// selected conversion source. Returns the native or shape error that stopped
/// inspection before a classification was reached.
pub(super) async fn verify(
    client: &Client,
    source_doctype: &str,
    source_name: &str,
    target_doctype: &str,
    target_name: &str,
) -> Result<LinkProof, ApiError> {
    let record = read_back(client, target_doctype, target_name).await?;
    let schema = collect_schema(client, target_doctype).await?;
    let root_meta = schema
        .get(target_doctype)
        .ok_or_else(|| ApiError::Shape(format!("'{target_doctype}' metadata was not collected")))?;
    let cx = WalkContext {
        schema: &schema,
        source_doctype,
        source_name,
    };

    let mut outcome = Outcome::default();
    let walked = walk_record(target_doctype, root_meta, &record, &cx, &mut outcome);
    if outcome.matched {
        // An exact positive match already collected outranks an unrelated
        // malformed-evidence error the walk hit afterward — see `walk_record`.
        return Ok(LinkProof::Verified);
    }
    walked?;

    Ok(if let Some(reason) = outcome.unavailable_reason {
        LinkProof::Unavailable(reason)
    } else if schema_is_source_expressible(&schema, source_doctype) {
        LinkProof::Missing
    } else {
        LinkProof::Unavailable(format!(
            "'{target_doctype}' carries no Link or Dynamic Link field capable of naming '{source_doctype}'"
        ))
    })
}

/// Read the created target back and confirm the response identifies the
/// document just created. The create response and mapper payload are never
/// relationship evidence — only this stored read-back is.
async fn read_back(client: &Client, doctype: &str, name: &str) -> Result<Value, ApiError> {
    let path = format!(
        "/api/v2/document/{}/{}",
        encode_segment(doctype),
        encode_segment(name)
    );
    let response = client.get(&path).await?;
    let record = response.pointer("/data").ok_or_else(|| {
        ApiError::Shape(format!(
            "read-back of '{doctype}' '{name}' carries no document data"
        ))
    })?;
    let read_doctype = record.pointer("/doctype").and_then(Value::as_str);
    let read_name = record.pointer("/name").and_then(Value::as_str);
    match (read_doctype, read_name) {
        (Some(rd), Some(rn)) if rd == doctype && rn == name => Ok(record.clone()),
        (Some(rd), Some(rn)) => Err(ApiError::Shape(format!(
            "read-back of '{doctype}' '{name}' returned '{rd}' '{rn}'"
        ))),
        (None, _) => Err(ApiError::Shape(format!(
            "read-back of '{doctype}' '{name}' carries no doctype identifier"
        ))),
        (_, None) => Err(ApiError::Shape(format!(
            "read-back of '{doctype}' '{name}' carries no document identifier"
        ))),
    }
}

/// Fetch metadata for `root_doctype` and every child doctype its schema
/// declares via Table or Table MultiSelect fields, transitively, once per
/// distinct doctype — including doctypes behind an empty child array, since
/// expressibility is a schema fact independent of stored rows. A doctype
/// already queued is never re-queued, which terminates a schema cycle.
async fn collect_schema(
    client: &Client,
    root_doctype: &str,
) -> Result<HashMap<String, Value>, ApiError> {
    let mut schema = HashMap::new();
    let mut queue = vec![root_doctype.to_owned()];
    let mut queued: HashSet<String> = queue.iter().cloned().collect();

    while let Some(doctype) = queue.pop() {
        let meta = meta::fetch(client, &doctype).await?;
        let fields = fields_of(&meta).ok_or_else(|| {
            ApiError::Shape(format!(
                "'{doctype}' metadata carries no usable fields array"
            ))
        })?;
        for field in fields {
            let fieldtype = field.get("fieldtype").and_then(Value::as_str).unwrap_or("");
            if fieldtype != "Table" && fieldtype != "Table MultiSelect" {
                continue;
            }
            let child_doctype = field
                .get("options")
                .and_then(Value::as_str)
                .filter(|d| !d.is_empty());
            let Some(child_doctype) = child_doctype else {
                let fieldname = field
                    .get("fieldname")
                    .and_then(Value::as_str)
                    .unwrap_or("?");
                return Err(ApiError::Shape(format!(
                    "'{doctype}' field '{fieldname}' declares a child table with no child doctype"
                )));
            };
            if queued.insert(child_doctype.to_owned()) {
                queue.push(child_doctype.to_owned());
            }
        }
        schema.insert(doctype, meta);
    }

    Ok(schema)
}

/// Field definitions from either the v2 metadata shape (`/data/fields`) or a
/// method-call shape (`/message/fields`).
fn fields_of(meta: &Value) -> Option<&Vec<Value>> {
    meta.pointer("/data/fields")
        .or_else(|| meta.pointer("/message/fields"))
        .and_then(Value::as_array)
}

/// Whether a Link field's declared `options` names the selected source
/// doctype — a schema-level fact independent of any stored value or of
/// whether the field even carries a `fieldname` to look one up with.
fn link_options_name_source(field: &Value, source_doctype: &str) -> bool {
    field
        .get("options")
        .and_then(Value::as_str)
        .is_some_and(|options| !matches!(compare(options, source_doctype), Compare::NoMatch))
}

/// Whether any doctype reachable from the root schema declares a field that
/// could, in principle, name the selected source — independent of any actual
/// stored value. Backs the `not-expressible` classification.
fn schema_is_source_expressible(schema: &HashMap<String, Value>, source_doctype: &str) -> bool {
    schema.values().any(|meta| {
        fields_of(meta)
            .map(|fields| {
                fields.iter().any(|field| {
                    match field.get("fieldtype").and_then(Value::as_str).unwrap_or("") {
                        "Link" => link_options_name_source(field, source_doctype),
                        "Dynamic Link" => {
                            let discriminator_name =
                                field.get("options").and_then(Value::as_str).unwrap_or("");
                            !discriminator_name.is_empty()
                                && fields
                                    .iter()
                                    .find(|f| {
                                        f.get("fieldname").and_then(Value::as_str)
                                            == Some(discriminator_name)
                                    })
                                    .is_some_and(|discriminator_field| {
                                        discriminator_is_recognized(
                                            discriminator_field,
                                            source_doctype,
                                        )
                                    })
                        }
                        _ => false,
                    }
                })
            })
            .unwrap_or(false)
    })
}

#[derive(Default)]
struct Outcome {
    matched: bool,
    unavailable_reason: Option<String>,
}

impl Outcome {
    fn flag_unavailable(&mut self, reason: String) {
        if self.unavailable_reason.is_none() {
            self.unavailable_reason = Some(reason);
        }
    }
}

/// How a stored value compares to the identity it is checked against. Exact
/// equality proves a relationship; a case-only match proves nothing but rules
/// out a confident negative, per the native backend's own collation ambiguity.
enum Compare {
    Match,
    CaseOnly,
    NoMatch,
}

fn compare(candidate: &str, expected: &str) -> Compare {
    if candidate == expected {
        Compare::Match
    } else if candidate.to_lowercase() == expected.to_lowercase() {
        Compare::CaseOnly
    } else {
        Compare::NoMatch
    }
}

fn permlevel_of(field: &Value) -> i64 {
    field.get("permlevel").and_then(Value::as_i64).unwrap_or(0)
}

/// Walk one record — the root document or one child row — against its own
/// doctype's field metadata: evaluate its Link and Dynamic Link fields, and
/// recurse into declared child tables actually present in the record.
fn walk_record(
    doctype: &str,
    meta: &Value,
    record: &Value,
    cx: &WalkContext,
    outcome: &mut Outcome,
) -> Result<(), ApiError> {
    let fields = fields_of(meta).ok_or_else(|| {
        ApiError::Shape(format!(
            "'{doctype}' metadata carries no usable fields array"
        ))
    })?;
    let record = record
        .as_object()
        .ok_or_else(|| ApiError::Shape(format!("'{doctype}' record is not a document object")))?;

    for field in fields {
        let fieldtype = field.get("fieldtype").and_then(Value::as_str).unwrap_or("");
        let Some(fieldname) = field.get("fieldname").and_then(Value::as_str) else {
            if fieldtype == "Link" && link_options_name_source(field, cx.source_doctype) {
                outcome.flag_unavailable(format!(
                    "'{doctype}' declares a Link field naming '{}' with no fieldname",
                    cx.source_doctype
                ));
            }
            continue;
        };

        match fieldtype {
            "Link" => evaluate_link(field, fieldname, record, cx, outcome),
            "Dynamic Link" => evaluate_dynamic_link(field, fieldname, fields, record, cx, outcome),
            "Table" | "Table MultiSelect" => {
                walk_table(field, fieldname, doctype, record, cx, outcome)?
            }
            _ => {}
        }
    }

    Ok(())
}

/// A Link field is source-expressible when its `options` names the selected
/// source doctype. An expressible field with no value, restricted by
/// `permlevel`, cannot be ruled out as hidden evidence.
fn evaluate_link(
    field: &Value,
    fieldname: &str,
    record: &Map<String, Value>,
    cx: &WalkContext,
    outcome: &mut Outcome,
) {
    let options = field.get("options").and_then(Value::as_str).unwrap_or("");
    if options.is_empty() {
        return;
    }
    let doctype_cmp = compare(options, cx.source_doctype);
    if matches!(doctype_cmp, Compare::NoMatch) {
        return;
    }

    match record.get(fieldname) {
        Some(Value::String(value)) => match (doctype_cmp, compare(value, cx.source_name)) {
            (Compare::Match, Compare::Match) => outcome.matched = true,
            (Compare::Match, Compare::NoMatch) => {}
            _ => outcome.flag_unavailable(format!(
                "field '{fieldname}' names the selected source under case-insensitive comparison only"
            )),
        },
        Some(Value::Null) | None => {
            if permlevel_of(field) > 0 {
                outcome.flag_unavailable(format!(
                    "field '{fieldname}' is restricted and its value cannot be observed"
                ));
            }
        }
        Some(_) => outcome.flag_unavailable(format!(
            "field '{fieldname}' names the selected source doctype but its stored value is not a string"
        )),
    }
}

/// A Dynamic Link pairs its own value with the discriminator field named by
/// `options`. The discriminator is recognized only as Link-to-DocType, a
/// Select whose options explicitly admit the source, or a string-bearing
/// free-text field; any other discriminator definition cannot be trusted to
/// carry a doctype identity, so it makes negative proof unavailable.
fn evaluate_dynamic_link(
    field: &Value,
    fieldname: &str,
    fields: &[Value],
    record: &Map<String, Value>,
    cx: &WalkContext,
    outcome: &mut Outcome,
) {
    let discriminator_name = field.get("options").and_then(Value::as_str).unwrap_or("");
    if discriminator_name.is_empty() {
        outcome.flag_unavailable(format!(
            "field '{fieldname}' is a Dynamic Link with no declared discriminator field"
        ));
        return;
    }
    let Some(discriminator_field) = fields
        .iter()
        .find(|f| f.get("fieldname").and_then(Value::as_str) == Some(discriminator_name))
    else {
        outcome.flag_unavailable(format!(
            "field '{fieldname}' names discriminator '{discriminator_name}', which is not declared"
        ));
        return;
    };
    if !discriminator_is_recognized(discriminator_field, cx.source_doctype) {
        outcome.flag_unavailable(format!(
            "field '{fieldname}' discriminator '{discriminator_name}' is not a recognized native discriminator"
        ));
        return;
    }

    let discriminator_value = match record.get(discriminator_name) {
        Some(Value::String(v)) => Some(v.as_str()),
        _ => None,
    };
    let Some(discriminator_value) = discriminator_value else {
        if permlevel_of(discriminator_field) > 0 {
            outcome.flag_unavailable(format!(
                "field '{discriminator_name}' is restricted and its value cannot be observed"
            ));
        }
        return;
    };

    let doctype_cmp = compare(discriminator_value, cx.source_doctype);
    if matches!(doctype_cmp, Compare::NoMatch) {
        return;
    }

    match record.get(fieldname) {
        Some(Value::String(value)) => match (doctype_cmp, compare(value, cx.source_name)) {
            (Compare::Match, Compare::Match) => outcome.matched = true,
            (Compare::Match, Compare::NoMatch) => {}
            _ => outcome.flag_unavailable(format!(
                "field '{fieldname}' names the selected source under case-insensitive comparison only"
            )),
        },
        Some(Value::Null) | None => {
            if permlevel_of(field) > 0 {
                outcome.flag_unavailable(format!(
                    "field '{fieldname}' is restricted and its value cannot be observed"
                ));
            }
        }
        _ => {}
    }
}

fn discriminator_is_recognized(field: &Value, source_doctype: &str) -> bool {
    match field.get("fieldtype").and_then(Value::as_str).unwrap_or("") {
        "Link" => field.get("options").and_then(Value::as_str) == Some("DocType"),
        "Select" => field
            .get("options")
            .and_then(Value::as_str)
            .is_some_and(|options| options.lines().any(|line| line.trim() == source_doctype)),
        other => FREE_TEXT_DISCRIMINATOR_TYPES.contains(&other),
    }
}

/// A Table or Table MultiSelect field's stored value must be the declared
/// child doctype's rows: an absent key or a non-array value is a shape
/// failure rather than an assumed-empty table, so permission-hidden and
/// genuinely-absent tables are never silently conflated.
fn walk_table(
    field: &Value,
    fieldname: &str,
    doctype: &str,
    record: &Map<String, Value>,
    cx: &WalkContext,
    outcome: &mut Outcome,
) -> Result<(), ApiError> {
    let child_doctype = field
        .get("options")
        .and_then(Value::as_str)
        .filter(|d| !d.is_empty())
        .ok_or_else(|| {
            ApiError::Shape(format!(
                "'{doctype}' field '{fieldname}' declares a child table with no child doctype"
            ))
        })?;
    let child_meta = cx.schema.get(child_doctype).ok_or_else(|| {
        ApiError::Shape(format!(
            "'{doctype}' field '{fieldname}' declares child doctype '{child_doctype}', whose metadata was not collected"
        ))
    })?;

    match record.get(fieldname) {
        Some(Value::Array(rows)) => {
            for row in rows {
                if !row.is_object() {
                    return Err(ApiError::Shape(format!(
                        "'{doctype}' field '{fieldname}' row is not a document object"
                    )));
                }
                walk_record(child_doctype, child_meta, row, cx, outcome)?;
            }
            Ok(())
        }
        Some(_) => Err(ApiError::Shape(format!(
            "'{doctype}' field '{fieldname}' is declared as a child table but its stored value is not an array"
        ))),
        None => Err(ApiError::Shape(format!(
            "'{doctype}' field '{fieldname}' declares a child table with no stored value"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn link_field(fieldname: &str, options: &str) -> Value {
        json!({ "fieldname": fieldname, "fieldtype": "Link", "options": options })
    }

    fn dynamic_link_field(fieldname: &str, discriminator: &str) -> Value {
        json!({ "fieldname": fieldname, "fieldtype": "Dynamic Link", "options": discriminator })
    }

    fn meta_with_fields(fields: Vec<Value>) -> Value {
        json!({ "data": { "fields": fields } })
    }

    /// Run a walk from `doctype`/`meta`/`record` against `schema`, checking a
    /// document for a relationship to `Sales Invoice` `ACC-SINV-2026-00001` —
    /// the fixed source identity every test in this module checks against.
    fn run_walk(
        doctype: &str,
        meta: &Value,
        record: &Value,
        schema: &HashMap<String, Value>,
    ) -> (Outcome, Result<(), ApiError>) {
        let cx = WalkContext {
            schema,
            source_doctype: "Sales Invoice",
            source_name: "ACC-SINV-2026-00001",
        };
        let mut outcome = Outcome::default();
        let result = walk_record(doctype, meta, record, &cx, &mut outcome);
        (outcome, result)
    }

    #[test]
    fn exact_link_match_verifies() {
        let meta = meta_with_fields(vec![link_field("against", "Sales Invoice")]);
        let record = json!({ "against": "ACC-SINV-2026-00001" });
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(outcome.matched);
        assert!(outcome.unavailable_reason.is_none());
    }

    #[test]
    fn containing_name_never_verifies() {
        let meta = meta_with_fields(vec![link_field("against", "Sales Invoice")]);
        let record = json!({ "against": "ACC-SINV-2026-000010" });
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_none());
    }

    /// The `evaluate_dynamic_link` sibling of this test
    /// (`dynamic_link_with_different_discriminator_never_verifies`) proves the
    /// same guarantee for Dynamic Link; this proves it for plain Link. Without
    /// the doctype guard in `evaluate_link`, a Link field aimed at an unrelated
    /// doctype whose value happens to equal the source name would verify.
    #[test]
    fn link_field_targeting_unrelated_doctype_never_verifies() {
        let meta = meta_with_fields(vec![link_field("customer_reference", "Customer")]);
        let record = json!({ "customer_reference": "ACC-SINV-2026-00001" });
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_none());
    }

    #[test]
    fn dynamic_link_with_different_discriminator_never_verifies() {
        let meta = meta_with_fields(vec![
            dynamic_link_field("reference_name", "reference_doctype"),
            link_field("reference_doctype", "DocType"),
        ]);
        let record = json!({
            "reference_doctype": "Purchase Invoice",
            "reference_name": "ACC-SINV-2026-00001",
        });
        let schema = HashMap::from([("Payment Entry Reference".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry Reference", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_none());
    }

    #[test]
    fn dynamic_link_with_matching_discriminator_verifies() {
        let meta = meta_with_fields(vec![
            dynamic_link_field("reference_name", "reference_doctype"),
            json!({ "fieldname": "reference_doctype", "fieldtype": "Link", "options": "DocType" }),
        ]);
        let record = json!({
            "reference_doctype": "Sales Invoice",
            "reference_name": "ACC-SINV-2026-00001",
        });
        let schema = HashMap::from([("Payment Entry Reference".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry Reference", &meta, &record, &schema);
        result.unwrap();
        assert!(outcome.matched);
    }

    #[test]
    fn select_discriminator_admitting_source_recognizes_match() {
        let meta = meta_with_fields(vec![
            dynamic_link_field("party", "party_type"),
            json!({
                "fieldname": "party_type",
                "fieldtype": "Select",
                "options": "Customer\nSupplier\nSales Invoice",
            }),
        ]);
        let record = json!({ "party_type": "Sales Invoice", "party": "ACC-SINV-2026-00001" });
        let schema = HashMap::from([("Journal Entry Account".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Journal Entry Account", &meta, &record, &schema);
        result.unwrap();
        assert!(outcome.matched);
    }

    #[test]
    fn unsupported_discriminator_type_is_unavailable_not_missing() {
        let meta = meta_with_fields(vec![
            dynamic_link_field("reference_name", "reference_kind"),
            json!({ "fieldname": "reference_kind", "fieldtype": "Int" }),
        ]);
        let record = json!({ "reference_kind": 1, "reference_name": "ACC-SINV-2026-00001" });
        let schema = HashMap::from([("Odd Reference".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Odd Reference", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_some());
    }

    #[test]
    fn restricted_null_candidate_is_unavailable() {
        let meta = meta_with_fields(vec![
            json!({ "fieldname": "against", "fieldtype": "Link", "options": "Sales Invoice", "permlevel": 1 }),
        ]);
        let record = json!({});
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_some());
    }

    #[test]
    fn ordinary_empty_candidate_is_nothing() {
        let meta = meta_with_fields(vec![link_field("against", "Sales Invoice")]);
        let record = json!({});
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_none());
    }

    #[test]
    fn case_only_doctype_match_is_unavailable() {
        let meta = meta_with_fields(vec![link_field("against", "sales invoice")]);
        let record = json!({ "against": "ACC-SINV-2026-00001" });
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_some());
    }

    #[test]
    fn nested_child_table_row_after_nonmatching_row_still_verifies() {
        let child_meta = meta_with_fields(vec![link_field("against", "Sales Invoice")]);
        let root_meta = meta_with_fields(vec![
            json!({ "fieldname": "items", "fieldtype": "Table", "options": "Payment Entry Reference" }),
        ]);
        let record = json!({
            "items": [
                { "against": "ACC-SINV-2026-99999" },
                { "against": "ACC-SINV-2026-00001" },
            ]
        });
        let schema = HashMap::from([
            ("Payment Entry".to_owned(), root_meta.clone()),
            ("Payment Entry Reference".to_owned(), child_meta),
        ]);
        let (outcome, result) = run_walk("Payment Entry", &root_meta, &record, &schema);
        result.unwrap();
        assert!(outcome.matched);
    }

    #[test]
    fn nested_table_multiselect_row_verifies_like_a_table() {
        let child_meta = meta_with_fields(vec![link_field("against", "Sales Invoice")]);
        let root_meta = meta_with_fields(vec![
            json!({ "fieldname": "tags", "fieldtype": "Table MultiSelect", "options": "Payment Entry Reference" }),
        ]);
        let record = json!({ "tags": [{ "against": "ACC-SINV-2026-00001" }] });
        let schema = HashMap::from([
            ("Payment Entry".to_owned(), root_meta.clone()),
            ("Payment Entry Reference".to_owned(), child_meta),
        ]);
        let (outcome, result) = run_walk("Payment Entry", &root_meta, &record, &schema);
        result.unwrap();
        assert!(outcome.matched);
    }

    #[test]
    fn missing_declared_table_key_is_a_shape_failure() {
        let root_meta = meta_with_fields(vec![
            json!({ "fieldname": "items", "fieldtype": "Table", "options": "Payment Entry Reference" }),
        ]);
        let record = json!({});
        let schema = HashMap::from([
            ("Payment Entry".to_owned(), root_meta.clone()),
            (
                "Payment Entry Reference".to_owned(),
                meta_with_fields(vec![link_field("against", "Sales Invoice")]),
            ),
        ]);
        let (_, result) = run_walk("Payment Entry", &root_meta, &record, &schema);
        assert!(matches!(result, Err(ApiError::Shape(_))));
    }

    #[test]
    fn non_array_table_value_is_a_shape_failure() {
        let root_meta = meta_with_fields(vec![
            json!({ "fieldname": "items", "fieldtype": "Table", "options": "Payment Entry Reference" }),
        ]);
        let record = json!({ "items": "not-an-array" });
        let schema = HashMap::from([
            ("Payment Entry".to_owned(), root_meta.clone()),
            (
                "Payment Entry Reference".to_owned(),
                meta_with_fields(vec![link_field("against", "Sales Invoice")]),
            ),
        ]);
        let (_, result) = run_walk("Payment Entry", &root_meta, &record, &schema);
        assert!(matches!(result, Err(ApiError::Shape(_))));
    }

    #[test]
    fn malformed_root_metadata_is_a_shape_failure() {
        let meta = json!({ "data": {} });
        let record = json!({});
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (_, result) = run_walk("Payment Entry", &meta, &record, &schema);
        assert!(matches!(result, Err(ApiError::Shape(_))));
    }

    #[test]
    fn malformed_child_metadata_is_a_shape_failure() {
        let root_meta = meta_with_fields(vec![
            json!({ "fieldname": "items", "fieldtype": "Table", "options": "Payment Entry Reference" }),
        ]);
        let record = json!({ "items": [{}] });
        let schema = HashMap::from([
            ("Payment Entry".to_owned(), root_meta.clone()),
            ("Payment Entry Reference".to_owned(), json!({ "data": {} })),
        ]);
        let (_, result) = run_walk("Payment Entry", &root_meta, &record, &schema);
        assert!(matches!(result, Err(ApiError::Shape(_))));
    }

    #[test]
    fn empty_child_table_is_missing_not_unavailable() {
        let root_meta = meta_with_fields(vec![
            json!({ "fieldname": "items", "fieldtype": "Table", "options": "Payment Entry Reference" }),
        ]);
        let record = json!({ "items": [] });
        let schema = HashMap::from([
            ("Payment Entry".to_owned(), root_meta.clone()),
            (
                "Payment Entry Reference".to_owned(),
                meta_with_fields(vec![link_field("against", "Sales Invoice")]),
            ),
        ]);
        let (outcome, result) = run_walk("Payment Entry", &root_meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_none());
        assert!(schema_is_source_expressible(&schema, "Sales Invoice"));
    }

    #[test]
    fn source_free_schema_is_not_expressible() {
        let meta = meta_with_fields(vec![link_field("customer", "Customer")]);
        let schema = HashMap::from([("Sales Order".to_owned(), meta)]);
        assert!(!schema_is_source_expressible(&schema, "Sales Invoice"));
    }

    /// A Dynamic Link's discriminator is `Int` — not a recognized native
    /// discriminator type — so the field cannot in fact name the source no
    /// matter what value it stores; the schema must not be reported expressible.
    #[test]
    fn dynamic_link_with_unresolvable_discriminator_is_not_source_expressible() {
        let meta = meta_with_fields(vec![
            dynamic_link_field("reference_name", "reference_kind"),
            json!({ "fieldname": "reference_kind", "fieldtype": "Int" }),
        ]);
        let schema = HashMap::from([("Odd Reference".to_owned(), meta)]);
        assert!(!schema_is_source_expressible(&schema, "Sales Invoice"));
    }

    #[test]
    fn dynamic_link_with_recognized_discriminator_is_source_expressible() {
        let meta = meta_with_fields(vec![
            dynamic_link_field("reference_name", "reference_doctype"),
            link_field("reference_doctype", "DocType"),
        ]);
        let schema = HashMap::from([("Payment Entry Reference".to_owned(), meta)]);
        assert!(schema_is_source_expressible(&schema, "Sales Invoice"));
    }

    /// End-to-end reproduction of the finding: an empty child table means the
    /// Dynamic Link inside it is never walked, so only the schema-level check
    /// decides expressibility. Because its discriminator is unresolvable, the
    /// schema must not be reported expressible — the same shape as
    /// `empty_child_table_is_missing_not_unavailable`, but proving the
    /// Unavailable-not-Missing side of that same combination.
    #[test]
    fn empty_child_table_with_unresolvable_dynamic_link_discriminator_is_not_expressible() {
        let child_meta = meta_with_fields(vec![
            dynamic_link_field("reference_name", "reference_kind"),
            json!({ "fieldname": "reference_kind", "fieldtype": "Int" }),
        ]);
        let root_meta = meta_with_fields(vec![
            json!({ "fieldname": "items", "fieldtype": "Table", "options": "Payment Entry Reference" }),
        ]);
        let record = json!({ "items": [] });
        let schema = HashMap::from([
            ("Payment Entry".to_owned(), root_meta.clone()),
            ("Payment Entry Reference".to_owned(), child_meta),
        ]);
        let (outcome, result) = run_walk("Payment Entry", &root_meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_none());
        assert!(!schema_is_source_expressible(&schema, "Sales Invoice"));
    }

    /// A Link definition with no `fieldname` cannot ever be looked up in the
    /// record, so it must not silently drop out of consideration when its
    /// `options` names the selected source.
    #[test]
    fn link_without_fieldname_that_targets_source_is_unavailable() {
        let meta = meta_with_fields(vec![
            json!({ "fieldtype": "Link", "options": "Sales Invoice" }),
        ]);
        let record = json!({});
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_some());
    }

    /// A Link definition with no `fieldname` that targets an unrelated
    /// doctype carries no evidence either way and must stay unremarkable.
    #[test]
    fn link_without_fieldname_targeting_unrelated_doctype_is_nothing() {
        let meta = meta_with_fields(vec![json!({ "fieldtype": "Link", "options": "Customer" })]);
        let record = json!({});
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_none());
    }

    /// A stored non-string value on a Link field that targets the source
    /// doctype is malformed evidence, not a silent non-match — it must not
    /// let the rest of the document settle for a definitive Missing.
    #[test]
    fn non_string_link_value_is_unavailable_not_silently_skipped() {
        let meta = meta_with_fields(vec![link_field("against", "Sales Invoice")]);
        let record = json!({ "against": 12345 });
        let schema = HashMap::from([("Payment Entry".to_owned(), meta.clone())]);
        let (outcome, result) = run_walk("Payment Entry", &meta, &record, &schema);
        result.unwrap();
        assert!(!outcome.matched);
        assert!(outcome.unavailable_reason.is_some());
    }
}
