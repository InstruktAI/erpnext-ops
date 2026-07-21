use serde::Serialize;
use serde_json::{json, Value};

/// Exit codes of the documented contract: 0 success, 1 site/API failure, 2 configuration
/// or usage failure.
pub const EXIT_OK: i32 = 0;
pub const EXIT_API: i32 = 1;
pub const EXIT_CONFIG: i32 = 2;

/// Print the machine-readable success envelope to stdout.
pub fn ok<T: Serialize>(data: T) -> i32 {
    let envelope = json!({ "ok": true, "data": data });
    println!(
        "{}",
        serde_json::to_string_pretty(&envelope).unwrap_or_else(|_| envelope.to_string())
    );
    EXIT_OK
}

/// Print the machine-readable failure envelope to stderr and return the exit code.
pub fn fail(kind: &str, message: &str, exit: i32) -> i32 {
    let envelope: Value = json!({ "ok": false, "error": { "kind": kind, "message": message } });
    eprintln!(
        "{}",
        serde_json::to_string_pretty(&envelope).unwrap_or_else(|_| envelope.to_string())
    );
    exit
}

/// Failure envelope that preserves partial results (e.g. writes that landed before
/// a later step failed), so a caller never mistakes partial success for a no-op.
pub fn fail_with_data<T: Serialize>(kind: &str, message: &str, data: T, exit: i32) -> i32 {
    let envelope: Value = json!({
        "ok": false,
        "error": { "kind": kind, "message": message },
        "data": data,
    });
    eprintln!(
        "{}",
        serde_json::to_string_pretty(&envelope).unwrap_or_else(|_| envelope.to_string())
    );
    exit
}
