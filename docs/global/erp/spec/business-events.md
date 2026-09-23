---
description: Minimal ERP business-event vocabulary for input handling, work coordination, and selected native-record changes, carried through the platform event envelope.
visibility: internal
delivered_by:
  - erpnext-business-event-mapping
---

# Business Events — Spec

## What it is

ERP events report selected business observations that bring a subscribed agent's
attention to current work. ERP integration owns their native meaning and mapping.
TeleClaude owns the event envelope, registry, routing, execution context, and
session activation. The event names describe observations, not commands to run
procedures or proof that their business effects remain current.

## Canonical fields

### Event vocabulary

| Event | Emission condition | Native anchor |
| --- | --- | --- |
| `erp.input.received` | New inbound business input is durably recorded for interpretation, including a request, reply, correction, or supporting information | The incoming Communication or explicitly captured business-input record and its source reference |
| `erp.work.completed` | An assignment's successful outcome is recorded and its native work is marked complete | The native Task, assignment reference, and stored completion report |
| `erp.work.blocked` | An explicit work report records an obstruction requiring attention beyond what the executor can presently resolve | The native work, assignment reference, and recorded obstruction with partial results and needed contribution |
| `erp.record.changed` | A configured native business-record change relevant to ongoing work occurs and is not represented by a more specific event above | The native document and the selected changed state, date, quantity, or relationship |

These four names form the initial event set. Work completion is the first
end-to-end acceptance path; all four belong to the capability's delivered scope.

### Selection and source mapping

The adapter observes configured native document events and recorded work outcomes.
Incoming input is routed for interpretation; the publisher does not classify its
free text to decide a business action. Source records created by humans, agents,
ERPNext, or other integrations are eligible under the same mapping. Emission is
not confined to writes performed through the CLI.

The record-change mapping explicitly selects document kinds and native changes
that merit attention. An ordinary save, unchanged value, technical metadata
update, or bookkeeping-only activity does not itself qualify. A specific input
or work event takes precedence over the generic record-change event for the same
observation; the publisher does not emit both representations of it.

An idle session, open Task, normal waiting condition, or missing reply does not
establish a work obstruction. The executor's explicit report establishes that
fact. Runtime/provider failures remain platform events. Due attention and missing
expected events use the platform's timer or periodic activation mechanisms;
they do not require another ERP event family or an ERP-owned polling service.

### Payload and correlation

Events use the platform's established envelope, occurrence identity, timestamps,
and routing metadata. The platform contract is discoverable with
`telec docs get teleclaude/spec/event-vocabulary`; this spec adds no second
envelope or transport protocol.

The ERP payload supplies:

- the configured source site and native document type/name;
- a reference to the recorded input or observed change, with enough change
  context to orient the receiver;
- the source occurrence or revision correlation required to recognize a
  repeated observation;
- for work events, the work/assignment reference and stored result or obstruction
  reference;
- related business-record references when the source supplies them.

Missing source attribution remains unknown rather than being inferred from an ERP
service user. Source actors describe who supplied business information; execution
authority comes from the framework-resolved subscription context. Credentials and
session authentication material are not event payload data.

Repeated delivery retains the correlation to the same source observation. The
receiving agent reconciles it with durable work and current ERP state. The
publisher does not add an independent acknowledgement, replay, or duplicate-work
engine around the platform's delivery contract.

### Subscription use

Input handling, work coordination, and business-state reconciliation are useful
interest groupings, not mandatory separate agents. A procedure can subscribe to
several of these events. The platform resolves each configured group's scoped
singleton, starts or wakes it, and supplies the event payload. Its activation
ownership remains distinct from self-service procedure selection.

## Known caveats

- ERP-native mapping requires configured source observation and the platform's
  procedure-subscription activation capability. Event names alone wire up neither.
- A received event directs attention; its payload is not a substitute for reading
  current native state before a consequential operation.
- This vocabulary requires no continuously running ERP-owned daemon. Source
  observation uses ERPNext's supported mechanisms and shared runtime services.

## See Also

- ~/.teleclaude/docs/erp/principle/operational-attention.md
- ~/.teleclaude/docs/erp/principle/business-input-traceability.md
- ~/.teleclaude/docs/erp/concept/operating-state.md
- ~/.teleclaude/docs/erp/spec/feature/erp-operations/business-event-mapping.md
- ~/.teleclaude/docs/software-development/policy/domain-ownership.md
