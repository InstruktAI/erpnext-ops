---
description: Agent-facing erpnext CLI command surface for operating per-tenant ERPNext sites.
---

# ERPNext CLI Surface — Spec

## What it is

The `erpnext` CLI is the agent-facing operating surface for per-tenant ERPNext sites. It
exposes generic, metadata-checked ERPNext operations with preview-by-default mutation,
site- and company-scoped context, activity capture over native records, reporting, imports,
and operational diagnostics.

The CLI is a deliberately thin, typed wrapper over Frappe's v2 REST abstraction. It owns
context selection, authentication, secret redaction, stable command and output shapes, the
explicit apply gate, basic preflight against live DocType metadata, and translation between
commands and native requests. ERPNext owns permissions, transactions, document validation,
workflow and docstatus behavior, reports, data import, and activity records. The CLI does
not simulate ERPNext and promises no guarantees the substrate does not provide. Agents use
the CLI contract below, not raw transport mechanics.

## Canonical fields

<!-- planned-change:erpnext-agent-admin-context -->
- Every command accepts `--company <company-slug>` or `--site <fqdn>` to operate that
  context for one invocation without changing the persisted selection.
<!-- change:erpnext-agent-admin-context -->
- Every command accepts `--company <company-slug>` or `--site <fqdn>` and
  `--as operator|admin` to operate that context and privilege for one invocation without
  changing the persisted selection. Invocation overrides default to operator authority;
  admin authority is always explicit.
<!-- /planned-change:erpnext-agent-admin-context -->
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
- `erpnext doc map <doctype> <name> --via <native-mapper> [--apply]`
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

Preview output for mutating commands contains:

- `summary` — the action restated in business language for the person who asked for it:
  the record by the name they know it by, the money where the payload carries it, whose
  books it lands in, and the consequence that outlives the action. Submitting names that
  the figures reach the books and the document can afterwards
  only be cancelled; deleting distinguishes itself from cancelling; converting names that
  a second conversion creates a second document.
- `context` — site, company, identity, and privilege mode.
- `operation` — the exact native request that starts apply: method, endpoint, and payload.
- `persistence` — for a document controller method, the returned document is validated as
  the requested record and saved as the second native step; the preview cannot show that
  payload before the site runs the method.
- `preflight` — findings from basic checks against live DocType metadata: unknown fields,
  missing mandatory fields, and invalid link targets, plus any lifecycle or permission
  conflict ERPNext reports without mutating.
- `apply_command` — the exact command form that applies the preview.

Every command answers with the same envelope: `ok` reporting whether the command
succeeded, and `data` carrying the result. Commands that hand back a site response carry
its payload under `data` rather than the vendor's own wrapper, so a result sits at one
depth whichever command produced it. Failures carry `kind` and `message` on stderr with a
non-zero exit.

## Known caveats

- The site is the tenant boundary; company selection scopes accounting inside the site.
<!-- planned-change:erpnext-agent-admin-context -->
- Routine work runs as the site operator; an operator failure never falls back to
  administrator authority. Administrator context selection is refused in this release —
  administrator bootstrap secrets live only in itsUP.
<!-- change:erpnext-agent-admin-context -->
- Routine work runs as the site operator; an operator failure never falls back to
  administrator authority. Administrative work uses an explicitly selected, dedicated
  company-scoped agent-admin identity. The bootstrap `Administrator` identity and its
  deployment secret remain unavailable to the CLI.
<!-- /planned-change:erpnext-agent-admin-context -->
- Mutating commands preview by default; `--apply` sends the native operation and reports
  ERPNext's result.
- Create and update commands check payload fields against live DocType metadata before
  sending and refuse a field the doctype does not define. Mandatory values ERPNext derives
  during validation are left to it: ERPNext remains the authority on validation,
  permissions, and transactions.
- Externally visible or irreversible actions proceed autonomously when they are determined by
  evidence, configuration, templates, and standard ERP practice. They surface only when a real
  decision boundary remains.
- Activity commands are thin operations over native Frappe and ERPNext records such as
  `Communication`, `Comment`, `ToDo`, `Event`, `CRM Note`, and linked business documents.
