---
description: Deterministic resolution of parties, items, and documents from loose business queries, with ambiguity as a structured blocker.
delivered_by:
  - erpnext-cli-initial
---

# Reference Resolution — Spec

## What it is

Business language names things loosely — "the Jansen invoice", "that solar panel" — while
ERPNext requires exact document identities. Resolution turns a loose query into an exact
reference or a structured decision boundary, never a guess. The command surface is governed
by `erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: A unique match resolves
Given exactly one party matches the query within the selected company scope
When the agent runs `erpnext resolve party <query>`
Then the output identifies the matching record with its doctype and exact name
And the same contract holds for `erpnext resolve item` and `erpnext resolve document`.

#### UC-2

Scenario: Multiple candidates are a structured ambiguity
Given more than one record plausibly matches the query
When the agent runs a resolve command
Then the output is a structured ambiguity result listing the candidates with distinguishing
fields
And no candidate is auto-selected.

#### UC-3

Scenario: No match is a structured miss
Given no record matches the query within the selected company scope
When the agent runs a resolve command
Then the output is a structured not-found result
And no record is created as a side effect.
