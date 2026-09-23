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
Encoding each business journey as a command would create an unbounded surface that
duplicates vendor decisions and must change whenever ERPNext changes. A native
operation, such as recording a task's completion, is one step selected by the
agent; deciding which business actions satisfy that task remains with the agent.

Agents already understand the business domain and can read the live site's metadata and native
state. The CLI earns its place by making those generic mechanisms safe and consistent, while
procedures preserve the business safeguards learned through dogfooding.

Concrete caller-side translation can justify a concise operation name even when
generic document access can express the request. The evidence is the existing
invocation and its native mapping, not predicted popularity. Native task identity
and state remain authoritative, so useful command ergonomics do not require a
second work store or scheduler. The value of a semantic command is removing that
demonstrated translation while leaving the same decisions with their owners.

## Implications

- A CLI addition names one reusable native operation. It does not choose a
  business outcome or execute an agent's plan.
- A business scenario that cannot be completed through the current CLI first identifies the
  missing Frappe or ERPNext mechanism. The implementation extends that mechanism once; the
  scenario remains its acceptance proof.
- Scenario-specific nouns and decisions stay in ERP procedures. Rust code does not decide how
  to sell, purchase, install, collect, reconcile, maintain, or close a business process.
- The CLI owns context, authentication, secret redaction, preview/apply behavior, stable output,
  basic live-metadata preflight, and honest verification of the native result.
- ERPNext owns permissions, validation, transactions, accounting, stock, workflow, mapping,
  reporting, and every other vendor business semantic.
- A semantic command proposal supplies the concrete generic invocation or native
  interaction it simplifies, the caller-side translation it removes, and the
  verified native operation that owns the result. Its inputs name the target and
  requested change or query; its output reports that operation's native result.
  These facts are evaluated before implementation, not inferred from hoped-for use.
- Prepare and review reject a semantic command that chooses the next business
  action, sequences a business journey, reproduces vendor validation or accounting
  decisions, or creates competing business state. A DocType-specific spelling is
  permitted when the proposal meets the single-operation and evidence criteria;
  generic expressibility alone neither rejects nor justifies it. Without a
  demonstrated caller benefit, the existing generic surface remains sufficient.
- Internal requests needed to implement one operation are implementation details,
  not authority to bundle a business workflow. Partial effects are reported
  honestly; a sequence of remote calls is not implicitly atomic.
- Runtime identity is system-provided context, not an argument the agent must
  reconstruct. A command claims provenance persistence only when its implemented
  recording surface actually supplies it.
- Shared identity, authorization, configuration, session lifecycle, and event
  delivery are consumed through their owning platform contracts. ERP code owns
  the mapping from that resolved context into native ERP records and operations;
  it does not implement a parallel resolver or read another context's private
  storage as an integration API. Missing integration is explicit rather than
  silently replaced with local policy.
- Dogfooding may reveal a missing primitive. It never justifies wrapping the triggering use case
  as a bespoke command.

## Tensions

- **Genericity vs convenience.** A semantic operation can remove demonstrated
  translation cost; every added command also creates a maintenance contract.
  Review weighs that concrete benefit against the added surface and rejects
  convenience that transfers business decisions from the agent or ERPNext.
- **Thinness vs safety.** Generic does not mean raw transport. The shared context, preview,
  redaction, failure, and result-verification contracts remain mandatory.
- **Reuse vs premature abstraction.** A new primitive follows an observed native API category,
  not an imagined future framework. The first business scenario proves the need; the vendor
  mechanism determines the abstraction.

## See Also

- ~/.teleclaude/docs/software-development/policy/domain-ownership.md
- ~/.teleclaude/docs/software-development/principle/architecture-boundary-purity.md
