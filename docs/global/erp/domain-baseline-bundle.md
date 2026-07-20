# erp — Platform Baseline

# Determinate Autonomy — Principle

## Principle

ERP agents proceed autonomously whenever the correct business action is determined from
evidence, company configuration, party preferences, templates, ERPNext state, and standard ERP
practice. They surface work to a human only when the human holds missing decision input that
the system cannot infer or obtain itself.

Importance is not a reason to stop. Financial, stock, customer, supplier, and project work
continue autonomously when the facts and business rule are clear. The agent's job is to keep
operating until it reaches a genuine decision boundary.

## Rationale

Small-business owners do not want to approve routine work because it feels important to the
machine. They want the system to handle standard operations, preserve overview, and interrupt
only when their business intent is actually needed.

ERPNext already encodes many standard flows. The `erpnext` CLI adds context resolution,
metadata preflight, preview/apply, activity capture, templates, and open-loop tracking. Those
mechanisms exist so agents can act, not so they can manufacture approval gates.

## Implications

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

## Tensions

- **Autonomy vs guessing.** Proceeding autonomously is correct only when the business action is
  determined. Missing facts, ambiguous targets, conflicting evidence, or absent business intent
  are decision boundaries.
- **Preview vs permission.** Preview exists to expose consequences before apply. It becomes
  human-facing only when consequences reveal a missing decision.
- **Templates vs freeform language.** Approved templates are deterministic automation. Custom
  intelligence-written external messages are expressive acts and start as drafts.
- **Configuration vs rigidity.** Configuration captures durable business preferences and party
  handling. It should make routine autonomy more predictable, not turn normal work into a gate
  matrix.

---

# Business Input Traceability — Principle

## Principle

Meaningful human business input that causes, influences, or may later justify ERP state
leaves a trace. The trace preserves the source input, the agent's interpretation, the
linked business objects, the action taken, and any unresolved follow-up.

Traceability is not a demand to record every utterance. It is the discipline of preserving
business-relevant input before it disappears into memory, chat history, or an irreversible
ERP mutation.

## Rationale

Business owners often speak from motion: a call in the car, a supplier deal at a job site,
a half-remembered customer promise, or a loose instruction about something due next week.
Those inputs may be too partial for a formal ERP document, but too valuable to lose.

ERPNext already carries timelines, communications, comments, tasks, and events. Agents need
a shared rule for when those primitives become the business memory layer. Without that rule,
agents either mutate ERP state too early or leave the human with no auditable trail for why
the later state exists.

## Implications

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

## Tensions

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

---

# ERP Operation Model — Concept

## What

Agents operate ERPNext through the `erpnext` CLI against per-tenant Frappe sites. The site
is the tenant boundary; companies inside a site are legal and accounting scopes. itsUP owns
site provisioning; the CLI operates an already-provisioned site.

- **Operating surface.** The `erpnext` CLI is the agent-facing surface for context, metadata,
  resolution, document operations, workflows, activity trails, evidence, reports, imports, and
  operational checks. Agents do not operate ERPNext by composing raw HTTP calls.
- **Substrate.** The CLI wraps Frappe's v2 REST abstraction. That substrate explains the generic
  document model, but it is not the normal instruction surface for agents.
- **Truth at runtime.** Field lists, mandatory flags, link targets, child tables, and custom
  fields are read through the CLI from the live site. Memory supplies the pattern; the site
  supplies the exact contract.
- **Preview by default.** Mutating CLI commands show the exact context and native operation
  that would be sent before anything is sent. Applying sends that operation and reports
  ERPNext's result. The CLI does not simulate ERPNext.
- **Identity.** Site administrative setup uses the site's administrator identity. Routine
  business work uses the site's operator identity. Person access uses ERPNext users
  permission-scoped on the site.
- **Business flow.** Business outcomes are achieved by composing CLI primitives and ERP
  procedures. The common selling lane is Lead/Customer -> Quotation -> Sales Order ->
  Sales Invoice -> Payment Entry.

## Why

Per-tenant sites keep operations lightweight while making tenant isolation structural in the
system of record. ERPNext is dynamic: installed apps, custom fields, workflows, and
permissions change the live shape of each site. The CLI absorbs that looseness into a safe
operating surface so agents can rely on preview, metadata preflight, context resolution,
activity capture, and structured failures instead of carrying those mechanics in every
procedure.

## See Also

- ~/.teleclaude/docs/erp/concept/multi-tenancy.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/concept/composable-business-procedures.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/spec/company-layout.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md

---

# ERP Multi-Tenancy — Concept

## What

Each tenant runs on its own Frappe site: a dedicated FQDN and a dedicated logical database
with its own scoped database user on the shared database server. One bench — one deployment
of the app processes and one MariaDB service — hosts many sites; Frappe resolves the site
from the request's Host header. itsUP provisions sites (database, files, routing, DNS)
through GitOps.

The site is the isolation boundary. An ERPNext `Company` inside a site is a legal and
accounting scope: a business group may operate several companies in one site with
consolidated accounting and shared masters. Company is never the tenancy mechanism.

Identities are site-scoped:

- **Site administrator** — bootstrap and administrative setup on one site: creating
  companies, users, and scoped credentials. Administrator credentials never leave the
  site's administrative context.
- **Site operator agent** — an ERPNext user plus API credentials on the site, used for
  routine business operations with the roles it needs and never administrator authority.
- **Human users** — ERPNext users on the site, permission-scoped to the roles and data they
  need.

