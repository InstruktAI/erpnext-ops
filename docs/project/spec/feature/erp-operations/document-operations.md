---
description: Metadata-checked ERPNext document operations with preview-by-default mutation and explicit apply.
---

# Document Operations — Spec

## What it is

Agents read and mutate ERPNext documents of any doctype through a stable command surface:
live schema introspection, listing, retrieval, and create/update/delete/copy with preview by
default and explicit apply. A preview shows the exact native operation that would be sent
without sending it; payloads are checked against live DocType metadata before any write.
ERPNext remains the authority on validation, permissions, and transactions. The command
shapes and preview output contract are governed by `erp/spec/cli-surface`.

## Canonical fields

### Use cases

#### UC-1:

```gherkin
Scenario: Live metadata introspection
Given an active site context
When the agent runs `erpnext meta show <doctype>`
Then the output reflects the live site schema for that doctype, including fields, mandatory
flags, link targets, and child tables
And the schema comes from the running site, not from a bundled model.
```

#### UC-2:

```gherkin
Scenario: List documents through the v2 envelope
Given documents of a doctype exist on the site
When the agent runs `erpnext doc list <doctype> --filter <field=value>`
Then the output is a machine-readable list of matching documents scoped by the active
identity's permissions.
```

#### UC-3:

```gherkin
Scenario: Retrieve a single document
Given a document exists and is readable by the active identity
When the agent runs `erpnext doc get <doctype> <name>`
Then the full document is returned in machine-readable form.
```

#### UC-4:

```gherkin
Scenario: Create previews by default and sends nothing
Given a valid payload for a doctype
When the agent runs `erpnext doc create <doctype> --data <json>` without `--apply`
Then no mutating request is sent to the site
And the output is a preview containing the context, the exact native request that would be
sent, preflight findings, and the exact apply command.
```

#### UC-5:

```gherkin
Scenario: Apply sends the previewed operation and reports ERPNext's result
Given a clean preview for a create
When the agent runs the same command with `--apply`
Then the payload is checked against live DocType metadata before sending
And the native operation is sent and ERPNext's result — success or failure — is returned in
machine-readable form.
```

#### UC-6:

```gherkin
Scenario: Preflight rejects a field the site cannot interpret
Given a payload containing a field the doctype does not define
When the agent runs a create or update with `--apply`
Then the command fails with a structured validation failure naming the offending field
And no mutating request is sent to the site.
```

#### UC-6b:

```gherkin
Scenario: Values the site derives are left to the site
Given a payload omitting mandatory fields ERPNext fills during validation
When the agent applies the create
Then the request is sent and the site's own validation decides the outcome.
```

#### UC-7:

```gherkin
Scenario: Update, delete, and copy follow the same preview contract
Given an existing document
When the agent runs `erpnext doc update`, `erpnext doc delete`, or `erpnext doc copy`
without `--apply`
Then each returns a preview and sends no mutating request
And each sends the native operation only with explicit `--apply`.
```

#### UC-8:

```gherkin
Scenario: A source token makes creation exactly idempotent
Given a create carrying `--source <token>`
When the agent applies it, loses the response, and applies the same command again
Then the retry finds the document whose name derives from the token and reports it as
replayed, sending no second create
And reusing the token with different data fails as a structured idempotency conflict.
```

#### UC-9:

```gherkin
Scenario: Native conversion reports the evidence for its source relationship
Given an agent selects a source document and a native mapper
When the agent explicitly applies the conversion and the site creates a target document
Then the CLI reads the stored target and inspects its native relationship definitions including child rows
And an exact relationship to the selected source is reported as verified
And an expressible relationship without a matching source is reported as a missing-link failure
And a relationship the inspected definitions cannot express is reported as a distinct unavailable-proof failure
And either failure retains the created document reference without claiming that creation was rolled back
And an incidental mention of the source in ordinary text never proves a relationship
```

#### UC-MAP1:

```gherkin
Scenario: Native mapper inputs influence document construction
Given an agent selects a source document and a native mapper
And the mapper accepts inputs that affect the document it constructs
When the agent supplies those mapper inputs for the conversion
Then the site receives them before constructing the target document
And the agent receives the target produced from those inputs
```

#### UC-MAP2:

```gherkin
Scenario: Target overrides apply after native mapping
Given an agent selects a source document and a native mapper
When the agent supplies target field overrides separately from mapper inputs
Then those overrides are applied to the document returned by the mapper
And they are not supplied as inputs to the mapper
```

#### UC-MAP3:

```gherkin
Scenario: Mapper arguments preserve the selected source
Given an agent selects a source document for native conversion
When the agent supplies mapper inputs that conflict with the selected source identity
Then the native mapper receives the selected source identity
And the remaining mapper inputs retain their supplied values
```

#### UC-MAP4:

```gherkin
Scenario: Invalid mapper inputs fail before contacting the site
Given an agent selects a source document for native conversion
When the supplied mapper inputs are malformed or are not an object of named values
Then the agent receives a structured input failure
And no request is sent to the site
```

#### UC-MAP5:

```gherkin
Scenario: Mapping preview and apply use the same final target
Given an agent selects a source document and a native mapper
And the site returns the same mapped target for the same inputs
When the agent previews a conversion and then explicitly applies that conversion
Then the preview creates no target document
And the target sent for creation matches the previewed target including its overrides
```
