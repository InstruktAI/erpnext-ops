use clap::{Parser, Subcommand};

use erpnext::config::{self, Paths, PrivilegeMode};
use erpnext::transport::Client;
use erpnext::{commands, output};

/// Agent-facing operating surface for per-tenant ERPNext sites.
///
/// Mutating commands preview by default: they show the exact native operation that
/// would be sent, without sending it. `--apply` sends it and reports the site's result.
#[derive(Parser)]
#[command(name = "erpnext", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Operating context: site, company, and privilege identity
    Context {
        #[command(subcommand)]
        command: ContextCommand,
    },
    /// Live DocType schema from the running site
    Meta {
        #[command(subcommand)]
        command: MetaCommand,
    },
    /// Deterministic resolution of parties, items, and documents
    Resolve {
        #[command(subcommand)]
        command: ResolveCommand,
    },
    /// Generic document operations through the Frappe v2 envelope
    Doc {
        #[command(subcommand)]
        command: DocCommand,
    },
    /// Document lifecycle operations honoring docstatus semantics
    Workflow {
        #[command(subcommand)]
        command: WorkflowCommand,
    },
    /// Operational diagnostics of the CLI's own ground
    Ops {
        #[command(subcommand)]
        command: OpsCommand,
    },
}

#[derive(Subcommand)]
enum ContextCommand {
    /// Show the active context, secret-safe
    Show,
    /// Select the operating context by company scaffold or site FQDN
    Use {
        /// Site FQDN to operate (resolved to its company scaffold, fail-closed)
        #[arg(long)]
        site: Option<String>,
        /// Company slug in the TeleClaude company registry
        #[arg(long)]
        company: Option<String>,
        /// Privilege identity: operator (default) or admin
        #[arg(long = "as", default_value = "operator")]
        privilege: PrivilegeMode,
    },
}

#[derive(Subcommand)]
enum MetaCommand {
    /// Show the live schema for a doctype
    Show { doctype: String },
}

#[derive(Subcommand)]
enum ResolveCommand {
    /// Resolve a customer or supplier by exact name or title
    Party { query: String },
    /// Resolve an item by exact code or item name
    Item { query: String },
    /// Resolve a document of a doctype by its exact name
    Document { doctype: String, query: String },
}

#[derive(Subcommand)]
enum DocCommand {
    /// List documents of a doctype
    List {
        doctype: String,
        /// Field filter as field=value (repeatable)
        #[arg(long = "filter", value_parser = parse_filter)]
        filters: Vec<(String, String)>,
        #[arg(long, default_value_t = 20)]
        limit: u32,
    },
    /// Get one document by name
    Get { doctype: String, name: String },
    /// Update a document (preview by default; --apply sends it)
    Update {
        doctype: String,
        name: String,
        /// JSON object payload of fields to change
        #[arg(long)]
        data: String,
        /// Send the native operation instead of previewing it
        #[arg(long)]
        apply: bool,
    },
    /// Delete a document (preview by default; --apply sends it)
    Delete {
        doctype: String,
        name: String,
        /// Send the native operation instead of previewing it
        #[arg(long)]
        apply: bool,
    },
    /// Copy a document into a new draft (preview by default; --apply sends it)
    Copy {
        doctype: String,
        name: String,
        /// JSON object of field overrides for the copy
        #[arg(long)]
        data: Option<String>,
        /// Send the native operation instead of previewing it
        #[arg(long)]
        apply: bool,
    },
    /// Create a document (preview by default; --apply sends it)
    Create {
        doctype: String,
        /// JSON object payload
        #[arg(long)]
        data: String,
        /// Send the native operation instead of previewing it
        #[arg(long)]
        apply: bool,
    },
}

#[derive(Subcommand)]
enum WorkflowCommand {
    /// Submit a draft document (preview by default)
    Submit {
        doctype: String,
        name: String,
        #[arg(long)]
        apply: bool,
    },
    /// Cancel a submitted document (preview by default)
    Cancel {
        doctype: String,
        name: String,
        #[arg(long)]
        apply: bool,
    },
    /// Run a whitelisted controller method on a document (preview by default)
    Method {
        doctype: String,
        name: String,
        method: String,
        /// JSON object payload for the method
        #[arg(long)]
        data: Option<String>,
        #[arg(long)]
        apply: bool,
    },
}

