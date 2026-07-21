use serde_json::json;

use crate::config::Context;
use crate::output;

/// `erpnext context show` / the result of `context use` — the active operating
/// context, secret-safe.
pub fn show(ctx: &Context) -> i32 {
    output::ok(json!({
        "site": ctx.site(),
        "site_url": ctx.site_url.as_str(),
        "company_slug": ctx.company_slug,
        "company": ctx.company,
        "privilege": ctx.privilege,
        "credential_home": format!("~/.teleclaude/companies/{}/erpnext.json", ctx.company_slug),
    }))
}
