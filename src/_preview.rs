//! One private, shared mutation-preview capability: assemble a truthful preview
//! from explicit native observations and a replayable invocation.
//!
//! This module owns none of the facts it renders. It never calls the transport —
//! every observed `Value` (a document body, a metadata response) is read by its
//! caller in `commands::doc` or `commands::workflow` and handed in already
//! resolved. It does not implement ERPNext business semantics, context
//! resolution, HTTP, idempotency, or controller persistence; those stay with
//! their existing owners.

use serde_json::{json, Map, Value};

use crate::command_syntax::{self, MutationCommand};
use crate::config::Context;

/// The native default fields every Frappe document carries regardless of DocType
/// metadata (frappe/model/__init__.py `default_fields`, version 15). A payload key
/// in this set is never an unknown field, whether or not the inspected metadata
/// happens to list it.
const DEFAULT_FIELDS: &[&str] = &[
    "doctype",
    "name",
    "owner",
    "creation",
    "modified",
    "modified_by",
    "docstatus",
    "idx",
];

/// A native fact that may not have been readable: a field absent from the
/// response, or present as JSON `null`. Both render as `observed: false` so a
/// caller can never mistake "we don't know" for a falsy or default value.
fn observed(source: &Value, pointer: &str) -> Value {
    match source.pointer(pointer) {
        Some(Value::Null) | None => json!({ "observed": false, "value": null }),
        Some(value) => json!({ "observed": true, "value": value }),
    }
}

/// Observed target facts for an existing document: lifecycle status, its own
/// company, and its modification marker. `doc` is the document body (the site
/// response's `data`/`message` payload already unwrapped by the caller).
pub fn target_json(doc: &Value) -> Value {
    json!({
        "docstatus": observed(doc, "/docstatus"),
        "company": observed(doc, "/company"),
        "modified": observed(doc, "/modified"),
    })
}

/// Observed schema facts for a DocType: whether it is submittable, which fields
/// stay editable after submit, and whether it tracks field-level change history.
/// `meta` is the raw metadata response in either site wrapper shape.
pub fn schema_json(meta: &Value) -> Value {
    let body = meta.pointer("/data").or_else(|| meta.pointer("/message"));
    let is_submittable = body
        .map(|b| observed(b, "/is_submittable"))
        .unwrap_or_else(|| json!({ "observed": false, "value": null }));
    let track_changes = body
        .map(|b| observed(b, "/track_changes"))
        .unwrap_or_else(|| json!({ "observed": false, "value": null }));
    let allow_on_submit_fields = match body
        .and_then(|b| b.pointer("/fields"))
        .and_then(Value::as_array)
    {
        Some(fields) => {
            let names: Vec<&str> = fields
                .iter()
                .filter(|f| f.pointer("/allow_on_submit").and_then(Value::as_i64) == Some(1))
                .filter_map(|f| f.pointer("/fieldname").and_then(Value::as_str))
                .collect();
            json!({ "observed": true, "value": names })
        }
        None => json!({ "observed": false, "value": null }),
    };
    json!({
        "is_submittable": is_submittable,
        "allow_on_submit_fields": allow_on_submit_fields,
        "track_changes": track_changes,
    })
}

/// Read a DocType's own declared fieldnames out of either response wrapper shape.
/// `None` means the metadata object or its `fields` collection could not be
/// validated — the caller must treat that as "the check did not run", never as
/// "nothing was found".
fn meta_fieldnames(meta: &Value) -> Option<Vec<&str>> {
    let fields = meta
        .pointer("/data/fields")
        .or_else(|| meta.pointer("/message/fields"))?
        .as_array()?;
    Some(
        fields
            .iter()
            .filter_map(|f| f.pointer("/fieldname").and_then(Value::as_str))
            .collect(),
    )
}

/// Outcome of checking a payload's top-level keys against the native default-field
/// set and live metadata.
pub enum FieldCheck {
    Clean,
    Unknown(Vec<String>),
    /// Metadata could not be validated: the check did not run. Never a clean pass.
    Unreadable,
}

