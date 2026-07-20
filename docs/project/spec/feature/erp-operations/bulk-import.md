---
description: Batch import of external business data through ERPNext's native import semantics, with preview and explicit apply.
delivered_by:
  - erpnext-cli-initial
---

# Bulk Import — Spec

## What it is

External business data enters ERPNext as previewed batches, never as blind writes. An import
previews the per-record mutations from a configured source into an identified batch;
applying that batch runs it through ERPNext's native data-import machinery and reports
honest per-row results — ERPNext owns the import execution. The initial configured source
kind is a CSV file mapped to a target doctype and fields; the source contract stays
extensible. The command surface is governed by `erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Import preview produces an identified batch without writing
Given a configured import source with pending records
When the agent runs `erpnext import preview --source <name>`
Then the output is a batch with a stable identifier and the per-record intended mutations
And no business document is created or changed on the site.

#### UC-2

Scenario: Apply executes the previewed batch through native import
Given a previewed batch identifier
When the agent runs `erpnext import apply --source <name> --batch <batch-id>`
Then the batch runs through ERPNext's native import machinery
And the outcome per record — created, updated, or failed with reason — is reported in
machine-readable form.
