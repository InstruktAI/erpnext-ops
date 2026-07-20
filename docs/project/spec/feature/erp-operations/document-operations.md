---
description: Metadata-validated ERPNext document operations with preview-by-default mutation and explicit apply.
delivered_by:
  - erpnext-cli-initial
---

# Document Operations — Spec

## What it is

Agents read and mutate ERPNext documents of any doctype through a stable command surface:
live schema introspection, listing, retrieval, and create/update/delete/copy with preview by
default and explicit apply. Payloads are dynamic JSON validated against live DocType metadata
before any write. The command shapes and preview output contract are governed by
`erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Live metadata introspection
Given an active company context
When the agent runs `erpnext meta show <doctype>`
Then the output reflects the live instance schema for that doctype, including fields,
mandatory flags, link targets, and child tables
And the schema comes from the running instance, not from a bundled model.

#### UC-2

Scenario: List documents through the v2 envelope
Given documents of a doctype exist within the selected company scope
When the agent runs `erpnext doc list <doctype> --filter <field=value>`
Then the output is a machine-readable list of matching documents scoped by the active
identity's permissions.

#### UC-3

Scenario: Retrieve a single document
Given a document exists and is readable by the active identity
When the agent runs `erpnext doc get <doctype> <name>`
Then the full document is returned in machine-readable form.

#### UC-4

Scenario: Create previews by default and writes nothing
Given a valid payload for a doctype
When the agent runs `erpnext doc create <doctype> --data <payload>` without `--apply`
Then no document is created on the instance
And the output is a preview containing context, preconditions, expected mutations,
side effects, activity, warnings, decision boundaries, and the exact apply command.

#### UC-5

Scenario: Apply performs the previewed mutation after preflight
Given a clean preview for a create
When the agent runs the same command with `--apply`
Then the payload is validated against live DocType metadata before writing
And the document is created and its identity returned in machine-readable form.

#### UC-6

Scenario: Preflight rejects an invalid payload without writing
Given a payload containing an unknown field or violating a mandatory or link constraint
When the agent runs a create or update with `--apply`
Then the command fails with a structured validation failure naming the offending input
And no write reaches the instance.

#### UC-7

Scenario: A stale preview invalidates instead of silently changing
Given a preview was produced and the underlying metadata or preconditions changed afterwards
When the agent applies that operation
Then the command reports the preview as invalidated and performs no write
And a fresh preview is required to proceed.

#### UC-8

Scenario: Update, delete, and copy follow the same preview contract
Given an existing document
When the agent runs `erpnext doc update`, `erpnext doc delete`, or `erpnext doc copy`
without `--apply`
Then each returns a preview and performs no write
And each performs the mutation only with explicit `--apply`.