#[derive(Subcommand)]
enum OpsCommand {
    /// Read-only checks: endpoint, identity, company, privilege separation
    Doctor,
}

fn parse_filter(raw: &str) -> Result<(String, String), String> {
    raw.split_once('=')
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .ok_or_else(|| format!("filter must be field=value, got '{raw}'"))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let paths = match Paths::from_home() {
        Ok(paths) => paths,
        Err(e) => std::process::exit(output::fail("config", &e.to_string(), output::EXIT_CONFIG)),
    };

    if let Command::Context { command } = &cli.command {
        let exit = match command {
            ContextCommand::Show => match config::resolve(&paths) {
                Ok(ctx) => commands::context::show(&ctx),
                Err(e) => output::fail("config", &e.to_string(), output::EXIT_CONFIG),
            },
            ContextCommand::Use {
                site,
                company,
                privilege,
            } => match config::select(&paths, site.as_deref(), company.as_deref(), *privilege) {
                Ok(ctx) => commands::context::show(&ctx),
                Err(e) => output::fail("config", &e.to_string(), output::EXIT_CONFIG),
            },
        };
        std::process::exit(exit);
    }

    let ctx = match config::resolve(&paths) {
        Ok(ctx) => ctx,
        Err(e) => std::process::exit(output::fail("config", &e.to_string(), output::EXIT_CONFIG)),
    };
    let client = match Client::new(ctx) {
        Ok(client) => client,
        Err(e) => std::process::exit(output::fail(
            "transport",
            &e.to_string(),
            output::EXIT_CONFIG,
        )),
    };

    let exit = match cli.command {
        Command::Context { .. } => unreachable!("handled above"),
        Command::Meta {
            command: MetaCommand::Show { doctype },
        } => commands::meta::show(&client, &doctype).await,
        Command::Resolve { command } => match command {
            ResolveCommand::Party { query } => commands::resolve::party(&client, &query).await,
            ResolveCommand::Item { query } => commands::resolve::item(&client, &query).await,
            ResolveCommand::Document { doctype, query } => {
                commands::resolve::document(&client, &doctype, &query).await
            }
        },
        Command::Doc { command } => match command {
            DocCommand::List {
                doctype,
                filters,
                limit,
            } => commands::doc::list(&client, &doctype, &filters, limit).await,
            DocCommand::Get { doctype, name } => commands::doc::get(&client, &doctype, &name).await,
            DocCommand::Create {
                doctype,
                data,
                apply,
            } => commands::doc::create(&client, &doctype, &data, apply).await,
            DocCommand::Update {
                doctype,
                name,
                data,
                apply,
            } => commands::doc::update(&client, &doctype, &name, &data, apply).await,
            DocCommand::Delete {
                doctype,
                name,
                apply,
            } => commands::doc::delete(&client, &doctype, &name, apply).await,
            DocCommand::Copy {
                doctype,
                name,
                data,
                apply,
            } => commands::doc::copy(&client, &doctype, &name, data.as_deref(), apply).await,
        },
        Command::Workflow { command } => match command {
            WorkflowCommand::Submit {
                doctype,
                name,
                apply,
            } => {
                commands::workflow::run_method(&client, &doctype, &name, "submit", None, apply)
                    .await
            }
            WorkflowCommand::Cancel {
                doctype,
                name,
                apply,
            } => {
                commands::workflow::run_method(&client, &doctype, &name, "cancel", None, apply)
                    .await
            }
            WorkflowCommand::Method {
                doctype,
                name,
                method,
                data,
                apply,
            } => {
                commands::workflow::run_method(
                    &client,
                    &doctype,
                    &name,
                    &method,
                    data.as_deref(),
                    apply,
                )
                .await
            }
        },
        Command::Ops {
            command: OpsCommand::Doctor,
        } => commands::ops::doctor(&client).await,
    };

    std::process::exit(exit);
}
