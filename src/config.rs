use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;
use url::Url;

/// Privilege mode of the active identity. Operator failures never fall back to admin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl std::str::FromStr for PrivilegeMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "operator" => Ok(PrivilegeMode::Operator),
            "admin" => Ok(PrivilegeMode::Admin),
            other => Err(format!(
                "privilege must be 'operator' or 'admin', got '{other}'"
            )),
        }
    }
}

/// API credential pair. Debug/Display never reveal the secret material.
#[derive(Clone, serde::Deserialize)]
pub struct Credentials {
    #[serde(default)]
    pub user: Option<String>,
    pub api_key: String,
    pub api_secret: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("user", &self.user)
            .field("api_key", &"[redacted]")
            .field("api_secret", &"[redacted]")
            .finish()
    }
}

/// The resolved operating context: which site, which company, which identity.
#[derive(Debug, Clone)]
pub struct Context {
    pub site_url: Url,
    pub company_slug: String,
    pub company: Option<String>,
    pub privilege: PrivilegeMode,
    pub credentials: Credentials,
}

/// The persisted selection: which company scaffold and privilege the agent chose.
/// Credentials are never stored here — they resolve from the company credential
/// home at command time.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Selection {
    pub company_slug: String,
    pub privilege: PrivilegeMode,
}

/// The per-company credential home entry: `<registry>/<slug>/erpnext.json`.
#[derive(serde::Deserialize)]
struct CredentialFile {
    site_url: String,
    company: Option<String>,
    operator: Credentials,
    #[serde(default)]
    admin: Option<Credentials>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("no operating context selected; run `erpnext context use --company <slug>` (or --site <fqdn>)")]
    NoSelection,
    #[error("selection file is invalid: {0}")]
    InvalidSelection(String),
    #[error("company '{0}' has no credential home entry ({1})")]
    NoCredentialHome(String, String),
    #[error("credential home entry for '{0}' is invalid: {1}")]
    InvalidCredentialFile(String, String),
    #[error("invalid site url in credential home for '{0}': {1}")]
    InvalidUrl(String, String),
    #[error("no company credential home matches site '{0}'")]
    UnknownSite(String),
    #[error("site '{0}' is served by multiple company scaffolds ({1}); select by --company")]
    AmbiguousSite(String, String),
    #[error("company '{0}' has no {1} credentials in credential home ({2}); no privilege fallback is allowed")]
    MissingPrivilegeCredentials(String, PrivilegeMode, String),
    #[error("company '{0}' has no declared user for its dedicated agent-admin credentials")]
    MissingAdminIdentity(String),
    #[error("bootstrap Administrator credentials are refused; configure a dedicated agent-admin identity for company '{0}'")]
    BootstrapAdministratorRefused(String),
    #[error("configured {0} identity '{1}' authenticated as '{2}'")]
    IdentityMismatch(PrivilegeMode, String, String),
    #[error("cannot locate home directory")]
    NoHome,
    #[error("io failure at {0}: {1}")]
    Io(String, String),
}

/// Filesystem roots the CLI operates on. Injectable so tests never touch real state.
#[derive(Debug, Clone)]
pub struct Paths {
    /// Selection state: `~/.config/erpnext/context.json`.
    pub selection_file: PathBuf,
    /// Company registry root: `~/.teleclaude/companies`.
    pub registry_root: PathBuf,
}

impl Paths {
    pub fn from_home() -> Result<Self, ConfigError> {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or(ConfigError::NoHome)?;
        Ok(Paths {
            selection_file: home.join(".config").join("erpnext").join("context.json"),
            registry_root: home.join(".teleclaude").join("companies"),
        })
    }
}

/// Persist a selection after validating it resolves to a credential home entry.
pub fn select(
    paths: &Paths,
    site: Option<&str>,
    company_slug: Option<&str>,
    privilege: PrivilegeMode,
) -> Result<Context, ConfigError> {
    let slug = match (company_slug, site) {
        (Some(slug), _) => slug.to_owned(),
        (None, Some(site)) => find_slug_by_site(&paths.registry_root, site)?,
        (None, None) => return Err(ConfigError::NoSelection),
    };

    let selection = Selection {
        company_slug: slug,
        privilege,
    };
    let ctx = resolve_selection(paths, &selection)?;

    if let Some(parent) = paths.selection_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| ConfigError::Io(parent.display().to_string(), e.to_string()))?;
    }
    let rendered = serde_json::to_string_pretty(&selection)
        .map_err(|e| ConfigError::InvalidSelection(e.to_string()))?;
    fs::write(&paths.selection_file, rendered)
        .map_err(|e| ConfigError::Io(paths.selection_file.display().to_string(), e.to_string()))?;

    Ok(ctx)
}

