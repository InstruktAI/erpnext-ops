use std::path::Path;

use serde_json::{json, Value};

use crate::output;
use crate::transport::Client;

/// `erpnext evidence attach` — attach a file to a business document through Frappe's
/// native upload endpoint, which permission-checks against the referenced document.
/// A summary, when given, lands on the document's timeline as a comment naming the
/// attachment.
pub async fn attach(
    client: &Client,
    reference: (&str, &str),
    file_path: &str,
    summary: Option<&str>,
) -> i32 {
    let (ref_doctype, ref_name) = reference;

    let bytes = match std::fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            return output::fail(
                "file",
                &format!("cannot read '{file_path}': {e}"),
                output::EXIT_CONFIG,
            )
        }
    };
    let file_name = Path::new(file_path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "attachment".to_owned());

    let uploaded = match client
        .post_file(
            "/api/method/upload_file",
            &file_name,
            bytes,
            &[
                ("doctype", ref_doctype),
                ("docname", ref_name),
                ("is_private", "1"),
            ],
        )
        .await
    {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };

    let file_url = uploaded
        .pointer("/message/file_url")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();

    let mut recorded: Vec<Value> = vec![json!({
        "kind": "File",
        "file_name": file_name,
        "file_url": file_url,
    })];

    if let Some(summary) = summary {
        let params = json!({
            "reference_doctype": ref_doctype,
            "reference_name": ref_name,
            "content": format!("{summary} (attachment: {file_url})"),
            "comment_email": "",
            "comment_by": "",
        });
        match client
            .post("/api/method/frappe.desk.form.utils.add_comment", &params)
            .await
        {
            Ok(v) => {
                recorded.push(json!({ "kind": "Comment", "result": v.pointer("/message/name") }))
            }
            Err(e) => {
                return output::fail_with_data(
                    e.kind(),
                    &format!("file attached but summary comment failed: {e}"),
                    json!({ "attached": recorded }),
                    output::EXIT_API,
                )
            }
        }
    }

    output::ok(json!({ "attached": recorded }))
}
