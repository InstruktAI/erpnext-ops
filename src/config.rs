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
}

impl Context {
    /// Resolve the operating context from the environment, fail-closed:
    /// every required input must be present and unambiguous or resolution fails.
    pub fn from_env() -> Result<Self, ConfigError> {
        // A missing .env is fine; explicit environment variables take precedence anyway.
        let _ = dotenvy::dotenv();

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
