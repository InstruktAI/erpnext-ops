---
description: Disciplines for agents acting on ERPNext site data through the erpnext CLI.
---

# ERP Operating Disciplines — Policy

## Rules

### Continuing work

- **Operate within the standing responsibility.** On activation or resumption,
  recover the scoped work and relevant changes before selecting the next action.
  A fresh human request is not required for work covered by the mandate.
- **Use native work as the plan.** Record independently delegated outcomes in
  ERPNext Tasks and retain their references through dispatch and reassignment.
  Current briefs name constraints, relevant records, outstanding contributions,
  and executor references. Local reasoning does not become a competing plan.
- **Reconcile changes before continuing affected work.** Preserve the source of a
  correction, revise unstarted work, and notify affected executors. Establish
  what happened in flight before repeating or correcting an effect. A sent
  message is not proof that another agent stopped.
- **Delegate a bounded outcome.** Supply the work reference, procedure ID and
  fetch instruction, expected result, and assignment-specific constraints.
  Executors load the procedure closure and choose operations within that scope.
  Parallelism follows independent work, not the number of incoming events.
- **Keep reports attributable to work.** Before marking a Task complete, record
  the material outcome and evidence references. Waiting or failed attempts carry
  their partial effects and missing contribution. The coordinator accepts a
  complete report associated with the assignment without routine re-verification
  or transcript inspection. A turn ending is not itself a completion report.
- **Distinguish business and execution identity.** Preserve native work IDs and
  references to assigned sessions. Caller identity is system-provided, not an
  agent-authored authentication field; an ERP operator account does not identify
  the human source of an instruction or distinguish every executing agent.
- **Preserve waiting obligations.** Keep the expected event, attention condition,
  and next contributor with the work. Use the configured session reminder, job,
  or event route appropriate to its owner. Inactivity alone does not discharge a
  worker; settle or transfer obligations before reassignment or closure.
- **Learn within the mandate.** Retain source-backed party preferences and
  observations. Outreach uses the company's delegated communication authority
  and an available, appropriate channel. Owner handoffs preserve the open loop.

### Working company

- **Development and testing operate on the sandbox company.** The sandbox company lives on
  the dedicated test tenant: its own site, holding no business's books. Work that exercises, develops,
  demonstrates, or verifies ERP capability runs against it, and the agent doing that work
  names it as the company. Business work runs against the company the human's request
  establishes. An agent never uses a business company to try something out.

### Native operations

- **Use the CLI surface.** Agents operate ERPNext through `erpnext`. Raw transport mechanics
  are implementation detail, not the agent-facing operating path.
- **Preview before mutation.** Mutating commands preview by default: the exact native
  operation that would be sent, without sending it. Apply only after the preview is clean or
  any remaining warnings have been resolved by evidence, configuration, standard ERP
  practice, or a human decision where one is genuinely required.
- **Let the CLI preflight live metadata.** Create and update commands check payloads against
  the live DocType shape before sending. Agents provide business intent and data; ERPNext
  remains the authority on validation, permissions, and transactions.
- **Create draft-first when state is not yet determined.** New business documents are created
  as drafts while business state is incomplete or uncertain. When the business event is
  determined and standard ERP practice calls for a submitted document, the agent proceeds.
- **Resolve references before writing.** Master-data and business-document creation resolves
  likely duplicates and ambiguous references through structured resolution results. Agents
  resolve candidates instead of creating parallel records blindly.
- **Operate as the scoped identity.** Routine business work uses the site's operator
  credentials. Administrator credentials are reserved for documented administrative setup.
- **Preserve business input traces.** Human input that creates, influences, or may later
  justify ERP state is recorded as activity linked to the relevant business object when one
  is known.
- **Close or carry open loops.** Unresolved commitments, missing details, expected deliveries,
  promised callbacks, and future checks are either resolved during the current operation or
  carried as follow-up work.
