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

Scenario: Rejected or malformed report result is a structured failure
Given a report that does not exist, is not permitted, or returns no valid column and row arrays
When the agent runs it
Then the command fails with a structured failure naming the cause
And no partial or guessed result is returned.

#### UC-3

Scenario: A substantive native report gets a bounded extended execution window
Given a permitted report whose native calculation needs longer than the ordinary request bound
When the agent runs the report
Then the report receives a report-specific bounded execution window and can complete
And a stalled report still fails with a structured transport error rather than hanging forever.
