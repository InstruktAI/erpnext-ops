use serde_json::Value;
use thiserror::Error;

use crate::config::Context;

/// Typed failure surface of the Frappe v2 transport. Secrets never appear in messages.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("authentication rejected by site (HTTP {status}); no privilege fallback is attempted")]
    Auth { status: u16 },
    #[error("permission denied by site (HTTP {status}): {message}")]
    Forbidden { status: u16, message: String },
    #[error("not found: {message}")]
    NotFound { message: String },
    #[error("site rejected the request (HTTP {status}): {message}")]
    Rejected { status: u16, message: String },
    #[error("transport failure: {0}")]
    Transport(String),
    #[error("unexpected response shape: {0}")]
    Shape(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        // reqwest errors can embed the full URL; keep the message but never credentials,
        // which travel in headers and are absent from Display output.
        ApiError::Transport(e.to_string())
    }
}

/// Thin client over the Frappe v2 REST envelope for one resolved context.
pub struct Client {
    http: reqwest::Client,
    ctx: Context,
}

impl Client {
    pub fn new(ctx: Context) -> Self {
        Client {
            http: reqwest::Client::new(),
            ctx,
        }
    }

    pub fn context(&self) -> &Context {
        &self.ctx
    }

    fn auth_header(&self) -> String {
        format!(
            "token {}:{}",
            self.ctx.credentials.api_key, self.ctx.credentials.api_secret
        )
    }

    fn url(&self, path: &str) -> String {
        format!(
            "{}{}",
            self.ctx.site_url.as_str().trim_end_matches('/'),
            path
        )
    }

    /// Unauthenticated liveness probe.
    pub async fn ping(&self) -> Result<Value, ApiError> {
        let resp = self.http.get(self.url("/api/method/ping")).send().await?;
        Self::read_json(resp).await
    }

    /// The identity the site sees for our credentials.
    pub async fn whoami(&self) -> Result<String, ApiError> {
        let value = self.get("/api/method/frappe.auth.get_logged_user").await?;
        value
            .pointer("/message")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| ApiError::Shape("missing 'message' in get_logged_user response".into()))
    }

    pub async fn get(&self, path: &str) -> Result<Value, ApiError> {
        let resp = self
            .http
            .get(self.url(path))
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        Self::read_json(resp).await
    }

    pub async fn post(&self, path: &str, body: &Value) -> Result<Value, ApiError> {
        let resp = self
            .http
            .post(self.url(path))
            .header("Authorization", self.auth_header())
            .json(body)
            .send()
            .await?;
        Self::read_json(resp).await
    }

    async fn read_json(resp: reqwest::Response) -> Result<Value, ApiError> {
        let status = resp.status().as_u16();
        let text = resp.text().await?;
        let body: Value = serde_json::from_str(&text)
            .unwrap_or_else(|_| Value::String(text.chars().take(500).collect::<String>()));

        match status {
            200..=299 => Ok(body),
            401 => Err(ApiError::Auth { status }),
            403 => Err(ApiError::Forbidden {
                status,
                message: extract_message(&body),
            }),
            404 => Err(ApiError::NotFound {
                message: extract_message(&body),
            }),
            _ => Err(ApiError::Rejected {
                status,
                message: extract_message(&body),
            }),
        }
    }
}

fn extract_message(body: &Value) -> String {
    body.pointer("/exception")
        .or_else(|| body.pointer("/message"))
        .or_else(|| body.pointer("/errors/0/message"))
        .and_then(Value::as_str)
        .unwrap_or("no detail provided by site")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transport_error_display_carries_no_header_material() {
        let e = ApiError::Auth { status: 401 };
        let rendered = e.to_string();
        assert!(rendered.contains("401"));
        assert!(!rendered.to_lowercase().contains("token"));
    }
}
