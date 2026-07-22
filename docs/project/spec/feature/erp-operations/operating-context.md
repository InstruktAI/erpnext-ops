---
description: Site, company, and privilege context selection for the erpnext CLI — explicit identity, fail-closed credentials, secret-safe display.
---

# Operating Context — Spec

## What it is

An agent selects which site it operates, which company inside that site, and under which
privilege identity, before any business operation. The site is the tenant boundary;
administrator and site-operator are distinct typed identities. The selected context
determines the credentials, the permission scope, and the accounting scope for every
subsequent command. The command surface is governed by `erp/spec/cli-surface`; the
credential layout by `erp/spec/company-layout`.

## Canonical fields

### Acceptance scenarios

<!-- planned-change:erpnext-agent-admin-context -->
#### UC-1

Scenario: Show the active operating context
Given a configured site context is selected
When the agent runs `erpnext context show`
Then the output names the site, the selected company, the identity, and the privilege mode
And no API key, secret, or other credential material appears in the output.
<!-- change:erpnext-agent-admin-context -->
#### UC-1

Scenario: Show the active operating context
Given a configured site context is selected
When the agent runs `erpnext context show`
Then the output names the site, the selected company, the identity, and the privilege mode
And no API key, secret, or other credential material appears in the output.
<!-- /planned-change:erpnext-agent-admin-context -->

#### UC-2

Scenario: Select a site-operator context
Given operator credentials exist for site `acme.erpnext.example.com`
When the agent runs `erpnext context use --site acme.erpnext.example.com --company acme`
Then subsequent commands run as that site's operator identity with company `acme` as the
accounting scope
And the selection is visible in `erpnext context show`.

<!-- planned-change:erpnext-agent-admin-context -->
#### UC-3

Scenario: Administrator context selection is refused
Given any site context
When the agent runs `erpnext context use --site <fqdn> --as admin`
Then the command fails with a structured failure naming administrator resolution as
unavailable
And it states that administrator bootstrap secrets live only in itsUP, never in the
company credential home.
<!-- change:erpnext-agent-admin-context -->
#### UC-3

Scenario: Select a dedicated agent-admin context
Given dedicated agent-admin credentials exist for site `acme.erpnext.example.com`
When the agent runs `erpnext context use --site acme.erpnext.example.com --as admin`
Then subsequent commands run as that site's dedicated agent-admin identity
And the bootstrap `Administrator` identity is refused
And no operator credential is used as fallback.
<!-- /planned-change:erpnext-agent-admin-context -->

<!-- planned-change:erpnext-agent-admin-context -->
#### UC-4

Scenario: Operator failure never falls back to administrator
Given a site-operator context is active and its credentials are rejected by the site
When the agent runs any command
Then the command fails with a structured authentication failure
And no retry or fallback under administrator authority occurs.
<!-- change:erpnext-agent-admin-context -->
#### UC-4

Scenario: Operator failure never falls back to administrator
Given a site-operator context is active and its credentials are rejected by the site
When the agent runs any command
Then the command fails with a structured authentication failure
And no retry or fallback under administrator authority occurs.
<!-- /planned-change:erpnext-agent-admin-context -->

#### UC-5

Scenario: Missing or ambiguous configuration fails closed
Given no credentials exist for the requested site, or the configuration is ambiguous
When the agent runs `erpnext context use --site <fqdn>`
Then the command fails with a structured blocker naming the missing or ambiguous input
And no default identity is guessed.

## Known caveats

- Credential material never appears in command arguments, stdout, logs, errors, or captured
  activity.
