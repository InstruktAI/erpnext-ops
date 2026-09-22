---
description: Agent-facing erpnext CLI command surface for operating per-tenant ERPNext sites.
---

# ERPNext CLI Surface — Spec

## What it is

The `erpnext` CLI is the agent-facing operating surface for per-tenant ERPNext sites. It
exposes generic ERPNext operations with preview-by-default mutation, site- and
company-scoped context, activity capture over native records, reporting, imports, and
operational diagnostics.

The CLI is a deliberately thin, typed wrapper over Frappe's REST surface. It owns context
selection, authentication, secret redaction, stable command and output shapes, the explicit
apply gate, an unknown-field check against live DocType metadata, and translation between
commands and native requests. ERPNext owns permissions, transactions, document validation,
workflow and docstatus behavior, reports, data import, and activity records. The CLI does
not simulate ERPNext and promises no guarantees the substrate does not provide. This is the
global agent-facing contract; transport calls and internal command architecture are
project-owned implementation details.

## Canonical fields

### Commands

- Every command accepts `--company <company-slug>` or `--site <fqdn>` and
  `--as operator|admin` to operate that context and privilege for one invocation without
  changing the persisted selection. Invocation overrides default to operator authority;
  admin authority is always explicit.
- `erpnext context show`
- `erpnext context use [--site <fqdn>] [--company <company-slug>] [--as operator|admin]` —
  at least one selector; the selection persists and credentials resolve from the company
  credential home at command time, fail-closed on unknown or ambiguous input.
- `erpnext meta show <doctype>`
- `erpnext resolve party <query>`
- `erpnext resolve item <query>`
- `erpnext resolve document <doctype> <query>`
- `erpnext doc list <doctype> [--filter <field=value>]... [--limit <n>]`
- `erpnext doc get <doctype> <name>`
- `erpnext doc create <doctype> --data <json> [--source <token>] [--apply]`
- `erpnext doc update <doctype> <name> --data <json> [--apply]`
- `erpnext doc delete <doctype> <name> [--apply]`
- `erpnext doc copy <doctype> <name> [--data <json>] [--apply]`
- `erpnext doc map <doctype> <name> --via <native-mapper> [--args <json>] [--data <json>] [--apply]`
<!-- planned:erpnext-module-method-call -->
- `erpnext method call <dotted-method> [--data <json>] [--apply]`
<!-- /planned:erpnext-module-method-call -->
- `erpnext workflow submit <doctype> <name> [--apply]`
- `erpnext workflow cancel <doctype> <name> [--apply]`
- `erpnext workflow amend <doctype> <name> --data <json> [--apply]`
- `erpnext workflow method <doctype> <name> <method> [--data <json>] [--apply]`
- `erpnext activity capture --summary <text> [--verbatim <text>] [--medium <value>] --reference <doctype> <name> [--follow-up <text>] [--due <date>]`
- `erpnext activity list --reference <doctype> <name> [--since <when>] [--until <when>]`
- `erpnext activity open-loops [--reference <doctype> <name>] [--due <when>]`
- `erpnext activity close-loop <name> [--note <text>]`
- `erpnext evidence attach --reference <doctype> <name> --file <path> [--summary <text>]`
- `erpnext report run <report-name> [--filter <field=value>]...`
- `erpnext import preview --file <csv> --doctype <target>`
- `erpnext import apply --batch <batch-id>`
- `erpnext ops doctor`

<!-- planned:erpnext-command-surface-export -->
The command inventory above is rendered from the binary's declared command surface, which
the binary emits as a deterministic, credential-free, site-independent description. The
root help carries one pointer to the ERP documentation and no per-command documentation
references.
<!-- /planned:erpnext-command-surface-export -->

### Preview

A mutating document, workflow, or mapping command previews by default and persists nothing
without `--apply`. Document and workflow previews read the native facts they report without
sending the mutating operation. Mapping also invokes the native mapper to obtain the mapped
payload. Preview output contains:

- `context` — site, company, and privilege mode of the invocation.
- `context` also names whether the identity is configured or authenticated, and a command
  targeting an existing document reports that document's own company separately from the
  selected context.
- `operation` — the exact native request that starts apply: method, endpoint, and payload.
<!-- planned-change:erpnext-module-method-call -->
- `persistence` — for a document controller method, the returned document is validated as
  the requested record and saved as the second native step; the preview cannot show that
  payload before the site runs the method.
<!-- change:erpnext-module-method-call -->
- `persistence` — for a document controller method, apply invokes the controller once and
  reports the requested document as stored afterwards. When the controller persisted its effect,
  apply sends no further write; when the effect exists only in the returned document, apply
  persists that document once, never over a newer stored version. The preview cannot show the
  returned document before the site runs the method.
<!-- /planned-change:erpnext-module-method-call -->
- `target` — for a command targeting an existing document: the document's current
  `docstatus`, its own company, and its modification marker, read from the site.
- `schema` — for a document operation: the DocType's `is_submittable`, its fields marked
  `allow_on_submit`, and `track_changes`, read from the site. An opaque module method
  carries no document or schema facts.
- `preflight` — the checks the preview actually ran and their findings: unknown payload
  fields against live metadata, and an explicit statement of what remains the site's
  validation.
- `apply_command` — the exact replayable invocation, carrying arguments, the source token,
  and context overrides. A replayed invocation is not a replayed result: state may have
  changed and the facts are read again on apply.

Preview exposes what the CLI observed and will send. Consequence and reversibility are the
agent's judgment from those facts and from the procedure it is following.

Writes that do not preview, each with its own effect:

- `activity capture` records the business memory directly; the record is the trace.
- `evidence attach` persists the file on the referenced document and may add timeline
  content.