- **Surface only decision boundaries.** Do not interrupt the human because an operation is
  important. Surface only missing business intent, conflicting evidence, ambiguous targets,
  absent authority, or configuration gaps the agent cannot resolve.
- **Respect submitted-document semantics.** `docstatus` is `0` draft, `1` submitted, and `2`
  cancelled. Submitted financial documents are changed through cancel-and-amend flows, not
  direct edits.
- **Establish the way back before relying on it.** Reversal is not one mechanism, and which
  correction a target supports is a fact about that target on that site, not a general rule.
  Before applying a mutation whose correctness is not yet determined, the agent reads the
  target's submittability and `docstatus` from live metadata, establishes from native state
  and vendor source which correction the site supports for it, names the dependencies that
  correction must clear and the read-back that proves it, and treats a mutation with no
  supported correction as irreversible under the applicable safety gate. An effect outside
  ERPNext is never recalled by a correction inside it.
- **Keep secrets out of output.** API keys, secrets, and login links are written only to their
  credential destination or sent through the documented ERPNext channel.

## Rationale

Standing operations require continuity across messages and sessions. A shared
current plan prevents repeated work, while a retained outcome trail preserves
accountability when intent changes. Bounded briefs and attributable results let
the coordinator trust workers without making transcript reading a second
execution of every assignment. Expectations and relationship evidence distinguish
useful attention from intrusive reminders or abandoned commitments.

ERPNext is a financial system of record. A wrong invoice, duplicated record, over-broad user,
or leaked operator secret has operational and legal consequences. The CLI is the safety layer
that lets agents keep acting: it shows the exact operation before mutation, resolves routine
mechanics, preserves traces, and surfaces only real decision boundaries.

## Scope

All agent operations against ERPNext sites.

## Enforcement

- Work continued from an invalidated brief without reconciliation is a defect.
- A delegated outcome with no durable work reference, or a completed item with
  no material result recorded, is a continuity defect.
- Treating turn completion, dispatch acceptance, or message delivery as business
  success is a defect.
- Routine duplicate verification of a complete worker outcome is not required;
  missing or contradictory outcome evidence is resolved before dependent work.
- Dropping a waiting obligation because its worker is idle is a defect.
- Inferring outreach authority from a learned personality or response pattern is
  a defect.
- Development, demonstration, or verification work performed against a business company
  rather than the sandbox company is a defect.
- A business operation performed outside the CLI is a defect unless the active procedure
  explicitly authorizes project-owned implementation work.
- A mutation applied without reviewing its preview is a defect.
- A write that bypasses CLI metadata preflight is a defect.
- Routine business work performed with administrator credentials is a defect unless the
  active procedure explicitly requires administrator authority.
- A human interruption for a decision the agent could determine from evidence, configuration,
  party preferences, templates, ERPNext state, or standard ERP practice is a defect.
- A submitted, cancelled, deleted, or outbound-sent document created without being visible in
  the preview is a defect.
- Duplicate records traceable to a missing natural-key check are a defect.
- A business mutation without a traceable source input is a defect when the mutation was caused
  by human instruction or informal business context.
- An unresolved commitment without follow-up is a defect.
- A mutation applied while its correctness was undetermined, without a named correction
  mechanism and read-back, is a defect; one with no such mechanism is irreversible and falls
  under the applicable safety gate.

## Exceptions

None.

## See Also

- ~/.teleclaude/docs/erp/principle/operational-attention.md
- ~/.teleclaude/docs/erp/concept/operating-state.md
- ~/.teleclaude/docs/erp/concept/business-relationships.md
- ~/.teleclaude/docs/erp/procedure/operating-cycle.md
- ~/.teleclaude/docs/general/procedure/ai-to-ai-operations.md
- ~/.teleclaude/docs/general/policy/reminder.md
- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/erp/concept/multi-tenancy.md
- ~/.teleclaude/docs/erp/principle/determinate-autonomy.md
- ~/.teleclaude/docs/erp/principle/business-input-traceability.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
