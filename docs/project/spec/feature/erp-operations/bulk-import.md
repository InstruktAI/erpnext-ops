---
description: Batch import of external business data with preview batches and explicit, drift-safe apply.
delivered_by:
  - erpnext-cli-initial
---

# Bulk Import — Spec

## What it is

External business data enters ERPNext as previewed batches, never as blind writes. An import
previews the per-record mutations from a configured source into an identified batch; applying
that batch performs exactly the previewed mutations. The command surface is governed by
`erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Import preview produces an identified batch without writing
Given a configured import source with pending records
When the agent runs `erpnext import preview --source <name>`
Then the output is a batch with a stable identifier and the per-record expected mutations
And no write reaches the instance.

#### UC-2

Scenario: Apply executes exactly the previewed batch
Given a previewed batch identifier
When the agent runs `erpnext import apply --source <name> --batch <batch-id>`
Then exactly the previewed mutations are applied
And the outcome per record is reported in machine-readable form.

#### UC-3

Scenario: A drifted batch invalidates instead of partially applying
Given the source data or instance state changed after the batch was previewed
When the agent applies that batch
Then the command reports the batch as invalidated and performs no writes for it
And a fresh preview is required.
