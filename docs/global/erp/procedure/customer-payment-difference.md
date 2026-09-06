---
description: Settle a customer invoice when the bank receives less than the allocated debt because of an approved fee or loss, without creating hidden credit.
---
# Customer Payment Difference — Procedure

## Goal

Record a customer receipt whose net bank amount differs from the debt it settles, preserving
the accepted fee or loss explicitly and leaving neither false outstanding debt nor hidden
customer credit.

## Preconditions

- The company context is active in `erpnext`.
- The submitted Sales Invoice and its current outstanding amount are resolved.
- The actual bank or cash receipt and its external reference are established.
- The difference has a proven cause and accepted business treatment, such as a bank fee borne
  by the company. An unexplained short payment remains customer debt.
- The target deduction or loss account and cost center are valid for the company.

## Steps

### 1. Establish the treatment before allocation

Compare the invoice outstanding, actual bank receipt, and source evidence. Restate the proposed
treatment in business language and use `confirm-business-intent` when accepting the difference
changes what the customer owes or what the company bears.

Do not label a shortfall as a fee, discount, write-off, withholding, or exchange difference
merely to make the Payment Entry balance. Each treatment has different accounting and tax
meaning.

A discount already granted by the invoice's stored payment schedule composes
`customer-early-payment-discount`; it is not an ad hoc payment difference.

### 2. Preserve the native invoice reference

Create the draft through ERPNext's native Payment Entry mapper for the Sales Invoice. Preserve
the mapper-produced reference row and its full invoice allocation. Add the approved difference
as a deduction or loss row with its account, cost center, amount, and explanation.

For a same-currency customer receipt, set both payment amount fields to the actual net bank
receipt. ERPNext normalizes the received amount from the paid amount; changing only the received
amount can restore the gross amount and turn the deduction into unintended unallocated customer
credit.

### 3. Verify the draft equation

Read the stored Payment Entry before submission. In the common same-currency receipt case,
require:

- invoice allocation equals actual bank receipt plus accepted deduction;
- paid and received amounts both equal the actual bank receipt;
- the deduction row equals the accepted difference and reaches the intended account;
- `difference_amount` and `unallocated_amount` are both zero; and
- the invoice reference, party, company, currency, posting date, and external reference remain
  correct.

For multi-currency receipts, use ERPNext's native exchange calculations and prove the equivalent
equation in company currency, including its generated exchange-gain or loss row.

### 4. Submit and prove the ledger

Submit the Payment Entry and read the payment, invoice, General Ledger, and receivables state
back. Require the invoice to be paid with zero outstanding and require the ledger to show:

- bank or cash debit equal to the actual receipt;
- deduction or loss debit equal to the accepted difference;
- receivable credit equal to the invoice allocation; and
- total debits equal total credits.

No customer credit may remain unless the customer genuinely overpaid.

## Outputs

- One submitted Payment Entry linked to the invoice and external receipt.
- An explicit, justified deduction or loss row for the accepted difference.
- A paid invoice with zero outstanding and no accidental unallocated credit.
- Balanced bank, deduction, and receivable ledger entries.

## Recovery

- If the draft carries an unexpected unallocated amount, difference amount, or gross bank
  receipt, stop before submission. Correct both payment amount fields and re-read the stored
  draft; never compensate by changing the invoice allocation without business cause.
- If the wrong Payment Entry was submitted and nothing downstream consumed it, cancel it before
  correcting the receipt. Require the invoice outstanding to return before replaying.
- Cancel the corrected Payment Entry before cancelling or amending its invoice. Prove the
  receivable and deduction ledger effects reverse together.
- Database cancellation does not alter the actual bank statement. Preserve the original bank
  evidence and record any real refund or later settlement as its own event.

## Discipline

A balanced Payment Entry can still encode the wrong story. The bank amount, customer debt, and
deduction are three separate facts that must reconcile without one impersonating another. Never
erase an unexplained short payment or manufacture customer credit to satisfy the arithmetic.

## See Also

- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/procedure/confirm-business-intent.md
- ~/.teleclaude/docs/erp/procedure/customer-early-payment-discount.md
- ~/.teleclaude/docs/erp/procedure/customer-invoice-and-payment.md
- ~/.teleclaude/docs/erp/procedure/customer-receipt-allocation.md
