---
description: Orient or resume an ERP coordinator or scoped worker, assess changed facts and unmet expectations, select or delegate useful work, and preserve native outcomes and continuity.
visibility: internal
activation_trigger: When an ERP agent starts or resumes a standing responsibility or bounded assignment, receives relevant human or event input, or wakes to assess due expectations and unfinished work.
---

# Operating Cycle — Procedure

## Required reads

@~/.teleclaude/docs/erp/principle/operational-attention.md
@~/.teleclaude/docs/erp/concept/operation-model.md
@~/.teleclaude/docs/erp/concept/operating-state.md
@~/.teleclaude/docs/erp/concept/composable-business-procedures.md
@~/.teleclaude/docs/erp/policy/operating-disciplines.md

## Goal

Progress the agent's ERP responsibility through informed selection of work and
native operations, leaving enough durable state for another activation or agent
to continue under the same mandate.

## Preconditions

- The operating site, company scope, and standing mandate or bounded assignment
  are available or resolvable from the entry context and company knowledge.
- The applicable ERP domain baseline is loaded once for this session.
- The activation identifies available input or points to the work to inspect.
  An empty queue or absence of new human input does not itself end the mandate.

## Steps

### 1. Orient from the responsibility and its records

Establish the operating context through the CLI and read the company mandate.
Distinguish the platform's resolved lineage and project/company relationship from
the ERP site's configured accounting scope and operator identity. Consult
`erp/spec/company-layout` and the live CLI context contract; do not assume that
platform identity automatically selects the ERP context in every implementation.
Identify whether this invocation holds the business overview, a bounded worker
assignment, or a scheduled job's declared scope. Retain the job's own outcome
reporting contract when it is a job invocation.

Read current native work, active assignment references, waiting obligations, and
unresolved effects in that scope. After compaction, recover from those records;
expand source history only where needed. Load `erp/spec/cli-surface` when command
discovery is needed and inspect live metadata for the records being operated.

### 2. Reconcile incoming changes

Read relevant human input, delivered events, and worker outcomes since the last
recorded assessment. Preserve source references and distinguish a request,
correction, observation, and exploratory discussion. Use the work's existing
identity before creating another task for an already-recorded need.

When a premise changes, revise the affected unstarted work and communicate the
change to its active owners through the existing session surface. Follow an
explicit stop immediately. Determine any in-flight effects before resuming;
activate `erp/procedure/resume-from-native-state` for uncertain prior mutations.
Preserve completed outcomes and record any corrective work separately.

### 3. Scan expectations against current reality

A coordinator reads a scoped overview. A worker expands only the landscape needed
for its assignment. An event directs the first inspection; periodic attention
looks for expected events that did not arrive and obligations that have gone stale.

Assess the focus areas relevant to the business:

| Focus | Inspect for |
| --- | --- |
| Enquiries and sales commitments | Unanswered enquiries, promised callbacks, quotations awaiting a decision, accepted work not progressed |
| Fulfillment and procurement | Customer promises, supplier commitments, shortages, overdue delivery, missing receipt confirmation, capacity conflicts |
| Service and project work | Milestones, acceptance, partial completion, handoffs, finished work awaiting billing |
| Billing and settlement | Unissued billing, outstanding receivables and payables, unallocated receipts, credits, disputes and missing supporting documents |
| Integrity and exceptions | Duplicate candidates, contradictory evidence, failed operations and uncertain external effects |
| Recurring obligations | Applicable renewals, service visits, expirations, reporting and period work within the standing mandate |
| Relationships and open loops | Missing replies, owner contributions, contact preferences and expectations that now merit follow-up |

Use native reports, record links, and activity as appropriate. Distinguish partial
fulfillment from complete or absent fulfillment. A changed date or cancelled
dependency requires business assessment rather than automatic propagation of
success. Normal waiting may require no further contact.

### 4. Choose work using domain judgment

