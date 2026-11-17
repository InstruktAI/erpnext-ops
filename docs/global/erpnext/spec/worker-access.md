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
| Reads | Staff records, everybody's hours, projects, tasks, customers with their addresses and contacts, suppliers, items, stock levels, material entries, leave requests, public calendar entries; each without its money and without a person's private details |
| Writes, own entries only | Hours: record, submit, cancel, amend, delete a draft. Material entries: record, change and delete as drafts. Leave requests: record and change while open. Calendar entries. Notes on the records they can read |
| Is refused | Invoices, payments and the ledger; creating or changing projects and tasks; submitting a material entry; approving a leave request; changing or deleting another person's entry; reading or recording the records about one person that staff administration and payroll keep, such as attendance, expense claims and salary slips |

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
| `Employee` | The record types about one person, listed below | None |

The record types about one person: Appraisal, Attendance, Attendance Request, Compensatory
Leave Request, Daily Work Summary, Employee Advance, Employee Checkin, Employee Grievance,
Employee Performance Feedback, Employee Promotion, Employee Referral, Employee Transfer,
Expense Claim, Goal, Leave Encashment, Overtime Slip, Shift Request, Training Feedback,
Employee Benefit Application, Employee Benefit Claim, Employee Benefit Ledger, Employee
Incentive, Employee Other Income, Employee Tax Exemption Declaration, Employee Tax Exemption
Proof Submission, Payroll Correction, Retention Bonus, Salary Slip, Salary Structure
Assignment, Salary Withholding.

**What a worker is shown of a record.** The fields below sit at permission level 3. The two
roles a staff login holds by itself do not read that level; every other role that reads the
record type at the first level reads it, and writes it where it writes the first level.

| Record type | Fields at that level |
| --- | --- |
| Staff record (Employee) | `gender`, `date_of_birth`, `date_of_joining`, `employment_type`, `employee_number`, `grade`, `personal_email`, `prefered_contact_email`, `prefered_email`, `unsubscribed`, `current_address`, `current_accommodation_type`, `permanent_address`, `permanent_accommodation_type`, `person_to_be_contacted`, `emergency_phone_number`, `relation`, `attendance_device_id`, `expense_approver`, `leave_approver`, `shift_request_approver`, `ctc`, `salary_currency`, `salary_mode`, `employee_advance_account`, `payroll_cost_center`, `bank_name`, `bank_ac_no`, `iban`, `marital_status`, `family_background`, `blood_group`, `health_details`, `health_insurance_provider`, `health_insurance_no`, `passport_number`, `valid_upto`, `date_of_issue`, `place_of_issue`, `bio`, `job_applicant`, `job_offer`, `scheduled_confirmation_date`, `final_confirmation_date`, `contract_end_date`, `notice_number_of_days`, `date_of_retirement`, `resignation_letter_date`, `relieving_date`, `held_on`, `new_workplace`, `leave_encashed`, `encashment_date`, `reason_for_leaving`, `feedback`, `create_user_permission`, `create_user_automatically` |
| Material (Item) | `standard_rate`, `valuation_rate`, `last_purchase_rate`, `max_discount` |
| Stock level (Bin) | `valuation_rate`, `stock_value` |
| Project | `estimated_costing`, `total_costing_amount`, `total_expense_claim`, `total_purchase_cost`, `total_sales_amount`, `total_billable_amount`, `total_billed_amount`, `total_consumed_material_cost`, `gross_margin`, `per_gross_margin` |
| Task | `total_costing_amount`, `total_expense_claim`, `total_billing_amount` |
| Activity Type | `costing_rate`, `billing_rate` |
| Hours (Timesheet) | `total_billable_amount`, `total_costing_amount`, `base_total_billable_amount`, `base_total_costing_amount`, `total_billed_amount`, `base_total_billed_amount`, `per_billed` |
| Hours, each line (Timesheet Detail) | `base_billing_rate`, `base_billing_amount`, `base_costing_rate`, `base_costing_amount`, `billing_rate`, `billing_amount`, `costing_rate`, `costing_amount` |
| Material entry (Stock Entry) | `total_outgoing_value`, `total_incoming_value`, `value_difference`, `total_additional_costs`, `total_amount` |
| Material entry, each line (Stock Entry Detail) | `basic_rate`, `additional_cost`, `valuation_rate`, `basic_amount`, `amount` |

## Known caveats

Observed on Frappe 16.56 with ERPNext 16.55 and its HR application 16.53. On a site with other versions each is
re-established before it is relied on.

- The HR application ships the staff role with rights on the record types about one
  person and narrows them per person by a restriction that also hides other people's hours
  and staff records. This reach leaves that restriction off: with the shipped rights a
  staff login read another person's attendance record. The staff role therefore holds no
  rights on those record types here, and a worker neither reads nor records them, their own
  included. They open when a company uses them and this reach is extended for it.
- Every login that works in the system reads hours, material entries and leave requests,
  because the right sits on the role all such logins hold.
- A field a login does not read comes back as zero when it is an amount and as empty
  otherwise. Under a worker's login a zero amount means not shown.
- The amounts behind those fields are still kept: hours a worker submitted carried their
  cost and raised the project's totals, and a material entry a worker drafted booked its
  value when a login with owner access submitted it.
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
