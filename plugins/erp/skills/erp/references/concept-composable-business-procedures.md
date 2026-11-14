# Composable Business Procedures — Concept

## What

ERP procedures describe the business safeguards that matter around ERPNext operations. They
are not tutorials for every button, field, or tool call. Agents work the mechanics through
the site's own operations (`erp/spec/agent-surface`); procedures
exist where business meaning, integrity, continuity, or decision boundaries are at stake.

The catalog has two useful procedure shapes:

- **Atoms.** Small reusable procedures for cross-cutting business concerns, such as capturing
  business input, confirming a consequential human instruction, preserving follow-up,
  recording received goods, or posting a customer payment.
- **Playbooks.** Larger procedures for common business outcomes, composed from atoms and
  native operations, such as solar offer-to-installation, custom order-to-fulfillment, or project
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


A procedure is not necessarily an agent command. General-purpose session briefs
can activate procedures without a dedicated command wrapper or sdlc
state machine. The agent interprets the procedure against live facts and chooses
individual native operations. One operation expresses one change; it never owns the
business sequence.

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
Everything else remains flexible and grounded in the live ERPNext instance through the
draft, preview, and apply loop.

## See Also

- this skill's SKILL.md
- references/concept-business-activity-trail.md
- this skill's SKILL.md
- references/procedure-confirm-business-intent.md
- this skill's SKILL.md
