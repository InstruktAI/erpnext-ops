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
- **Agent-admin identity** — dedicated ERPNext user and API credentials on the site, used
  only for explicitly selected administrative operations. It is distinct from the operator
  and from the bootstrap `Administrator` identity.
- **Company credential home** — `erpnext.json` in the TeleClaude company registry's
  per-company scaffold: the site base URL, company name, and separate operator and
  agent-admin API credentials, owner-readable only. It belongs to the company, not to a
  human user, and is the single source the CLI resolves credentials from at command time.
- **Business knowledge** — company-specific operating context such as products, price lists,
  proposal templates, pipeline meaning, and tone.

## Allowed values

- `site FQDN`: lower-case DNS name routed to the shared bench.
- `company-slug`: lower-case kebab-case, unique in the TeleClaude company registry.
- Credential fields: site base URL, company name, operator API key and secret, and optional
  agent-admin user, API key, and secret. Selecting a privilege whose credentials are absent
  fails closed; credentials never fall back across identities.

## Known caveats

- TeleClaude organization membership, project assignment, ERP site, accounting
  Company, and ERP User are separate identities. A matching name or slug alone
  does not establish their association or decide a consuming operation's authority.
- The CLI resolves the chosen company credential home from explicit selectors or
  persisted selection; platform lineage is not part of that selection path.
- Bootstrap `Administrator` credentials never belong in the company credential home; they
  remain deployment secrets in itsUP. The company credential home may carry only the
  dedicated agent-admin identity used for explicit administrative operations.
- Human ERPNext users are distinct from the site operator identity.
- Companies sharing one site belong to the same business group and share the site's operator
  identity.
