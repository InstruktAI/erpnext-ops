---
description: Managed ERP invocations consume the framework-resolved principal and company context of human-directed or event-started work, map that company to its configured ERP association, and preserve attribution without agent-supplied caller fields.
visibility: internal
delivered_by:
  - erpnext-session-context
---

# Session Operating Context — Spec

## What it is

Managed ERP invocations consume TeleClaude's established execution context through
the platform's published boundary. ERP integration maps that context to native
site, accounting-company, and operator selections. Software-development ownership
and boundary policies govern this composition: identity and shared authorization
remain with their owners; ERP code owns its native mapping and operation semantics.

## Canonical fields

### Mapping and ownership

| Concern | Authority | ERP integration responsibility |
| --- | --- | --- |
| Calling session, its principal and origin | TeleClaude identity and session contracts | Consume the resolved execution identity for human-directed and system organization work alike; add no agent-authored caller flag |
| The session's company context | TeleClaude company resolution: membership, the selection carried through dispatch, and the company an event's source maps to | Consume the resolved company; never derive it from the working directory, the source checkout, or the project a session runs in |
| ERP site and accounting Company | ERP company configuration and native metadata | Map the resolved company to its configured ERP association |
| ERP operator/admin credential selection | ERP operating-context contract | Resolve the operator identity; a company context never selects the dedicated agent-admin identity by itself |
| Business input author and executing agent | Source input and platform execution context | Keep both references distinguishable from the ERP service account |

A managed invocation originates in one of two ways, and the platform establishes
its company context in both. Human-directed work runs under the human principal it
carries, and its company is the one that human's request established: carried
through dispatch and inherited by delegated sessions, or automatic when the human
belongs to exactly one company. Organization work started by an event runs under a
system principal, and its company is the one the event's configured source maps to;
no human identity is attributed to it. Where a session runs, which project it runs
in, and who owns that project neither select nor restrict its company.

ERP integration maps the resolved company to that company's configured ERP
association, which supplies the site, the accounting company, and the identities.
The company context arrives already validated: the platform checks a requested
company against the requester's membership when it resolves the context, and ERP
code neither re-checks, re-derives, nor defaults it. A session with no resolved
company, or with an ambiguous one, remains valid for other work; an ERP operation
from it receives a structured context failure that names how a company is
established for the session.

A session changes company by establishing a different context through the
platform, not per ERP invocation. In a managed session, a selector naming the
session's own company or a site within its association changes nothing; a selector
naming another company, or a site outside that association, is refused with the
same remedy and the operation does not run.

Privilege is a separate axis from company selection. Managed resolution supplies
the operator identity; the dedicated agent-admin identity is requested explicitly
and evaluated through the same context and authorization boundary, under the
privilege separation and refusal rules the standalone operating-context contract
already establishes.

A managed session's requested context governs that session's own invocations. It
never rewrites a selection another session resolves from, in either direction.

A managed invocation
does not fall back to a home-wide persisted selection when its managed context is
missing or inconsistent. Standalone terminal use follows its documented explicit
context contract and is not labelled as a managed session.

Session context is supplied automatically by the system and read through the
platform's published surface; ERP code parses no platform-private state and keeps
no resolver of its own. Shared configuration, authorization, identity,
and error contracts are accessed through their published surfaces. Where the
public surface is insufficient, the gap belongs to that owner rather than a local
replacement policy.

Attribution carries the resolved execution and native record references in the
structured operation result. A persistent business trail claims that attribution
only when the corresponding native recording operation succeeds. No credential or
authentication token is included in those references.

### Use cases

#### UC-SCTX1:

```gherkin
Scenario: A managed agent operates in its established business context
Given an agent session has a framework-resolved principal and a company context mapped to an ERP site and company
When the agent invokes an ERP operation without supplying caller identity
Then the operation uses that configured native context and permitted ERP identity
And the structured result identifies the executing context without credentials
```

#### UC-SCTX2:

