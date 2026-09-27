---
description: Agents discover and invoke the ERP CLI from any working context on a supported execution host, without entering its source checkout or loading tenant credentials for discovery.
visibility: internal
delivered_by:
  - erpnext-cli-availability
---

# CLI Availability — Spec

## What it is

The ERP domain's operating tool is available wherever supported agents execute.
Its publisher owns the CLI and its installation contract; host tooling owns
provisioning and updates. Domain awareness lets an agent discover the capability
without making the publisher repository its business working directory.

## Canonical fields

- **Invocation:** the installed `erpnext` command is discoverable on the normal
  command path from an agent's working context.
- **Discovery:** help, version and the declared command surface are readable
  without selecting a company, loading business credentials, or contacting a site.
- **Ownership:** the source repository supplies the installable artifact and
  supported installation/update contract. The host's established tooling lifecycle
  consumes that contract; every business session does not build its own binary.
- **Business scope:** installing the tool establishes availability, not ERP
  authority. Native operations still use the resolved operating-context contract.
- **Failure:** an unsupported host or failed installation/update has an explicit
  diagnostic and does not claim that the CLI is available.
- **Freshness:** the installed tool is the delivered version. A delivered change
  reaches it through the host tooling lifecycle without a manual step, and
  undelivered work never replaces it.
- **Completion:** installation makes the command's subcommands and options
  complete in operator interactive shells. Agent shells load no completions and
  are unaffected.
- **Disclosure:** the ERP domain baseline carries the installed CLI's full command
  listing and its most-used command groups, rendered from the installed binary's
  own help.

### Use cases

#### UC-CLIA1:

```gherkin
Scenario: An agent invokes the installed capability away from its source
Given an agent execution host has provisioned the supported ERP CLI
And the agent's working directory is unrelated to the CLI source repository
When the agent invokes the ERP command
Then the installed CLI is found through the normal command path
And the agent does not enter or build the source checkout to invoke it
```

#### UC-CLIA2:

```gherkin
Scenario: Capability discovery has no business prerequisites
Given an agent on a supported host has no selected ERP company or credentials
When it requests the CLI's help, version or declared command surface
Then it receives the requested discovery information
And no tenant connection is required
```

#### UC-CLIA3:

```gherkin
Scenario: Installation updates the tool without changing business context
Given an operator uses the supported installation or update path on an agent host
When that operation completes successfully
Then subsequent agent invocations find the installed tool and can inspect its version
And company credentials and existing business context are not replaced by tool installation
```

#### UC-CLIA4:

```gherkin
Scenario: A failed installation is observable
Given an operator provisions the ERP CLI on an unsupported or failing host
When provisioning cannot establish an invocable tool
Then the operator receives an actionable failure
And provisioning does not report the capability as available
```

#### UC-CLIA5:

```gherkin
Scenario: The installed capability is the delivered version
Given an agent execution host has provisioned the supported ERP CLI
And a change to the CLI is in progress but not yet delivered
When an agent invokes the ERP command and inspects its version
Then it reaches the previously delivered version
When the change is delivered
Then subsequent agent invocations reach the newly delivered version
And the host received it without a manual installation step
```

#### UC-CLIA6:

```gherkin
Scenario: An operator's shell completes the installed command
Given an operator on a supported host has provisioned the ERP CLI
When the operator starts typing the ERP command in an interactive shell
Then the shell offers the command's subcommands and options as completions
And agent shells on the same host behave exactly as before installation
```

#### UC-CLIA7:

```gherkin
Scenario: The ERP domain discloses the installed CLI to every agent
Given the ERP CLI is installed on the host that publishes the ERP documentation
When an agent loads the ERP domain baseline
Then it reads every command of the installed CLI with its arguments and description
And that listing matches what the installed binary reports as its own help
```

## Known caveats

- Documentation publication and binary provisioning are distinct capabilities.
- Global availability does not select a company from the current directory or
  confer authority over a tenant.

## See Also

- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
- ~/.teleclaude/docs/erp/spec/feature/erp-operations/session-context.md
