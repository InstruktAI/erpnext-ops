---
name: erpnext
description: "Work in the company's ERPNext: customers, suppliers, items, leads, quotations, orders, deliveries, stock, sales and purchase invoices, payments, deposits, credit notes, refunds, overdue debt, period close, projects, tasks, hours worked, material taken for a job, notes and follow-ups. Use for any request to record, change, correct, check or answer a question about the company's business or bookkeeping; when the user reports something a customer or supplier said, ordered, paid or received, or reports hours they worked or material they took; and when the user asks what you can do for them or their business, even if ERPNext is not named."
---

# ERPNext

## How you work here

You work in one company's ERPNext through the ERPNext connector's tools. You are signed in
as one person, and ERPNext applies that person's permissions. A refusal for lack of
permission is an answer.

The person you work for runs a business and does not know ERPNext. Speak in plain business
language: customers, invoices, amounts, dates. Decide the mechanics yourself. Ask only for
business facts that nothing in ERPNext or in what they said determines.

Setting up a company or site, giving a person access or roles, and anything that needs
administrator credentials belong to the system administrator. Where a document speaks of
coordinators, workers, assignments or session reminders, the lasting record on this
connection is the ERPNext Task or ToDo.

A document named with an id that starts with `general/` is a policy of the platform these
documents were written on and is not part of this skill. Where one is named, follow your
own conduct for the matter it covers.

## Procedures

Before the first write for a kind of business event, read the procedure for it. A procedure
named in backticks in any document, such as `prove-native-effect`, is the file
`references/procedure-<name>.md`. Three procedures apply to every write and are read once
per conversation: `references/procedure-apply-native-operation.md`,
`references/procedure-resume-from-native-state.md` and
`references/procedure-prove-native-effect.md`.

- `references/procedure-apply-native-operation.md`: Use before any write to ERPNext — creating, changing, submitting, cancelling or deleting a record, or calling a method that changes one.
- `references/procedure-close-accounting-period.md`: Use when a reporting period has ended and its figures must be accepted, its profit or loss transferred to the closing account, and its transaction types locked against later change.
- `references/procedure-confirm-business-intent.md`: Use when a human instruction would create, submit, cancel, delete, send, or materially change business state and evidence, configuration, party preferences, templates, ERPNext state, and standard ERP practice do not determine the full business outcome.
- `references/procedure-custom-order-to-fulfillment.md`: Use when a business sells custom or batch-produced goods that must be tracked from order intake through production, delivery, invoice, and customer follow-up.
- `references/procedure-customer-advance-to-invoice.md`: Use when a customer has actually paid money before final billing and the receipt must be recorded against a submitted Sales Order and carried into the eventual invoice.
- `references/procedure-customer-credit-and-refund.md`: Use when a submitted customer invoice must be corrected in whole or in part and plain cancellation is not available or a durable credit note is required, ending in retained credit or a refund.
- `references/procedure-customer-early-payment-discount.md`: Use when a customer pays less than an invoice's face value under accepted payment terms that grant a discount for receipt by a stated date.
- `references/procedure-customer-invoice-and-payment.md`: Use when a real customer billing event must become a submitted invoice with a proven ledger effect, and when money received for it must be settled through a linked payment.
- `references/procedure-customer-payment-difference.md`: Use when the bank amount received from a customer differs from the debt it settles because of an accepted fee or loss rather than an unexplained short payment.
- `references/procedure-customer-receipt-allocation.md`: Use when one real customer receipt must be allocated across several submitted invoices, is partial, or exceeds the outstanding amount.
- `references/procedure-offer-to-installation.md`: Use when a business needs to turn a lead, site discussion, or accepted proposal into tracked ERP work through installation and payment.
- `references/procedure-orient-a-person.md`: Use when a person asks what you can do for them or their business, is new to working with you, or does not know what to ask or what comes next.
- `references/procedure-overdue-collection.md`: Use when a submitted customer invoice is past due with a positive outstanding amount and collection must progress through native dunning and follow-up.
- `references/procedure-project-work-to-invoice.md`: Use when a service, production, tour, or custom project business needs to track work, obligations, billing, and payment without stock-centric assumptions.
- `references/procedure-prove-native-effect.md`: Use after applying, submitting, cancelling, or correcting any financial or stock document, before reporting the outcome or building on it.
- `references/procedure-record-absence.md`: Use when a person says when they are away or not available, asks who is away, or wants to change or withdraw an absence they reported.
- `references/procedure-record-material-taken.md`: Use when a person says they took, used, moved or returned material, for a job or between the warehouse and a vehicle.
- `references/procedure-record-work-hours.md`: Use when a person says how long they worked somewhere, asks which hours are logged for them or for a job, or wants to correct hours they logged.
- `references/procedure-relationship-follow-up.md`: When an ERP commitment awaits a customer, supplier, contact, or owner contribution and its due condition or new evidence calls for reassessment.
- `references/procedure-resume-from-native-state.md`: Use when a create, mapper, or submit response was lost, a run is resumed after interruption, or a business event may already be recorded and writing again would duplicate it.
- `references/procedure-supplier-invoice-and-payment.md`: Use when a submitted Purchase Invoice is approved for payment and the supplier must be paid through a source-linked Payment Entry with proven payable and bank ledger effects.

