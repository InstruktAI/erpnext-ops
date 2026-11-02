---
description: Site, company, and privilege context selection for the erpnext CLI — explicit identity, fail-closed credentials, secret-safe display.
---

# Operating Context — Spec

## What it is

<!-- planned-change:erpnext-session-context -->
An agent selects which site it operates, which company inside that site, and under which
privilege identity, before any business operation. The site is the tenant boundary;
administrator and site-operator are distinct typed identities. The selected context
determines the credentials, the permission scope, and the accounting scope for every
subsequent command. The command surface is governed by `erp/spec/cli-surface`; the
credential layout by `erp/spec/company-layout`.
<!-- change:erpnext-session-context -->
Every business command names the company it operates, by its company slug
(`--company`) or its site (`--site`), and the CLI resolves that company's site,
accounting company and privilege identity for that one invocation. The site is the
tenant boundary; administrator and site-operator are distinct typed identities. The
resolved context determines the credentials, the permission scope and the accounting
scope of that command only. No selection is stored between commands, so concurrent
invocations never depend on one another's choice, no command inherits a company it did
not name, and the working directory plays no part. The command surface is governed by
`erp/spec/cli-surface`; the credential layout by `erp/spec/company-layout`.
<!-- /planned-change:erpnext-session-context -->

## Canonical fields

### Acceptance scenarios

<!-- planned-change:erpnext-session-context -->
#### UC-1

Scenario: Show the active operating context
Given a configured site context is selected
When the agent runs `erpnext context show`
Then the output names the site, the selected company, the identity, and the privilege mode
And no API key, secret, or other credential material appears in the output.

#### UC-2

Scenario: Select a site-operator context
Given operator credentials exist for site `acme.erpnext.example.com`
When the agent runs `erpnext context use --site acme.erpnext.example.com --company acme`
Then subsequent commands run as that site's operator identity with company `acme` as the
accounting scope
And the selection is visible in `erpnext context show`.

#### UC-3

Scenario: Select a dedicated agent-admin context
Given dedicated agent-admin credentials exist for site `acme.erpnext.example.com`
When the agent runs `erpnext context use --site acme.erpnext.example.com --as admin`
Then subsequent commands run as that site's dedicated agent-admin identity
And the bootstrap `Administrator` identity is refused
And no operator credential is used as fallback.

<!-- change:erpnext-session-context -->
#### UC-1

```gherkin
Scenario: Show the operating context a command resolves
Given operator credentials are configured for company `acme`
When the agent runs `erpnext context show --company acme`
Then the output names the site, the company, the identity, and the privilege mode
And no API key, secret, or other credential material appears in the output
```

#### UC-2

```gherkin
Scenario: A command runs in the company it names
Given operator credentials are configured for company `acme` on site `acme.erpnext.example.com`
When the agent runs a business command naming `--company acme` or `--site acme.erpnext.example.com`
Then that command runs as the site's operator identity with company `acme` as the accounting scope
And the next command resolves only the company that command names
And the result is the same from any working directory
```

#### UC-3

```gherkin
Scenario: Select the dedicated agent-admin identity for one command
Given dedicated agent-admin credentials are configured for company `acme`
When the agent runs a command naming `--company acme --as admin`
Then that command runs as the site's dedicated agent-admin identity
And the bootstrap `Administrator` identity is refused
And no operator credential is used as fallback
And a later command naming `--company acme` without `--as` runs as the operator identity
```

<!-- /planned-change:erpnext-session-context -->
#### UC-4

Scenario: Operator failure never falls back to administrator
Given a site-operator context is active and its credentials are rejected by the site
When the agent runs any command
Then the command fails with a structured authentication failure
And no retry or fallback under administrator authority occurs.

<!-- planned-change:erpnext-session-context -->
#### UC-5

Scenario: Missing or ambiguous configuration fails closed
Given no credentials exist for the requested site, or the configuration is ambiguous
When the agent runs `erpnext context use --site <fqdn>`
Then the command fails with a structured blocker naming the missing or ambiguous input
And no default identity is guessed.

<!-- change:erpnext-session-context -->
#### UC-5

```gherkin
Scenario: Missing or ambiguous configuration fails closed
Given no credentials exist for the named company or site, or the site is served by more than one configured company
When the agent runs a command naming that company or site
Then the command fails with a structured blocker naming the missing or ambiguous input
And no request reaches any site
And no default identity is guessed
```

#### UC-6

```gherkin
Scenario: A command that names no company fails before any request
Given companies are configured on the host
When the agent runs a business command naming neither a company nor a site
Then the command fails with a structured context failure naming `--company`, `--site` and the configured companies
And no request reaches any site
And no previously chosen company is used in its place
```

#### UC-7

```gherkin
Scenario: Execution attribution is recorded and never blocks a command
Given an agent runs a business command naming its company
When the command completes
Then its structured result identifies the executing session when the platform reports one
And the command runs whether or not the platform reports one
And no credential or authentication token appears in the result
```

<!-- /planned-change:erpnext-session-context -->
## Known caveats

- Credential material never appears in command arguments, stdout, logs, errors, or captured
  activity.
