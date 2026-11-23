---
description: Nothing is booked for a new company until the chart of accounts its setup created has been checked and, where it fails, replaced.
---

# Chart Before Booking — Policy

## Rules

- **A new company's chart of accounts is checked before its first booking.** ERPNext
  accepts the chart's customer and supplier accounts as the company's defaults, and the
  chart classifies customers and stock as assets and suppliers as liabilities.
- **A chart that fails the check is replaced before the first booking.** Where another
  system carries the company's statutory books, the replacement is ERPNext's standard
  chart. Where ERPNext carries them, the replacement is decided with the company's
  accountant: another chart, or a corrected one.
- **Until the chart is settled, nothing that posts to the books is submitted** for that
  company: no invoice, payment or stock movement. The company's statement of what is ready
  says so.

## Rationale

A chart that puts customers among liabilities or stock among expenses makes a balance
sheet and a profit figure drawn from it wrong, while every single booking still balances
and looks correct. The error is cheapest to correct before anything is booked: ERPNext
documents that its chart importer replaces a company's chart only while the company has no
ledger entries.

The check is made on the site because a shipped chart is not guaranteed to pass it. On a
site running Frappe 16.56 with ERPNext 16.55, the chart shipped for one country placed its
customer accounts under a liability root and its stock accounts under an expense root, and
ERPNext refused the chart's customer account as the company's default receivable account.

Where another system carries the statutory books, ERPNext's books give the company its own
view of its business, and the statutory classification stays in that other system with the
accountant. The standard chart then needs no accounting decision: on the site above,
ERPNext set a company's customer, supplier, stock, income and expense defaults from it,
each under the root the check expects.

## Scope

Every company onboarded through this project.

## Enforcement

The onboarding procedure performs the check
(`project/erpnext-ops/procedure/customer-onboarding`). Reviewers of an onboarding
look for its recorded result before the first booking.

## Exceptions

None.

## See Also

- https://docs.frappe.io/erpnext/chart-of-accounts-importer
