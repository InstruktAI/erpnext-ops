---
description: Site-independent discovery of the binary's declared command surface and its generated documentation inventory.
delivered_by:
  - erpnext-command-surface-export
---
# Command Discovery — Spec

## What it is

Agents discover the command surface declared by the installed binary without selecting a
tenant, progressively: root help lists every command, each command's help discloses its
detail, and `erp/spec/tools/erpnext-cli` carries the rendered listing in the ERP domain
baseline. The surface stays generic under `project/principle/generic-cli-substrate`.

## Canonical fields

### Use cases

#### UC-DISC1:

```gherkin
Scenario: Declared commands are discoverable without a site
Given an agent has the CLI binary and no configured operating context
When the agent requests its declared command surface
Then the CLI emits a deterministic description of its declared commands and arguments
And it neither reads credentials nor contacts a site
```

#### UC-DISC2:

```gherkin
Scenario: Documentation inventory comes from the declared surface
Given an agent reads the CLI command inventory rendered for a binary
When that inventory is produced
Then it is rendered from that binary's declared command-surface export
And it requires no separately maintained command list
```

#### UC-DISC3:

```gherkin
Scenario: Root help points to the operating documentation
Given an agent requests the CLI's root help
When the help is displayed
Then it carries one pointer to the ERP documentation
And per-command documentation pointers are not duplicated throughout the help
```

#### UC-DISC4:

```gherkin
Scenario: Root help lists the whole command tree
Given an agent has the CLI binary and no configured operating context
When the agent requests the CLI's root help
Then every declared command in the tree is listed with its arguments and a one-line description
And no command is reachable only by requesting a group's help first
```

#### UC-DISC5:

```gherkin
Scenario: A command's own help discloses its detail
Given an agent has found a command in the root help listing
When the agent requests that command's help
Then it receives the command's full arguments, options and their meaning
And it needs no site, credentials or selected operating context to do so
```

## See Also

- ~/.teleclaude/docs/erp/spec/cli-surface.md
- docs/project/principle/generic-cli-substrate.md