- `activity list` folds a document's native timeline into one ordered trail carrying both
  what happened and what is still owed: contacts, comments, and outstanding follow-ups,
  each naming its actor and, where the record has one, its status. The site groups these
  by its own vocabulary — a follow-up arrives among the assignments and a contact names
  its actor as the sender. A record the site leaves untimestamped survives a `--since` or
  `--until` bound, because a bounded read that drops outstanding work defeats the reason
  for reading it.
- A follow-up is stored as a `ToDo` carrying its own reference to the business document,
  so one party holds as many open commitments as the business owes it. The native
  assignment API binds one open assignment per user per document and answers a refused
  second one with success; a commitment layer built on it would keep the first and lose
  every later one without a trace.
- `activity close-loop` honours an open commitment and reads the stored status back: a
  site that accepts the request while leaving the loop open is reported as a failure,
  never as success. A `--note` records what honoured it on the business document's trail,
  where the next reader of that party's history finds it.
- Import commands use ERPNext's native data-import semantics with preview and explicit
  apply, reporting honest per-row results. The vendor gates Data Import to privileged
  roles; an operator context receives the site's typed permission failure.
- Lifecycle transitions drive `docstatus` through the document envelope and read the
  document back; a site that answers the request while leaving `docstatus` unchanged is
  reported as a failed transition, never as success. `workflow amend` builds the new draft
  carrying `amended_from` and applies only to a cancelled document.
- `workflow method` invokes the site's whitelisted document controller. When the controller
  returns a changed document, apply refuses a different doctype or name, saves that exact
  returned document, and reports the stored result. A controller that ran but could not save
  is a partial failure carrying the method result, never success.
- `doc map` passes the source under every argument name the native mappers use, because the
  selling chain names it `source_name` and the accounts chain names it `dt` and `dn`; the
  site binds what its signature declares. Fields the mapper leaves empty — a delivery date,
  a payment reference — are supplied with `--data`, and the mapper is never second-guessed.
- `doc map` converts a document along an ERPNext chain through the native mapper the caller
  names, so the site builds the target document; the created document is read back and a
  result that references no source fails. Mapping the same source twice creates a second
  document — resolve before mapping.
- Cancelling a submitted document undoes what it did to everything that counted it, as
  long as nothing downstream has consumed the effect. A cancelled partial invoice returns
  the order's per-row billed quantities and its billed percentage to nothing, returns the
  project's billed total to nothing, and clears the customer's balance — leaving the work
  billable again with no residue. Undo is therefore a real operating move rather than a
  repair of last resort, and the state to check afterwards is the source document's own
  per-row quantities.
- Billing part of an order adjusts the quantities on the rows the mapper produced; it never
  replaces the item list. Each mapped row carries the identifier of the order row it bills,
  and that per-row link is what the site uses to track how much of the order remains. A
  replacement list drops it, and the resulting invoice still posts, still charges the
  customer, and still counts toward the project — while the order reads as entirely
  unbilled and offers the same work for billing again. The order's per-row billed and
  delivered quantities are therefore the state a resumed or repeated run reads before
  acting.
- `--source` suits documents ERPNext names from a series; a master with a natural key, such
  as an item code, carries its own identity and needs none.
- `doc create --source <token>` is exactly idempotent: the document name derives
  deterministically from site, doctype, and token; a retry replays the existing document,
  divergent token reuse is a structured conflict, and such documents opt out of native
  series naming.
- `report run` returns the report's own columns and rows as the site produces them. A
  native report names its data rows by field and may close with a totals row positioned by
  column order rather than named, so a reader keys on the column list rather than assuming
  every row carries the same shape. The report is ERPNext's; the CLI neither reshapes its
  rows nor recomputes its figures.
- `ops doctor` verifies the CLI's own operating ground: configured endpoint, authenticated
  identity, selected company, privilege mode, credential separation, and fail-closed
  configuration. It does not test ERPNext's permission implementation.
- This is the global agent-facing contract. Transport calls and internal command
  architecture are project-owned implementation details, not global ERP doctrine.
