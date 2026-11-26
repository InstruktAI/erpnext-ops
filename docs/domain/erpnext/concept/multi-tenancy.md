---
description: Site-per-tenant ERPNext — Frappe sites as the isolation boundary, companies as accounting scopes inside a site, identities scoped to a site.
---

# ERP Multi-Tenancy — Concept

## What

Each tenant runs on its own Frappe site: a dedicated FQDN and a dedicated database that only
that site's connection reaches. How sites are provisioned and hosted belongs to the
deployment that runs ERPNext, not to the work done on a site.

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
leaked operator secret exposes one site rather than every tenant. This mirrors the vendor's
own production architecture: Frappe Cloud hosts one site per tenant.

## See Also

- [erpnext/procedure/person-onboarding](../procedure/person-onboarding.md)
- [erpnext/policy/operating-disciplines](../policy/operating-disciplines.md)
