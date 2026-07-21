---
description: Onboard a company onto its own ERPNext site — ensure the site exists, then establish the company and its operator identity.
activation_trigger: Use when a business, legal company, or operating entity must become an operable tenant with its own ERPNext site.
---

# ERP Company Onboarding — Procedure

## Goal

A company is operable on its own ERPNext site: the site exists on the bench, the Company
record exists inside it, and a dedicated operator identity with stored credentials performs
routine work.

## Preconditions

- The company slug, site FQDN, and required company details are known.
- The TeleClaude company registry contains the company's scaffold — the company credential
  home. Deployment state lives in itsUP; operating credentials live in the TeleClaude
  company registry.

## Steps

### 1. Ensure the tenant site exists

The company's site is deployed app infrastructure, and creating it is apps-domain work:
activate `apps/procedure/operate-a-deployed-app` for the `erpnext` app, which resolves the
site-creation procedure. Continue only when the site answers at its FQDN and its
administrator bootstrap credentials are reachable through the approved secret channel.
Site deletion is likewise apps-domain work, human-gated, and never part of onboarding.

### 2. Initialize the site and create the Company record

A fresh site refuses company creation until ERPNext's native setup wizard has run: the
wizard seeds the masters (warehouse types, fiscal year, chart of accounts) and creates the
first company. Complete it as the site administrator with the company's grounded payload —
name, abbreviation, country, currency, fiscal year, chart of accounts, timezone, no demo
data. The step is repeatable: a completed wizard is skipped, and additional companies on a
site are created as ordinary `Company` documents through the CLI's preview/apply flow.

### 3. Create the operator user

Create an ERPNext user for the site's operator agent. ERPNext roles are additive, not
hierarchical: a Manager role does not imply its User role. Grant the User role for each
business surface the operator works (e.g. `Sales User` alongside `Sales Manager`), plus the
dedicated `Item Manager` role for item master maintenance. The operator must not carry
administrator authority. Re-granting an existing role is a no-op; the step is repeatable.

### 4. Mint the operator API credentials

Generate the operator user's API key and secret. Treat the secret as one-time sensitive
output. Re-generation is rotation: update the credential home in the same pass.

### 5. Store the operator credentials

Record the site base URL, company name, operator API key, and operator API secret in the
company credential home: `erpnext.json` in the company's TeleClaude registry scaffold,
readable only by its owner. Writing it is idempotent — the entry is the single source the
CLI resolves at command time. Do not copy administrator credentials there — administrator
bootstrap secrets live only in itsUP.

### 6. Verify the operator identity

Authenticate with the operator credentials, confirm the logged-in user is the operator,
confirm access to the intended company data, and confirm the identity lacks administrator
authority.

## Outputs

- The company's site exists and answers at its FQDN.
- ERPNext `Company` record inside the site.
- Site operator user with API credentials stored in the company credential home.
- Verified operator identity.

## Recovery

- If key generation is repeated, treat it as rotation and update the company credential home.
- If an existing API secret cannot decrypt, inspect the site's encryption key before treating
  the operator credential as wrong.
- If verification shows administrator authority on the operator, revoke the operator
  credentials and repair roles before continuing.
- If the site does not answer at its FQDN, route back through the apps-domain entry point
  (`apps/procedure/operate-a-deployed-app`) — site availability is app infrastructure, not
  ERP work.

## Discipline

This procedure crosses the strongest privilege boundary in ERP operations. Administrator
authority exists only to bootstrap the site and create the scoped operator; it is not the
operating identity for business work. State lives where it belongs: site deployment in
itsUP, operating credentials in the TeleClaude company registry. Verify
the operator identity before declaring the company operable. Never echo secrets into chat,
logs, or shell history.

## See Also

- ~/.teleclaude/docs/erp/concept/multi-tenancy.md
- ~/.teleclaude/docs/erp/procedure/person-onboarding.md