/// Resolve a context for one invocation without touching the persisted selection.
/// An explicit company or site makes a command self-contained, so concurrent callers
/// sharing a home directory never depend on each other's stored choice.
pub fn resolve_override(
    paths: &Paths,
    site: Option<&str>,
    company_slug: Option<&str>,
    privilege: PrivilegeMode,
) -> Result<Context, ConfigError> {
    let slug = match (company_slug, site) {
        (Some(slug), _) => slug.to_owned(),
        (None, Some(site)) => find_slug_by_site(&paths.registry_root, site)?,
        (None, None) => return Err(ConfigError::NoSelection),
    };
    resolve_selection(
        paths,
        &Selection {
            company_slug: slug,
            privilege,
        },
    )
}

/// Resolve the selected company under an explicit privilege for one invocation,
/// without changing the persisted selection.
pub fn resolve_with_privilege(
    paths: &Paths,
    privilege: PrivilegeMode,
) -> Result<Context, ConfigError> {
    let mut selection = read_selection(paths)?;
    selection.privilege = privilege;
    resolve_selection(paths, &selection)
}

/// Resolve the active context from the persisted selection and the credential home.
pub fn resolve(paths: &Paths) -> Result<Context, ConfigError> {
    let selection = read_selection(paths)?;
    resolve_selection(paths, &selection)
}

fn read_selection(paths: &Paths) -> Result<Selection, ConfigError> {
    let raw = fs::read_to_string(&paths.selection_file).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ConfigError::NoSelection
        } else {
            ConfigError::Io(paths.selection_file.display().to_string(), e.to_string())
        }
    })?;
    serde_json::from_str(&raw).map_err(|e| ConfigError::InvalidSelection(e.to_string()))
}

fn resolve_selection(paths: &Paths, selection: &Selection) -> Result<Context, ConfigError> {
    let slug = &selection.company_slug;
    let file = credential_file_path(&paths.registry_root, slug);
    let raw = fs::read_to_string(&file)
        .map_err(|_| ConfigError::NoCredentialHome(slug.clone(), file.display().to_string()))?;
    let parsed: CredentialFile = serde_json::from_str(&raw)
        .map_err(|e| ConfigError::InvalidCredentialFile(slug.clone(), e.to_string()))?;

    let site_url = Url::parse(&parsed.site_url)
        .map_err(|e| ConfigError::InvalidUrl(slug.clone(), format!("{}: {e}", parsed.site_url)))?;
    if site_url.scheme() != "https" && site_url.host_str() != Some("localhost") {
        return Err(ConfigError::InvalidUrl(
            slug.clone(),
            format!("{}: only https endpoints are accepted", parsed.site_url),
        ));
    }

    let credentials = match selection.privilege {
        PrivilegeMode::Operator => parsed.operator,
        PrivilegeMode::Admin => {
            let credentials = parsed.admin.ok_or_else(|| {
                ConfigError::MissingPrivilegeCredentials(
                    slug.clone(),
                    PrivilegeMode::Admin,
                    file.display().to_string(),
                )
            })?;
            let user = credentials
                .user
                .as_deref()
                .ok_or_else(|| ConfigError::MissingAdminIdentity(slug.clone()))?;
            if user.eq_ignore_ascii_case("Administrator") {
                return Err(ConfigError::BootstrapAdministratorRefused(slug.clone()));
            }
            credentials
        }
    };

    Ok(Context {
        site_url,
        company_slug: slug.clone(),
        company: parsed.company,
        privilege: selection.privilege,
        credentials,
    })
}

