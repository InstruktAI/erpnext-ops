---
description: What a worker's own login reaches on a company's ERPNext site — reads people and jobs, writes only their own hours and material entries, no access to the books — and the roles that produce it.
---

# Worker Access — Spec

## What it is

The reach of a person who does the company's work on site and reports it themselves: their
hours, the material they take, when they are away. A worker reads what concerns people and
jobs and writes only their own entries. The reach is produced by ERPNext roles on the
worker's own login, so ERPNext enforces it whatever an agent attempts.

## Canonical fields

| The worker | Reach |
| --- | --- |
| Reads | Staff records, everybody's hours, projects, tasks, customers with their addresses and contacts, suppliers, items, stock levels, material entries, leave requests, public calendar entries |
| Writes, own entries only | Hours: record, submit, cancel, amend, delete a draft. Material entries: record, change and delete as drafts. Leave requests: record and change while open. Calendar entries. Notes on the records they can read |
| Is refused | Invoices, payments and the ledger; creating or changing projects and tasks; submitting a material entry; approving a leave request; changing or deleting another person's entry |

The reach is built from three things on the site:

- **A staff record linked to the login.** It names the person on their hours.
- **A read role**, `Worker`: read on the record types of the first row.
- **An own-entries role**, `Worker Own Entries`: the rights of the second row, granted only
  on records the person created.

Two of ERPNext's own grants are restricted to records the person created, because
unrestricted they let one worker change another's entries: the `Employee` role, which
linking a login to a staff record adds, on hours and on leave requests; and the right every
desk user holds on calendar entries.

## Known caveats

Observed on Frappe 16.56 with ERPNext 16.55. On a site with other versions each is
re-established before it is relied on.

- A worker can name another person's staff record on an hours record. The record still
  shows who created it.
- A record created for a worker by someone else is not the worker's own: they read it and
  cannot change it.
- ERPNext refuses an hours record that overlaps another of the same person.

## See Also

- ~/.teleclaude/docs/erp/procedure/person-onboarding.md
- ~/.teleclaude/docs/erp/procedure/record-work-hours.md
- ~/.teleclaude/docs/erp/procedure/record-material-taken.md
- ~/.teleclaude/docs/erp/procedure/record-absence.md