Apply company priorities and the standing mandate. Weigh imminent commitments and
consequences of delay, unresolved effects that obstruct safe continuation,
dependencies whose resolution releases other work, and aging obligations that
repeated urgent arrivals can obscure. Prefer determined, actionable work where it
serves those needs. There is no universal numeric score or required traversal
that exhausts one category before considering another.

Resolve factual uncertainty through available evidence. Surface only the missing
human-owned contribution under the governing autonomy and leadership contracts.
Record a material priority change with its reason in the work trail.

### 5. Execute or delegate a focused outcome

For delegation, activate `general/procedure/ai-to-ai-operations`. Select a suitable
procedure from the self-service ERP catalogue's description and activation condition; read its
body when needed to resolve scope. Send the native work reference, the canonical
procedure ID and `telec docs get <id> --closure` instruction, expected outcome,
business references, and constraints. Record the returned session reference with
the assignment. Do not reproduce the procedure body or invent a command wrapper.
For work already assigned to a configured job or event owner, route the relevant
input to that owner rather than starting an independent execution of its procedure.

Reuse an available worker when appropriate, explicitly settling its previous
assignment. Dispatch independent work within available capacity; consider shared
records before parallel execution. A worker can delegate a bounded part within
its mandate while retaining responsibility for that contribution.

For execution, load the selected procedure closure and compose individual CLI
operations. Use existing native Task, activity, and evidence operations. Inspect
the current command surface rather than assuming specialized task commands or
automatic provenance hooks exist. The executor applies the native proof required
by the relevant ERP procedure before reporting its result.

### 6. Record the outcome and next attention

Record material results and evidence references against the work before changing
its completion state. A waiting or failed attempt records what succeeded, what
remains uncertain, and the next contribution needed. Preserve native references
so retry or replacement continues the same work.

When a child turn notification arrives, read the associated work record. Accept
its complete, attributable outcome without replaying its checks. Use transcript
or live-session inspection for missing results, contradictory signals, or needed
diagnosis under the session-operation procedure. A worker leaves its result in
its own turn output; the platform's parent notification carries attention upward.

Update dependent work from the recorded outcome and current business facts.
Retain or transfer outstanding expectations and select an appropriate configured
event, session reminder, or periodic check. Apply `general/policy/reminder` before
arming or cancelling reminders. Yield when no useful action is due; continue the
standing responsibility on its next activation. Review idle workers for reuse or
closure only after accounting for their obligations and descendants.

## Outputs

- Native work reflects the current outcome sought, ownership, progress, and next
  contribution.
- Material input and outcomes have attributable activity and evidence references.
- Delegations have bounded briefs and recorded executor references.
- Waiting obligations have a responsible owner and an attention condition.
- A scheduled invocation also produces the report its job contract requires.

## Recovery

- **Missing mandate or ambiguous context:** recover from the owning records and
  company knowledge; resolve the remaining authority or business question before
  mutation.
- **Unavailable event route or delivery channel:** retain the obligation and
  report the integration gap; do not claim that a wake or message was delivered.
- **Uncertain effect:** reconcile native state before repeating the action; retain
  the uncertainty if the authoritative system is unavailable.
- **Partial or contradictory worker report:** request the missing contribution or
  inspect the relevant evidence; do not start another worker solely from silence.
- **Procedure gap:** use scoped domain judgment and live metadata where the action
  is determined. Preserve a specific gap for governed documentation improvement;
  do not invent an unavailable command or broaden the mandate.

## See Also

- ~/.teleclaude/docs/erp/procedure/relationship-follow-up.md
- ~/.teleclaude/docs/erp/procedure/resume-from-native-state.md
- ~/.teleclaude/docs/erp/procedure/prove-native-effect.md
- ~/.teleclaude/docs/general/procedure/ai-to-ai-operations.md
- ~/.teleclaude/docs/general/procedure/job-authoring.md
- ~/.teleclaude/docs/general/policy/reminder.md
- ~/.teleclaude/docs/general/principle/session-lifecycle.md
