---
description: Nothing is booked for a new company until the chart of accounts its setup created has been checked and, where it fails, decided with the company's accountant.
---

# Chart Before Booking — Policy

## Rules

- **A new company's chart of accounts is checked before its first booking.** ERPNext
  accepts the chart's customer and supplier accounts as the company's defaults, and the
  chart classifies customers and stock as assets and suppliers as liabilities.
- **A chart that fails the check is decided with the company's accountant**: another chart,
  or a corrected one.
- **Until the chart is settled, nothing that posts to the books is submitted** for that
  company: no invoice, payment or stock movement. The company's statement of what is ready
  says so.

## Rationale

A chart cannot be replaced once anything is booked against it. A chart that puts customers
among liabilities or stock among expenses makes ERPNext's own balance sheet and profit
figure wrong, and makes a period close sweep the wrong accounts, while every single booking
still balances and looks correct. The charts ERPNext ships per country differ in quality,
so the check is made on the site and not assumed from the country.

## Scope

Every company onboarded through this project.

## Enforcement

The onboarding procedure performs the check
(`local/projects/erpnext-ops/procedure/customer-onboarding`). Reviewers of an onboarding
look for its recorded result before the first booking.

## Exceptions

None.