```gherkin
Scenario: Concurrent sessions retain their own context
Given two managed agent sessions share an operating-system home and have different established business contexts
When one session changes its requested operating context
And the other session next invokes an ERP operation
Then the other operation retains its own framework-resolved context
```

#### UC-SCTX3:

```gherkin
Scenario: An unresolved managed context is visible
Given a managed agent invocation lacks a resolvable or consistent native mapping for its established context
When the agent attempts an ERP operation
Then it receives a structured context failure before a business request is sent
And no persisted selection from another context substitutes for the missing mapping
```

#### UC-SCTX4:

```gherkin
Scenario: A context selector cannot move a managed session to another company
Given a managed agent session with a resolved company
When the agent names a different company, or a site outside that company's ERP association, for an operation
Then it receives a structured refusal naming how a company is established for a session
And the operation does not run for the named company
```

#### UC-SCTX5:

```gherkin
Scenario: Delegation preserves lineage and distinguishes execution
Given an agent delegates ERP work to a child session under its established lineage and company context
When the child performs a native operation
Then the operation retains that lineage and runs for the company the child inherited
And its execution attribution distinguishes the child from the parent and the ERP service account
```

#### UC-SCTX6:

```gherkin
Scenario: Standalone context selection remains explicit
Given a terminal invocation has no managed agent session
When it uses the documented standalone ERP context selection
Then it operates under that selection's existing identity and privilege contract
And its result does not claim managed-session provenance
```

#### UC-SCTX7:

```gherkin
Scenario: Working location does not replace the established business context
Given a managed agent has an established execution identity and business context
When it invokes ERP operations from two unrelated working directories
Then both invocations resolve the same ERP site, accounting company, permitted identity and lineage
```

#### UC-SCTX8:

```gherkin
Scenario: The session's established company selects the business
Given a managed agent session whose company context resolves to a company with a configured ERP association
When the agent invokes an ERP operation naming no company or site
Then the operation runs against the site and accounting company that association configures
And neither the working location nor the project the session runs in changes that selection
```

#### UC-SCTX9:

#### UC-SCTX10:

```gherkin
Scenario: Managed resolution takes the operator identity and escalation stays explicit
Given a managed agent session whose resolved business has both an operator and a dedicated agent-admin identity configured
When the agent invokes an ERP operation without requesting a privilege
Then the operation runs under the operator identity
And an explicit request for the dedicated agent-admin identity resolves through the same context and authorization boundary
And the bootstrap administrator identity is refused
```

#### UC-SCTX11:

```gherkin
Scenario: A managed session's context request leaves other contexts untouched
Given a managed agent session and a standalone terminal invocation share an operating-system home
When the managed session requests a different company for its own operations
Then that request governs the managed session's subsequent invocations
And the selection the standalone invocation resolves from is unchanged
And a later standalone invocation still resolves the context it selected for itself
```

#### UC-SCTX12:

```gherkin
Scenario: A session without a resolved company is told how to establish one
Given a managed agent session whose company context is absent or ambiguous
When the agent invokes an ERP operation naming no company or site
Then it receives a structured context failure naming how a company is established for the session
And no persisted selection supplies a business in its place
```

#### UC-SCTX13:

```gherkin
Scenario: Event-started organization work runs for its source company without a human identity
Given an organization procedure started by an ERP event under a system principal
And the event's configured source maps to a company with an ERP association
When the procedure's agent invokes an ERP operation
Then the operation runs under that company's operator identity
And its attribution names the system execution and names no human as its origin
```

## Known caveats

- Platform roles and organizational roles remain separate. This feature does not
  introduce a new identity model or reinterpret the platform's standing roles.
- A company context identifies an ERP site only through that company's configured
  ERP association. A company without one carries no ERP business, and an ERP
  operation for it receives the structured context failure.

## See Also

- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
- ~/.teleclaude/docs/erp/spec/company-layout.md
- ~/.teleclaude/docs/software-development/policy/domain-ownership.md
- ~/.teleclaude/docs/software-development/principle/architecture-boundary-purity.md
