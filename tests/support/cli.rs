use httpmock::MockServer;
use serde_json::{json, Value};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedRequest {
    pub method: String,
    pub path: String,
}

#[allow(dead_code)]
pub struct OrderedResponse {
    pub request: ObservedRequest,
    pub status: u16,
    pub body: Value,
}

#[allow(dead_code)]
impl OrderedResponse {
    pub fn new(method: &str, path: &str, status: u16, body: Value) -> Self {
        Self {
            request: ObservedRequest {
                method: method.into(),
                path: path.into(),
            },
            status,
            body,
        }
    }
}

/// Serve a request-ordered response sequence for flows that read one endpoint
/// before and after a mutation. The per-server request log is the observed order;
/// mismatches receive a failure response. `httpmock` custom matchers are function
/// pointers, so they cannot carry the per-server sequence state needed here.
#[allow(dead_code)]
pub struct OrderedResponseServer {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<ObservedRequest>>>,
    server: Option<JoinHandle<()>>,
}

#[allow(dead_code)]
impl OrderedResponseServer {
    pub fn start(responses: Vec<OrderedResponse>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ordered mock server");
        let address = listener.local_addr().expect("ordered mock address");
        let requests = Arc::new(Mutex::new(Vec::new()));
        let observed = Arc::clone(&requests);
        let server = std::thread::spawn(move || {
            let mut response_index = 0;
            while let Ok((mut stream, _)) = listener.accept() {
                let Some(request) = read_request(&mut stream) else {
                    break;
                };
                if request.path == "/__shutdown__" {
                    break;
                }
                observed
                    .lock()
                    .expect("ordered mock request log")
                    .push(request.clone());

                let response = responses.get(response_index);
                let (status, body) = match response {
                    Some(response) if response.request == request => {
                        response_index += 1;
                        (response.status, response.body.clone())
                    }
                    _ => (500, json!({ "error": "unexpected request" })),
                };
                write_response(stream, status, &body);
            }
        });

        Self {
            address,
            requests,
            server: Some(server),
        }
    }

    pub fn base_url(&self) -> String {
        format!("http://localhost:{}", self.address.port())
    }

    pub fn finish(mut self) -> Vec<ObservedRequest> {
        self.stop();
        self.requests
            .lock()
            .expect("ordered mock request log")
            .clone()
    }

    fn stop(&mut self) {
        if let Some(server) = self.server.take() {
            if let Ok(mut stream) = TcpStream::connect(self.address) {
                let _ = stream.write_all(
                    b"GET /__shutdown__ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
                );
            }
            server.join().expect("ordered mock server exits");
        }
    }
}

impl Drop for OrderedResponseServer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[allow(dead_code)]
fn read_request(stream: &mut TcpStream) -> Option<ObservedRequest> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).ok()?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?.to_owned();
    let path = parts.next()?.split('?').next()?.to_owned();
    let mut content_length = 0_usize;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).ok()?;
        if line == "\r\n" || line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse().ok()?;
            }
        }
    }
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).ok()?;
    Some(ObservedRequest { method, path })
}

#[allow(dead_code)]
fn write_response(mut stream: TcpStream, status: u16, body: &Value) {
    let body = serde_json::to_vec(body).expect("serialize ordered mock response");
    let reason = match status {
        200 => "OK",
        403 => "Forbidden",
        404 => "Not Found",
        _ => "Internal Server Error",
    };
    let headers = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(headers.as_bytes());
    let _ = stream.write_all(&body);
    let _ = stream.flush();
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
