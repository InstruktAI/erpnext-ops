---
description: Deterministic resolution of parties, items, and documents from business queries, with ambiguity as a structured result.
delivered_by:
  - erpnext-cli-initial
---

# Reference Resolution — Spec

## What it is

Business language names things loosely — "the Jansen invoice", "that solar panel" — while
ERPNext requires exact document identities. Resolution runs standard ERPNext searches over
the query and returns an exact reference, a structured candidate list, or a structured miss —
never a guess. Interpreting fuzzy business meaning stays with the agent; the CLI only makes
the search deterministic and its outcome structured. The command surface is governed by
`erp/spec/cli-surface`.

## Canonical fields

### Acceptance scenarios

#### UC-1

Scenario: A unique match resolves
Given exactly one party matches the query on the site
When the agent runs `erpnext resolve party <query>`
Then the output identifies the matching record with its doctype and exact name
And the same contract holds for `erpnext resolve item` and `erpnext resolve document`.

#### UC-2

Scenario: Multiple candidates are a structured ambiguity
Given more than one record matches the query
When the agent runs a resolve command
Then the output is a structured ambiguity result listing the candidates with distinguishing
fields
And no candidate is auto-selected.

#### UC-3

Scenario: No match is a structured miss
Given no record matches the query on the site
When the agent runs a resolve command
Then the output is a structured not-found result
And no record is created as a side effect.
