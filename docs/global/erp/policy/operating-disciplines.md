---
description: Disciplines for agents acting on ERPNext site data through the erpnext CLI.
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
- **Establish the way back before relying on it.** Reversal is not one mechanism: a draft
  edit is inverted from its version history, a deletion is restored, a submitted document is
  cancelled and amended, a link or allocation is undone by its own native inverse, and an
  external effect is compensated, never recalled. Before applying a mutation whose correctness
  is not yet determined, the agent reads the target's submittability and `docstatus` from live
  metadata, names the mechanism that would correct it, the dependencies that mechanism must
  clear, and the read-back that proves the correction, and treats a mutation with no such
  mechanism as irreversible under the applicable safety gate.
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
- A mutation applied while its correctness was undetermined, without a named correction
  mechanism and read-back, is a defect; one with no such mechanism is irreversible and falls
  under the applicable safety gate.

## Exceptions

None.

## See Also

- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/erp/concept/multi-tenancy.md
- ~/.teleclaude/docs/erp/principle/determinate-autonomy.md
- ~/.teleclaude/docs/erp/principle/business-input-traceability.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
