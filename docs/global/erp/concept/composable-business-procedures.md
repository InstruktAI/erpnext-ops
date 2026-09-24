---
description: ERP procedures are composable business safeguards, not replacements for agent intelligence or CLI metadata.
---
# Composable Business Procedures — Concept

## What

ERP procedures describe the business safeguards that matter around ERPNext operations. They
are not tutorials for every button, field, or command. Agents use the `erpnext` CLI for
mechanical operation, preview, metadata preflight, trace capture, and application; procedures
exist where business meaning, integrity, continuity, or decision boundaries are at stake.

The catalog has two useful procedure shapes:

- **Atoms.** Small reusable procedures for cross-cutting business concerns, such as capturing
  business input, confirming a consequential human instruction, preserving follow-up,
  recording received goods, or posting a customer payment.
- **Playbooks.** Larger procedures for common business outcomes, composed from atoms and
  CLI operations, such as solar offer-to-installation, custom order-to-fulfillment, or project
  work-to-invoice.

The catalogue is also the delegation surface. A description states the outcome
and scope; an activation trigger states when the procedure applies. A coordinator
selects from this metadata and dispatches the canonical snippet ID with the work
reference, expected outcome, and relevant constraints. It uses the catalogue's
`telec docs get <id> --closure` instruction rather than reproducing the body in
the brief. The worker reads that closure before acting. An agent selecting work
within its own mandate enters through the same procedure.

Catalogue selection does not supersede configured activation ownership. A worker
can load shared procedural knowledge for its assignment without independently
starting the job or event-driven responsibility that also uses that knowledge.

<!-- planned:erpnext-business-event-mapping -->
For automatically activated ERP procedures, event subscriptions and
`activation_trigger` are mutually exclusive metadata. The platform starts or
wakes the lineage-scoped group singleton for matching events. The self-service
catalogue excludes those subscribed procedures; supporting procedures may have
neither activation form. Event data carries the work context into execution,
and the executor loads the procedure closure before acting.
<!-- /planned:erpnext-business-event-mapping -->

A procedure is not necessarily an agent command. General-purpose session briefs
can activate procedures without a dedicated command wrapper or sdlc
state machine. The agent interprets the procedure against live facts and chooses
individual CLI operations. A CLI command expresses one operation; implementation
requests within that operation do not make it the owner of the business sequence.

ERP domain entry loads its baseline once. Its progressive index supplies the
remaining concepts and references; its activatable catalogue supplies procedure
descriptions and triggers. Executors expand only the knowledge their work needs.

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

- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/erp/procedure/operating-cycle.md
- ~/.teleclaude/docs/general/procedure/ai-to-ai-operations.md
- ~/.teleclaude/docs/erp/principle/business-input-traceability.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/procedure/confirm-business-intent.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
