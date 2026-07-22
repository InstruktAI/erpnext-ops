---
description: Read-only diagnosis of the CLI's operating ground — endpoint, identity, company, privilege mode, credential separation, fail-closed configuration.
---

# Operational Diagnostics — Spec

## What it is

Before an agent trusts a context for business work, it can prove the context is what it
claims: which site the CLI is talking to, which identity is authenticated, which company is
selected, which privilege mode is active, and that credential separation and fail-closed
configuration hold. Diagnostics verify the CLI's own functionality against the live site —
they do not test ERPNext's permission implementation, and tenant isolation is structural
(site-per-tenant, per `erp/concept/multi-tenancy`). Diagnostics are read-only. The command
surface is governed by `erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

<!-- planned-change:erpnext-agent-admin-context -->
#### UC-1

Scenario: Doctor reports the operating ground truth
Given an active site context
When the agent runs `erpnext ops doctor`
Then the structured output names the configured endpoint, the site it actually answers as,
the authenticated identity, the selected company's presence on the site, the privilege
mode, and whether operator and administrator credentials are kept separate
And each check reports pass, fail, or indeterminate with a reason.

#### UC-2

Scenario: Doctor detects a broken or misconfigured context
Given a context whose endpoint is unreachable, whose credentials are rejected, or whose
selected company does not exist on the site
When the agent runs `erpnext ops doctor`
Then the affected checks fail with structured reasons naming the gap
And the command exits non-zero without guessing a working configuration.

#### UC-3

Scenario: Diagnostics never mutate
Given any context
When the agent runs `erpnext ops doctor`
Then no document, setting, or activity record on the site is created or changed.
<!-- change:erpnext-agent-admin-context -->
#### UC-1

Scenario: Doctor reports the operating ground truth
Given an active site context
When the agent runs `erpnext ops doctor`
Then the structured output names the configured endpoint, the site it actually answers as,
the authenticated identity, the selected company's presence on the site, the privilege
mode, and whether operator and administrator credentials are kept separate
And each check reports pass, fail, or indeterminate with a reason.

#### UC-2

Scenario: Doctor detects a broken or misconfigured context
Given a context whose endpoint is unreachable, whose credentials are rejected, or whose
selected company does not exist on the site
When the agent runs `erpnext ops doctor`
Then the affected checks fail with structured reasons naming the gap
And the command exits non-zero without guessing a working configuration.

#### UC-3

Scenario: Diagnostics never mutate
Given any context
When the agent runs `erpnext ops doctor`
Then no document, setting, or activity record on the site is created or changed.
<!-- /planned-change:erpnext-agent-admin-context -->

#### UC-4

Scenario: Open loops are visible
Given open follow-up records exist on the site
When the agent runs `erpnext activity open-loops --due <when>`
Then the due and overdue open loops are listed in machine-readable form.
