---
description: Where each kind of ERP knowledge belongs — CLI guarantee, vendor mechanism, business safeguard, company preference, or deployment fact — and what never becomes prose because a live source answers it.
---
# ERP Documentation Authoring — Policy

## Rules

### Classify every addition by its owner before writing it

Each statement an author wants to add is one of five kinds, and the kind decides
where it lives. A statement that mixes kinds is split before it is placed.

| Kind                 | What it is                                                                                           | Where it lives                                                                                                   |
| -------------------- | ---------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| CLI guarantee        | Behavior the `erpnext` binary promises and this project verifies                                     | `erp/spec/cli-surface` as a stated guarantee or limit; its acceptance in a project feature spec                   |
| Vendor mechanism     | How Frappe or ERPNext behaves: lifecycle, validation, accounting, mapping, methods, permissions      | Vendor source and documentation, cited by the procedure that depends on it; never restated as a general law       |
| Business safeguard   | A decision boundary, integrity check, proof, or recovery move that keeps ERP data trustworthy         | An ERP procedure: an atom when several scenarios need it, a playbook step when one scenario does                  |
| Company preference   | Delegation, accounts, templates, tone, party handling for one company                                | The company's configuration and knowledge, never a global snippet                                                |
| Deployment fact      | Installed versions and apps, site provisioning, capability availability on a bench                   | The deployment project that owns the bench; a procedure names the capability it needs, not the version that has it |

### The CLI spec states guarantees and limits, nothing else

- `erp/spec/cli-surface` carries the operating boundary, the command inventory,
  the envelope, the preview contract, and explicit limits of what the CLI does
  not check or promise.
- It never carries how a native mechanism behaves, which argument a vendor
  mapper binds, what a cancellation does to a document's totals, or when an
  agent should use one command over another. Each of those has an owner in the
  table above.
- A guarantee the spec states is one the binary delivers now. Intended behavior
  the binary does not yet deliver is carried only as a pending section under
  the delivering slug, never as live text.
- A universal claim about vendor behavior is not a guarantee and does not enter
  the spec. Vendor behavior is conditional on version, configuration, document
  state, and controller; it is described in the bounded form the consuming
  procedure needs.

### Live native schema is discovered, never catalogued

Field inventories, mandatory flags, link targets, role lists, DocType lists,
configured accounts, and method signatures are read from the site through
`erpnext meta show`, `erpnext doc get`, and vendor source at the moment they
matter. A snippet names the discovery surface and the invariant that depends on
the result; it does not copy the result.

### A vendor trap becomes a bounded safeguard, not a law

A behavior learned by dogfooding enters documentation as the safeguard it
justifies: what to check, what evidence proves it, and what to do when the check
fails. The safeguard sits in the procedure that needs it, bounded to that
scenario, and names the conditions under which it was observed. One successful
sandbox case never becomes a statement about every document, every version, or
every site.

### Shared safeguards are composed, not repeated

A safeguard that several procedures need is written once as an atom that names
its trigger, its invariant, the evidence that satisfies it, and its recovery
boundary. Playbooks compose the atom and keep only their scenario-specific
truth: the postings, eligibility rules, amounts, and postconditions that belong
to that business event. An atom is extracted only when a second consumer
exists; a lone safeguard stays in its procedure.

### Discovery is part of completeness

- A procedure an agent may need without already knowing its name carries an
  `activation_trigger` that states the situation that calls for it, matching the
  body's actual precondition rather than a broader paraphrase.
- A subordinate atom is reached from at least one composing procedure. An atom
  no procedure composes is not complete.
- Cross-cutting rules enter the ERP baseline; scenario knowledge stays
  on-demand behind its trigger.

### Claims are as certain as their evidence

A snippet distinguishes what was observed on a site, what is assumed from
vendor documentation, and what remains unverified. Success reported by a
command, an effect proven by read-back, and an external consequence that no
read-back can prove are stated as three different things.

### References run from documentation to the CLI

Snippets cite stable CLI identifiers: command names, arguments, failure kinds.
The binary carries its own mechanical help and one entry point to the ERP
documentation; it does not embed snippet identifiers per command or a table of
which procedure applies to which command.

### Safety authority is never written away

No procedure or spec text turns a destructive, irreversible, or
security-sensitive action into a routine one by omission. Determinate autonomy
decides when an agent proceeds; the applicable safety gate still applies when it
does.

## Rationale

- The ERP corpus grows through dogfooding, and each discovery arrives as a
  sentence that reads like a fact. Without a placement rule, the sentence lands
  in the most-read document, the CLI spec, where vendor lore buries the contract
  and every agent pays for it on every load.
- Vendor behavior copied into a global snippet is a second source of truth
  with a shelf life. Frappe and ERPNext change per release and per site
  configuration, and a copied universal is wrong precisely when it is relied on.
- A spec that promises what the binary does not do trains agents to trust
  checks that never ran. Pending sections keep intended behavior visible
  without making it a lie.
- Safeguards repeated across procedures drift apart one edit at a time, and the
  weakest copy is the one an agent happens to load.
- A procedure with no trigger is invisible to the agent who needs it most, the
  one who does not yet know it exists.
- This policy specializes `general/policy/snippet-value-contract` for the ERP
  domain; it adds the ownership classes and the discovery rule that the general
  contract leaves to each domain, and repeats nothing from it.

## Scope

- Every snippet under the ERP domain and every project snippet of an ERP
  operating layer: specs, procedures, concepts, principles, policies, and
  feature specs.
- Every author and reviewer of those snippets, in any session.

## Enforcement

- A reviewer applying the doc-hygiene lane to an ERP snippet applies this
  policy to the changed regions and frontmatter. A statement in the wrong owner's
  home is a finding; a universal vendor claim is a finding; a guarantee the
  binary does not deliver outside a pending section is a finding; a copied
  schema value is a finding.
- A new or renamed procedure without an `activation_trigger` when it is
  independently activatable, or an atom with no composing consumer, is a
  finding.
- Whether a sentence is a misplaced fact or a needed safeguard is reviewer
  judgment. No checker grades snippet text.

## Exceptions

- None.

## See Also

- ~/.teleclaude/docs/general/policy/snippet-value-contract.md — the general value test this policy specializes.
- ~/.teleclaude/docs/erp/concept/composable-business-procedures.md — the atom and playbook shapes this policy places knowledge into.
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md — the operating rules agents follow; this policy governs how those rules are written down.
