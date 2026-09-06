---
description: Reconcile a financial period, transfer its profit or loss, lock completed transaction types, and prove both the close and any reversal from native ledger state.
activation_trigger: Use when a reporting period has ended and its figures must be accepted, its profit or loss transferred to the closing account, and its transaction types locked against later change.
---
# Close Accounting Period — Procedure

## Required reads

@~/.teleclaude/docs/erp/policy/operating-disciplines.md
@~/.teleclaude/docs/erp/principle/business-input-traceability.md
@~/.teleclaude/docs/erp/concept/composable-business-procedures.md

## Goal

Finish a reporting period with reconciled business state, transfer its profit or loss to the
company's closing account, and prevent later transactions from silently changing the accepted
figures.

## Preconditions

- The company context and fiscal year resolve unambiguously.
- The period has ended and its start follows the previous submitted Period Closing Voucher, or
  the fiscal-year start when none exists.
- The person or agent responsible for the accounts has established which reporting period is
  being accepted and which transaction types must be locked.
- The company has an enabled Equity or Liability account in its base currency for retained
  profit or loss.

## Steps

### 1. Resolve the close boundary

Resolve any existing Period Closing Voucher and Accounting Period for the company and dates.
Reuse the accepted close; do not create a parallel voucher for the same boundary. Carry missing
source documents or unresolved accounting judgments as open loops instead of closing around
them.

### 2. Reconcile the period

Review receivables, payables, bank and cash balances, taxes, stock valuation, payroll, assets,
depreciation, accruals, deferrals, bad debts, and other balances that apply to the company's
declared capabilities. Resolve unmatched payments and incomplete source chains. Run the native
General Ledger, Trial Balance, Balance Sheet, Profit and Loss, receivable, and payable reports
for the exact period. Require balanced debits and credits and explain every material balance;
the close does not repair incomplete books.

### 3. Transfer profit or loss

Preview a Period Closing Voucher carrying the company, fiscal year, exact start and end dates,
closing account, and traceable remarks. Apply and submit it only after the preview matches the
accepted close.

Read the voucher back and require submitted state plus completed GL processing. Inspect its
General Ledger entries. ERPNext closes Profit and Loss balances separately for each accounting-
dimension tuple, including Cost Center, Finance Book, and Project. Reconcile every reversed
income or expense amount to the source General Ledger for the same tuple, reconcile the
opposite amount to the closing account, and require total debits to equal total credits. Do not
mistake two dimension groups for duplication merely because they share an account.

### 4. Prove the post-close reports

Run the General Ledger, Profit and Loss, Trial Balance, and Balance Sheet for the closed boundary
again. In the General Ledger, require each closed income and expense balance to net to zero when
the voucher is included, and require the closing account to receive the equal opposite result.
The Profit and Loss report may continue to present the period's income, expenses, and profit as
performance for that range; require its profit or loss to agree with the amount transferred by
the voucher. Require the Trial Balance and Balance Sheet to agree with the resulting closing
account balance. A submitted voucher or `Completed` processing flag is not sufficient without
this report-level proof.

### 5. Lock accepted transaction types

Create one Accounting Period for the accepted dates and company. Mark only the submittable
transaction types whose historical posting dates must no longer change. Read the period back
and verify its dates, enabled state, and closed-document rows. Prove the lock with a harmless
sandbox or pre-production mutation dated inside the boundary; ERPNext must reject it before
the lock is relied on in live books.

## Outputs

- Reconciled period reports whose debit and credit totals agree.
- A submitted Period Closing Voucher with completed, dimension-aware closing entries.
- Profit or loss transferred to the configured Equity or Liability account.
- An enabled Accounting Period that rejects the selected backdated transaction types.
- Open loops for every unresolved accounting item excluded from the accepted close.

## Recovery

- If GL processing reports `In Progress`, wait and read the voucher again; never infer success
  from submission alone. If it reports `Failed`, preserve the error and investigate before any
  retry.
- If closing entries do not reconcile to source balances by dimension, stop and cancel the
  voucher before relying on its reports.
- Cancelling a Period Closing Voucher creates reversing GL entries and preserves the audit
  trail. Verify the voucher is cancelled, processing completes, and its net General Ledger
  effect is zero.
- A later submitted Period Closing Voucher can prevent cancellation of an earlier one. Correct
  closes in reverse dependency order.
- To correct a locked period, deliberately disable or remove its Accounting Period, record and
  verify the correction, rerun the close, and restore the lock. Never leave the period open as
  an incidental side effect of repair.

## Discipline

Closing is acceptance of a complete set of figures, not a cleanup command and not a substitute
for reconciliation. Dimension-aware entries can look duplicated when read without their
Project, Cost Center, or Finance Book, while a superficially balanced voucher can still encode
the wrong business result. Prove the close from source balances through voucher entries to
post-close reports, then lock the accepted boundary. Reopening is an explicit correction flow
whose audit trail must survive.

## See Also

- https://docs.frappe.io/erpnext/closing-accounting-books-in-erpnext
- https://docs.frappe.io/erpnext/period-closing-voucher
- https://docs.frappe.io/erpnext/accounting-period
