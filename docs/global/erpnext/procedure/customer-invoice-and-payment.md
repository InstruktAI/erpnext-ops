---
description: Post a taxed or tax-exempt customer invoice, prove its ledger effect, and settle it through a linked payment.
activation_trigger: Use when a real customer billing event must become a submitted invoice with a proven ledger effect, and when money received for it must be settled through a linked payment.
---
# Customer Invoice And Payment — Procedure

## Required reads

@~/.teleclaude/docs/erpnext/policy/operating-disciplines.md
@~/.teleclaude/docs/erpnext/principle/business-input-traceability.md
@~/.teleclaude/docs/erpnext/concept/composable-business-procedures.md
@~/.teleclaude/docs/erpnext/procedure/resume-from-native-state.md
@~/.teleclaude/docs/erpnext/procedure/prove-native-effect.md

## Goal

Turn a real customer billing event into a submitted invoice and, when money is received, a
linked payment whose document state and ledger entries agree.

## Preconditions

- The agent is connected to the company's site.
- The customer and billed items resolve unambiguously.
- The billing trigger, quantities, prices, currency, payment terms, and tax treatment are
  established from accepted business state or resolved as a genuine decision boundary.
- The company has suitable income, receivable, tax, and bank or cash accounts.

## Steps

### 1. Preserve the billing basis

Record the accepted order, completed work, delivery, milestone, or other billing evidence on
the relevant business object. Carry any unresolved billing condition as an open loop instead
of invoicing prematurely.

### 2. Resolve before creating

Resolve the customer, source order or delivery, and any existing invoice for the same billing
event. Reuse the existing document when it represents the same event. Store the outside
reference of the billing event on the invoice, so a later search finds it.

### 3. Build and verify the draft

Prefer ERPNext's native mapper when the invoice follows an order or delivery, preserving its
row-level source links. Otherwise create the draft directly.

Read the stored draft back and verify its customer, company, source links, income accounts,
net amount, tax rows, tax amount, and gross amount. A selected tax template does not by itself
prove that headless creation populated the invoice's tax rows. When they are absent, resolve
the applicable template and populate its native tax rows before submission. Zero-rated,
exempt, and out-of-scope treatment must be explicit rather than inferred from an empty tax
table.

### 4. Post and prove the ledger

Preview and submit the invoice only after the draft is correct. Read it back and require the
stored `docstatus` to be submitted. Inspect its native General Ledger entries or General Ledger
report and require:

- receivables debit equals the gross invoice amount;
- income credit equals the net amount;
- each tax credit or debit equals the corresponding invoice tax row and reaches an account of
  the configured tax type; and
- total debits equal total credits.

### 5. Record receipt and settle

Record a payment only when receipt is true. Resolve any existing Payment Entry linked to the
invoice or carrying the same external payment reference before invoking the native payment
mapper. Preview the mapped payment, verify its party, bank or cash account, amount, allocation,
and invoice reference, then apply and submit it.

Compose `customer-receipt-allocation` when the receipt is partial, settles several invoices,
or exceeds the current outstanding amount.

Compose `customer-early-payment-discount` when the invoice's accepted payment schedule grants a
discount for receipt by a stated date. Eligibility is established from the actual receipt date
before mapping; it is never manufactured from a short payment.

Read the invoice and payment back. A fully paid invoice must report zero outstanding and a
paid status. Its receivable debit and the payment's receivable credit must net to zero, while
the payment posts the received amount to the intended bank or cash account.

When the due date passes while a balance remains, compose `overdue-collection`. Never create a
Payment Entry merely to close a Dunning or make the invoice appear settled.

## Outputs

- A submitted invoice whose tax totals and balanced ledger effect agree.
- A submitted, source-linked Payment Entry when receipt has occurred.
- Zero invoice outstanding and zero net receivable for a fully settled invoice.
- A traceable billing basis and visible follow-up for anything still unresolved.

## Recovery

- On a lost create or mapper response, compose `resume-from-native-state`: an invoice
  resolves by its source links and outside reference, a payment by its invoice link and
  external reference.
- If tax rows or totals are absent or wrong, correct the draft and re-read it before submission.
- Correct a submitted invoice through cancel-and-amend. Reverse or cancel dependent payments
  before cancelling the invoice they settle.
- A sent invoice or real-world payment instruction is an outbound effect and is not undone by
  database cancellation; preserve the correction trail and communicate it explicitly.

## Discipline

An invoice is not proven because its grand total looks plausible, and a payment is not proven
because a mapper returned a document. Verification composes `prove-native-effect`: stored document
state, native links, and balanced ledger entries. Resumption composes
`resume-from-native-state`, never the last command an agent remembers running.

## See Also

- ~/.teleclaude/docs/erpnext/procedure/confirm-business-intent.md
- ~/.teleclaude/docs/erpnext/procedure/customer-early-payment-discount.md
- ~/.teleclaude/docs/erpnext/procedure/customer-receipt-allocation.md
- ~/.teleclaude/docs/erpnext/procedure/overdue-collection.md
- ~/.teleclaude/docs/erpnext/procedure/project-work-to-invoice.md
- ~/.teleclaude/docs/erpnext/procedure/resume-from-native-state.md
- ~/.teleclaude/docs/erpnext/procedure/prove-native-effect.md
