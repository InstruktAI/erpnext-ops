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
    /// Operate this company for this invocation, ignoring the persisted selection
    #[arg(long, global = true)]
    company: Option<String>,
    /// Operate this site for this invocation, ignoring the persisted selection
    #[arg(long, global = true)]
    site: Option<String>,
    /// Privilege identity for this invocation: operator (default) or admin
    #[arg(long = "as", global = true)]
    privilege: Option<PrivilegeMode>,
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
    /// Business activity trail over native records
    Activity {
        #[command(subcommand)]
        command: ActivityCommand,
    },
    /// Attach evidence files to business documents
    Evidence {
        #[command(subcommand)]
        command: EvidenceCommand,
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
    /// Batch import through ERPNext's native Data Import machinery
    Import {
        #[command(subcommand)]
        command: ImportCommand,
    },
    /// Run native reports with filters
    Report {
        #[command(subcommand)]
        command: ReportCommand,
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
        /// Privilege identity to persist for the selected context
        #[arg(long = "as")]
        privilege: Option<PrivilegeMode>,
    },
}

#[derive(Subcommand)]
enum MetaCommand {
    /// Show the live schema for a doctype
    Show { doctype: String },
}

#[derive(Subcommand)]
enum ActivityCommand {
    /// Record business input; optionally open a follow-up loop
    Capture {
        /// Faithful summary of the input
        #[arg(long)]
        summary: String,
        /// Verbatim source text worth preserving
        #[arg(long)]
        verbatim: Option<String>,
        /// Communication medium (phone, email, chat...); recorded as a Communication
        #[arg(long)]
        medium: Option<String>,
        /// Linked business document: DOCTYPE NAME
        #[arg(long, num_args = 2, value_names = ["DOCTYPE", "NAME"])]
        reference: Option<Vec<String>>,
        /// Unresolved commitment to carry as an open loop
        #[arg(long = "follow-up")]
        follow_up: Option<String>,
        /// Due date for the follow-up (YYYY-MM-DD)
        #[arg(long)]
        due: Option<String>,
    },
    /// Time-ordered activity for a reference and window
    List {
        /// Linked business document: DOCTYPE NAME
        #[arg(long, num_args = 2, value_names = ["DOCTYPE", "NAME"])]
        reference: Option<Vec<String>>,
        #[arg(long)]
        since: Option<String>,
        #[arg(long)]
        until: Option<String>,
    },
    /// Open follow-up loops, optionally due-bounded
    OpenLoops {
        /// Linked business document: DOCTYPE NAME
        #[arg(long, num_args = 2, value_names = ["DOCTYPE", "NAME"])]
        reference: Option<Vec<String>>,
        #[arg(long)]
        due: Option<String>,
    },
    /// Close an open follow-up loop once the commitment is honoured
    CloseLoop {
        /// The follow-up record's name
        name: String,
        /// What honoured it, recorded on the business document's trail
        #[arg(long)]
        note: Option<String>,
    },
}

fn reference_pair(reference: &Option<Vec<String>>) -> Option<(&str, &str)> {
    reference
        .as_ref()
        .and_then(|r| Some((r.first()?.as_str(), r.get(1)?.as_str())))
}

