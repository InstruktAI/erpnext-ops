---
description: Set a company up on its newly created ERPNext site — the setup wizard and the Company record, outgoing mail, system settings, posting defaults, and the operator and agent-admin identities with their stored credentials.
---

# Company Site Setup — Procedure

## Goal

A company is operable on its own ERPNext site: the Company record exists inside the site,
a dedicated operator identity performs routine work, and a distinct agent-admin identity
performs explicit administrative work without exposing the bootstrap `Administrator`
identity.

## Preconditions

- The company's site exists and answers at its FQDN, and its administrator bootstrap
  credentials are reachable through the approved secret channel
  (`project/erpnext-ops/procedure/create-site`).
- The company slug and required company details are known.
- The TeleClaude company registry contains the company's scaffold — the company credential
  home. Deployment state lives in itsUP; operating credentials live in the TeleClaude
  company registry.

## Steps

### 1. Initialize the site and create the Company record

A fresh site refuses company creation until ERPNext's native setup wizard has run: the
wizard seeds the masters (warehouse types, fiscal year, chart of accounts) and creates the
first company. Complete it as the site administrator with the company's grounded payload —
name, abbreviation, country, currency, fiscal year, chart of accounts, timezone, no demo
data. Resolve the chart of accounts through the site's native
`get_charts_for_country` method and pass the exact returned identifier; a translated display
fragment is not a template identifier. Treat the wizard as complete only after reading back
the Company, fiscal year, and seeded masters. The step is repeatable: a completed and
verified wizard is skipped, and additional companies on a site are created as ordinary
`Company` documents under administrator authority.

### 2. Configure the outgoing email account

Confirm the site inherits the bench-wide outgoing sender and prove it with a received test
message. ERPNext delivers person invitations, password resets, and business correspondence
through this path, so onboarding does not continue on configuration presence alone. A
company that supplies its own sender follows `project/erpnext-ops/procedure/email-account`; its
site-local Email Account overrides the bench-wide fallback. Re-running either verification
or an already-matching override changes nothing.

### 3. Review system settings

Review the site's system settings — time zone, date and number formats, session expiry, and
password policy — against the company's operating reality. The setup wizard seeds them from
its country and currency answers; this step confirms them rather than assuming them.

### 4. Set the company's posting defaults

Set the company's default income account, and confirm its expense and receivable defaults.
The setup wizard seeds a chart of accounts without binding revenue to it, so every invoice
fails until an income account exists. Re-running the step against a configured company
changes nothing.

### 5. Create the operator user

Create an ERPNext user for the site's operator agent. ERPNext roles are additive, not
hierarchical: a Manager role does not imply its User role. Grant the User role for each
business surface the operator works (e.g. `Sales User` alongside `Sales Manager`), plus the
dedicated `Item Manager` role for item master maintenance and the `Purchase Master Manager`
and `Sales Master Manager` roles for price, supplier, and grouping master maintenance. The
operator must not carry administrator authority. Re-granting an existing role is a no-op; the step is repeatable.

### 6. Mint the operator API credentials

Generate the operator user's API key and secret. Treat the secret as one-time sensitive
output. Re-generation is rotation: update the credential home in the same pass.

### 7. Store the operator credentials

Record the site base URL, company name, operator API key, and operator API secret in the
company credential home: `erpnext.json` in the company's TeleClaude registry scaffold,
readable only by its owner. Writing it is idempotent — the entry is the single source the
unattended agent takes the site's credentials from. The bootstrap `Administrator` credential remains only in
itsUP and is never written to the company credential home.

### 8. Verify the operator identity

Authenticate with the operator credentials, confirm the logged-in user is the operator,
confirm access to the intended company data, and confirm the identity lacks administrator
authority.

People other than the operator receive their logins in customer onboarding
(`project/erpnext-ops/procedure/customer-onboarding`), through
`erpnext/procedure/person-onboarding`, which grants each person the roles their company role
carries. The site's own Getting Started checklist enumerates this same ground; onboarding is
complete when every one of its steps is satisfied or deliberately skipped.

### 9. Create the dedicated agent-admin user

Use the infrastructure-held bootstrap `Administrator` identity once to create a distinct
system user for agent administration. Grant `System Manager` and the business-manager roles
required by the company's declared capabilities. The operator and agent-admin users remain
different identities; routine business roles are never widened merely to avoid switching
context.

### 10. Mint and store the agent-admin API credentials

Generate the dedicated agent-admin user's API key and one-time API secret. Store them under
the admin entry in the same owner-readable company credential home as the operator entry,
including the declared agent-admin user identity. Never copy the bootstrap `Administrator`
credential into this file. Re-generation is rotation and updates only the agent-admin entry.
This uses Frappe's native per-user token and role model; onboarding introduces no parallel
authentication or credential-brokering mechanism.

### 11. Verify both operating identities

Authenticate once with the operator credentials and once with the agent-admin credentials.
For each, read the live authenticated user, its roles and the company it reaches. Require
the two identities to differ and the operator to hold no administrator authority. On
mismatch, missing credentials, or bootstrap `Administrator` authentication, onboarding fails
closed before administrative work begins.

## Outputs

- The outgoing email account is configured, posting defaults are set, and system settings
  are reviewed.
- The company is operable end to end: a quotation converts to an order, an invoice posts,
  and a payment records without further configuration.
- ERPNext `Company` record inside the site.
- Site operator user with API credentials stored in the company credential home.
- Verified operator identity.

- Dedicated agent-admin user with separately stored API credentials.
- Verified operator and agent-admin identities, with bootstrap `Administrator` absent from
  the operating credential home.

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
- If the site does not answer at its FQDN, follow `itsup/procedure/gitops-workflow` (and
  the recovery steps in `project/erpnext-ops/procedure/create-site`) — site availability is deployment
  infrastructure, not ERP work.

- If the configured agent-admin authenticates as another user or as bootstrap
  `Administrator`, stop before mutation, revoke the suspect credential, and repeat the
  dedicated-user provisioning and verification steps.
- If agent-admin key generation is repeated, treat it as rotation and replace only the admin
  entry in the company credential home in the same pass.

## Discipline

This procedure crosses the strongest privilege boundary in ERP operations. Bootstrap
`Administrator` authority exists only in infrastructure custody; explicit administrative
work uses the dedicated agent-admin identity, and routine work uses the operator. State
lives where it belongs: site deployment and bootstrap recovery in itsUP, operating
credentials in the TeleClaude company registry. Verify both identities before declaring
the company operable. Never echo secrets into chat, logs, or shell history.

## See Also

- [project/erpnext-ops/procedure/customer-onboarding](customer-onboarding.md)
- [project/erpnext-ops/procedure/create-site](create-site.md)
- [project/erpnext-ops/procedure/email-account](email-account.md)
- [erpnext/concept/multi-tenancy](../../domain/erpnext/concept/multi-tenancy.md)
- [erpnext/procedure/person-onboarding](../../domain/erpnext/procedure/person-onboarding.md)
- https://docs.frappe.io/framework/user/en/api/rest
- https://docs.frappe.io/erpnext/administrator
- https://docs.aws.amazon.com/IAM/latest/UserGuide/root-user-best-practices.html
