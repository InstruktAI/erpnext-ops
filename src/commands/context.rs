use serde_json::json;

use crate::config::Context;
use crate::output;

/// `erpnext context show` — the active operating context, secret-safe.
pub fn show(ctx: &Context) -> i32 {
    output::ok(json!({
        "site": ctx.site(),
        "site_url": ctx.site_url.as_str(),
        "company": ctx.company,
        "privilege": ctx.privilege,
        "identity": "resolved at runtime via ops doctor (whoami)",
    }))
}
