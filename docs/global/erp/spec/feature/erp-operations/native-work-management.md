---
description: Single-operation semantic CLI access to native ERPNext work items, assignments and attributable outcomes, preserving native IDs, state and business history.
visibility: internal
delivered_by:
  - erpnext-native-work-management
---

# Native Work Management — Spec

## What it is

Agents manage delegated outcomes through concise individual operations over native
ERPNext work records. The coordinator chooses work, scope, order and concurrency;
the executor establishes the business result. The CLI records the chosen operation
and reports the native result, without a second work store, board, or business
workflow engine.

## Canonical fields

### Native work mapping

The binding uses live metadata and the supported native operations on the target
site. The native reference is site, DocType and document name.

| Work meaning | Native binding |
| --- | --- |
| Work item | Task; its native name is the work ID |
| Brief and expected outcome | Task subject and description |
| Grouping | Project where useful; group/parent Task for nested work |
| Dependencies | Native Task dependencies; business readiness remains the agent's judgment |
| Priority and planned attention | Task priority and expected dates, with outstanding commitments retained as linked follow-ups |
| Business progress and completion | Task status and native completion fields supported by the site |
| Material outcome, source input and evidence | Linked native activity and document/file references |
| ERP-assigned user | Native user assignment; kept distinct from the executing agent session |
| Executing session and assignment occurrence | An explicit native association to the platform-resolved execution context, queryable with the work record |

The session/assignment association preserves native task identity through
reassignment and uses the site's configured native work mapping. It is not
inferred from the ERP user's name or implemented as an external task database.
Missing required native fields or configuration are surfaced rather than silently
dropped.

### Operation boundary

The semantic surface covers creating, inspecting, assigning, updating,
recording an outcome, and withdrawing native work. Each command expresses one
operation. The agent orders those operations and chooses their content. Native
IDs are reused across resumption; system-provided session context supplies
execution attribution without requiring the agent to pass its own identity.

Native status values are preserved. A waiting condition or failed attempt records
its reason, partial effects, and next contribution without inventing an
unsupported Task status. Completing work requires the executor's declared
outcome; the CLI checks that its native recording succeeded, not whether the
agent's business judgment is correct. The coordinator can consume that durable
report without routine transcript inspection or repeating the executor's proof.

An outcome is associated with the assignment that produced it. A repeated report
does not create duplicate completion history, and a late report from an earlier
assignment does not complete its successor. Relevant native dependency behavior
is exposed honestly, including a cancelled prerequisite; it does not substitute
for the coordinator's decision about what work is now appropriate.

### Use cases

#### UC-NWM1:

```gherkin
Scenario: A coordinator creates recoverable native work
Given an agent coordinator has a bounded outcome to delegate in its resolved ERP context
When it creates a work item through the semantic CLI surface
Then it receives the native work reference
And the brief and intended outcome are stored in ERPNext and visible through its native work view
```

#### UC-NWM2:

```gherkin
Scenario: Assignment preserves work identity
Given a native work item has been assigned to an agent session
When the coordinator reassigns that work to another session
Then the native work identity remains unchanged
And the current executor and the prior assignment history are distinguishable
```

#### UC-NWM3:

```gherkin
Scenario: A worker records completion for its assignment
Given an agent worker has established its assigned business outcome
When it records the result and completes its native work
Then the durable report carries the outcome, evidence references and resolved execution attribution for that assignment
And the coordinator can determine completion from the work record without reading the worker's transcript
```

#### UC-NWM4:

```gherkin
Scenario: Waiting and partial failure remain actionable
Given an agent worker has performed only part of its assignment or is waiting for a contribution
When it records that state
Then the native work retains the partial result, unresolved condition and next contributor
And it is not reported as completed
```

#### UC-NWM5:

```gherkin
Scenario: Repeated and late reports preserve assignment history
Given an agent assignment has a recorded outcome and the work has since been reassigned
When a report for the earlier assignment arrives again
Then the existing outcome remains attributable to the earlier assignment without a duplicate completion record
And the current assignment is not marked complete by that report
```

#### UC-NWM6:

```gherkin
Scenario: An accepted native request without its expected stored change is not success
Given an agent invokes a work-state operation
When the site accepts the request but the expected work-state change is not stored
Then the CLI reports the unresolved result with the native reference
And it does not report successful completion of that operation
```

#### UC-NWM7:

```gherkin
Scenario: Withdrawal preserves the business trail
Given an agent coordinator withdraws uncompleted native work after a change of direction
When the withdrawal is recorded
Then the work no longer appears as an outstanding commitment
And its input, prior attempts and withdrawal reason remain traceable
```

## Known caveats

- A completed task is an audit of accomplished work, not an immutable snapshot of
  the world. Native editability and history guarantees remain the site's facts.
- Semantic command discovery uses the CLI's declared surface; no second
  handwritten command inventory is introduced.
- Native effect verification inside the business operation remains the
  executor's duty; reading the recorded result is the coordinator's ordinary
  completion surface.

## See Also

- ~/.teleclaude/docs/erp/spec/cli-surface.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/concept/operating-state.md
- https://docs.frappe.io/erpnext/tasks
- https://docs.frappe.io/erpnext/project-views