/// Verify that the site authenticated the configured identity. Admin commands use this
/// before doing work so bootstrap credentials cannot be smuggled in under another label.
pub fn validate_authenticated_identity(
    context: &Context,
    authenticated_user: &str,
) -> Result<(), ConfigError> {
    if authenticated_user.eq_ignore_ascii_case("Administrator") {
        return Err(ConfigError::BootstrapAdministratorRefused(
            context.company_slug.clone(),
        ));
    }
    if let Some(configured_user) = context.credentials.user.as_deref() {
        if configured_user != authenticated_user {
            return Err(ConfigError::IdentityMismatch(
                context.privilege,
                configured_user.to_owned(),
                authenticated_user.to_owned(),
            ));
        }
    }
    Ok(())
}

fn credential_file_path(registry_root: &Path, slug: &str) -> PathBuf {
    registry_root.join(slug).join("erpnext.json")
}

/// Site-first selection: find the one company scaffold whose credential home serves
/// the site. Ambiguity and misses are structured failures, never guesses.
fn find_slug_by_site(registry_root: &Path, site: &str) -> Result<String, ConfigError> {
    let entries = fs::read_dir(registry_root)
        .map_err(|e| ConfigError::Io(registry_root.display().to_string(), e.to_string()))?;

    let mut matches: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let slug = entry.file_name().to_string_lossy().into_owned();
        let file = credential_file_path(registry_root, &slug);
        let Ok(raw) = fs::read_to_string(&file) else {
            continue;
        };
        let Ok(parsed) = serde_json::from_str::<CredentialFile>(&raw) else {
            continue;
        };
        if Url::parse(&parsed.site_url)
            .ok()
            .and_then(|u| u.host_str().map(str::to_owned))
            == Some(site.to_owned())
        {
            matches.push(slug);
        }
    }

    match matches.len() {
        0 => Err(ConfigError::UnknownSite(site.to_owned())),
        1 => Ok(matches.remove(0)),
        _ => Err(ConfigError::AmbiguousSite(
            site.to_owned(),
            matches.join(", "),
        )),
    }
}

