---
visibility: internal
number: 0001
date: 2026-11-12
description: Agents operate ERPNext through the MCP server Frappe ships in the framework, from a cloud agent and from a computer alike; the erpnext CLI is no longer the agent surface.
---
# Frappe MCP Server Is the ERP Agent Surface — ADR

## Context

Agents operate each tenant's ERPNext site on behalf of the people of that company. The
surface they used was this repository's `erpnext` CLI. It was built under the platform
default that a CLI is the agent's primary surface and no MCP server is a default one
(`sdlc/principle/architectural-defaults`), at a time when Frappe shipped no agent surface
of its own, and no comparison with other surfaces was recorded.

Three things changed.

- Frappe ships an MCP server inside the framework from version 16.53, served by every site.
  It offers generic tools over documents and whitelisted methods, schema and method
  discovery, read-only and destructive annotations on each tool, and a site setting whose
  text is sent to every connecting agent. A caller authenticates as one ERPNext user,
  through that user's own login or API token, and every call runs under that user's roles.
- The people served work with an agent hosted in the cloud by its vendor, inside their
  company's organization there. Such an agent has no shell that holds a custom binary and
  no credentials file. It reaches another system only through a connector that each person
  authorizes for themselves. Its only workers are its own short-lived subagents.
- Work on code stays with agents on a computer, where the platform's CLI-first tooling
  runs. Operating a business application through its API needs none of that tooling.

The route was exercised on the sandbox site at Frappe 16.56 and ERPNext 16.55 under the
operator identity, through the MCP server only: a sales invoice went from draft to
submitted with its ledger entries; its payment was built by ERPNext's own mapper and
settled the invoice; a purchase receipt posted to the stock ledger; an item price was
created; a create outside the identity's roles was refused; an edit of a submitted
document was refused.

## Decision

Agents operate ERPNext through the MCP server Frappe ships in the framework. This holds
for an agent in the cloud and for an agent on a computer, so the ERP doctrine states each
procedure once, in terms of that server's tools.

The `erpnext` CLI is no longer the agent surface for ERP operation and receives no further
capability.

The mutation discipline of read, draft, submit, read back is carried by the doctrine and
by ERPNext's own draft state. No tool enforces it.

## Consequences

Gained:

- Every agent action carries the identity and the ERPNext permissions of the person it
  serves. People's work needs no shared operator credential and no credential on their
  own machines.
- There is no custom ERP surface to build, test, and release. A capability ERPNext adds is
  reachable as soon as a site runs it.
- A tenant site can deliver operating guidance to every connecting agent through its own
  settings.

Given up or newly owed:

- A tenant site must run Frappe 16.53 or later. The site version is now something the
  doctrine depends on.
- Only submittable documents have a draft. A create or update of a master record takes
  effect at once, so the discipline there is to read, state the intended change, write,
  and read back.
- A create silently drops fields the document type does not have. Reading the schema
  before a write and reading the document back after it are obligations of the agent; the
  CLI used to refuse such a field before sending.
- A repeated create makes a second document. The CLI's idempotent create has no
  counterpart, so an agent that retries searches first.
- The change history of a document is not readable under business roles. Evidence of who
  changed what needs administrative authority.
- A report that ERPNext prepares in the background returns no rows on the first call.
- ERPNext's bulk import tool is reserved to system administrators.
- This domain overrides the platform's CLI-first default with an evidenced reason
  (`sdlc/principle/architectural-defaults`). Unattended work on a computer needs an MCP
  client path that holds a stored identity.

## Alternatives Considered

- **Keep the CLI as the only surface.** A cloud agent cannot run it, and giving each
  person their own identity would mean distributing API credentials to their machines.
- **CLI on a computer, MCP server in the cloud.** Every procedure would be stated twice
  and the two statements would drift apart.
- **A third-party assistant app installed in each site.** It offers more tools, including
  a validate-only create. It adds an app to every tenant's bench under a copyleft licence,
  its direction is toward a paid hosted offering, and the framework's own server covered
  the operations exercised.
- **Frappe's own command-line client.** It is a CLI and meets the same limit in the cloud.
- **An MCP server of our own in front of the CLI.** It would keep the preview and the
  idempotent create, at the price of custom infrastructure that repeats what the framework
  ships and that would have to broker each person's identity itself.