/// Check payload keys against the exact recognized vocabulary: the eight native
/// default fields plus every fieldname the live metadata declares. This is the
/// one field-check every mutation preview and every fresh create/update apply
/// consults, so preview and apply can never evolve separate definitions.
pub fn check_fields(meta: &Value, payload: &Map<String, Value>) -> FieldCheck {
    match meta_fieldnames(meta) {
        None => FieldCheck::Unreadable,
        Some(known) => {
            let unknown: Vec<String> = payload
                .keys()
                .filter(|k| !DEFAULT_FIELDS.contains(&k.as_str()) && !known.contains(&k.as_str()))
                .cloned()
                .collect();
            if unknown.is_empty() {
                FieldCheck::Clean
            } else {
                FieldCheck::Unknown(unknown)
            }
        }
    }
}

impl FieldCheck {
    /// Gate a fresh create or update: `Ok(())` when every supplied key is
    /// recognized, otherwise the failure envelope's kind and message. Copy, map,
    /// and amend report findings through `preflight_json` but never gate through
    /// this — native validation stays the site's judgement for those operations.
    pub fn gate(&self) -> Result<(), (&'static str, String)> {
        match self {
            FieldCheck::Clean => Ok(()),
            FieldCheck::Unknown(fields) => Err((
                "preflight",
                format!("unknown field(s): {}", fields.join(", ")),
            )),
            FieldCheck::Unreadable => Err((
                "shape",
                "live metadata fields could not be validated".to_string(),
            )),
        }
    }
}

const SITE_VALIDATION_NOTE: &str =
    "mandatory, link-target, accounting, stock, workflow, and permission validation remain the site's decision";

fn sorted_keys(payload: &Map<String, Value>) -> Vec<String> {
    let mut keys: Vec<String> = payload.keys().cloned().collect();
    keys.sort();
    keys
}

/// The preview's field-check report: which keys were inspected, which are
/// genuinely unrecognized, and an explicit statement of what stays the site's
/// decision. Reports findings without gating; gating for fresh create/update is
/// `FieldCheck::gate`, applied separately by the caller.
pub fn preflight_json(doctype: &str, meta: &Value, payload: &Map<String, Value>) -> Value {
    match check_fields(meta, payload) {
        FieldCheck::Unreadable => json!({
            "checked": false,
            "reason": "live metadata fields could not be validated",
        }),
        FieldCheck::Clean => json!({
            "checked": true,
            "doctype": doctype,
            "checked_keys": sorted_keys(payload),
            "unknown_fields": Value::Array(vec![]),
            "site_validation": SITE_VALIDATION_NOTE,
        }),
        FieldCheck::Unknown(unknown) => json!({
            "checked": true,
            "doctype": doctype,
            "checked_keys": sorted_keys(payload),
            "unknown_fields": unknown,
            "site_validation": SITE_VALIDATION_NOTE,
        }),
    }
}

/// A native method's `--data` is opaque method arguments, never document fields:
/// no unknown-field claim is made about that payload.
pub fn opaque_preflight_json() -> Value {
    json!({
        "checked": false,
        "reason": "method arguments are opaque; no document-field claim is made",
    })
}

/// A lifecycle transition's payload carries no caller-supplied fields to check.
pub fn no_fields_preflight_json() -> Value {
    json!({
        "checked": false,
        "reason": "the lifecycle payload carries no caller-supplied fields to check",
    })
}

/// The operating identity's basis for a preview: always configuration, since no
/// preview authenticates against the site to learn who it is actually talking as.
pub fn context_json(context: &Context) -> Value {
    json!({
        "site": context.site(),
        "company": context.company,
        "privilege": context.privilege,
        "identity": {
            "basis": "configuration",
            "configured_user": context.credentials.user,
            "authenticated_user": Value::Null,
        },
    })
}

