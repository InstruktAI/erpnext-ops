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
| Is refused | Invoices, payments and the ledger; creating or changing projects and tasks; submitting a material entry; approving a leave request; changing or deleting another person's entry; reading a record about another person, such as their attendance, expense claims or salary slips |

The reach is built from two things on the site:

- **A staff record with a login created from it.** ERPNext gives a login created from a
  staff record its `Employee` role, and every login that works in the system holds
  `Desk User`. The staff record names the person on their hours.
- **Rights on those two roles**, each at the first permission level:

| Role | Record types | Rights |
| --- | --- | --- |
| `Employee` | Activity Type, Address, Bin, Contact, Customer, Employee, Item, Project, Supplier, Task, Warehouse | Read, report, print |
| `Desk User` | Timesheet, Stock Entry, Leave Application | Read, report, print |
| `Employee` | Timesheet | Read, write, create, submit, cancel, amend, delete, on records the person created |
| `Employee` | Stock Entry | Read, write, create, delete, on records the person created |
| `Employee` | Leave Application | Read, write, create, report, print, e-mail, share, on records the person created |
| `Desk User` | Event | Read, write, create, report, print, e-mail, share, on records the person created |
| `Employee` | The record types about one person, listed below | The rights ERPNext ships, on records the person created |

The record types about one person: Appraisal, Attendance, Attendance Request, Compensatory
Leave Request, Daily Work Summary, Employee Advance, Employee Checkin, Employee Grievance,
Employee Performance Feedback, Employee Promotion, Employee Referral, Employee Transfer,
Expense Claim, Goal, Leave Encashment, Overtime Slip, Shift Request, Training Feedback,
Employee Benefit Application, Employee Benefit Claim, Employee Benefit Ledger, Employee
Incentive, Employee Other Income, Employee Tax Exemption Declaration, Employee Tax Exemption
Proof Submission, Payroll Correction, Retention Bonus, Salary Slip, Salary Structure
Assignment, Salary Withholding.

## Known caveats

Observed on Frappe 16.56 with ERPNext 16.55. On a site with other versions each is
re-established before it is relied on.

- ERPNext ships the staff role with read on the record types about one person and narrows
  it per person by a restriction that also hides other people's hours and staff records.
  This reach leaves that restriction off, so those record types are narrowed to records
  the person created: before that, a staff login read another person's attendance record,
  and afterwards it did not. A record of that kind that someone else creates for a person,
  such as a salary slip, is then not readable by that person either.
- A worker can name another person's staff record on an hours record. The record still
  shows who created it.
- A record created for a worker by someone else is not the worker's own: they read it and
  cannot change it.
- ERPNext refuses an hours record that overlaps another of the same person.
- ERPNext refuses a material entry, also as a draft, for a material that has no known
  value.

## See Also

- ~/.teleclaude/docs/erpnext/procedure/person-onboarding.md
- ~/.teleclaude/docs/erpnext/procedure/record-work-hours.md
- ~/.teleclaude/docs/erpnext/procedure/record-material-taken.md
- ~/.teleclaude/docs/erpnext/procedure/record-absence.md