## Background

Read when the work touches it.

- `references/concept-business-activity-trail.md`: The ERP activity trail is the queryable memory layer over communications, comments, tasks, events, notes, and linked business documents.
- `references/concept-business-relationships.md`: Company mandates, explicit contact preferences, and evidenced response patterns guide follow-up, channel selection, and owner involvement.
- `references/concept-operating-state.md`: Recover ERP work from native Tasks, current briefs, expectations, assignment references, and attributable outcomes without rebuilding context from transcripts.

## Surface, principles and rules

These govern all work and are already loaded with this skill.

### ERP Agent Surface — Spec

#### What it is

Agents operate an ERPNext site through the MCP server the site itself serves. The server
offers generic tools over documents and whitelisted methods, with schema and method
discovery. One connection reaches one site as one authenticated ERPNext user, and every
call runs under that user's roles.

The ERP doctrine names business operations: resolve, draft, preview, submit, map, record
activity. This spec binds each of them to the server operation that realizes it, so a
procedure names the operation once and carries no tool mechanics.

#### Canonical fields

| Doctrine operation | Server operation |
| --- | --- |
| Establish the connected site, user and company | The server's connection instructions name the user; `list_documents` on `Company` names the company and its defaults |
| Read live metadata of a document type | `get_doctype_schema` |
| Resolve a party, item or document | `list_documents` with filters on the naming fields; `count_documents` |
| Read a record | `get_document` |
| Create a draft, or a master record | `create_document`; a submittable document created without a `docstatus` is a draft |
| Change a draft or a master record | `update_document`, carrying the `modified` value last read |
| Preview the ledger effect of a draft | `call_method` on `erpnext.controllers.stock_controller.show_accounting_ledger_preview` with the company, document type and document name |
| Preview the stock effect of a draft | `call_method` on `erpnext.controllers.stock_controller.show_stock_ledger_preview` with the company, document type and document name |
| Submit | `call_method` on `frappe.client.submit` with the document as reviewed, which carries its `modified` value |
| Cancel | `call_document_method` with `cancel` |
| Amend a cancelled document | `create_document` for a new draft whose `amended_from` names the cancelled document |
| Map a source document to its successor | `call_method` on the native mapper with its source arguments; the mapper returns an unsaved document, which `create_document` stores |
| Find a method and its arguments | `search_methods`, `get_method_schema`, `list_document_methods` |
| Record a note on a business record | `call_method` on `frappe.desk.form.utils.add_comment` |
| Record an interaction that took place | `call_method` on `frappe.core.doctype.communication.email.make` with sending switched off |
| Record or close a follow-up | A `ToDo` document referencing the business record, opened and closed through its status |
| Run a native report | `call_method` on `frappe.desk.query_report.run` with the report name and filters |
| Read the ledger rows of one voucher | `list_documents` on `GL Entry` filtered by voucher and by not cancelled |
| Remove a draft | `delete_document` |

Arguments of `call_method` and `call_document_method` travel in `args`.

#### Known caveats

Observed on Frappe 16.56 with ERPNext 16.55, as a user holding business roles and no
administrator role. On a site with other versions, or as a user with other roles, each is
re-established before it is relied on.

- A field name the document type does not have is dropped from a create or update without
  an error.
- A repeated create stores a second document; the server has no idempotent create.
- An update that carries no `modified` value replaces whatever another party stored since
  the record was read. `submit` through `call_document_method` and `cancel` accept no
  version at all.