- `import preview` stages native Data Import records and the uploaded file without
  importing target business records; `import apply` runs the staged batch.

Requests a preview does send:

- `doc map` preview invokes the named mapper method on the site to obtain the mapped
  payload it shows. The site persists nothing for a non-persisting mapper; only `--apply`
  creates the target document.

### Envelope

Every command answers with the same envelope: `ok` reporting whether the command
succeeded, and `data` carrying the result. A site response that carries the vendor's `data`
wrapper is unwrapped, so document results sit at one depth; a site response with no `data`
wrapper is carried as it stands. Failures carry `kind` and `message` on stderr with a
non-zero exit.

## Known caveats

- Routine work runs as the site operator; an operator failure never falls back to
  administrator authority. Administrative work uses an explicitly selected, dedicated
  company-scoped agent-admin identity. The bootstrap `Administrator` identity and its
  deployment secret remain unavailable to the CLI.
- Create and update commands refuse a payload field the doctype does not define. They do
  not check mandatory values or link targets; those remain the site's validation, and a
  create that omits values ERPNext derives is sent as written.
- `doc create --source <token>` is exactly idempotent: the document name derives
  deterministically from site, doctype, and token; a retry replays the existing document,
  divergent token reuse is a structured conflict, and such documents opt out of native
  series naming.
- `doc update` sends the payload as written; the site decides whether a submitted
  document accepts it.
- Lifecycle transitions drive `docstatus` through the document envelope and read the
  document back; a site that answers the request while leaving `docstatus` unchanged is
  reported as a failed transition, never as success. `workflow amend` builds the new draft
  carrying `amended_from` and applies only to a cancelled document.
<!-- planned-change:erpnext-module-method-call -->
- `workflow method` invokes the site's whitelisted document controller. When the controller
  returns a changed document, apply refuses a different doctype or name, saves that exact
  returned document, and reports the stored result. A controller that ran but could not save
  is a partial failure carrying the method result, never success.
<!-- change:erpnext-module-method-call -->
- `workflow method` invokes the site's whitelisted document controller once and then reads the
  document back. When the controller persisted its effect, apply reports that stored result
  without a further write. When the effect exists only in the returned document, apply refuses a
  different doctype or name, persists that returned document once through the native save, which
  rejects a newer stored version, and reports the stored result. Which case applies is read from
  the document's own stored state, never from the doctype or method name. A different returned
  document, a persistence need that cannot be established, a rejected persistence, or an
  unreadable stored document is a typed failure, never success.
<!-- /planned-change:erpnext-module-method-call -->
<!-- planned:erpnext-module-method-call -->
- `method call` invokes a dotted native module method through Frappe's standard method endpoint.
  Its optional `--data` is an opaque JSON object of method arguments, never document fields.
  The CLI previews or sends the request under the selected context. Its preview carries
  `context`, `operation` (the dotted method and opaque payload), a `preflight` that states no
  field check applies to opaque method arguments, and `apply_command`; it reads no document
  and no DocType, so it carries no `target`, `schema`, or `persistence`. On apply, a native
  response that is a JSON object is reported with its `message` value under `data`, and a
  method that returns nothing reports `data` as null; a response that is not a JSON object is
  a typed failure, never success. It does not interpret business state, read back records, or
  substitute behavior when the method is unavailable.
<!-- /planned:erpnext-module-method-call -->
- `doc map` passes the source under the native mapper argument names `source_name`, `dt`, and
  `dn`; the site binds only the names its signature declares. `--args` supplies additional
  native mapper inputs before the site builds the target document; `--data` overrides
  target-document fields only after mapping.
- `doc map` reads the created document back and verifies its relationship to the source
  through the DocType's Link and Dynamic Link fields, including child rows, reporting one
  of three outcomes: verified, missing, or not expressible through the inspected metadata.
  A verified relationship reports success. Missing and not-expressible relationships
  report distinct structured failures retaining the created document's reference; the
  latter reports unavailable proof, not evidence that the relationship is absent.
  Preservation of every source-row identity and correctness of consumed quantities remain
  the calling procedure's proof.
- Mapping the same source twice creates a second document; the command carries no replay
  protection.
- Activity commands are thin operations over native Frappe and ERPNext records such as
  `Communication`, `Comment`, `ToDo`, `Event`, `CRM Note`, and linked business documents.
- `activity list` folds a document's native timeline into one ordered trail carrying both
  what happened and what is still owed: contacts, comments, and outstanding follow-ups,
  each naming its actor and, where the record has one, its status. A record the site
  leaves untimestamped survives a `--since` or `--until` bound.
- A follow-up is stored as a `ToDo` carrying its own reference to the business document,
  so one party holds as many open commitments as the business owes it.
- `activity close-loop` honours an open commitment and reads the stored status back: a
  site that accepts the request while leaving the loop open is reported as a failure,
  never as success. A `--note` records what honoured it on the business document's trail.
- Import commands use ERPNext's native data-import semantics with preview and explicit
  apply, reporting honest per-row results. An operator context that the site does not
  permit to import receives the site's typed permission failure.
- `report run` returns the report's own columns and rows as the site produces them. The
  report is ERPNext's; the CLI neither reshapes its rows nor recomputes its figures.
- `ops doctor` verifies the CLI's own operating ground: configured endpoint, authenticated
  identity, selected company, privilege mode, credential separation, and fail-closed
  configuration. It does not test ERPNext's permission implementation.

## See Also

- ~/.teleclaude/docs/erp/concept/multi-tenancy.md — the site as tenant boundary and the company as accounting scope.
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md — when an agent previews, applies, resolves, and surfaces.
- ~/.teleclaude/docs/erp/spec/company-layout.md — identities and the credential home the context resolves from.
