use clap::{Parser, Subcommand};

use erpnext::config::Context;
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
}

#[derive(Subcommand)]
enum MetaCommand {
    /// Show the live schema for a doctype
    Show { doctype: String },
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

fn client_or_exit(ctx: Context) -> Client {
    match Client::new(ctx) {
        Ok(client) => client,
        Err(e) => std::process::exit(output::fail(
            "transport",
            &e.to_string(),
            output::EXIT_CONFIG,
        )),
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let ctx = match Context::from_env() {
        Ok(ctx) => ctx,
        Err(e) => std::process::exit(output::fail("config", &e.to_string(), output::EXIT_CONFIG)),
    };

    let exit = match cli.command {
        Command::Context {
            command: ContextCommand::Show,
        } => commands::context::show(&ctx),
        Command::Meta {
            command: MetaCommand::Show { doctype },
        } => commands::meta::show(&client_or_exit(ctx), &doctype).await,
        Command::Doc { command } => {
            let client = client_or_exit(ctx);
            match command {
                DocCommand::List {
                    doctype,
                    filters,
                    limit,
                } => commands::doc::list(&client, &doctype, &filters, limit).await,
                DocCommand::Get { doctype, name } => {
                    commands::doc::get(&client, &doctype, &name).await
                }
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
            }
        }
        Command::Workflow { command } => {
            let client = client_or_exit(ctx);
            match command {
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
            }
        }
        Command::Ops {
            command: OpsCommand::Doctor,
        } => commands::ops::doctor(&client_or_exit(ctx)).await,
    };

    std::process::exit(exit);
}
