---
description: Batch import of external business data through ERPNext's native import semantics, with preview and explicit apply.
---

# Bulk Import — Spec

## What it is

External business data enters ERPNext as previewed batches, never as blind writes. An import
previews a CSV file's records for a target doctype into an identified native batch;
applying that batch runs it through ERPNext's native data-import machinery and reports
honest per-row results — ERPNext owns the import execution, and the vendor gates it to
privileged roles. The command surface is governed by `erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Import preview produces an identified batch without writing
Given a CSV file of records for a target doctype
When the agent runs `erpnext import preview --file <csv> --doctype <target>`
Then the output is a batch with a stable identifier and the per-record intended mutations
And no business document is created or changed on the site.

#### UC-2

Scenario: Apply executes the previewed batch through native import
Given a previewed batch identifier
When the agent runs `erpnext import apply --batch <batch-id>`
Then the batch runs through ERPNext's native import machinery
And the outcome per record — created, updated, or failed with reason — is reported in
machine-readable form.
