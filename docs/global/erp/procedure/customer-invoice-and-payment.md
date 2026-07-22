---
description: Post a taxed or tax-exempt customer invoice, prove its ledger effect, and settle it through a linked payment.
---
# Customer Invoice And Payment — Procedure

## Required reads

@~/.teleclaude/docs/erp/policy/operating-disciplines.md
@~/.teleclaude/docs/erp/principle/business-input-traceability.md
@~/.teleclaude/docs/erp/concept/composable-business-procedures.md

## Goal

Turn a real customer billing event into a submitted invoice and, when money is received, a
linked payment whose document state and ledger entries agree.

## Preconditions

- The company context is active in `erpnext`.
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
event. Reuse the existing document when it represents the same event. Use a source token when
creating a series-named invoice directly so a lost response can be replayed without creating a
second invoice.

### 3. Build and verify the draft

Prefer ERPNext's native mapper when the invoice follows an order or delivery, preserving its
row-level source links. Otherwise create the draft directly. Inspect the CLI preview before
applying either mutation.

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

Read the invoice and payment back. A fully paid invoice must report zero outstanding and a
paid status. Its receivable debit and the payment's receivable credit must net to zero, while
the payment posts the received amount to the intended bank or cash account.

## Outputs

- A submitted invoice whose tax totals and balanced ledger effect agree.
- A submitted, source-linked Payment Entry when receipt has occurred.
- Zero invoice outstanding and zero net receivable for a fully settled invoice.
- A traceable billing basis and visible follow-up for anything still unresolved.

## Recovery

- If a direct create response is lost, replay the same source token and payload; never change
  the token to escape a conflict.
- If a mapper response is lost, inspect the invoice's or payment's native links and external
  reference before mapping again.
- If tax rows or totals are absent or wrong, correct the draft and re-read it before submission.
- Correct a submitted invoice through cancel-and-amend. Reverse or cancel dependent payments
  before cancelling the invoice they settle.
- A sent invoice or real-world payment instruction is an outbound effect and is not undone by
  database cancellation; preserve the correction trail and communicate it explicitly.

## Discipline

An invoice is not proven because its grand total looks plausible, and a payment is not proven
because a mapper returned a document. Verification crosses three layers: stored document
state, native links, and balanced ledger entries. Resume from those observed facts rather than
from the last command an agent remembers running.

## See Also

- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/principle/business-input-traceability.md
- ~/.teleclaude/docs/erp/concept/composable-business-procedures.md
- ~/.teleclaude/docs/erp/procedure/confirm-business-intent.md
- ~/.teleclaude/docs/erp/procedure/project-work-to-invoice.md
