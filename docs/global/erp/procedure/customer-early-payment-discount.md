---
description: Settle a customer invoice under an agreed early-payment discount, using
activation_trigger: Use when a customer pays less than an invoice's face value under accepted payment terms that grant a discount for receipt by a stated date.
  the receipt date to prove eligibility and preserving the discount as an explicit
  ledger effect.
---
# Customer Early-Payment Discount — Procedure

## Required reads

- @~/.teleclaude/docs/erp/procedure/confirm-business-intent.md
- @~/.teleclaude/docs/erp/procedure/customer-invoice-and-payment.md
- @~/.teleclaude/docs/erp/procedure/customer-receipt-allocation.md

## Goal

Settle a customer invoice for less than its face value when the customer's accepted payment
terms grant a discount for receipt by a stated date, while keeping the bank receipt, discount,
receivable, and invoice schedule in agreement.

## Preconditions

- The company context is active in `erpnext`.
- The submitted Sales Invoice carries the accepted Payment Terms Template and a stored payment
  schedule with the discount type, amount, validity basis, discount date, and outstanding amount.
- The Payment Terms Template allocates payment by payment term, so ERPNext preserves the payment-
  term reference and updates the corresponding payment-schedule row during settlement.
- Receipt is true, and its actual receipt date, net amount, currency, bank or cash account, and
  external reference are established from source evidence.
- The company has a designated customer-discount expense account and cost center. When tax loss
  on early-payment discounts is enabled, its configured tax treatment is also available.

## Steps

### 1. Prove contractual eligibility

Read the submitted invoice and payment schedule immediately before recording the receipt.
Require the actual receipt date to be on or before the stored discount date and recompute only
the simple comparison, not ERPNext's discount amount. A date on which the agent happens to run is
not evidence of when money arrived.

When the receipt is late, stop treating it as discounted and compose the ordinary receipt path
for the full amount. A discretionary concession after expiry is a separate business decision,
not an early-payment discount.

### 2. Build from the individual invoice

Invoke ERPNext's native Payment Entry mapper for that Sales Invoice and pass the actual receipt
date as the mapper's `reference_date` argument. The date is mapper input because ERPNext decides
discount eligibility while building the target; changing the Payment Entry date after mapping is
too late. Do not use the bulk outstanding-invoice path, which does not apply the invoice's early-
payment discount.

Set the target Payment Entry `posting_date` to that same actual receipt date. `reference_date`
governs the mapper's eligibility decision; `posting_date` governs when the receipt and discount
reach the ledger. Neither may default to the date on which the agent records the transaction.

Preserve the mapper-produced invoice and payment-term reference. Supply the configured discount
expense account and cost center on the mapper-produced deduction when ERPNext leaves the account
unresolved.

### 3. Verify the draft equation

Read the draft Payment Entry and require:

- paid and received amounts equal the actual net receipt;
- invoice allocation equals the gross amount settled;
- the difference between gross allocation and net receipt equals the eligible discount;
- the deduction reaches the configured discount account and cost center;
- the reference names the invoice and its payment term; and
- the posting date equals the actual receipt date; and
- difference and unallocated amounts are both zero.

For a receipt after the discount date, require the mapper to return the full outstanding amount
with no early-payment deduction. Never preserve a stale discount by overriding dates or amounts
on an already-mapped document.

### 4. Submit and prove the ledger

Submit the Payment Entry and read the payment, invoice, payment schedule, receivables, and General
Ledger state back. Require the invoice to be paid with zero outstanding, the payment schedule to
record both the net paid amount and discounted amount, and the ledger to show:

- bank or cash debit equal to the net receipt;
- customer-discount expense debit equal to the discount;
- receivable credit equal to the gross invoice allocation; and
- total debits equal total credits.

When the company books tax loss separately, require ERPNext's native split between income loss
and tax loss to agree with the configured accounts and the invoice's tax composition.

## Outputs

- One submitted Payment Entry linked to the discounted invoice and payment term.
- A paid invoice and payment schedule whose net receipt plus discount equals gross settlement.
- Explicit, balanced bank, customer-discount, receivable, and optional tax-loss ledger effects.
- Evidence that the actual receipt date satisfied the contractual discount window and became the
  payment's ledger posting date.

## Recovery

- If the mapped draft applies a discount for an ineligible date, stop before submission and remap
  with the receipt date as a native mapper argument and target posting date; do not repair the
  stale result by changing only the mapped document after eligibility was computed.
- If the discount account is absent, configure it before recording the receipt. Never borrow a
  bank-fee, write-off, or unrelated expense account merely to make the entry balance.
- If the submitted Payment Entry is wrong and nothing downstream consumed it, cancel it and
  require the invoice's full outstanding amount and payment-schedule discount state to return
  before replaying.
- Cancelling the ERPNext payment does not reverse the bank receipt. Preserve its external evidence
  and record any real refund or later correction as its own event.

## Discipline

An early-payment discount is a term of sale, not a convenient write-off after a short receipt.
Eligibility comes from the invoice's stored payment schedule and the real receipt date; the
ledger account comes from company configuration. Passing dates only after mapping can produce a
plausible Payment Entry with an expired discount, so mapper inputs and target overrides remain
strictly distinct.

## See Also

- ~/.teleclaude/docs/erp/procedure/customer-payment-difference.md
