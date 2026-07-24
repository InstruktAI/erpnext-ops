---
id: project/principle/generic-cli-substrate
type: principle
domain: project
scope: project
description: Keep the erpnext CLI generic over native Frappe and ERPNext mechanisms; express business scenarios by composing those primitives in procedures.
---
# Generic CLI Substrate — Principle

## Principle

The `erpnext` CLI exposes reusable native Frappe and ERPNext mechanisms. Business scenarios
compose those mechanisms through ERP procedures; they do not become scenario-specific CLI
commands or local reimplementations of ERPNext behavior.

## Rationale

ERPNext spans many industries and workflows while presenting a small set of recurring
technical mechanisms: documents and metadata, lifecycle and controller methods, native
mappers, module-level methods, reports, imports, activity records, evidence, and diagnostics.
Encoding each business outcome as a command would create an unbounded surface that duplicates
vendor knowledge and must change whenever ERPNext changes.

Agents already understand the business domain and can read the live site's metadata and native
state. The CLI earns its place by making those generic mechanisms safe and consistent, while
procedures preserve the business safeguards learned through dogfooding.

## Implications

- A CLI addition names and exposes a reusable native mechanism, not an industry, document
  journey, or business outcome.
- A business scenario that cannot be completed through the current CLI first identifies the
  missing Frappe or ERPNext mechanism. The implementation extends that mechanism once; the
  scenario remains its acceptance proof.
- Scenario-specific nouns and decisions stay in ERP procedures. Rust code does not decide how
  to sell, purchase, install, collect, reconcile, maintain, or close a business process.
- The CLI owns context, authentication, secret redaction, preview/apply behavior, stable output,
  basic live-metadata preflight, and honest verification of the native result.
- ERPNext owns permissions, validation, transactions, accounting, stock, workflow, mapping,
  reporting, and every other vendor business semantic.
- Prepare and review reject a proposed command whose shape is coupled to one DocType or workflow
  when a parameterized native mechanism can express it.
- Dogfooding may reveal a missing primitive. It never justifies wrapping the triggering use case
  as a bespoke command.

## Tensions

- **Genericity vs convenience.** A dedicated command can look simpler for one workflow, but
  transfers vendor semantics into this repository and multiplies the long-term surface.
- **Thinness vs safety.** Generic does not mean raw transport. The shared context, preview,
  redaction, failure, and result-verification contracts remain mandatory.
- **Reuse vs premature abstraction.** A new primitive follows an observed native API category,
  not an imagined future framework. The first business scenario proves the need; the vendor
  mechanism determines the abstraction.
