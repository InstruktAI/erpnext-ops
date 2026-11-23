---
description: Disciplines for agents acting on ERPNext site data through the site's MCP server.
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
  demonstrates, or verifies ERP capability connects to the sandbox site. Business work
  connects to the site of the company the human's request establishes. Before a write the
  agent establishes which site, user and company it is connected as, so it never relies on a
  company chosen earlier, and never uses a business company to try something out.
- **A site's own access is accepted on that site.** Whether a company's site holds the
  access its specifications state is established on that site, before its people start,
  with test logins and test records that are named as tests and removed afterwards. The
  acceptance makes no attempt that posts to the books.

### Documents

- **Follow the company's document mapping.** Where a company's documents live and how
  their folders are named follows the mapping the company states about itself. Where it
  states none of its own, `erpnext/policy/documents-convention` decides, including what
  comes first when nothing is recorded yet.

### Native operations

- **Use the site's MCP server.** Agents operate ERPNext through the MCP server the site
  serves, as the user it authenticates. `erpnext/spec/agent-surface` names the server operation
  that realizes each operation this doctrine speaks of.
- **Preview before posting.** A document that posts to the books or to stock is created as a
  draft, read back, and its ledger and stock effects previewed before it is submitted. Submit only when
  the draft is correct or any remaining doubt has been resolved by evidence, configuration,
  standard ERP practice, or a human decision where one is genuinely required. A master
  record has no draft: the agent states the intended change, writes it, and reads it back.
- **Read the document type before writing and the record after.** The server stores a write
  without the fields the document type does not have and reports no error. The agent
  composes a write from the live document type and compares the stored record with its
  intent. ERPNext remains the authority on validation, permissions, and transactions.
- **Write against what was read.** A change or a submission carries the version of the
  record the agent reviewed. ERPNext then refuses it when another party changed the record
  in between, where an unmarked write replaces their change or submits a draft nobody
  reviewed. A cancellation cannot carry a version: the agent reads the record immediately
  before cancelling and cancels only the state it reviewed.
- **Create draft-first when state is not yet determined.** New business documents are created
  as drafts while business state is incomplete or uncertain. When the business event is
  determined and standard ERP practice calls for a submitted document, the agent proceeds.
- **Resolve references before writing.** Master-data and business-document creation resolves
  likely duplicates and ambiguous references by searching native records first. A repeated
  create stores a second record, so agents resolve candidates instead of creating parallel
  records blindly.
- **Operate as the scoped identity.** An agent working for a person acts under that person's
  own ERPNext login. Unattended work uses the site's operator credentials. Administrator
  credentials are reserved for documented administrative setup.
- **The business is the company's, the system is its administrator's.** A person's agent
  carries out what that person asks about their own business, its configuration included,
  within what their login allows. The system itself, such as the company's statement and
  the shape of its forms, changes under administrator authority
  (`erpnext/principle/business-and-system`). Asked for such a change under another login,
  or refused one by ERPNext, the agent records the wish as a to-do assigned to the system
  administrator and tells the person it is with them.
- **A login's address changes on every side at once.** The address is how a person is
  recognized by ERPNext and by whatever else the company's people reach it from, such as a
  messaging app. Whether a person's address changes is the company's owner's or
  organizer's to say; anyone else who asks is told to ask them. ERPNext lets only a system
  administrator rename another person's login, so the rename is done under administrator
  authority: asked by an owner or organizer, their agent does not
  attempt it, records the change as a to-do assigned to the system administrator and
  tells them it is with them. The agent that carries the change out keeps the sides in
  line itself: it reads every place that knows the person by the old address, changes
  each one its access reaches, tells the person exactly what only they can change, and
  reads back that every side holds the same address.
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
or leaked operator secret has operational and legal consequences. No tool enforces the
sequence of read, draft, preview, submit and read back: ERPNext's own draft state and these
rules carry it. An agent that skips a step meets no refusal, only the consequence in the
books, which is why each step is stated as a rule.

Where the line between a company's business and its system runs, and why ERPNext holds it,
is stated in `erpnext/principle/business-and-system`.

The rights of a site exist on that site alone, so that they were applied as specified can
be shown nowhere else. What the sandbox shows is that the specified rights produce the
specified reach.

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
  rather than the sandbox company is a defect; the acceptance of a site's own access is the
  one verification made there.
- A business operation performed outside the site's MCP server is a defect unless the active
  procedure explicitly authorizes another path.
- Reviewers apply the native-operation rules to the record each write leaves: its read-back
  and, for a submission, its preview.
- Routine business work performed with administrator credentials is a defect unless the
  active procedure explicitly requires administrator authority.
- A human interruption for a decision the agent could determine from evidence, configuration,
  party preferences, templates, ERPNext state, or standard ERP practice is a defect.
- A submission, cancellation, deletion, or outbound message the agent had not established as
  the expected effect before applying it is a defect.
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

- [erpnext/principle/operational-attention](../principle/operational-attention.md)
- [erpnext/concept/operating-state](../concept/operating-state.md)
- [erpnext/concept/business-relationships](../concept/business-relationships.md)
- [erpnext/procedure/operating-cycle](../procedure/operating-cycle.md)
- general/procedure/ai-to-ai-operations
- [erpnext/concept/operation-model](../concept/operation-model.md)
- [erpnext/concept/multi-tenancy](../concept/multi-tenancy.md)
- [erpnext/principle/determinate-autonomy](../principle/determinate-autonomy.md)
- [erpnext/principle/business-input-traceability](../principle/business-input-traceability.md)
- [erpnext/concept/business-activity-trail](../concept/business-activity-trail.md)
- [erpnext/spec/agent-surface](../spec/agent-surface.md)
- [erpnext/principle/business-and-system](../principle/business-and-system.md)
- [erpnext/spec/owner-access](../spec/owner-access.md)
- [erpnext/policy/documents-convention](documents-convention.md)
