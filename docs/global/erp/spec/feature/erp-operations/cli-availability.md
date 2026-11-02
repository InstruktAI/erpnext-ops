---
description: Agents discover and invoke the ERP CLI from any working context on a host that holds its repository checkout, without entering that checkout or loading tenant credentials for discovery.
visibility: internal
delivered_by:
  - erpnext-cli-availability
---

# CLI Availability — Spec

## What it is

The ERP domain's operating tool is available on every host that holds a checkout
of its publisher repository. That checkout's install step is the whole
provisioning path: it builds the release binary, gives it its stable code
identity where the host's operating system requires one
(`sdlc/procedure/macos-stable-code-signing`), places it in the user-local
executable directory that the platform's agent shell contract puts on the command
path, and registers shell completion. Domain awareness lets an agent discover the
capability without making the publisher repository its business working
directory.

## Canonical fields

- **Supported host:** a host that holds a checkout of the publisher repository.
  A host without one has no ERP CLI.
- **Invocation:** the installed `erpnext` command is discoverable on the normal
  command path from an agent's working context.
- **Discovery:** help and version are readable without selecting a company,
  loading business credentials, or contacting a site.
- **Installation:** the checkout's declared install step installs and updates the
  tool. The platform's commit-boundary maintenance runs that step whenever the
  checkout's main advances with changes to the tool's sources; no business session
  builds its own binary.
- **Business scope:** installing the tool establishes availability, not ERP
  authority. Native operations still use the resolved operating-context contract.
- **Failure:** an install step that cannot establish an invocable tool fails with
  an explicit diagnostic and does not claim that the CLI is available.
- **Freshness:** the installed tool is the version on the checkout's main. A change
  reaches it when that main advances, without a manual step, and work that is not
  on main never replaces it.
- **Completion:** installation makes the command's subcommands and options
  complete in operator interactive shells. Agent shells load no completions and
  are unaffected.

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
When it requests the CLI's help or version
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
Given an operator provisions the ERP CLI on a host where the install step fails
When provisioning cannot establish an invocable tool
Then the operator receives an actionable failure
And provisioning does not report the capability as available
```

#### UC-CLIA5:

```gherkin
Scenario: The installed capability is the delivered version
Given an agent execution host has provisioned the supported ERP CLI
And a change to the CLI is in progress but not yet on main
When an agent invokes the ERP command and inspects its version
Then it reaches the version already on main
When the host's checkout of main advances to include the change
Then subsequent agent invocations reach the new version
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

## Known caveats

- Freshness on a host follows that host's checkout. Nothing in this contract
  pulls a checkout: a host whose checkout is not advanced keeps the version it
  last installed until its main advances.
- Documentation publication and binary provisioning are distinct capabilities.
- Global availability does not select a company from the current directory or
  confer authority over a tenant.

## See Also

- ~/.teleclaude/docs/erp/concept/operation-model.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