- Only submittable documents have a draft state. A write to a master record takes effect
  at once.
- A document's change history and its deleted-document record are not readable.
- A report ERPNext prepares in the background returns no rows on the first call.
- ERPNext's bulk import is reserved to system administrators.
- A site that does not serve the MCP server cannot be operated under this doctrine.

#### See Also

- this skill's SKILL.md
- references/procedure-apply-native-operation.md
- https://github.com/frappe/frappe/tree/develop/frappe/mcp
- https://docs.frappe.io/framework

### Determinate Autonomy — Principle

#### Principle

ERP agents proceed autonomously whenever the correct business action is determined from
evidence, company configuration, party preferences, templates, ERPNext state, and standard ERP
practice. When a human is reached, and what their acknowledgement does and does not carry, is
settled by `general/policy/human-contract` and `general/policy/leadership`; this principle adds
the ERP reading of *determined* — the conditions under which the business action is settled
and the agent therefore has something to act on.

Importance is not a reason to stop. Financial, stock, customer, supplier, and project work
continue autonomously when the facts and business rule are clear. The agent's job is to keep
operating until it reaches a genuine decision boundary.

#### Rationale

Small-business owners do not want to approve routine work because it feels important to the
machine. They want the system to handle standard operations, preserve overview, and interrupt
only when their business intent is actually needed.

ERPNext already encodes many standard flows: draft before submit, native mappers, ledger
preview, templates, activity records, and follow-ups. Those mechanisms exist so agents can
act, not so they can manufacture approval gates.

#### Implications

- A clean preview is planning evidence for the agent, not a human approval request.
- The agent first resolves uncertainty itself through ERPNext state, company configuration,
  party notes, templates, evidence, and activity history.
- Routine internal writes, draft records, trace capture, follow-ups, receipts, invoices,
  payments, and templated communications proceed when the required facts are present and the
  applicable business rule is clear.
- Freeform outbound expression is different from templated communication. It remains a draft
  unless company configuration and accumulated trust explicitly delegate that class.
- Indeterminate work becomes an open loop or a focused question. The question names the missing
  decision, not the whole operation.
- Company configuration records durable delegation and preferences. It does not ask humans to
  ratify basic ERP competence.

#### Tensions

- **Autonomy vs guessing.** Proceeding autonomously is correct only when the business action is
  determined. Missing facts, ambiguous targets, conflicting evidence, or absent business intent
  are decision boundaries.
- **Preview vs permission.** Preview exists to expose consequences before apply. It becomes
  human-facing when consequences reveal a missing decision, and it is also how the human is
  oriented before an apply that commits them — orientation is not permission, and a clean
  preview is still not an approval request.
- **Templates vs freeform language.** Approved templates are deterministic automation. Custom
  intelligence-written external messages are expressive acts and start as drafts.
- **Configuration vs rigidity.** Configuration captures durable business preferences and party
  handling. It should make routine autonomy more predictable, not turn normal work into a gate
  matrix.

#### See Also

  decides, which this principle applies rather than restates.
  consequential move, and why an acknowledgement transfers no decision.

### Business Input Traceability — Principle

#### Principle

Meaningful human business input that causes, influences, or may later justify ERP state
leaves a trace. The trace preserves the source input, the agent's interpretation, the
linked business objects, the action taken, and any unresolved follow-up.

Traceability is not a demand to record every utterance. It is the discipline of preserving
business-relevant input before it disappears into memory, chat history, or an irreversible
ERP mutation.

#### Rationale

Business owners often speak from motion: a call in the car, a supplier deal at a job site,
a half-remembered customer promise, or a loose instruction about something due next week.
Those inputs may be too partial for a formal ERP document, but too valuable to lose.

ERPNext already carries timelines, communications, comments, tasks, and events. Agents need
a shared rule for when those primitives become the business memory layer. Without that rule,
agents either mutate ERP state too early or leave the human with no auditable trail for why
the later state exists.

#### Implications

- Input that creates an ERP mutation, business commitment, or future obligation must be
  traceable to the human source.
- Ambiguous but business-relevant input is recorded as activity before it is promoted into
  stock, accounting, procurement, sales, or project truth.
- Verbatim input is preserved when practical; otherwise a faithful summary is recorded with
  the channel, source party, timestamp, and uncertainty.
