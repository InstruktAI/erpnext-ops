---
description: Onboard a company onto its own ERPNext site — mutate itsUP desired state to create the site, then establish the company and its operator identity.
activation_trigger: Use when a business, legal company, or operating entity must become an operable tenant with its own ERPNext site.
---

# ERP Company Onboarding — Procedure

## Goal

A company is operable on its own ERPNext site: the site exists on the bench, the Company
record exists inside it, and a dedicated operator identity with stored credentials performs
routine work.

## Preconditions

- The `itsup` CLI is installed and on PATH.
- The company slug, site FQDN, and required company details are known.
- The TeleClaude company registry has (or will get) the company's scaffold — the company
  credential home. Neither this repository nor the procedure holds any state; deployment
  state lives in itsUP, operating credentials live in the TeleClaude company registry.

## Steps

### 1. Create the site by mutating itsUP desired state

The ERPNext bench is deployed by itsUP as its `erpnext` project; which sites exist is itsUP
state and nothing else's. Follow the itsUP GitOps workflow
(`infra/procedure/itsup-gitops-workflow`) to add the site:

1. `itsup pull`, then locate the project files with `itsup list-project-files erpnext`.
2. In the project's compose file, add the new site FQDN to the site-creation step so
   `bench new-site --install-app erpnext` runs for it — site creation is idempotent and
   skips existing sites — with a dedicated admin bootstrap password drawn from the project's
   secrets.
3. In `itsup-project.yml`, add an ingress entry routing the new FQDN to the
   `erpnext-frontend` service; the frontend resolves the site from the Host header.
4. Add the site's admin bootstrap password to the itsUP secrets (`itsup decrypt <name>`,
   edit, `itsup encrypt <name> --delete`). Never write it anywhere else.
5. `itsup validate`, then `itsup commit`. The container host reconciles the running stack.
6. Verify the site answers at its FQDN before continuing.

Site deletion is the destructive mirror of this step (dropping a tenant database) and is
never performed as part of onboarding; it requires an explicit human decision and a
host-side runbook.

### 2. Create the Company record

Create the ERPNext `Company` record inside the site. This is the legal and accounting scope
the site's documents attach to; the site itself is the tenant boundary.

### 3. Create the operator user

Create an ERPNext user for the site's operator agent. Grant only the roles needed for
routine business operations. The operator must not carry administrator authority.

### 4. Mint the operator API credentials

Generate the operator user's API key and secret. Treat the secret as one-time sensitive
output.

### 5. Store the operator credentials

Record the site base URL, operator API key, and operator API secret in the company
credential home in the TeleClaude company registry. Do not copy administrator credentials
there — administrator bootstrap secrets live only in itsUP.

### 6. Verify the operator identity

Authenticate with the operator credentials, confirm the logged-in user is the operator,
confirm access to the intended company data, and confirm the identity lacks administrator
authority.

## Outputs

- The site exists in itsUP desired state and answers at its FQDN.
- ERPNext `Company` record inside the site.
- Site operator user with API credentials stored in the company credential home.
- Verified operator identity.

## Recovery

- If key generation is repeated, treat it as rotation and update the company credential home.
- If an existing API secret cannot decrypt, inspect the site's encryption key before treating
  the operator credential as wrong.
- If verification shows administrator authority on the operator, revoke the operator
  credentials and repair roles before continuing.
- If reconciliation does not surface the site, follow the itsUP GitOps workflow's recovery
  guidance rather than mutating the host directly.

## Discipline

This procedure crosses the strongest privilege boundary in ERP operations. Administrator
authority exists only to bootstrap the site and create the scoped operator; it is not the
operating identity for business work. State lives where it belongs: sites in itsUP,
operating credentials in the TeleClaude company registry, nothing in this repository. Verify
the operator identity before declaring the company operable. Never echo secrets into chat,
logs, or shell history.

## See Also

- ~/.teleclaude/docs/erp/concept/multi-tenancy.md
- ~/.teleclaude/docs/erp/procedure/person-onboarding.md