#[derive(Subcommand)]
enum EvidenceCommand {
    /// Attach a file to the referenced document (permission-checked by the site)
    Attach {
        /// Linked business document: DOCTYPE NAME
        #[arg(long, num_args = 2, value_names = ["DOCTYPE", "NAME"], required = true)]
        reference: Vec<String>,
        /// Path of the file to attach
        #[arg(long)]
        file: String,
        /// Timeline note recorded with the attachment
        #[arg(long)]
        summary: Option<String>,
    },
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
    /// Convert a document along an ERPNext chain via its native mapper
    Map {
        doctype: String,
        name: String,
        /// Native mapper method, e.g. erpnext.selling.doctype.quotation.quotation.make_sales_order
        #[arg(long)]
        via: String,
        /// JSON object of native inputs passed to the mapper before it builds the target
        #[arg(long)]
        args: Option<String>,
        /// JSON object of fields the mapper does not fill
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
        /// Source token for exact idempotency: the document name derives from it, so
        /// a retry finds the existing document instead of creating a second one
        #[arg(long)]
        source: Option<String>,
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
    /// Amend a cancelled document into a new draft (preview by default)
    Amend {
        doctype: String,
        name: String,
        /// JSON object of field changes for the amended draft
        #[arg(long, default_value = "{}")]
        data: String,
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
enum ImportCommand {
    /// Parse a CSV into an identified batch with the native template preview
    Preview {
        /// CSV file to import
        #[arg(long)]
        file: String,
        /// Target doctype for the records
        #[arg(long)]
        doctype: String,
    },
    /// Start exactly the previewed batch
    Apply {
        /// Batch identifier returned by import preview
        #[arg(long)]
        batch: String,
    },
}

#[derive(Subcommand)]
enum ReportCommand {
    /// Run a report and return machine-readable columns and rows
    Run {
        report_name: String,
        /// Report filter as field=value (repeatable)
        #[arg(long = "filter", value_parser = parse_filter)]
        filters: Vec<(String, String)>,
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
            } => match config::select(
                &paths,
                site.as_deref(),
                company.as_deref(),
                privilege
                    .or(cli.privilege)
                    .unwrap_or(PrivilegeMode::Operator),
            ) {
                Ok(ctx) => commands::context::show(&ctx),
                Err(e) => output::fail("config", &e.to_string(), output::EXIT_CONFIG),
            },
        };
        std::process::exit(exit);
    }

    let resolved = if cli.company.is_some() || cli.site.is_some() {
        config::resolve_override(
            &paths,
            cli.site.as_deref(),
            cli.company.as_deref(),
            cli.privilege.unwrap_or(PrivilegeMode::Operator),
        )
    } else if let Some(privilege) = cli.privilege {
        config::resolve_with_privilege(&paths, privilege)
    } else {
        config::resolve(&paths)
    };
    let ctx = match resolved {
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
        Command::Activity { command } => match command {
            ActivityCommand::Capture {
                summary,
                verbatim,
                medium,
                reference,
                follow_up,
                due,
            } => {
                commands::activity::capture(
                    &client,
                    commands::activity::Capture {
                        summary: &summary,
                        verbatim: verbatim.as_deref(),
                        medium: medium.as_deref(),
                        reference: reference_pair(&reference),
                        follow_up: follow_up.as_deref(),
                        due: due.as_deref(),
                    },
                )
                .await
            }
            ActivityCommand::List {
                reference,
                since,
                until,
            } => {
                commands::activity::list(
                    &client,
                    reference_pair(&reference),
                    since.as_deref(),
                    until.as_deref(),
                )
                .await
            }
            ActivityCommand::OpenLoops { reference, due } => {
                commands::activity::open_loops(&client, reference_pair(&reference), due.as_deref())
                    .await
            }
            ActivityCommand::CloseLoop { name, note } => {
                commands::activity::close_loop(&client, &name, note.as_deref()).await
            }
        },
        Command::Evidence { command } => match command {
            EvidenceCommand::Attach {
                reference,
                file,
                summary,
            } => {
                let reference = Some(reference);
                let Some(pair) = reference_pair(&reference) else {
                    std::process::exit(output::fail(
                        "reference-required",
                        "evidence attach requires --reference <doctype> <name>",
                        output::EXIT_CONFIG,
                    ));
                };
                commands::evidence::attach(&client, pair, &file, summary.as_deref()).await
            }
        },
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
            DocCommand::Map {
                doctype,
                name,
                via,
                args,
                data,
                apply,
            } => {
                commands::doc::map(
                    &client,
                    &doctype,
                    &name,
                    &via,
                    args.as_deref(),
                    data.as_deref(),
                    apply,
                )
                .await
            }
            DocCommand::Create {
                doctype,
                data,
                source,
                apply,
            } => commands::doc::create(&client, &doctype, &data, source.as_deref(), apply).await,
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
            } => commands::workflow::submit(&client, &doctype, &name, apply).await,
            WorkflowCommand::Cancel {
                doctype,
                name,
                apply,
            } => commands::workflow::cancel(&client, &doctype, &name, apply).await,
            WorkflowCommand::Amend {
                doctype,
                name,
                data,
                apply,
            } => commands::workflow::amend(&client, &doctype, &name, &data, apply).await,
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
        Command::Import { command } => match command {
            ImportCommand::Preview { file, doctype } => {
                commands::import::preview(&client, &file, &doctype).await
            }
            ImportCommand::Apply { batch } => commands::import::apply(&client, &batch).await,
        },
        Command::Report {
            command:
                ReportCommand::Run {
                    report_name,
                    filters,
                },
        } => commands::report::run(&client, &report_name, &filters).await,
        Command::Ops {
            command: OpsCommand::Doctor,
        } => commands::ops::doctor(&client).await,
    };

    std::process::exit(exit);
}
