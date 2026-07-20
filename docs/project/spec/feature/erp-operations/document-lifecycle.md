---
description: ERPNext document lifecycle operations — submit, cancel, amend, and workflow methods honoring docstatus semantics.
delivered_by:
  - erpnext-cli-initial
---

# Document Lifecycle — Spec

## What it is

Submitted ERPNext documents are financial or operational records with native lifecycle
semantics: `docstatus` 0 is draft, 1 is submitted, 2 is cancelled. Agents drive that
lifecycle through explicit workflow commands that preview consequences before applying,
per the command contract in `erp/spec/cli-surface` and the submitted-document rules in
`erp/policy/operating-disciplines`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Submit previews its consequences
Given a draft document of a submittable doctype
When the agent runs `erpnext workflow submit <doctype> <name>` without `--apply`
Then the preview names the expected docstatus transition and known side effects such as
stock, accounting, or linked-document effects
And no state changes on the instance.

#### UC-2

Scenario: Submit applies the lifecycle transition
Given a clean submit preview
When the agent applies the submit
Then the document's docstatus becomes submitted
And the result is returned in machine-readable form.

#### UC-3

Scenario: Cancel a submitted document
Given a submitted document
When the agent previews and applies `erpnext workflow cancel <doctype> <name>`
Then the document's docstatus becomes cancelled
And side effects known before application appear in the preview.

#### UC-4

Scenario: Amend a cancelled document
Given a cancelled document
When the agent previews and applies `erpnext workflow amend <doctype> <name> --data <payload>`
Then a new amended draft linked to the cancelled document is created with the payload applied.

#### UC-5

Scenario: Direct edits to submitted documents are refused
Given a submitted document
When the agent attempts `erpnext doc update` on it
Then the command fails with a structured failure naming the lifecycle conflict
And the failure points to the cancel-and-amend path instead of editing in place.

#### UC-6

Scenario: Doctype-specific workflow methods follow the preview contract
Given a doctype exposing a native workflow method
When the agent runs `erpnext workflow method <doctype> <name> <method>`
Then the method call previews by default and executes only with explicit `--apply`.
