use httpmock::MockServer;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_HOME_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub fn run_cli_against(server: &MockServer, args: &[&str]) -> Output {
    run_cli_at(&server.base_url().replace("127.0.0.1", "localhost"), args)
}

pub fn run_cli_at(site_url: &str, args: &[&str]) -> Output {
    let home = test_home(site_url, false);
    let output = run_cli_from_home(&home, args);
    fs::remove_dir_all(home).expect("remove test home");
    output
}

pub fn test_home(site_url: &str, include_admin: bool) -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let sequence = TEST_HOME_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let home = std::env::temp_dir().join(format!(
        "erpnext-cli-contract-{}-{unique}-{sequence}",
        std::process::id(),
    ));
    let company_home = home.join(".teleclaude/companies/test-co");
    fs::create_dir_all(&company_home).expect("test credential home");
    let credentials = fixture_credentials(site_url, include_admin);
    fs::write(
        company_home.join("erpnext.json"),
        serde_json::to_vec(&credentials).expect("credential json"),
    )
    .expect("write credentials");

    home
}

pub fn fixture_credentials(site_url: &str, include_admin: bool) -> Value {
    let mut credentials = json!({
        "site_url": site_url,
        "company": "Test Company",
        "operator": {
            "user": "operator@instrukt.ai",
            "api_key": "test-key",
            "api_secret": "test-secret"
        }
    });
    if include_admin {
        credentials["admin"] = json!({
            "user": "agent-admin@instrukt.ai",
            "api_key": "admin-key",
            "api_secret": "admin-secret"
        });
    }
    credentials
}

pub fn run_cli_from_home(home: &Path, args: &[&str]) -> Output {
    let mut invocation = vec!["--company", "test-co"];
    invocation.extend_from_slice(args);
    run_cli_with_home(home, &invocation)
}

pub fn run_cli_with_home(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_erpnext"))
        .env("HOME", home)
        .args(args)
        .output()
        .expect("run erpnext CLI")
}

pub fn json_output(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("machine-readable JSON envelope")
}

/// Execute a rendered `apply_command` string through a real POSIX shell against the
/// just-built binary, in an isolated child-process home. This is the one place a
/// replay invocation is proven end-to-end: the emitted string is handed to `sh -c`
/// exactly as a human would paste it, never reconstructed by the test itself.
/// Unused in test binaries that never replay a preview — this shared support module
/// is compiled once per binary via `#[path]`, and not every binary calls every helper.
#[allow(dead_code)]
pub fn run_replay(home: &Path, apply_command: &str) -> Output {
    let binary = PathBuf::from(env!("CARGO_BIN_EXE_erpnext"));
    let bin_dir = binary.parent().expect("binary has a parent directory");
    let path = format!("{}:/usr/bin:/bin", bin_dir.display());
    Command::new("sh")
        .arg("-c")
        .arg(apply_command)
        .env("HOME", home)
        .env("PATH", path)
        .output()
        .expect("run replay through a real shell")
}
