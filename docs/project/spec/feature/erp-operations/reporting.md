---
description: Running native ERPNext reports with filters and machine-readable results.
---

# Reporting — Spec

## What it is

Agents answer business questions from native ERPNext reports rather than reassembling data
from raw documents. A report runs by name with filters and returns machine-readable rows
scoped by the active identity's permissions. The command surface is governed by
`erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Run a report with filters
Given an active company context and a native report available to the identity
When the agent runs `erpnext report run <report-name> --filter <field=value>`
Then the report's rows are returned in machine-readable form
And the results respect the identity's permission scope.

#### UC-2

Scenario: Unknown report or missing permission is a structured failure
Given a report name that does not exist or is not permitted for the active identity
When the agent runs it
Then the command fails with a structured failure naming the cause
And no partial or guessed result is returned.
