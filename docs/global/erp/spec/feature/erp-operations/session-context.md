---
description: Managed ERP invocations consume framework-resolved identity and operating scope, map them to configured ERP identities, and preserve attribution without agent-supplied caller fields.
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
| Calling session and inherited principal | TeleClaude identity and session contracts | Consume the resolved execution identity; do not add an agent-authored caller flag |
| Company membership, project assignment and coverage | TeleClaude organizational relationship resolution | Consume the distinct facts without inventing a default organization or equating membership with every operation's permission |
| Operating project and its declared owner | TeleClaude context/configuration contracts | Use the established scope rather than infer it from a filesystem name |
| ERP site and accounting Company | ERP company configuration and native metadata | Resolve the explicit association to the platform operating scope |
| ERP operator/admin credential selection | ERP operating-context contract | Preserve privilege separation and native permissions |
| Business input author and executing agent | Source input and platform execution context | Keep both references distinguishable from the ERP service account |

Explicit company/site selectors express a requested ERP context through the same
mapping contract. They do not replace framework resolution. A managed invocation
does not fall back to a home-wide persisted selection when its managed context is
missing or inconsistent. Standalone terminal use follows its documented explicit
context contract and is not labelled as a managed session.

Session context is supplied automatically by the system. Its existing identity
file is an implementation input owned by the platform, not a new format parsed
independently in every ERP command. Shared configuration, authorization, identity,
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
Given an agent session has framework-resolved lineage and an operating scope mapped to an ERP site and company
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
Scenario: A context selector uses the shared decision boundary
Given a managed agent session requests a different company or site for an operation
When the CLI resolves the request
Then the request is evaluated through the established platform context and authorization contracts and the ERP mapping
And the agent receives the resolved context or a structured refusal
And the CLI does not apply an independent copy of platform identity policy
```

#### UC-SCTX5:

```gherkin
Scenario: Delegation preserves lineage and distinguishes execution
Given an agent delegates ERP work to a child session under its established lineage
When the child performs a native operation
Then the operation retains that lineage and the child's resolved operating scope
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

## Known caveats

- Platform roles and organizational roles remain separate. This feature does not
  introduce a new identity model or reinterpret the platform's standing roles.
- A company relationship does not identify a unique ERP site by itself. The
  operating association is configured and resolved independently of the caller's
  current directory.

## See Also

- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
- ~/.teleclaude/docs/erp/spec/company-layout.md
- ~/.teleclaude/docs/software-development/policy/domain-ownership.md
- ~/.teleclaude/docs/software-development/principle/architecture-boundary-purity.md
