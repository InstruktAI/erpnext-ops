use serde_json::{json, Value};

use crate::output;
use crate::transport::Client;

/// Bulk import rides ERPNext's native Data Import machinery, which the vendor gates
/// to privileged roles: an operator context receives the site's typed permission
/// failure rather than the CLI widening any role. A preview is an identified
/// `Data Import` document with the parsed template; apply starts exactly that batch.
pub async fn preview(client: &Client, file_path: &str, target_doctype: &str) -> i32 {
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
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "import.csv".to_owned());

    // 1. The batch identity: a native Data Import document.
    let create_payload = json!({
        "reference_doctype": target_doctype,
        "import_type": "Insert New Records",
        "mute_emails": 1,
    });
    let created = match client
        .post("/api/v2/document/Data%20Import", &create_payload)
        .await
    {
        Ok(v) => v,
        Err(e) => return output::fail(e.kind(), &e.to_string(), output::EXIT_API),
    };
    let Some(batch) = created
        .pointer("/data/name")
        .and_then(Value::as_str)
        .map(str::to_owned)
    else {
        return output::fail(
            "shape",
            "Data Import creation returned no name",
            output::EXIT_API,
        );
    };

    // 2. Attach the CSV to the batch document.
    let uploaded = match client
        .post_file(
            "/api/method/upload_file",
            &file_name,
            bytes,
            &[
                ("doctype", "Data Import"),
                ("docname", &batch),
                ("is_private", "1"),
            ],
        )
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return output::fail_with_data(
                e.kind(),
                &format!("batch created but file upload failed: {e}"),
                json!({ "batch": batch }),
                output::EXIT_API,
            )
        }
    };
    let file_url = uploaded
        .pointer("/message/file_url")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();

    if let Err(e) = client
        .put(
            &format!("/api/v2/document/Data%20Import/{batch}"),
            &json!({ "import_file": file_url }),
        )
        .await
    {
        return output::fail_with_data(
            e.kind(),
            &format!("file uploaded but could not bind to the batch: {e}"),
            json!({ "batch": batch, "file_url": file_url }),
            output::EXIT_API,
        );
    }

    // 3. The native template preview: parsed columns, mapped fields, warnings.
    let preview = match client
        .post(
            "/api/method/frappe.core.doctype.data_import.data_import.get_preview_from_template",
            &json!({ "data_import": batch }),
        )
        .await
    {
        Ok(v) => v.pointer("/message").cloned().unwrap_or(Value::Null),
        Err(e) => {
            return output::fail_with_data(
                e.kind(),
                &format!("batch prepared but preview failed: {e}"),
                json!({ "batch": batch, "file_url": file_url }),
                output::EXIT_API,
            )
        }
    };

    output::ok(json!({
        "batch": batch,
        "target_doctype": target_doctype,
        "file_url": file_url,
        "preview": preview,
        "apply_command": format!("erpnext import apply --batch {batch}"),
    }))
}

/// `erpnext import apply --batch <name>` — start exactly the previewed batch through
/// the native importer and report the batch status.
pub async fn apply(client: &Client, batch: &str) -> i32 {
    if let Err(e) = client
        .post(
            "/api/method/frappe.core.doctype.data_import.data_import.form_start_import",
            &json!({ "data_import": batch }),
        )
        .await
    {
        return output::fail(e.kind(), &e.to_string(), output::EXIT_API);
    }

    let status = match client
        .get(&format!("/api/v2/document/Data%20Import/{batch}"))
        .await
    {
        Ok(v) => v
            .pointer("/data/status")
            .cloned()
            .unwrap_or_else(|| json!("Started")),
        Err(_) => json!("Started"),
    };

    output::ok(json!({ "batch": batch, "status": status }))
}
