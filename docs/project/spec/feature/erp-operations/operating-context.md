---
description: Company and privilege context selection for the erpnext CLI — explicit identity, fail-closed credentials, secret-safe display.
delivered_by:
  - erpnext-cli-initial
---

# Operating Context — Spec

## What it is

An agent selects which company it operates and under which privilege identity before any
business operation. Company is the tenancy noun; administrator and company-operator are
distinct typed identities. The selected context determines the credentials, the permission
scope, and the accounting boundary for every subsequent command. The command surface is
governed by `erp/spec/cli-surface`; the credential layout by `erp/spec/company-layout`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Show the active operating context
Given a configured company context is selected
When the agent runs `erpnext context show`
Then the output names the site, the selected company, the identity, and the privilege mode
And no API key, secret, or other credential material appears in the output.

#### UC-2

Scenario: Select a company-operator context
Given operator credentials exist for company `acme`
When the agent runs `erpnext context use --company acme`
Then subsequent commands run as the company-scoped operator identity for `acme`
And the selection is visible in `erpnext context show`.

#### UC-3

Scenario: Administrator context is explicit
Given administrator credentials are configured
When the agent runs `erpnext context use --company acme --as admin`
Then subsequent commands run under the administrator identity
And the privilege mode is reported as administrator, never implied silently.

#### UC-4

Scenario: Operator failure never falls back to administrator
Given a company-operator context is active and its credentials are rejected by the instance
When the agent runs any command
Then the command fails with a structured authentication failure
And no retry or fallback under administrator authority occurs.

#### UC-5

Scenario: Missing or ambiguous configuration fails closed
Given no credentials exist for the requested company, or the configuration is ambiguous
When the agent runs `erpnext context use --company <company>`
Then the command fails with a structured blocker naming the missing or ambiguous input
And no default identity is guessed.

## Known caveats

- Credential material never appears in command arguments, stdout, logs, errors, or captured
  activity.
