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

- Every command accepts `--company <company-slug>` or `--site <fqdn>` to operate that
  context for one invocation without changing the persisted selection.
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
- `erpnext evidence attach --reference <doctype> <name> --file <path> [--summary <text>]`
- `erpnext report run <report-name> [--filter <field=value>]...`
- `erpnext import preview --file <csv> --doctype <target>`
- `erpnext import apply --batch <batch-id>`
- `erpnext ops doctor`

Preview output for mutating commands contains:

- `summary` — the action restated in business language for the person who asked for it:
  the record by the name they know it by, the money where the payload carries it, whose
  books it lands in, the consequence that outlives the action, and the words that approve
  it. Submitting names that the figures reach the books and the document can afterwards
  only be cancelled; deleting distinguishes itself from cancelling; converting names that
  a second conversion creates a second document.
- `context` — site, company, identity, and privilege mode.
- `operation` — the exact native request that would be sent: method, endpoint, and payload.
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
- Import commands use ERPNext's native data-import semantics with preview and explicit
  apply, reporting honest per-row results. The vendor gates Data Import to privileged
  roles; an operator context receives the site's typed permission failure.
- Lifecycle transitions drive `docstatus` through the document envelope and read the
  document back; a site that answers the request while leaving `docstatus` unchanged is
  reported as a failed transition, never as success. `workflow amend` builds the new draft
  carrying `amended_from` and applies only to a cancelled document.
- `doc map` passes the source under every argument name the native mappers use, because the
  selling chain names it `source_name` and the accounts chain names it `dt` and `dn`; the
  site binds what its signature declares. Fields the mapper leaves empty — a delivery date,
  a payment reference — are supplied with `--data`, and the mapper is never second-guessed.
- `doc map` converts a document along an ERPNext chain through the native mapper the caller
  names, so the site builds the target document; the created document is read back and a
  result that references no source fails. Mapping the same source twice creates a second
  document — resolve before mapping.
- `--source` suits documents ERPNext names from a series; a master with a natural key, such
  as an item code, carries its own identity and needs none.
- `doc create --source <token>` is exactly idempotent: the document name derives
  deterministically from site, doctype, and token; a retry replays the existing document,
  divergent token reuse is a structured conflict, and such documents opt out of native
  series naming.
- `ops doctor` verifies the CLI's own operating ground: configured endpoint, authenticated
  identity, selected company, privilege mode, credential separation, and fail-closed
  configuration. It does not test ERPNext's permission implementation.
- This is the global agent-facing contract. Transport calls and internal command
  architecture are project-owned implementation details, not global ERP doctrine.
