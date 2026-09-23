---
description: Expose reusable native ERPNext operations with a concise semantic surface and generic document access; agents compose business workflows and ERPNext owns their semantics.
---
# Generic CLI Substrate — Principle

## Principle

The `erpnext` CLI exposes reusable native Frappe and ERPNext operations. A command
expresses one operation in useful domain language; agents choose the operations
and their order through ERP procedures. Generic document and method access covers
the wider native surface. Neither form reimplements ERPNext business behavior.

## Rationale

ERPNext spans many industries and workflows while presenting a small set of recurring
technical mechanisms: documents and metadata, lifecycle and controller methods, native
mappers, module-level methods, reports, imports, activity records, evidence, and diagnostics.
Encoding each business outcome as a command would create an unbounded surface that duplicates
vendor knowledge and must change whenever ERPNext changes.

Agents already understand the business domain and can read the live site's metadata and native
state. The CLI earns its place by making those generic mechanisms safe and consistent, while
procedures preserve the business safeguards learned through dogfooding.

Repeated work-management operations benefit from concise names that remove
mechanical translation without moving judgment into code. Native task identity
and state remain authoritative, so useful command ergonomics do not require a
second work store or scheduler.

## Implications

- A CLI addition names one reusable native operation, including a stable
  work-management operation where a semantic command reduces repeated mechanical
  translation. It does not choose a business outcome or execute an agent's plan.
- A business scenario that cannot be completed through the current CLI first identifies the
  missing Frappe or ERPNext mechanism. The implementation extends that mechanism once; the
  scenario remains its acceptance proof.
- Scenario-specific nouns and decisions stay in ERP procedures. Rust code does not decide how
  to sell, purchase, install, collect, reconcile, maintain, or close a business process.
- The CLI owns context, authentication, secret redaction, preview/apply behavior, stable output,
  basic live-metadata preflight, and honest verification of the native result.
- ERPNext owns permissions, validation, transactions, accounting, stock, workflow, mapping,
  reporting, and every other vendor business semantic.
- Prepare and review distinguish a useful semantic operation from a bespoke
  scenario wrapper. A native DocType-specific command can earn its place through
  repeated use and stable meaning; vendor validation and native state remain with
  ERPNext. Generic access remains available for the long tail.
- Internal requests needed to implement one operation are implementation details,
  not authority to bundle a business workflow. Partial effects are reported
  honestly; a sequence of remote calls is not implicitly atomic.
- Runtime identity is system-provided context, not an argument the agent must
  reconstruct. A command claims provenance persistence only when its implemented
  recording surface actually supplies it.
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
