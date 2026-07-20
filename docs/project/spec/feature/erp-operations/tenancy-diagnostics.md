---
description: Operational diagnosis of site identity, privilege mode, and company isolation, including negative cross-company evidence.
delivered_by:
  - erpnext-cli-initial
---

# Tenancy Diagnostics — Spec

## What it is

Before an agent trusts a context for business work, it can prove the context is what it
claims: which site, which company, which identity, which privilege mode, which permissions —
and that company isolation actually holds. Diagnostics are read-only and produce structured
evidence, including negative evidence that a company operator cannot reach another company's
records. The command surface is governed by `erp/spec/cli-surface`; the tenancy model by
`erp/concept/multi-tenancy`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Doctor reports the operating ground truth
Given an active company context
When the agent runs `erpnext ops doctor`
Then the structured output names the site identity, selected company, privilege mode,
effective permissions, and company-scoped isolation prerequisites such as company-wise
master filtering assumptions
And each check reports pass, fail, or indeterminate with a reason.

#### UC-2

Scenario: Isolation is proven with negative evidence
Given a second isolation-canary company exists on the instance
When the agent runs `erpnext ops doctor` under the first company's operator identity
Then the evidence includes a negative check proving the operator cannot read the canary
company's records
And a positive-only check of the operator's own company is reported as insufficient
isolation evidence.

#### UC-3

Scenario: Diagnostics never mutate
Given any context
When the agent runs `erpnext ops doctor` or `erpnext ops open-loops`
Then no document, setting, or activity record on the instance is created or changed.

#### UC-4

Scenario: Open loops are visible per company
Given open follow-up records exist for the selected company
When the agent runs `erpnext ops open-loops --due <when>`
Then the due and overdue open loops for that company are listed in machine-readable form.
