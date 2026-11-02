use httpmock::prelude::*;
use serde_json::{json, Value};
use std::fs;

#[path = "support/cli.rs"]
mod cli;

#[test]
// spec: project/spec/feature/erp-operations/activity-trail UC-4
fn evidence_attach_uploads_to_the_reference_and_records_its_summary() {
    let server = MockServer::start();
    let file_path = std::env::temp_dir()
        .join(format!("erpnext-activity-evidence-{}", std::process::id()))
        .join("evidence.txt");
    fs::create_dir_all(file_path.parent().expect("fixture parent"))
        .expect("create evidence fixture directory");
    fs::write(&file_path, "evidence").expect("write evidence fixture");
    let upload = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/upload_file")
            .body_contains("name=\"doctype\"")
            .body_contains("Customer")
            .body_contains("C-1");
        then.status(200).json_body(json!({
            "message": { "file_url": "/private/files/proof.txt" }
        }));
    });
    let trail = server.mock(|when, then| {
        when.method(POST)
            .path("/api/method/frappe.desk.form.utils.add_comment")
            .body_contains("\"reference_doctype\":\"Customer\"")
            .body_contains("\"reference_name\":\"C-1\"")
            .body_contains("Evidence summary (attachment: /private/files/proof.txt)");
        then.status(200)
            .json_body(json!({ "message": { "name": "COMMENT-1" }}));
    });

    let file = file_path.to_string_lossy().into_owned();
    let output = cli::run_cli_against(
        &server,
        &[
            "evidence",
            "attach",
            "--reference",
            "Customer",
            "C-1",
            "--file",
            &file,
            "--summary",
            "Evidence summary",
        ],
    );
    fs::remove_file(file_path).expect("remove evidence fixture");
    fs::remove_dir_all(
        std::env::temp_dir().join(format!("erpnext-activity-evidence-{}", std::process::id())),
    )
    .expect("remove evidence fixture directory");

    assert!(output.status.success(), "{output:?}");
    let envelope: Value = cli::json_output(&output.stdout);
    insta::assert_json_snapshot!(envelope, @r#"
    {
      "data": {
        "attached": [
          {
            "file_name": "evidence.txt",
            "file_url": "/private/files/proof.txt",
            "kind": "File"
          },
          {
            "kind": "Comment",
            "result": "COMMENT-1"
          }
        ]
      },
      "ok": true
    }
    "#);
    assert_eq!(upload.hits(), 1);
    assert_eq!(trail.hits(), 1);
}
