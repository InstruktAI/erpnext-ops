---
description: Standing ERP responsibility, coordinator and worker scopes, activation, and the boundary between agent judgment, TeleClaude execution, and native ERPNext operations.
---

# ERP Operation Model — Concept

## What

### Responsibility and execution

An ERP coordinator holds the overview for an agreed operating context and is the
human's continuing point of contact. The context names the site, company scope,
standing responsibilities, and company mandate. This is a business responsibility,
not acquisition of TeleClaude's platform fleet-coordinator role.

The coordinator maintains native work records, reconciles new input, and gives
workers coherent outcomes with a small focus. Workers select the knowledge and
actions needed within that scope. The activatable-procedure catalogue supplies
descriptions and triggers for both dispatch and self-selection; a dispatcher need
not load a procedure body unless its selection or scope is unclear. The executor
loads the chosen procedure and its required reads.

Agents keep long-running sessions while their responsibilities continue. A wake,
restart, or compaction resumes from the current work record and relevant changes.
Session context makes continuation cheaper; it does not replace durable state.
Completing a turn, completing an assignment, and completing a standing mandate are
distinct events.

### Activation and scale

Human input, delivered business events, due expectations, and worker outcomes
direct attention to existing work. They are context for judgment, not a fixed
business state machine. An event concerning work with an owner reaches that owner
through the configured route rather than automatically creating a new worker.

Concurrency follows independent actionable work and available capacity. The
coordinator considers shared records and business dependencies before dispatch.
An available worker can receive another bounded assignment; a worker awaiting a
response or child outcome still holds work. Inactivity prompts a responsibility
check, not automatic termination. Closure follows settlement or explicit transfer
of outstanding work, children, and waiting obligations under the session contract.

### Platform composition

TeleClaude supplies sessions, dispatch, subscriptions, history, reminders,
scheduled jobs, identity, and configured messaging and event delivery. Their
published contracts own the mechanics:

- Session reminders carry future attention into an existing session. Scheduled
  jobs execute their declared procedure and retain their own run-report contract;
  they are not assumed to reuse the standing coordinator session.
- A periodic assignment inspects unmet expectations and stale work. Its configured
  scope determines whether it resolves them or hands them to the standing owner.
- A child turn notification prompts inspection of its work record. A recorded
  outcome can settle the assignment without routine transcript tailing; a turn
  notification alone is not a completed business outcome.
- Platform identity comes from the system's session context. Agents do not invent
  or pass their own identity as authentication. ERP operator identity, executing
  session, and the human or external source of input remain distinct facts.
- External events and outbound channels require configured integrations and
  addressable recipients. A platform capability does not establish that a site's
  event route or a contact's messaging channel is configured.

### ERPNext operating surface

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
- **Work state.** Native Tasks carry delegated outcomes; their relationships and
  available views provide the work overview. Native activity and follow-up records
  carry the business trail. A second board is not the operating authority.
- **Business flow.** Agents compose individual CLI operations and ERP procedures.
  Agent briefs orient responsibility; procedures supply applicable knowledge;
  CLI commands perform operations. The agent determines their order.
- **Outward integration.** Reflecting ERP state to an external system is the job of an
  adapter — one kind of company-coupled app in the company `apps/` home
  (see `general/concept/company-knowledge`), running outside the tenant site. ERP is the
  system of record; projections flow one way, never a bidirectional sync into the site.

## Why

One continuing coordinator gives the human an overview while focused workers
expand execution capacity. Catalogue-driven delegation keeps startup context
small and lets the same procedure serve a dispatched worker or an agent selecting
its own next work. Keeping work in ERPNext avoids competing task histories.

Per-tenant sites keep operations lightweight while making tenant isolation structural in the
system of record. ERPNext is dynamic: installed apps, custom fields, workflows, and
permissions change the live shape of each site. The CLI absorbs that looseness into a safe
operating surface so agents can rely on preview, metadata preflight, context resolution,
activity capture, and structured failures instead of carrying those mechanics in every
procedure.

## See Also

- ~/.teleclaude/docs/erp/principle/operational-attention.md
- ~/.teleclaude/docs/erp/concept/operating-state.md
- ~/.teleclaude/docs/erp/concept/business-relationships.md
- ~/.teleclaude/docs/erp/procedure/operating-cycle.md
- ~/.teleclaude/docs/general/procedure/ai-to-ai-operations.md
- ~/.teleclaude/docs/general/procedure/job-authoring.md
- ~/.teleclaude/docs/general/policy/reminder.md
- ~/.teleclaude/docs/general/principle/session-lifecycle.md
- ~/.teleclaude/docs/general/concept/company-knowledge.md
- ~/.teleclaude/docs/general/spec/tools/telec-cli.md
- ~/.teleclaude/docs/erp/concept/multi-tenancy.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/concept/composable-business-procedures.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/spec/company-layout.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
