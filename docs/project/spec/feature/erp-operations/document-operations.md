---
description: Metadata-checked ERPNext document operations with preview-by-default mutation and explicit apply.
---

# Document Operations — Spec

## What it is

Agents read and mutate ERPNext documents of any doctype through a stable command surface:
live schema introspection, listing, retrieval, and create/update/delete/copy with preview by
default and explicit apply. A preview shows the exact native operation that would be sent
without sending it; payloads are checked against live DocType metadata before any write.
ERPNext remains the authority on validation, permissions, and transactions. The command
shapes and preview output contract are governed by `erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Live metadata introspection
Given an active site context
When the agent runs `erpnext meta show <doctype>`
Then the output reflects the live site schema for that doctype, including fields, mandatory
flags, link targets, and child tables
And the schema comes from the running site, not from a bundled model.

#### UC-2

Scenario: List documents through the v2 envelope
Given documents of a doctype exist on the site
When the agent runs `erpnext doc list <doctype> --filter <field=value>`
Then the output is a machine-readable list of matching documents scoped by the active
identity's permissions.

#### UC-3

Scenario: Retrieve a single document
Given a document exists and is readable by the active identity
When the agent runs `erpnext doc get <doctype> <name>`
Then the full document is returned in machine-readable form.

#### UC-4

Scenario: Create previews by default and sends nothing
Given a valid payload for a doctype
When the agent runs `erpnext doc create <doctype> --data <json>` without `--apply`
Then no mutating request is sent to the site
And the output is a preview containing the context, the exact native request that would be
sent, preflight findings, and the exact apply command.

#### UC-5

Scenario: Apply sends the previewed operation and reports ERPNext's result
Given a clean preview for a create
When the agent runs the same command with `--apply`
Then the payload is checked against live DocType metadata before sending
And the native operation is sent and ERPNext's result — success or failure — is returned in
machine-readable form.

#### UC-6

Scenario: Preflight rejects an invalid payload without sending
Given a payload containing an unknown field, a missing mandatory field, or an invalid link
target
When the agent runs a create or update with `--apply`
Then the command fails with a structured validation failure naming the offending input
And no mutating request is sent to the site.

#### UC-7

Scenario: Update, delete, and copy follow the same preview contract
Given an existing document
When the agent runs `erpnext doc update`, `erpnext doc delete`, or `erpnext doc copy`
without `--apply`
Then each returns a preview and sends no mutating request
And each sends the native operation only with explicit `--apply`.

#### UC-8

Scenario: A source token makes creation exactly idempotent
Given a create carrying `--source <token>`
When the agent applies it, loses the response, and applies the same command again
Then the retry finds the document whose name derives from the token and reports it as
replayed, sending no second create
And reusing the token with different data fails as a structured idempotency conflict.
