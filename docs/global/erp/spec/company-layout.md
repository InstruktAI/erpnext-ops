---
description: Site and company identity and credential layout for operating per-tenant ERPNext sites.
---

# ERP Site & Company Layout — Spec

## What it is

Defines the identity and credential contract agents use when operating ERPNext. The Frappe
site is the tenant and credential boundary; companies exist inside a site as legal and
accounting scopes. The site credential home stores the credentials that authorize routine
work on that site.

## Canonical fields

- **site FQDN** — the site's fully qualified domain name; resolves the tenant and its
  database on the shared bench.
- **company-slug** — stable kebab-case identity for a company operated on a site, unique in
  the TeleClaude company registry.
- **Company record** — ERPNext `Company` document inside the site; a legal and accounting
  scope, not a tenancy boundary.
- **Operator identity** — ERPNext user and API credentials on the site, used for routine
  business operations.
<!-- planned:erpnext-agent-admin-context -->
- **Agent-admin identity** — dedicated ERPNext user and API credentials on the site, used
  only for explicitly selected administrative operations. It is distinct from the operator
  and from the bootstrap `Administrator` identity.
<!-- /planned:erpnext-agent-admin-context -->
<!-- planned-change:erpnext-agent-admin-context -->
- **Company credential home** — `erpnext.json` in the TeleClaude company registry's
  per-company scaffold: the site base URL, company name, and operator API key and secret,
  owner-readable only. It belongs to the company, not to a human user, and is the single
  source the CLI resolves credentials from at command time.
<!-- change:erpnext-agent-admin-context -->
- **Company credential home** — `erpnext.json` in the TeleClaude company registry's
  per-company scaffold: the site base URL, company name, and separate operator and
  agent-admin API credentials, owner-readable only. It belongs to the company, not to a
  human user, and is the single source the CLI resolves credentials from at command time.
<!-- /planned-change:erpnext-agent-admin-context -->
- **Business knowledge** — company-specific operating context such as products, price lists,
  proposal templates, pipeline meaning, and tone.

## Allowed values

- `site FQDN`: lower-case DNS name routed to the shared bench.
- `company-slug`: lower-case kebab-case, unique in the TeleClaude company registry.
<!-- planned-change:erpnext-agent-admin-context -->
- Credential fields: site base URL, operator API key, operator API secret.
<!-- change:erpnext-agent-admin-context -->
- Credential fields: site base URL, company name, operator API key and secret, and optional
  agent-admin user, API key, and secret. Selecting a privilege whose credentials are absent
  fails closed; credentials never fall back across identities.
<!-- /planned-change:erpnext-agent-admin-context -->

## Known caveats

<!-- planned-change:erpnext-agent-admin-context -->
- Administrator credentials do not belong in the company credential home; they exist for
  site bootstrap and administrative setup only, and site-deployment secrets live in itsUP.
<!-- change:erpnext-agent-admin-context -->
- Bootstrap `Administrator` credentials never belong in the company credential home; they
  remain deployment secrets in itsUP. The company credential home may carry only the
  dedicated agent-admin identity used for explicit administrative operations.
<!-- /planned-change:erpnext-agent-admin-context -->
- Human ERPNext users are distinct from the site operator identity.
- Companies sharing one site belong to the same business group and share the site's operator
  identity.
