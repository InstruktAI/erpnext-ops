---
description: What the login of a company's owner or organizer reaches on the company's ERPNext site — the whole daily business, with the records that set how the system works written by the system administrator alone — and how the site produces it.
---

# Owner Access — Spec

## What it is

The reach of the people who run a company in its ERPNext: its owner and its organizer.
They carry the daily business in full and organize it as they see fit. The records that
set how the system itself works are written by the system administrator, who keeps the
site, and read by everyone else. The reach is produced by ERPNext roles and rights on the
person's own login, so ERPNext enforces it whatever an agent attempts.

## Canonical fields

| The owner or organizer | Reach |
| --- | --- |
| Runs | Customers, suppliers, materials and their groups, price lists, stock locations, stock movements and counts, projects and tasks, hours, staff records, leave and its approval, departments, job titles, holiday lists, quotations, orders, invoices and payments, notes and to-dos |
| Reads, and is refused a change | The company record with its default accounts; the chart of accounts and the tool that replaces it; cost centres; tax templates and tax categories; payment methods; the settings of accounts, stock, selling, buying, HR, payroll, customer relations, point of sale, subscriptions, item variants and appointment booking |
| Is refused | Logins, roles and rights; system settings, among them the company's own statement; custom fields, workflows and print layouts; e-mail accounts and integrations; bulk import |

The reach is built from two things on the site:

- **The business roles without the administrator role.** The login holds the user and
  manager roles of each business area and does not hold `System Manager`.
- **Change rights moved to the administrator.** On each record type of the second row, the
  rights to write, create, delete and import belong to `System Manager` alone, and every
  other role keeps reading it.

A wish to change one of these records goes to the system administrator as a to-do assigned
to them in the same system, with the wording wanted.

## Known caveats

Observed on Frappe 16.56 with ERPNext 16.55 and its HR application 16.53. On a site with
other versions each is re-established before it is relied on.

- Changing a right of a record type gives the site its own set of rights for that record
  type. After an ERPNext upgrade these are compared with the rights the new version ships.
- The routine agent login holds the business roles and is refused the same changes; setup
  that writes these records uses the administrative login.
- Daily work that posts to the books is unaffected: a login with owner access submitted
  and cancelled a stock movement and drafted an invoice with a tax template.
- A login with owner access creates no logins, so a new person's login is created by the
  system administrator.

## See Also

- ~/.teleclaude/docs/erpnext/procedure/person-onboarding.md
- ~/.teleclaude/docs/erpnext/spec/worker-access.md
