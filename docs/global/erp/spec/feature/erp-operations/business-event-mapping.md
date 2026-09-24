---
description: ERP-owned mapping of the minimal business-event vocabulary from native input, work outcomes and selected record changes into the platform's scoped procedure activation.
visibility: internal
delivered_by:
  - erpnext-business-event-mapping
---

# Business Event Mapping — Spec

## What it is

ERP integration maps native ERPNext events into business facts for the existing
TeleClaude relay and procedure-activation contract. ERP code owns the meaning of
the source record and event; the platform owns shared ingress, identity,
authorization, subscription matching, singleton activation, and session delivery.
This is an integration of those owners, not an ERP-specific event bus.

## Canonical fields

### Initial mapping boundary

The first end-to-end binding is a native Task completion change. It connects the
work item's stored result to the responsible subscribed procedure without routine
transcript inspection. The delivered MVP covers all four events defined by
`erp/spec/business-events`, which owns their names, meanings, selection rules,
and payload requirements. This contract binds those observations to native ERP
sources and the platform activation surface; it does not add a mail/chat
connector or enumerate the whole ERP domain.

| Native fact | Mapped meaning |
| --- | --- |
| Configured source site and document type/name | Source identity and native work reference |
| Native document event and stored task state | Observed work change; distinguished from proof of an external business effect |
| Source occurrence/revision and available delivery identity | Correlation sufficient to relate repeated delivery to the same observation |
| Stored assignment and result references where present | Link to the work and execution report; absent provenance remains absent |
| Platform-resolved registration and operating scope | Routing context for the subscriber; not a principal chosen from business payload text |

The source-to-event mapping and emitted payload are documented with the integration
and verified against supported ERPNext metadata and webhook semantics. Native
provider details stay in ERP-owned code/configuration. Any missing platform
extension is scoped to its actual owner rather than reproduced in this repository.

### Activation contract

A subscribed ERP procedure uses the platform's supported event-subscription
metadata and has no self-selection activation trigger. Its event group starts or
wakes one singleton under the framework-resolved stable lineage and operating
scope. Matching events supply their payload and native references to that agent;
an active holder receives the event through the existing session delivery route.
The group does not create one worker per event and does not acquire the unrelated
platform fleet-coordinator or integrator role.

Configured subscriptions take effect through the platform contract. A delivery or
activation failure is surfaced through the relevant owner rather than silently
treated as successful work or transferred to an arbitrary agent. An event is a
reason to assess current work, not an instruction to repeat its business effects.

### Use cases

#### UC-BEM1:

```gherkin
Scenario: A native work completion reaches its responsible procedure
Given an ERP task has a stored completion report
And a procedure subscribes to that work-change event in the established operating context
When the native event passes through the configured integration
Then the responsible agent receives the event context and native work reference through the platform delivery surface
And the event remains linked to the stored completion report
```

#### UC-BEM2:

```gherkin
Scenario: Events reuse the scoped singleton
Given a subscribed procedure has a live singleton for an established lineage and operating scope
When another matching ERP event arrives
Then that agent receives the event
And no second singleton is started for the same group and context
```

#### UC-BEM3:

```gherkin
Scenario: Business facts retain the registered execution context
Given an ERP event is associated with a configured source and subscriber context
When the event is mapped and delivered to an agent
Then its native site and work references remain attributable to that source
And the agent's lineage and operating scope come from the framework's resolved context
And business payload content does not redefine that execution context
```

#### UC-BEM4:

```gherkin
Scenario: Repeated observations remain recognizable
Given a native work-change observation has already reached its subscribed agent
When the same source observation is delivered again
Then its correlation still identifies the same native observation
And the agent can reconcile it with the existing work rather than infer a new assignment
```

#### UC-BEM5:

```gherkin
Scenario: Missing routing capability is an explicit integration result
Given an agent or operator configures a native work event for procedure activation
When a required mapping or platform routing capability cannot be resolved
Then the configuration or delivery reports the specific integration gap
And it does not claim that the procedure was activated
```

#### UC-BEM6:

```gherkin
Scenario: Recorded business input reaches its subscribed agent
Given new inbound business input has been recorded with a source reference
And an agent procedure subscribes to incoming business input in that operating context
When the native observation passes through the configured mapping
Then the subscribed agent receives an input-received observation with the source record reference
And the publisher does not decide from the input's prose which business action to perform
```

#### UC-BEM7:

```gherkin
Scenario: An explicit work obstruction reaches its coordinator
Given an agent executor has recorded an obstruction that requires a contribution beyond what it can presently resolve
When that work report passes through the configured mapping
Then the responsible subscribed agent receives the obstruction observation
And it can reach the assignment, recorded partial result and required contribution through the supplied references
```

#### UC-BEM8:

```gherkin
Scenario Outline: Normal work state does not imply an obstruction
Given the only observed condition is <condition>
And no work obstruction has been explicitly reported
When the ERP event mapping assesses that observation
Then it does not emit a work-obstruction event to a subscribed agent

Examples:
  | condition |
  | an idle agent session |
  | an open task |
  | a normal waiting condition |
  | a reply that has not arrived |
```

#### UC-BEM9:

```gherkin
Scenario: Selected changes outside the agent CLI remain observable
Given a subscribed agent follows a configured business-record change
When a human changes that native record directly in ERPNext
And the observation is not represented by a more specific input or work event
Then the agent receives a record-change observation with the native reference and relevant change context
```

#### UC-BEM10:

```gherkin
Scenario: A specific observation is not emitted again as a generic change
Given a native observation qualifies as an input or work-outcome event
And the same observation also matches a configured generic record-change selection
When the publisher maps that observation for subscribed agents
Then it emits the specific observation without an additional generic record-change representation
```

#### UC-BEM11:

```gherkin
Scenario: Unselected and bookkeeping-only changes do not wake business procedures
Given a native record save contains no selected business change
When the ERP event mapping processes that save
Then it emits no generic business-record-change event for a subscribed agent
```

## Known caveats

- End-to-end activation consumes TeleClaude's procedure-subscription singleton
  activation capability. The ERP mapping does not supply a parallel implementation
  when that platform capability is unavailable.
- Shared transport and control-delivery guarantees remain those of their owning
  contracts. This mapping adds no independent acknowledgement or retry protocol.
- Periodic review handles missing expected business events. It does not duplicate
  event-driven responsibility or reinterpret a delivery failure as business success.
- Event delivery requires the source site's configured webhook and the platform's
  existing ingress route. A published procedure alone supplies neither source
  connectivity nor an operating-context binding.
- ERP mapping uses native source observation and shared platform runtime services;
  it introduces no continuously running ERP-owned daemon.

## See Also

- ~/.teleclaude/docs/erp/spec/business-events.md
- ~/.teleclaude/docs/erp/spec/feature/erp-operations/session-context.md
- ~/.teleclaude/docs/erp/spec/feature/erp-operations/native-work-management.md
- ~/.teleclaude/docs/erp/principle/operational-attention.md
- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/sdlc/policy/domain-ownership.md
- https://docs.frappe.io/framework/user/en/guides/integration/webhooks