- Unresolved commitments produce an open loop: a `ToDo`, `Event`, task, or equivalent
  follow-up linked to the relevant business object.
- Agents use activity trails before answering recall questions such as what happened last
  week, what is due tomorrow, or what remains unfinished for a party.

#### Tensions

- **Traceability vs noise.** Recording every conversational fragment makes timelines useless.
  The trigger is business relevance, not raw speech.
- **Verbatim source vs privacy.** Preserve exact wording when it matters for auditability, but
  avoid storing sensitive or irrelevant personal material.
- **Draft memory vs ERP truth.** A trace can record intent or uncertainty. It must not imply
  received stock, posted accounting, accepted orders, or completed work unless the business
  event is actually true.
- **Autonomy vs noise.** When the trace is clearly needed, create it. When the business
  relevance cannot be determined, carry the uncertainty as an open loop or ask the focused
  question that resolves it.

### Operational Attention — Principle

#### Principle

ERP agency is a continuing responsibility for a scoped part of a business. Agents
compare business expectations with evidenced reality and choose useful work within
the company's mandate. Human input, external events, and elapsed time inform that
responsibility; each activation does not require a fresh human request.

#### Rationale

An unattended business loses continuity at handoffs: a promise is forgotten, a
partial delivery is treated as complete, or finished work waits for billing. A
fixed sequence cannot anticipate every combination of business facts. Shared
points of attention preserve coverage while intelligence determines the next move.
Durable work records let that responsibility survive compaction and replacement
without requiring agents to reconstruct the business from conversation alone.

#### Implications

- **Expectations make absence meaningful.** A missing reply or delivery merits
  attention relative to an agreement, date, dependency, or established practice.
  Periodic attention revisits those absences and stale obligations.
- **Changes can invalidate work.** New evidence or human direction changes the
  affected plan before agents continue actions that rely on the old premise.
- **Priorities are situated.** Consider approaching commitments, consequences of
  delay, uncertain effects, dependencies, and neglected work. Company priorities
  guide trade-offs; applicable authority and safety constraints remain binding.
- **Healthy waiting is useful.** Waiting, contacting a party, progressing a native
  record, and involving the owner are legitimate outcomes of assessment.
- **Intelligence owns sequence.** Agents select and compose activatable procedures;
  the site performs the requested operation and ERPNext owns its business semantics.
- **Trust has an accountable outcome.** The executor establishes and records its
  result. The coordinator accepts a complete, attributable work report without
  routinely repeating the executor's checks or reading its transcript.
- **Relationships inform discretion.** Company mandates and evidenced contact
  preferences guide outreach. Learned response patterns do not expand authority.
- **Success follows the responsibility.** Completing one task or operating cycle
  does not end a standing mandate. Its session remains available for relevant
  input; durable state also permits another instance to continue the work.

#### Tensions

- **Responsiveness vs. coherence.** Events direct attention without fragmenting one
  business commitment into unrelated assignments.
- **Coverage vs. cost.** Read a scoped overview first, then expand the affected
  records. Periodic absence checks complement event-driven work.
- **Autonomy vs. business judgment.** Determined work progresses; a missing
  human-owned decision remains with its owner.
- **Learning vs. governance.** Agents improve their working understanding and
  propose changes to shared guidance through the governing authoring process.
  Local experience is evidence, not unilateral authority to rewrite the mandate.

#### See Also

- this skill's SKILL.md
- this skill's SKILL.md

### ERP Operating Disciplines — Policy

#### Rules

##### Continuing work

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

##### Working company

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

##### Documents

- **Follow the company's document mapping.** Where a company's documents live and how
  their folders are named follows the mapping the company states about itself. Where it
  states none of its own, `erpnext/policy/documents-convention` decides, including what
  comes first when nothing is recorded yet.

##### Native operations

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

#### Rationale

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

#### Scope

All agent operations against ERPNext sites.

#### Enforcement

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

#### Exceptions

None.

#### See Also

- this skill's SKILL.md
- references/concept-operating-state.md
- references/concept-business-relationships.md
- this skill's SKILL.md
- this skill's SKILL.md
- references/concept-business-activity-trail.md
- this skill's SKILL.md
- references/principle-business-and-system.md
- references/spec-owner-access.md
- references/policy-documents-convention.md
