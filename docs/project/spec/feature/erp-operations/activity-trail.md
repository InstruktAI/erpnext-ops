---
description: Capture, recall, and follow-up over business activity and evidence as thin operations on native Frappe and ERPNext records.
delivered_by:
  - erpnext-cli-initial
---

# Activity Trail — Spec

## What it is

Agents preserve business input, actions, follow-ups, and evidence as native Frappe and
ERPNext records (`Communication`, `Comment`, `ToDo`, `Event`, `CRM Note`, linked business
documents) and query them back. The commands are thin, predictable operations over those
records — no bespoke aggregation layer. The trail is the memory layer described in
`erp/concept/business-activity-trail`; the command surface is governed by
`erp/spec/cli-surface`; the recording discipline by `erp/principle/business-input-traceability`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: Capture business input against a document
Given an active site context and a referenced business document
When the agent runs `erpnext activity capture --summary <text> --reference <doctype> <name>`
Then a native activity record linked to the referenced document is created
And the source summary — and verbatim text when provided — is preserved on the record.

#### UC-2

Scenario: A follow-up becomes an open loop
Given a capture that includes `--follow-up <text> --due <date>`
When the agent applies it
Then a native `ToDo` with the due date exists, linked to the referenced document
And it appears in `erpnext activity open-loops` until resolved.

#### UC-3

Scenario: Recall the trail for a reference and window
Given activity records exist for a document across time
When the agent runs `erpnext activity list --reference <doctype> <name> --since <when>`
Then the matching records are returned in machine-readable, time-ordered form
And `erpnext activity recall` runs the same query across the native record kinds.

#### UC-4

Scenario: Attach evidence to a document
Given a local file relevant to a business document
When the agent runs `erpnext evidence attach --reference <doctype> <name> --file <path>`
Then the file is attached to the referenced document with its summary
And the attachment is visible in the document's trail.

#### UC-5

Scenario: The trail never carries credentials
Given any activity capture or evidence attachment
When the record is written
Then no API key, secret, or credential material appears in the recorded content.