## Why

Company-based isolation inside one shared site rides on opt-in permission filtering, leaks
through shared masters, reports, and link searches, and needs continuous behavioral proof to
trust. Site-per-tenant makes isolation structural: a site's database connection can only
reach its own schema, so no ERPNext permission misconfiguration can cross tenants, and a
leaked operator secret exposes one site rather than the fleet. The marginal cost of a tenant
is a schema, a site entry, and a DNS route — no additional containers. This mirrors the
vendor's own production architecture: Frappe Cloud hosts one site per tenant on shared
benches backed by MariaDB.

## See Also

- ~/.teleclaude/docs/erp/procedure/company-onboarding.md
- ~/.teleclaude/docs/erp/procedure/person-onboarding.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md

---

# Business Activity Trail — Concept

## What

The business activity trail is the queryable memory layer over human input, agent actions,
follow-ups, and linked ERP documents. Agents access it through `erpnext activity` and related
CLI commands. It lets agents answer questions such as what happened last week, what is due
tomorrow, what was said about a customer, or why a supplier commitment exists.

The trail is assembled behind the CLI from ERPNext and Frappe primitives:

- `Communication` records human interactions and messages with medium, sender, content,
  date, and optional document references.
- `Comment` records timeline notes attached to any referenced document.
- `ToDo` records open loops, due dates, assignees, and referenced documents.
- `Event` records calendar-style calls, meetings, reminders, and participants.
- `CRM Note` records notes inside Leads and Opportunities.
- Business documents such as Lead, Opportunity, Customer, Supplier, Quotation, Sales Order,
  Purchase Order, Project, Task, Invoice, and Payment Entry provide the objects the activity
  links to.

## Why

Humans do not keep a perfect operational overview. They trust the system to remember what was
said, what was planned, what is still open, and what should happen next. The `erpnext`
activity surface gives agents the cheap recall path needed to support that trust without
loading every ERP document in full.

The trail also separates memory from mutation. A verbal supplier promise can be captured as
activity and follow-up before it becomes a Purchase Order or received stock. A customer call
can be recorded before it becomes an Opportunity, Quotation, or support Issue. The activity
trail keeps business context visible while the stronger ERP documents remain reserved for
confirmed business state.

## See Also

- ~/.teleclaude/docs/erp/concept/composable-business-procedures.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md

---

# Composable Business Procedures — Concept

## What

ERP procedures describe the business safeguards that matter around ERPNext operations. They
are not tutorials for every button, field, or command. Agents use the `erpnext` CLI for
mechanical operation, preview, metadata preflight, trace capture, and application; procedures
exist where business meaning, integrity, continuity, or decision boundaries are at stake.

The catalog has two useful procedure shapes:

- **Atoms.** Small reusable procedures for cross-cutting business concerns, such as capturing
  business input, preserving follow-up, recording received goods, or posting a customer
  payment.
- **Playbooks.** Larger procedures for common business outcomes, composed from atoms and
  CLI operations, such as solar offer-to-installation, custom order-to-fulfillment, or project
  work-to-invoice.

## Why

Agents already carry broad ERP and business knowledge. Over-documenting small mechanical
steps makes agents rigid and can steer them away from better judgment. Under-documenting
business safeguards loses the practices that keep ERP data trustworthy.

Composable procedures keep the useful middle: they name the moments where agents must
preserve intent, avoid premature state changes, create follow-up, or identify the missing
decision input that blocks autonomous continuation.
Everything else remains flexible and grounded in the live ERPNext instance through the CLI
preview/apply loop.

## See Also

- ~/.teleclaude/docs/erp/principle/business-input-traceability.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md

---

# ERP Operating Disciplines — Policy

## Rules

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
- **Keep secrets out of output.** API keys, secrets, and login links are written only to their
  credential destination or sent through the documented ERPNext channel.

## Rationale

ERPNext is a financial system of record. A wrong invoice, duplicated record, over-broad user,
or leaked operator secret has operational and legal consequences. The CLI is the safety layer
that lets agents keep acting: it shows the exact operation before mutation, resolves routine
mechanics, preserves traces, and surfaces only real decision boundaries.

## Scope

All agent operations against ERPNext sites.

## Enforcement

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

## Exceptions

None.

## See Also

- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/erp/concept/multi-tenancy.md
- ~/.teleclaude/docs/erp/principle/determinate-autonomy.md
- ~/.teleclaude/docs/erp/principle/business-input-traceability.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md

---

## Baseline index — load via `telec docs get <id>` when relevant

- `erp/spec/cli-surface` — Agent-facing erpnext CLI command surface for operating per-tenant ERPNext sites.

## Activatable Procedures — load via `telec docs get <id>` before acting

- `erp/procedure/company-onboarding` — Use when a business, legal company, or operating entity must become an operable tenant with its own ERPNext site.
- `erp/procedure/custom-order-to-fulfillment` — Use when a business sells custom or batch-produced goods that must be tracked from order intake through production, delivery, invoice, and customer follow-up.
- `erp/procedure/offer-to-installation` — Use when a business needs to turn a lead, site discussion, or accepted proposal into tracked ERP work through installation and payment.
- `erp/procedure/person-onboarding` — Use when a human user needs ERPNext login access, roles, and permissions on a company's site.
- `erp/procedure/project-work-to-invoice` — Use when a service, production, tour, or custom project business needs to track work, obligations, billing, and payment without stock-centric assumptions.
