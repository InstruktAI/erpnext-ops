---
description: Agent-facing erpnext CLI command surface for operating per-tenant ERPNext sites.
delivered_by:
  - erpnext-cli-initial
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

- `erpnext context show`
- `erpnext context use --site <fqdn> [--company <company>] [--as operator|admin]`
- `erpnext meta show <doctype> [--refresh]`
- `erpnext resolve party <query>`
- `erpnext resolve item <query>`
- `erpnext resolve document <doctype> <query>`
- `erpnext doc list <doctype> [--filter <field=value>]... [--limit <n>]`
- `erpnext doc get <doctype> <name>`
- `erpnext doc create <doctype> --data <json|@file> [--source <text|@file>] [--follow-up <text>] [--due <when>] [--apply]`
- `erpnext doc update <doctype> <name> --data <json|@file> [--source <text|@file>] [--follow-up <text>] [--due <when>] [--apply]`
- `erpnext doc delete <doctype> <name> [--apply]`
- `erpnext doc copy <doctype> <name> [--data <json|@file>] [--apply]`
- `erpnext workflow submit <doctype> <name> [--apply]`
- `erpnext workflow cancel <doctype> <name> [--apply]`
- `erpnext workflow amend <doctype> <name> --data <json|@file> [--apply]`
- `erpnext workflow method <doctype> <name> <method> [--data <json|@file>] [--apply]`
- `erpnext activity capture --summary <text> [--verbatim <text>] [--medium <value>] [--reference <doctype> <name>] [--follow-up <text>] [--due <date>]`
- `erpnext activity list [--reference <doctype> <name>] [--since <when>] [--until <when>] [--summaries]`
- `erpnext activity open-loops [--reference <doctype> <name>] [--due <when>]`
- `erpnext activity recall [--reference <doctype> <name>] [--since <when>] [--until <when>]`
- `erpnext evidence attach --reference <doctype> <name> --file <path> [--summary <text>]`
- `erpnext report run <report-name> [--filter <field=value>]...`
- `erpnext import preview --source <name> [--since <when>]`
- `erpnext import apply --source <name> --batch <batch-id>`
- `erpnext ops doctor [--site <fqdn>]`
- `erpnext ops open-loops [--company <company>] [--due <when>]`

Preview output for mutating commands contains:

- `context` — site, company, identity, and privilege mode.
- `operation` — the exact native request that would be sent: method, endpoint, and payload.
- `preflight` — findings from basic checks against live DocType metadata: unknown fields,
  missing mandatory fields, and invalid link targets, plus any lifecycle or permission
  conflict ERPNext reports without mutating.
- `apply_command` — the exact command form that applies the preview.

## Known caveats

- The site is the tenant boundary; company selection scopes accounting inside the site.
- Administrative commands require the explicit administrator context; routine work runs as
  the site operator. An operator failure never falls back to administrator authority.
- Mutating commands preview by default; `--apply` sends the native operation and reports
  ERPNext's result.
- Create and update commands run basic preflight against live DocType metadata before
  sending; ERPNext remains the authority on validation, permissions, and transactions.
- Externally visible or irreversible actions proceed autonomously when they are determined by
  evidence, configuration, templates, and standard ERP practice. They surface only when a real
  decision boundary remains.
- Activity commands are thin operations over native Frappe and ERPNext records such as
  `Communication`, `Comment`, `ToDo`, `Event`, `CRM Note`, and linked business documents.
- Import commands use ERPNext's native data-import semantics with preview and explicit
  apply, reporting honest per-row results.
- `ops doctor` verifies the CLI's own operating ground: configured endpoint, authenticated
  identity, selected company, privilege mode, credential separation, and fail-closed
  configuration. It does not test ERPNext's permission implementation.
- This is the global agent-facing contract. Transport calls and internal command
  architecture are project-owned implementation details, not global ERP doctrine.