impl Context {
    /// Site host name (the tenant identity).
    pub fn site(&self) -> &str {
        self.site_url.host_str().unwrap_or("<invalid>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_paths(tag: &str) -> Paths {
        let base = std::env::temp_dir().join(format!("erpnext-cfg-{tag}-{}", std::process::id()));
        fs::create_dir_all(base.join("companies")).expect("temp registry");
        Paths {
            selection_file: base.join("config").join("context.json"),
            registry_root: base.join("companies"),
        }
    }

    fn write_company(paths: &Paths, slug: &str, site: &str, company: &str) {
        let dir = paths.registry_root.join(slug);
        fs::create_dir_all(&dir).expect("company dir");
        fs::write(
            dir.join("erpnext.json"),
            format!(
                r#"{{"site_url":"https://{site}","company":"{company}","operator":{{"api_key":"k","api_secret":"s"}}}}"#
            ),
        )
        .expect("credential file");
    }

    #[test]
    fn select_by_company_persists_and_resolves() {
        let paths = temp_paths("by-company");
        write_company(&paths, "instrukt-ai", "one.erpnext.example", "Instrukt AI");

        let ctx = select(&paths, None, Some("instrukt-ai"), PrivilegeMode::Operator)
            .expect("selection resolves");
        assert_eq!(ctx.site(), "one.erpnext.example");
        assert_eq!(ctx.company.as_deref(), Some("Instrukt AI"));

        let resolved = resolve(&paths).expect("persisted selection resolves");
        assert_eq!(resolved.company_slug, "instrukt-ai");
        fs::remove_dir_all(paths.registry_root.parent().unwrap()).ok();
    }

    #[test]
    fn select_by_site_is_fail_closed_on_ambiguity() {
        let paths = temp_paths("ambiguous");
        write_company(&paths, "a", "shared.erpnext.example", "A");
        write_company(&paths, "b", "shared.erpnext.example", "B");

        let err = select(
            &paths,
            Some("shared.erpnext.example"),
            None,
            PrivilegeMode::Operator,
        )
        .expect_err("two scaffolds for one site must not auto-pick");
        assert!(matches!(err, ConfigError::AmbiguousSite(..)));
        fs::remove_dir_all(paths.registry_root.parent().unwrap()).ok();
    }

    #[test]
    fn no_selection_is_a_structured_blocker() {
        let paths = temp_paths("noselect");
        let err = resolve(&paths).expect_err("missing selection must not guess");
        assert!(matches!(err, ConfigError::NoSelection));
        fs::remove_dir_all(paths.registry_root.parent().unwrap()).ok();
    }

    /// project/spec/feature/erp-operations/operating-context UC-3
    #[test]
    fn missing_admin_credentials_never_fall_back_to_operator() {
        let paths = temp_paths("admin");
        write_company(&paths, "x", "x.erpnext.example", "X");
        let err = select(&paths, None, Some("x"), PrivilegeMode::Admin)
            .expect_err("admin resolution must not use operator credentials");
        assert!(matches!(err, ConfigError::MissingPrivilegeCredentials(..)));
        fs::remove_dir_all(paths.registry_root.parent().unwrap()).ok();
    }

    /// project/spec/feature/erp-operations/operating-context UC-3
    #[test]
    fn dedicated_agent_admin_credentials_resolve() {
        let paths = temp_paths("dedicated-admin");
        let dir = paths.registry_root.join("x");
        fs::create_dir_all(&dir).expect("company dir");
        fs::write(
            dir.join("erpnext.json"),
            r#"{"site_url":"https://x.erpnext.example","company":"X","operator":{"api_key":"operator-key","api_secret":"operator-secret"},"admin":{"user":"agent-admin@instrukt.ai","api_key":"admin-key","api_secret":"admin-secret"}}"#,
        )
        .expect("credential file");

        let ctx = select(&paths, None, Some("x"), PrivilegeMode::Admin)
            .expect("dedicated admin resolves");
        assert_eq!(ctx.privilege, PrivilegeMode::Admin);
        assert_eq!(
            ctx.credentials.user.as_deref(),
            Some("agent-admin@instrukt.ai")
        );
        assert_eq!(ctx.credentials.api_key, "admin-key");
        fs::remove_dir_all(paths.registry_root.parent().unwrap()).ok();
    }

    /// project/spec/feature/erp-operations/operating-context UC-3
    #[test]
    fn bootstrap_administrator_credentials_are_refused() {
        let paths = temp_paths("bootstrap-admin");
        let dir = paths.registry_root.join("x");
        fs::create_dir_all(&dir).expect("company dir");
        fs::write(
            dir.join("erpnext.json"),
            r#"{"site_url":"https://x.erpnext.example","company":"X","operator":{"api_key":"operator-key","api_secret":"operator-secret"},"admin":{"user":"Administrator","api_key":"admin-key","api_secret":"admin-secret"}}"#,
        )
        .expect("credential file");

        let err = select(&paths, None, Some("x"), PrivilegeMode::Admin)
            .expect_err("bootstrap identity must remain unavailable");
        assert!(matches!(
            err,
            ConfigError::BootstrapAdministratorRefused(..)
        ));
        fs::remove_dir_all(paths.registry_root.parent().unwrap()).ok();
    }

    /// project/spec/feature/erp-operations/operating-context UC-3
    #[test]
    fn authenticated_admin_must_match_the_configured_identity() {
        let context = Context {
            site_url: Url::parse("https://x.erpnext.example").expect("url"),
            company_slug: "x".into(),
            company: Some("X".into()),
            privilege: PrivilegeMode::Admin,
            credentials: Credentials {
                user: Some("agent-admin@instrukt.ai".into()),
                api_key: "key".into(),
                api_secret: "secret".into(),
            },
        };

        let err = validate_authenticated_identity(&context, "other-admin@instrukt.ai")
            .expect_err("the authenticated identity must match configuration");
        assert!(matches!(err, ConfigError::IdentityMismatch(..)));
    }

    #[test]
    fn credentials_debug_never_reveals_secrets() {
        let c = Credentials {
            user: Some("agent@instrukt.ai".into()),
            api_key: "k-123".into(),
            api_secret: "s-456".into(),
        };
        let rendered = format!("{c:?}");
        assert!(!rendered.contains("k-123"));
        assert!(!rendered.contains("s-456"));
        assert!(rendered.contains("[redacted]"));
    }
}
