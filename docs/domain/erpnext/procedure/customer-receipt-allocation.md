---
description: Allocate a received customer payment across invoices, preserve any excess as credit, and prove partial or complete settlement from native state.
activation_trigger: Use when one real customer receipt must be allocated across several submitted invoices, is partial, or exceeds the outstanding amount.
---
# Customer Receipt Allocation — Procedure

## Required reads

- [erpnext/procedure/resume-from-native-state](resume-from-native-state.md)

## Goal

Record one real customer receipt against one or more submitted invoices, leaving each invoice
and any excess customer credit in an exact, resumable state.

## Preconditions

- The agent is connected to the company's site.
- Receipt of the money is true and its customer, amount, currency, bank or cash account,
  posting date, and external reference are established.
- Every target invoice is submitted, belongs to that customer and company, and has current
  outstanding state available from ERPNext.

## Steps

### 1. Resolve the receipt before recording it

Resolve Payment Entries for the customer and external payment reference. Reuse the existing
entry when it represents the same bank or cash event. A lost response never justifies recording
the same receipt again.

### 2. Read what is currently collectible

Read each target invoice and the native receivables report immediately before allocation. Use
the current outstanding amount, not the invoice's original total or an earlier observation.
Exclude cancelled, paid, disputed, or otherwise ineligible invoices according to the accepted
business state.

### 3. Build the allocation without losing source rows

Use ERPNext's native Payment Entry mapper for a single invoice. For several invoices or a
receipt created before its invoices were known, use the native outstanding-payment or payment-
reconciliation path. Preserve the mapper-produced reference rows and adjust their allocated
amounts rather than replacing their invoice links.

Before submission, require:

- each allocation is positive and no greater than that invoice's current outstanding amount;
- total allocated amount equals the sum of the reference rows;
- paid or received amount equals allocated amount plus unallocated amount, after any native
  deductions or exchange adjustments; and
- the customer, accounts, currency, posting date, and external reference match the receipt.

Compose `customer-payment-difference` when an approved bank fee or loss makes the actual bank
receipt smaller than the invoice allocation. An unexplained short payment remains outstanding.

### 4. Submit and prove the result

Submit the Payment Entry and read the stored entry, every referenced invoice, and the native
receivables and ledger reports back.

- A partial allocation reduces the invoice outstanding by exactly the allocated amount and
  leaves it partly paid.
- The final allocation reduces outstanding to zero and leaves the invoice paid.
- An excess receipt leaves the invoice paid, records the excess as the Payment Entry's
  unallocated amount, and exposes an equal customer balance in the configured receivable or
  customer-advance account. It is not revenue and is not silently consumed by another invoice.
- Bank or cash debit and the customer-side credit equal the actual receipt under the company's
  configured advance-account treatment, and total debits equal total credits.

### 5. Resume from the remaining state

For a later instalment, invoke the native mapper again and require its allocation to equal the
invoice's then-current outstanding amount. For a later invoice, allocate an existing customer
credit only through ERPNext's native advance or payment-reconciliation state and verify the
remaining credit afterwards.

## Outputs

- One submitted Payment Entry per real receipt, regardless of how many invoices it settles.
- Exact per-invoice outstanding and payment status after every partial or final allocation.
- A visible, reusable customer credit for any excess receipt.
- Balanced bank, customer, and adjustment ledger entries.

## Recovery

- On a lost Payment Entry response, compose `resume-from-native-state`: the receipt resolves
  by its external reference and stored reference rows.
- If an allocation is wrong and no downstream action consumed it, cancel the invoice-linked
  Payment Entry before correcting or cancelling its invoice. Re-read all invoice outstanding
  amounts before replaying.
- A correction is proven per `prove-native-effect`; a real refund is a separate outbound
  payment event that cancellation never performs.
- Never turn an excess receipt into a larger invoice allocation, write-off, or revenue merely
  to make the difference disappear.

## Discipline

One receipt may settle part of one invoice, several invoices, or more than the customer owes.
The invariant is not one payment per invoice; it is one Payment Entry per real receipt, with
native reference rows explaining every allocated unit and an explicit balance explaining the
rest.

## See Also

- [erpnext/policy/operating-disciplines](../policy/operating-disciplines.md)
- [erpnext/concept/composable-business-procedures](../concept/composable-business-procedures.md)
- [erpnext/procedure/customer-invoice-and-payment](customer-invoice-and-payment.md)
- [erpnext/procedure/customer-payment-difference](customer-payment-difference.md)
- [erpnext/procedure/customer-advance-to-invoice](customer-advance-to-invoice.md)
- [erpnext/procedure/customer-credit-and-refund](customer-credit-and-refund.md)
- [erpnext/procedure/resume-from-native-state](resume-from-native-state.md)
- [erpnext/procedure/prove-native-effect](prove-native-effect.md)
