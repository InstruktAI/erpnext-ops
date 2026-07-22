---
description: ERPNext document lifecycle operations — submit, cancel, amend, and workflow methods honoring docstatus semantics.
---

# Document Lifecycle — Spec

## What it is

Submitted ERPNext documents are financial or operational records with native lifecycle
semantics: `docstatus` 0 is draft, 1 is submitted, 2 is cancelled. Agents drive that
lifecycle through explicit workflow commands that preview the native operation before
sending it; ERPNext owns the lifecycle rules and their consequences. The command contract is
`erp/spec/cli-surface`; the submitted-document rules are in
`erp/policy/operating-disciplines`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Submit previews the native operation
Given a draft document of a submittable doctype
When the agent runs `erpnext workflow submit <doctype> <name>` without `--apply`
Then the preview names the exact native request and the intended docstatus transition
And no mutating request is sent to the site.

#### UC-2

Scenario: Submit applies the lifecycle transition
Given a clean submit preview
When the agent applies the submit
Then the native submit is sent and ERPNext's result is reported
And on success the document's docstatus is submitted.

#### UC-3

Scenario: Cancel a submitted document
Given a submitted document
When the agent previews and applies `erpnext workflow cancel <doctype> <name>`
Then the native cancel is sent and ERPNext's result is reported
And on success the document's docstatus is cancelled.

#### UC-4

Scenario: Amend a cancelled document
Given a cancelled document
When the agent previews and applies `erpnext workflow amend <doctype> <name> --data <json>`
Then a new draft carrying `amended_from` is created with the payload applied
And amending a document that is not cancelled fails without creating anything.

#### UC-5

Scenario: Direct edits to submitted documents fail toward cancel-and-amend
Given a submitted document
When the agent attempts `erpnext doc update` on it
Then the command fails with a structured failure carrying ERPNext's lifecycle rejection
And the failure points to the cancel-and-amend path instead of editing in place.

#### UC-6

Scenario: A transition the site silently declines is reported as failure
Given a document the site will not move to the requested state
When the agent applies a submit or a cancel and the site answers without changing
`docstatus`
Then the command fails, naming the state the document remains in
And no success is reported for a transition that did not take effect.

#### UC-7

Scenario: Doctype-specific workflow methods follow the preview contract
Given a doctype exposing a native workflow method
When the agent runs `erpnext workflow method <doctype> <name> <method>`
Then the method call previews the native request by default and sends it only with explicit
`--apply`.
