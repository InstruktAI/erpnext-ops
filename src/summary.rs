//! Plain-language restatement of what a command is about to do.
//!
//! A preview states the native request so an agent can check it. That shape is unreadable
//! to the person who asked for the work, and a misread instruction is only caught before
//! it runs. Each preview therefore also carries one sentence in business language, naming
//! the record, the money where it applies, and the consequence that outlives the action.

use serde_json::Value;

/// What a command is about to do, in the vocabulary of the business rather than the API.
pub enum Action<'a> {
    Create {
        doctype: &'a str,
        payload: &'a Value,
    },
    Update {
        doctype: &'a str,
        name: &'a str,
        payload: &'a Value,
    },
    Delete {
        doctype: &'a str,
        name: &'a str,
    },
    Submit {
        doctype: &'a str,
        name: &'a str,
    },
    Cancel {
        doctype: &'a str,
        name: &'a str,
    },
    Amend {
        doctype: &'a str,
        name: &'a str,
    },
    Convert {
        source_doctype: &'a str,
        source_name: &'a str,
        target_doctype: &'a str,
    },
    Method {
        doctype: &'a str,
        name: &'a str,
        method: &'a str,
    },
}

/// State the action in one sentence, followed by its lasting consequence where it has one,
/// and the words that approve it.
pub fn describe(action: &Action, company: Option<&str>) -> String {
    let (sentence, consequence) = match action {
        Action::Create { doctype, payload } => (
            match label(payload) {
                Some(l) => format!("Create a new {} for {l}.", lower(doctype)),
                None => format!("Create a new {}.", lower(doctype)),
            },
            amount(payload).map(|a| format!("It is worth {a}.")),
        ),
        Action::Update {
            doctype,
            name,
            payload,
        } => (
            format!("Change {} on {} {name}.", fields(payload), lower(doctype)),
            None,
        ),
        Action::Delete { doctype, name } => (
            format!("Permanently delete {} {name}.", lower(doctype)),
            Some("Deleting removes the record entirely; it is not a cancellation.".into()),
        ),
        Action::Submit { doctype, name } => (
            format!("Submit {} {name}.", lower(doctype)),
            Some(
                "Submitting finalises it: the figures reach the books, and afterwards it can \
                 only be cancelled, never edited."
                    .into(),
            ),
        ),
        Action::Cancel { doctype, name } => (
            format!("Cancel {} {name}.", lower(doctype)),
            Some(
                "Cancelling reverses its effect on the books and leaves the record in place \
                 for the audit trail."
                    .into(),
            ),
        ),
        Action::Amend { doctype, name } => (
            format!(
                "Create a corrected copy of cancelled {} {name}.",
                lower(doctype)
            ),
            Some("The cancelled original stays as it is.".into()),
        ),
        Action::Convert {
            source_doctype,
            source_name,
            target_doctype,
        } => (
            format!(
                "Turn {} {source_name} into a new {}.",
                lower(source_doctype),
                lower(target_doctype)
            ),
            Some(
                "Converting the same source twice creates a second document; look for an \
                 existing one first."
                    .into(),
            ),
        ),
        Action::Method {
            doctype,
            name,
            method,
        } => (format!("Run {method} on {} {name}.", lower(doctype)), None),
    };

    let mut out = match company {
        Some(c) => format!("{sentence} This is in the books of {c}."),
        None => sentence,
    };
    if let Some(c) = consequence {
        out.push(' ');
        out.push_str(&c);
    }
    out.push_str(" If this is correct, say approve.");
    out
}

/// ERPNext names doctypes in title case; a sentence reads better with them lowered, while
/// an acronym-shaped name stays as it is.
fn lower(doctype: &str) -> String {
    if doctype
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase())
    {
        return doctype.to_string();
    }
    doctype.to_lowercase()
}

/// The name a person would recognise the record by.
fn label(payload: &Value) -> Option<String> {
    for key in [
        "customer_name",
        "supplier_name",
        "item_name",
        "employee_name",
        "party_name",
        "title",
        "subject",
        "item_code",
    ] {
        if let Some(v) = payload.get(key).and_then(Value::as_str) {
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

/// The figure that makes an action consequential, when the payload carries one.
fn amount(payload: &Value) -> Option<String> {
    for key in ["grand_total", "total", "paid_amount"] {
        if let Some(v) = payload.get(key).and_then(Value::as_f64) {
            if v != 0.0 {
                return Some(format!("{v:.2}"));
            }
        }
    }
    None
}

/// Name the fields a change touches, so the person sees the extent of it.
fn fields(payload: &Value) -> String {
    match payload.as_object() {
        Some(map) if !map.is_empty() => {
            let mut names: Vec<&str> = map.keys().map(String::as_str).collect();
            names.sort_unstable();
            match names.len() {
                1 => names[0].to_string(),
                _ => {
                    let last = names.pop().unwrap_or_default();
                    format!("{} and {last}", names.join(", "))
                }
            }
        }
        _ => "nothing".into(),
    }
}
