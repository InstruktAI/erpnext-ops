---
description: The installed erpnext CLI's full command listing and its most-used command groups, rendered from the binary's own help, for agents operating ERPNext sites.
visibility: internal
delivered_by:
  - erpnext-cli-availability
---

# ERPNext CLI Tool — Spec

## What it is

The `erpnext` command every supported agent host has on its command path, disclosed
progressively: the full command listing below, then each command's own help. Run
`erpnext <command> -h` for any command not expanded here.

This spec carries no design. Its listings are rendered from the installed binary
at publication time, so they always match the tool agents invoke. How a native
operation behaves once a command reaches the site is owned by the ERP procedures,
not by this listing.

## Canonical fields

Every command, with its arguments and one-line description:

<!-- @exec: erpnext -h -->

The command groups below are the ones agents use in nearly every business
operation, expanded for quick reference.

### `erpnext context`

<!-- @exec: erpnext context -h -->

### `erpnext doc`

<!-- @exec: erpnext doc -h -->

### `erpnext workflow`

<!-- @exec: erpnext workflow -h -->

### `erpnext activity`

<!-- @exec: erpnext activity -h -->

## Known caveats

- The listings are rendered from the binary installed on the publishing host; a
  host where the CLI is not installed cannot publish this spec.
- Installing the CLI confers no ERP authority. Every business command resolves
  its operating context at invocation.

## See Also

- ~/.teleclaude/docs/erp/spec/feature/erp-operations/cli-availability.md
