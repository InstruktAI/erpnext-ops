---
description: Generic invocation of a whitelisted module-level Frappe or ERPNext method through the erpnext CLI, with preview by default, an opaque payload, and honest native results.
delivered_by:
  - erpnext-module-method-call
---

# Module Method Call — Spec

## Required reads

@docs/project/principle/generic-cli-substrate.md
@~/.teleclaude/docs/erp/spec/cli-surface.md

## What it is

Agents invoke any whitelisted module-level Frappe or ERPNext method through one generic
command that owns context, authentication, secret redaction, the preview and apply gate, and
honest reporting of the native result. The method path and its JSON object are opaque to the
CLI; ERPNext owns the method's validation, permissions, transactions, and business meaning.
Business scenarios compose this primitive with existing generic reads in ERP procedures. The
command contract is `erp/spec/cli-surface`; the boundary is
`project/principle/generic-cli-substrate`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Preview sends nothing
Given an active site context and a dotted method path
When the agent runs `erpnext method call <dotted-method> [--data <json>]` without `--apply`
Then the output is a preview carrying the resolved context, the exact native request for the
named method with the supplied object, and the apply command
And no request reaches the site.

#### UC-2

Scenario: Apply sends the opaque object to the named method
Given a preview for a dotted method and a JSON object
When the agent runs the same command with `--apply`
Then the object is sent unchanged to the named method under the selected context
And the native result is returned under the standard `data` envelope.

#### UC-3

Scenario: Malformed or non-object data is refused before any request
Given a `--data` value that is not valid JSON or not a JSON object
When the agent runs the command with `--apply`
Then the command fails with the structured failure shape naming the boundary violation
And no request reaches the site.

#### UC-4

Scenario: A site refusal is reported as the site answers
Given a method the site does not expose, a permission the identity lacks, or a call the
method rejects
When the agent applies the command
Then the command fails with the structured failure shape carrying the site's typed answer
And no fallback request is sent and no success is reported.

#### UC-5

Scenario: A transport failure is a typed failure
Given the site cannot be reached or answers outside the expected envelope
When the agent applies the command
Then the command fails with the structured failure shape naming the transport outcome
And credentials appear nowhere in the output.
