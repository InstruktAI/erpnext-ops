---
description: Native facts and replayable invocations in read-only mutation previews.
---
# Mutation Preview — Spec

## What it is

Agents inspect observed native facts and the intended request before applying a mutation.
The preview contract is defined by `erp/spec/cli-surface`; the CLI owns observation and
request fidelity under `local/projects/erpnext-ops/principle/generic-cli-substrate`.

## Canonical fields

### Use cases

#### UC-PRE1:

```gherkin
Scenario: Existing-document previews expose observed target facts
Given an agent selects an existing document for a mutation
When the agent previews that mutation
Then the preview reports the document's observed lifecycle status, company, and modification marker
And the document's company is distinguished from the selected operating company
And the preview sends no mutating operation
```

#### UC-PRE2:

```gherkin
Scenario: Document previews distinguish inspected schema from site validation
Given an agent supplies document fields for a document operation
When the agent previews the operation
Then the preview reports the live schema's submittability, fields allowed after submission, and change tracking
And it identifies the unknown supplied fields found against that schema
And it names which checks ran and which validation remains the site's responsibility
And it does not substitute local business validation for the site's decision
```

#### UC-PRE3:

```gherkin
Scenario: Preview identity reports the basis of its claim
Given an agent previews a mutation under a selected operating identity
When the CLI reports that identity in the preview context
Then it distinguishes configured identity from identity authenticated by the site
And it never claims authentication from configuration alone
```

#### UC-PRE4:

```gherkin
Scenario: The apply invocation preserves the previewed intent
Given an agent previews a mutation with arguments and optional source and context selections
When the agent executes the apply invocation provided by the preview
Then the invocation preserves those arguments, source token, and selected context
And it enables explicit apply without placeholders or manual reconstruction
```

#### UC-PRE5:

```gherkin
Scenario: Applying observes current state again
Given an agent has previewed a mutation
And relevant native facts change after that preview
When the agent applies the mutation
Then the CLI reads the applicable facts again
And the prior preview is never treated as a reservation of state or validation
```

#### UC-PRE6:

```gherkin
Scenario: Preview makes no inferred business claims
Given an agent previews a mutation
When the CLI emits the preview
Then it exposes its observations and intended native operation
And it emits no generated business-consequence or reversibility summary
```

#### UC-PRE7:

```gherkin
Scenario: Opaque module methods do not imply document knowledge
Given an agent previews an available opaque module-level method
When the CLI describes that invocation
Then it carries no document or schema facts inferred from the method or its payload
And it performs no document introspection
```

## See Also

- ~/.teleclaude/docs/erp/spec/cli-surface.md
- docs/local/principle/generic-cli-substrate.md
- docs/local/spec/feature/erp-operations/document-operations.md
- docs/local/spec/feature/erp-operations/document-lifecycle.md
