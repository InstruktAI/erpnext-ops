---
description: This repository authors tests. Spec-bound behavior is proven by the compiled erpnext binary against a mocked site; the corpus is kept and grown under the global and Rust testing policies.
---

# Test Authorship — Policy

## Rules

- **This repository opts into test authorship.** The corpus under `tests/` is
  kept and grown. Every mandate of `software-development/policy/testing` and
  `software-development/policy/rust/testing` governs here: scenario binding,
  the functional level, file organization, coverage against the scenario
  inventory, and assertion craft.
- **The compiled binary against a mocked site is the verification surface.**
  A spec-bound test spawns the real `erpnext` binary from an isolated
  credential home and drives it against an in-process HTTP mock of the Frappe
  site. HTTP is the only faked line; everything inside the process runs for
  real. Each such test binds one feature-spec scenario with a `// spec:`
  comment naming the spec id and use-case id.
- **No test contacts a tenant site.** A test that reaches a configured
  ERPNext endpoint, reads live credentials, or leaves a business record behind
  is a defect regardless of what it asserts.
- **Colocated units stay in their lane.** `#[cfg(test)]` modules over pure
  functions are permitted by the Rust testing policy, are not spec-bound, and
  never stand in for the compiled-binary binding of a scenario.
- **The `test` checkpoint action is bound.** This repository's `teleclaude.yml`
  binds the platform `test` action to its source and test trees, so a turn
  that changes them owes a test run before it closes.

## Rationale

The global default forbids authorship because a check written by the session
that wrote the code supplies both the work and the standard. Here the standard
comes from outside the session: scenarios are authored at prepare under
authorized intent and pass independent review before a build binds them, and
the test drives the real binary, which cannot patch the code it exercises.
Rust's compiler and type system carry unit-level correctness, which is why the
corpus needs no mocks of in-process collaborators.

ERPNext is a financial system of record and the only live surface is a tenant
site. A per-change live probe would mutate business records to prove a CLI
change, so the mocked-site run is the verification that is both independent of
the author and safe to repeat.

## Scope

This repository only.

## Enforcement

- Reviewers apply the tests lane and every `TQ-*` predicate of
  `software-development/policy/quality-predicate-registry` in full.
- A test that enters through an internal symbol, fakes an in-process
  collaborator, or asserts string content by literal is a finding under the
  testing policy.
- A test that contacts a tenant site is a blocking finding.
- A plan that delivers a behavioral delta without naming the compiled-binary
  test that binds its scenario is not plan-complete.

## Exceptions

- None.

## See Also

- ~/.teleclaude/docs/software-development/policy/testing.md — the opt-in default this policy declares against, and the doctrine it adopts.
- ~/.teleclaude/docs/software-development/policy/rust/testing.md — the language idioms, colocated-unit exception, and integration-test placement.
- ~/.teleclaude/docs/software-development/concept/functional-test-boundary.md — what counts as the real surface and the only legitimate fakes.
