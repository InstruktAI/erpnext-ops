use std::fmt;

use thiserror::Error;
use url::Url;

/// Privilege mode of the active identity. Operator failures never fall back to admin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivilegeMode {
    Operator,
    Admin,
}

impl fmt::Display for PrivilegeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrivilegeMode::Operator => write!(f, "operator"),
            PrivilegeMode::Admin => write!(f, "admin"),
        }
    }
}

/// API credential pair. Debug/Display never reveal the secret material.
#[derive(Clone)]
pub struct Credentials {
    pub api_key: String,
    pub api_secret: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("api_key", &"[redacted]")
            .field("api_secret", &"[redacted]")
            .finish()
    }
}

/// The resolved operating context: which site, which company, which identity.
#[derive(Debug, Clone)]
pub struct Context {
    pub site_url: Url,
    pub company: Option<String>,
    pub privilege: PrivilegeMode,
    pub credentials: Credentials,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing configuration: {0} (set it in the environment or .env)")]
    Missing(&'static str),
    #[error("invalid site url in ERPNEXT_URL: {0}")]
    InvalidUrl(String),
    #[error("invalid .env file: {0} (quote values containing spaces)")]
    EnvFile(String),
}

/// A missing .env is fine (explicit environment wins); a malformed one fails closed —
/// a suppressed parse error would silently drop every variable after the bad line and
/// misreport a later variable as missing.
fn apply_env_file<T>(result: Result<T, dotenvy::Error>) -> Result<(), ConfigError> {
    match result {
        Ok(_) => Ok(()),
        Err(dotenvy::Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(ConfigError::EnvFile(e.to_string())),
    }
}

impl Context {
    /// Resolve the operating context from the environment, fail-closed:
    /// every required input must be present and unambiguous or resolution fails.
    pub fn from_env() -> Result<Self, ConfigError> {
        apply_env_file(dotenvy::dotenv())?;

        let raw_url =
            std::env::var("ERPNEXT_URL").map_err(|_| ConfigError::Missing("ERPNEXT_URL"))?;
        let site_url =
            Url::parse(&raw_url).map_err(|e| ConfigError::InvalidUrl(format!("{raw_url}: {e}")))?;
        if site_url.scheme() != "https" && site_url.host_str() != Some("localhost") {
            return Err(ConfigError::InvalidUrl(format!(
                "{raw_url}: only https endpoints are accepted"
            )));
        }

        let api_key = std::env::var("ERPNEXT_API_KEY")
            .map_err(|_| ConfigError::Missing("ERPNEXT_API_KEY"))?;
        let api_secret = std::env::var("ERPNEXT_API_SECRET")
            .map_err(|_| ConfigError::Missing("ERPNEXT_API_SECRET"))?;

        let company = std::env::var("ERPNEXT_COMPANY")
            .ok()
            .filter(|c| !c.trim().is_empty());

        let privilege = match std::env::var("ERPNEXT_PRIVILEGE").ok().as_deref() {
            None | Some("operator") => PrivilegeMode::Operator,
            Some("admin") => PrivilegeMode::Admin,
            Some(other) => {
                return Err(ConfigError::InvalidUrl(format!(
                    "ERPNEXT_PRIVILEGE must be 'operator' or 'admin', got '{other}'"
                )))
            }
        };

        Ok(Context {
            site_url,
            company,
            privilege,
            credentials: Credentials {
                api_key,
                api_secret,
            },
        })
    }

    /// Site host name (the tenant identity).
    pub fn site(&self) -> &str {
        self.site_url.host_str().unwrap_or("<invalid>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_env_file_fails_closed_instead_of_truncating() {
        let dir = std::env::temp_dir().join(format!("erpnext-env-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let file = dir.join(".env");
        std::fs::write(&file, "ERPNEXT_COMPANY=Instrukt AI\nERPNEXT_API_KEY=k\n").expect("write");

        let outcome = apply_env_file(dotenvy::from_path(&file));
        let err = outcome.expect_err("unquoted value with spaces must surface a parse error");
        assert!(matches!(err, ConfigError::EnvFile(_)));
        assert!(err.to_string().contains("quote values containing spaces"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn missing_env_file_is_fine() {
        let absent = std::path::Path::new("/nonexistent/erpnext-test/.env");
        assert!(apply_env_file(dotenvy::from_path(absent)).is_ok());
    }

    #[test]
    fn credentials_debug_never_reveals_secrets() {
        let c = Credentials {
            api_key: "k-123".into(),
            api_secret: "s-456".into(),
        };
        let rendered = format!("{c:?}");
        assert!(!rendered.contains("k-123"));
        assert!(!rendered.contains("s-456"));
        assert!(rendered.contains("[redacted]"));
    }
}
