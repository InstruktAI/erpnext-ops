---
description: Site-per-tenant ERPNext hosting — Frappe sites as the isolation boundary, companies as accounting scopes inside a site.
---

# ERP Multi-Tenancy — Concept

## What

Each tenant runs on its own Frappe site: a dedicated FQDN and a dedicated logical database
with its own scoped database user on the shared database server. One bench — one deployment
of the app processes and one MariaDB service — hosts many sites; Frappe resolves the site
from the request's Host header. itsUP provisions sites (database, files, routing, DNS)
through GitOps.

The sandbox is a dedicated test tenant: its own site, holding the sandbox company and
reserved for development and testing. It is not a variant or copy of a business's site,
so testing never shares a database with business records.

The site is the isolation boundary. An ERPNext `Company` inside a site is a legal and
accounting scope: a business group may operate several companies in one site with
consolidated accounting and shared masters. Company is never the tenancy mechanism.

Identities are site-scoped:

- **Site administrator** — bootstrap and administrative setup on one site: creating
  companies, users, and scoped credentials. Administrator credentials never leave the
  site's administrative context.
- **Site operator agent** — an ERPNext user plus API credentials on the site, used for
  routine business operations with the roles it needs and never administrator authority.
- **Human users** — ERPNext users on the site, permission-scoped to the roles and data they
  need.

## Why

Company-based isolation inside one shared site rides on opt-in permission filtering, leaks
through shared masters, reports, and link searches, and needs continuous behavioral proof to
trust. Site-per-tenant makes isolation structural: a site's database connection can only
reach its own schema, so no ERPNext permission misconfiguration can cross tenants, and a
leaked operator secret exposes one site rather than the fleet. The marginal cost of a tenant
is a schema, a site entry, and a DNS route — no additional containers. This mirrors the
vendor's own production architecture: Frappe Cloud hosts one site per tenant on shared
benches backed by MariaDB.

## See Also

- ~/.teleclaude/docs/erp/procedure/company-onboarding.md
- ~/.teleclaude/docs/erp/procedure/person-onboarding.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
