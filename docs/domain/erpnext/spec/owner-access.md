---
description: What the login of a company's owner or organizer reaches on the company's ERPNext site — the whole business and its configuration, with the system itself left to the system administrator — and the roles that produce it.
---

# Owner Access — Spec

## What it is

The reach of the people who run a company in its ERPNext: its owner and its organizer.
They carry the business in full and configure it as they see fit; the system they work in
is its administrator's (`erpnext/principle/business-and-system`). The reach is produced by
ERPNext roles on the person's own login, so ERPNext enforces it whatever an agent attempts.

## Canonical fields

| The owner or organizer | Reach |
| --- | --- |
| Runs and configures | Customers, suppliers, materials with their groups and variants, price lists, stock locations, stock movements and counts, projects and tasks, hours, staff records, leave and its approval, departments, job titles, holiday lists, quotations, orders, invoices and payments, notes and to-dos; the company record, the chart of accounts, cost centres, tax and payment setup, and the settings of each business area |
| Adds and removes people | Adds a staff record and creates the person's login from it; the person receives ERPNext's mail to set a password and holds worker access (`erpnext/spec/worker-access`). Marks a staff record as left, which switches that person's login off |
| Is refused | System-wide settings, among them the company's own statement; giving a login a role; changing another person's login; custom fields, workflows and print layouts; e-mail accounts and integrations; bulk import |

The reach is built from two things on the site: the login holds the user and manager roles
of each business area and does not hold `System Manager`; and `HR Manager` may read and
create the login record type at the first permission level, which is what ERPNext's own
method for creating a login from a staff record,
`erpnext.setup.doctype.employee.employee.create_user`, needs.

| The login holds |
| --- |
| `Accounts Manager`, `Accounts User`, `HR Manager`, `HR User`, `Item Manager`, `Maintenance Manager`, `Maintenance User`, `Projects Manager`, `Projects User`, `Purchase Manager`, `Purchase Master Manager`, `Purchase User`, `Sales Manager`, `Sales Master Manager`, `Sales User`, `Stock Manager`, `Stock User`, `Support Team` |

## Known caveats

Observed on Frappe 16.56 with ERPNext 16.55 and its HR application 16.53. On a site with
other versions each is re-established before it is relied on.

- A login created from a staff record holds only the roles ERPNext gives it. A login with
  owner access is created by the system administrator.
- Daily work that posts to the books goes through under this reach: a login with it
  submitted and cancelled a stock movement and drafted an invoice with a tax template.

## See Also

- [erpnext/principle/business-and-system](../principle/business-and-system.md)
- [erpnext/procedure/person-onboarding](../procedure/person-onboarding.md)
- [erpnext/spec/worker-access](worker-access.md)
- https://github.com/frappe/frappe/blob/v16.56.0/frappe/core/doctype/user/user.json
