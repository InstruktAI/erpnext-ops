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

<!-- planned:erpnext-session-context -->
### Selection rules

- A business command names its company with `--company <company-slug>` or its site
  with `--site <fqdn>`. Naming a company resolves that company's credential home,
  also when its site serves other configured companies; naming a site resolves the
  one company configured for it.
- Each of these fails before any request reaches a site, with a structured context
  failure naming the remedy, and no stored or default choice takes its place:
  - the command names neither a company nor a site — the failure names `--company`,
    `--site` and the companies configured on the host;
  - the named company has no credential home, or the named site matches no
    configured company;
  - the named site is served by more than one configured company — the failure
    names those companies and `--company`.
- When the platform reports the session a command runs in, the command's structured
  result identifies that executing session; when it reports none, the result carries
  no executing session and the command runs unchanged. No credential or
  authentication token appears in either result.

<!-- /planned:erpnext-session-context -->
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

#### UC-6

#### UC-7

<!-- /planned-change:erpnext-session-context -->
## Known caveats

- Credential material never appears in command arguments, stdout, logs, errors, or captured
  activity.
