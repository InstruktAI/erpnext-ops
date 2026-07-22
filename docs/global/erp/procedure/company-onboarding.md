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
data. Resolve the chart of accounts through the site's native
`get_charts_for_country` method and pass the exact returned identifier; a translated display
fragment is not a template identifier. Treat the wizard as complete only after reading back
the Company, fiscal year, and seeded masters. The step is repeatable: a completed and
verified wizard is skipped, and additional companies on a site are created as ordinary
`Company` documents through the CLI's preview/apply flow.

### 3. Configure the outgoing email account

Confirm the site inherits the bench-wide outgoing sender and prove it with a received test
message. ERPNext delivers person invitations, password resets, and business correspondence
through this path, so onboarding does not continue on configuration presence alone. A
company that supplies its own sender follows `apps/procedure/erpnext/email-account`; its
site-local Email Account overrides the bench-wide fallback. Re-running either verification
or an already-matching override changes nothing.

### 4. Review system settings

Review the site's system settings — time zone, date and number formats, session expiry, and
password policy — against the company's operating reality. The setup wizard seeds them from
its country and currency answers; this step confirms them rather than assuming them.

### 5. Set the company's posting defaults

Set the company's default income account, and confirm its expense and receivable defaults.
The setup wizard seeds a chart of accounts without binding revenue to it, so every invoice
fails until an income account exists. Re-running the step against a configured company
changes nothing.

### 6. Create the operator user

Create an ERPNext user for the site's operator agent. ERPNext roles are additive, not
hierarchical: a Manager role does not imply its User role. Grant the User role for each
business surface the operator works (e.g. `Sales User` alongside `Sales Manager`), plus the
dedicated `Item Manager` role for item master maintenance. The operator must not carry
administrator authority. Re-granting an existing role is a no-op; the step is repeatable.

### 7. Mint the operator API credentials

Generate the operator user's API key and secret. Treat the secret as one-time sensitive
output. Re-generation is rotation: update the credential home in the same pass.

### 8. Store the operator credentials

Record the site base URL, company name, operator API key, and operator API secret in the
company credential home: `erpnext.json` in the company's TeleClaude registry scaffold,
readable only by its owner. Writing it is idempotent — the entry is the single source the
CLI resolves at command time. Do not copy administrator credentials there — administrator
bootstrap secrets live only in itsUP.

### 9. Verify the operator identity

Authenticate with the operator credentials, confirm the logged-in user is the operator,
confirm access to the intended company data, and confirm the identity lacks administrator
authority.

People other than the operator are invited through
`erp/procedure/person-onboarding`, which grants each person the roles their company role
carries. The site's own Getting Started checklist enumerates this same ground; onboarding is
complete when every one of its steps is satisfied or deliberately skipped.

## Outputs

- The company's site exists and answers at its FQDN.
- The outgoing email account is configured, posting defaults are set, and system settings
  are reviewed.
- The company is operable end to end: a quotation converts to an order, an invoice posts,
  and a payment records without further configuration.
- ERPNext `Company` record inside the site.
- Site operator user with API credentials stored in the company credential home.
- Verified operator identity.

## Recovery

- If key generation is repeated, treat it as rotation and update the company credential home.
- If the setup endpoint reports success but the Company or seeded masters are absent, inspect
  the site's setup error before continuing. Correct the grounded payload, clear only the
  inconsistent setup-completion markers, and replay the native wizard; do not hand-build the
  missing ERP records.
- If the available chart list contains a qualified name such as
  `Netherlands - Grootboekschema`, use that full identifier rather than its display fragment.
- If invitation mail is not present in the outgoing queue or recipient inbox, the email step
  is incomplete even when user creation succeeded; repair the sender and resend the native
  password-reset message.
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