/// The concrete values a replay invocation needs for one mutation subcommand.
/// Each variant carries exactly the arguments that subcommand's Clap declaration
/// accepts, in the shapes `main.rs` already resolves them to.
pub enum ReplayArgs<'a> {
    DocCreate {
        doctype: &'a str,
        data: &'a Value,
        source: Option<&'a str>,
    },
    DocUpdate {
        doctype: &'a str,
        name: &'a str,
        data: &'a Value,
    },
    DocDelete {
        doctype: &'a str,
        name: &'a str,
    },
    DocCopy {
        doctype: &'a str,
        name: &'a str,
        data: Option<&'a Value>,
    },
    DocMap {
        doctype: &'a str,
        name: &'a str,
        via: &'a str,
        args: Option<&'a Value>,
        data: Option<&'a Value>,
    },
    WorkflowSubmit {
        doctype: &'a str,
        name: &'a str,
    },
    WorkflowCancel {
        doctype: &'a str,
        name: &'a str,
    },
    WorkflowAmend {
        doctype: &'a str,
        name: &'a str,
        data: &'a Value,
    },
    WorkflowMethod {
        doctype: &'a str,
        name: &'a str,
        method: &'a str,
        data: Option<&'a Value>,
    },
    MethodCall {
        method: &'a str,
        data: Option<&'a Value>,
    },
}

impl ReplayArgs<'_> {
    fn command(&self) -> MutationCommand {
        match self {
            Self::DocCreate { .. } => MutationCommand::DocCreate,
            Self::DocUpdate { .. } => MutationCommand::DocUpdate,
            Self::DocDelete { .. } => MutationCommand::DocDelete,
            Self::DocCopy { .. } => MutationCommand::DocCopy,
            Self::DocMap { .. } => MutationCommand::DocMap,
            Self::WorkflowSubmit { .. } => MutationCommand::WorkflowSubmit,
            Self::WorkflowCancel { .. } => MutationCommand::WorkflowCancel,
            Self::WorkflowAmend { .. } => MutationCommand::WorkflowAmend,
            Self::WorkflowMethod { .. } => MutationCommand::WorkflowMethod,
            Self::MethodCall { .. } => MutationCommand::MethodCall,
        }
    }

    /// This replay's positional values, named and ordered to match
    /// `MutationCommand::positionals` for the same variant.
    /// `positional_order_matches_declared_names` locks that correspondence for
    /// every variant, so drift is a test failure, never a runtime check.
    fn named_positional_values(&self) -> Vec<(&'static str, &str)> {
        match self {
            Self::DocCreate { doctype, .. } => vec![(command_syntax::DOCTYPE, *doctype)],
            Self::DocUpdate { doctype, name, .. }
            | Self::DocDelete { doctype, name }
            | Self::DocCopy { doctype, name, .. }
            | Self::DocMap { doctype, name, .. }
            | Self::WorkflowSubmit { doctype, name }
            | Self::WorkflowCancel { doctype, name }
            | Self::WorkflowAmend { doctype, name, .. } => vec![
                (command_syntax::DOCTYPE, *doctype),
                (command_syntax::NAME, *name),
            ],
            Self::WorkflowMethod {
                doctype,
                name,
                method,
                ..
            } => vec![
                (command_syntax::DOCTYPE, *doctype),
                (command_syntax::NAME, *name),
                (command_syntax::METHOD, *method),
            ],
            Self::MethodCall { method, .. } => vec![(command_syntax::METHOD, *method)],
        }
    }

    /// Positional values, in the order `named_positional_values` already produces
    /// them for this variant. No lookup and no fallible unwrap in this path;
    /// congruence with `MutationCommand::positionals` is a compile-independent
    /// invariant locked by a colocated test, not a runtime check.
    fn positional_values(&self) -> Vec<&str> {
        self.named_positional_values()
            .into_iter()
            .map(|(_, value)| value)
            .collect()
    }

    fn flag_values(&self) -> Vec<(&'static str, String)> {
        fn json_arg(value: &Value) -> String {
            value.to_string()
        }

        match self {
            Self::DocCreate { data, source, .. } => {
                let mut flags = vec![(command_syntax::DATA, json_arg(data))];
                if let Some(token) = source {
                    flags.push((command_syntax::SOURCE, (*token).to_string()));
                }
                flags
            }
            Self::DocUpdate { data, .. } | Self::WorkflowAmend { data, .. } => {
                vec![(command_syntax::DATA, json_arg(data))]
            }
            Self::DocDelete { .. } | Self::WorkflowSubmit { .. } | Self::WorkflowCancel { .. } => {
                vec![]
            }
            Self::DocCopy { data, .. } => data
                .map(|d| vec![(command_syntax::DATA, json_arg(d))])
                .unwrap_or_default(),
            Self::DocMap {
                via, args, data, ..
            } => {
                let mut flags = vec![(command_syntax::VIA, (*via).to_string())];
                if let Some(a) = args {
                    flags.push((command_syntax::ARGS, json_arg(a)));
                }
                if let Some(d) = data {
                    flags.push((command_syntax::DATA, json_arg(d)));
                }
                flags
            }
            Self::WorkflowMethod { data, .. } | Self::MethodCall { data, .. } => data
                .map(|d| vec![(command_syntax::DATA, json_arg(d))])
                .unwrap_or_default(),
        }
    }
}

