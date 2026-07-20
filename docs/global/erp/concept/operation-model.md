---
description: How agents operate per-tenant ERPNext sites through the erpnext CLI, runtime metadata, and site-scoped identities.
---

# ERP Operation Model — Concept

## What

Agents operate ERPNext through the `erpnext` CLI against per-tenant Frappe sites. The site
is the tenant boundary; companies inside a site are legal and accounting scopes. itsUP owns
site provisioning; the CLI operates an already-provisioned site.

- **Operating surface.** The `erpnext` CLI is the agent-facing surface for context, metadata,
  resolution, document operations, workflows, activity trails, evidence, reports, imports, and
  operational checks. Agents do not operate ERPNext by composing raw HTTP calls.
- **Substrate.** The CLI wraps Frappe's v2 REST abstraction. That substrate explains the generic
  document model, but it is not the normal instruction surface for agents.
- **Truth at runtime.** Field lists, mandatory flags, link targets, child tables, and custom
  fields are read through the CLI from the live site. Memory supplies the pattern; the site
  supplies the exact contract.
- **Preview by default.** Mutating CLI commands show the exact context and native operation
  that would be sent before anything is sent. Applying sends that operation and reports
  ERPNext's result. The CLI does not simulate ERPNext.
- **Identity.** Site administrative setup uses the site's administrator identity. Routine
  business work uses the site's operator identity. Person access uses ERPNext users
  permission-scoped on the site.
- **Business flow.** Business outcomes are achieved by composing CLI primitives and ERP
  procedures. The common selling lane is Lead/Customer -> Quotation -> Sales Order ->
  Sales Invoice -> Payment Entry.

## Why

Per-tenant sites keep operations lightweight while making tenant isolation structural in the
system of record. ERPNext is dynamic: installed apps, custom fields, workflows, and
permissions change the live shape of each site. The CLI absorbs that looseness into a safe
operating surface so agents can rely on preview, metadata preflight, context resolution,
activity capture, and structured failures instead of carrying those mechanics in every
procedure.

## See Also

- ~/.teleclaude/docs/erp/concept/multi-tenancy.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/concept/composable-business-procedures.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/spec/company-layout.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
