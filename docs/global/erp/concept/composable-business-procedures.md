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
- ~/.teleclaude/docs/erp/procedure/confirm-business-intent.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
