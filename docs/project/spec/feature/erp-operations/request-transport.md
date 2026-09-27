---
description: How the erpnext CLI sends native requests — a mutating request is sent once to its exact endpoint and a redirect answering it is a typed failure, while a read follows a redirect to its target.
delivered_by:
  - erpnext-module-method-call
---

# Request Transport — Spec

## Required reads

@~/.teleclaude/docs/erp/spec/cli-surface.md

## What it is

Every `erpnext` command reaches the site through one shared transport. A mutating request
(POST, PUT, DELETE, including file uploads and requests with a command-specific timeout) is
sent once, as written, to the endpoint the command names. A redirect in answer to it is a
typed failure and is never followed, so no write is ever re-sent under another HTTP method.
A read (GET) follows a redirect to its target. The command contract is `erp/spec/cli-surface`.

## Canonical fields

### Use cases

#### UC-TR1:

```gherkin
Scenario: A mutating request answered with a redirect is not re-sent
Given a site that answers a mutating request with a redirect to another location
When the agent applies any mutating erpnext command
Then the command fails with a typed failure naming the redirect
And no request reaches the redirect target
And no success is reported.
```

#### UC-TR2:

```gherkin
Scenario: A read answered with a redirect follows it
Given a site that answers a read request with a redirect to another location
When the agent runs a reading erpnext command
Then the read is served from the redirect target
And the command reports that result.
```