/// The exact replayable invocation for a previewed mutation: the resolved
/// company and privilege are pinned so replay does not depend on a later
/// `context use`, every argument and its boundary survives shell quoting, and
/// `--apply` is always present. Never executed here — this only renders text.
pub fn apply_command(context: &Context, args: &ReplayArgs) -> String {
    let [group, subcommand] = args.command().tokens();
    let mut tokens: Vec<String> = vec![
        command_syntax::BINARY.to_string(),
        format!("--{}", command_syntax::COMPANY),
        context.company_slug.clone(),
        format!("--{}", command_syntax::AS),
        context.privilege.to_string(),
        group.to_string(),
        subcommand.to_string(),
    ];
    tokens.extend(args.positional_values().into_iter().map(str::to_string));
    for (flag, value) in args.flag_values() {
        tokens.push(format!("--{flag}"));
        tokens.push(value);
    }
    tokens.push(format!("--{}", command_syntax::APPLY));

    tokens
        .iter()
        .map(|token| command_syntax::shell_quote(token))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The documented preview shape: observation, exact native operation, and exact
/// replay. No summary, consequence, or reversibility narrative is generated.
pub struct Preview {
    pub context: Value,
    pub operation: Value,
    pub target: Option<Value>,
    pub schema: Option<Value>,
    pub preflight: Value,
    pub apply_command: String,
    pub idempotency: Option<Value>,
    pub intended_state: Option<&'static str>,
    pub persistence: Option<&'static str>,
}

impl Preview {
    pub fn into_json(self) -> Value {
        let mut envelope = Map::new();
        envelope.insert("preview".into(), json!(true));
        envelope.insert("context".into(), self.context);
        envelope.insert("operation".into(), self.operation);
        if let Some(target) = self.target {
            envelope.insert("target".into(), target);
        }
        if let Some(schema) = self.schema {
            envelope.insert("schema".into(), schema);
        }
        envelope.insert("preflight".into(), self.preflight);
        if let Some(idempotency) = self.idempotency {
            envelope.insert("idempotency".into(), idempotency);
        }
        if let Some(intended_state) = self.intended_state {
            envelope.insert("intended_state".into(), json!(intended_state));
        }
        if let Some(persistence) = self.persistence {
            envelope.insert("persistence".into(), json!(persistence));
        }
        envelope.insert("apply_command".into(), json!(self.apply_command));
        Value::Object(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observed_distinguishes_absent_and_null_from_present() {
        let doc = json!({ "docstatus": 1, "company": null });
        assert_eq!(
            target_json(&doc).pointer("/docstatus"),
            Some(&json!({ "observed": true, "value": 1 }))
        );
        assert_eq!(
            target_json(&doc).pointer("/company"),
            Some(&json!({ "observed": false, "value": null }))
        );
        assert_eq!(
            target_json(&doc).pointer("/modified"),
            Some(&json!({ "observed": false, "value": null }))
        );
    }

    #[test]
    fn check_fields_recognizes_the_exact_native_default_set() {
        let meta = json!({ "data": { "fields": [ { "fieldname": "customer_name" } ] } });
        let payload = json!({
            "customer_name": "Acme",
            "owner": "agent@instrukt.ai",
            "creation": "2026-01-01",
            "modified": "2026-01-01",
            "modified_by": "agent@instrukt.ai",
            "docstatus": 0,
            "idx": 1,
        });
        assert!(matches!(
            check_fields(&meta, payload.as_object().unwrap()),
            FieldCheck::Clean
        ));
    }

    #[test]
    fn check_fields_reports_unreadable_metadata_distinctly_from_clean() {
        let meta = json!({ "data": {} });
        let payload = json!({ "customer_name": "Acme" });
        assert!(matches!(
            check_fields(&meta, payload.as_object().unwrap()),
            FieldCheck::Unreadable
        ));
    }

    #[test]
    fn apply_command_pins_company_and_privilege_and_quotes_data() {
        use crate::config::{Credentials, PrivilegeMode};

        let context = Context {
            site_url: url::Url::parse("https://x.erpnext.example").unwrap(),
            company_slug: "instrukt-ai".into(),
            company: Some("Instrukt AI".into()),
            privilege: PrivilegeMode::Operator,
            credentials: Credentials {
                user: None,
                api_key: "test-api-key".into(),
                api_secret: "test-api-secret".into(),
            },
        };
        let payload = json!({ "customer_name": "Acme's" });
        let rendered = apply_command(
            &context,
            &ReplayArgs::DocCreate {
                doctype: "Customer",
                data: &payload,
                source: None,
            },
        );
        assert!(rendered.contains("'--company' 'instrukt-ai'"));
        assert!(rendered.contains("'--as' 'operator'"));
        assert!(rendered.contains("'doc' 'create' 'Customer'"));
        assert!(rendered.ends_with("'--apply'"));
        assert!(!rendered.contains(&context.credentials.api_key));
        assert!(!rendered.contains(&context.credentials.api_secret));
    }

    /// `positional_values` zips `named_positional_values` by position rather than
    /// looking each name up, so nothing at runtime re-derives the correspondence
    /// this test locks: for every `MutationCommand` variant, the names
    /// `named_positional_values` returns must equal `MutationCommand::positionals`,
    /// in the same order.
    #[test]
    fn named_positional_values_order_matches_declared_positionals_for_every_variant() {
        let data = Value::Null;
        let samples: Vec<ReplayArgs> = vec![
            ReplayArgs::DocCreate {
                doctype: "D",
                data: &data,
                source: None,
            },
            ReplayArgs::DocUpdate {
                doctype: "D",
                name: "N",
                data: &data,
            },
            ReplayArgs::DocDelete {
                doctype: "D",
                name: "N",
            },
            ReplayArgs::DocCopy {
                doctype: "D",
                name: "N",
                data: None,
            },
            ReplayArgs::DocMap {
                doctype: "D",
                name: "N",
                via: "V",
                args: None,
                data: None,
            },
            ReplayArgs::WorkflowSubmit {
                doctype: "D",
                name: "N",
            },
            ReplayArgs::WorkflowCancel {
                doctype: "D",
                name: "N",
            },
            ReplayArgs::WorkflowAmend {
                doctype: "D",
                name: "N",
                data: &data,
            },
            ReplayArgs::WorkflowMethod {
                doctype: "D",
                name: "N",
                method: "M",
                data: None,
            },
        ];

        for args in &samples {
            let declared = args.command().positionals();
            let named: Vec<&str> = args
                .named_positional_values()
                .iter()
                .map(|(name, _)| *name)
                .collect();
            assert_eq!(
                named,
                declared,
                "named_positional_values order must match MutationCommand::positionals for {:?}",
                args.command()
            );
        }
    }
}
