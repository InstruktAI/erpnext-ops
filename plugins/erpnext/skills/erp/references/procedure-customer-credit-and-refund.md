Read first, unless already read in this conversation: `references/procedure-confirm-business-intent.md`, `references/procedure-customer-invoice-and-payment.md`.

# Customer Credit And Refund — Procedure

## Goal

Correct all or part of a submitted customer invoice while preserving its legal and accounting
trail, then either retain the customer credit for later allocation or record a refund whose
document state and ledger effect agree.

## Preconditions

- The original Sales Invoice is submitted and resolves unambiguously.
- The correction basis identifies the affected rows, quantities, net amounts, taxes, and
  whether any goods physically return to stock.
- The customer has paid, the invoice cannot simply be cancelled, or the correction requires a
  durable credit note.
- The choice between retaining the credit and refunding it is determined from accepted business
  state or resolved as a genuine decision boundary.

## Steps

### 1. Preserve and resolve the correction basis

Record the return, pricing correction, service reduction, or other accepted basis on the
customer's activity trail. Resolve existing Sales Invoices whose `return_against` points to the
original invoice. Reuse a matching draft or submitted credit note instead of mapping the source
again.

### 2. Map the native credit note

Map the original Sales Invoice through ERPNext's native sales-return mapper. Inspect the preview
and require `is_return`, `return_against`, negative quantities and amounts, negative tax rows,
and each row's source-invoice link. For a partial credit, reduce quantities on the mapper's own
rows and remove only rows that are not being credited; never rebuild the item table in a way
that discards its source links. Enable stock return only when goods physically returned and the
warehouse and serial or batch evidence are established.

Apply the map and read the stored draft back. Require the credited net, tax, and gross amounts
to equal the accepted correction before submission.

### 3. Submit and prove the credit

Preview and submit the credit note. Read it back and require submitted state, return status, and
the original invoice in `return_against`. Inspect its General Ledger entries and require:

- receivables credit equals the gross credit;
- income debit equals the net credit;
- each tax debit equals the corresponding negative tax row; and
- total debits equal total credits.

The submitted credit note carries a negative outstanding amount until that credit is allocated
or refunded.

### 4. Retain or refund the credit

If the customer keeps the credit, leave it outstanding and carry any promised future allocation
as an open loop. Before allocating it to another invoice, resolve both documents and use the
native reconciliation path so both outstanding amounts prove the allocation.

If money has actually been returned, resolve an existing Payment Entry linked to the credit note
or carrying the same external refund reference. Use the running site's native refund path. When
its payment-entry mapper produces a direct credit-note allocation, require payment type `Pay`,
the intended bank or cash account as the paid-from account, receivables as the paid-to account,
the credit-note reference, and its negative allocation. When it instead produces an unallocated
refund, submit that Payment Entry and use native Payment Reconciliation against the original
Sales Invoice as directed by the site's reconciliation flow. Apply and submit only after the
refund is true; never manufacture outstanding amounts or allocation rows to force either shape.

### 5. Prove settlement

Read the credit note and Payment Entry back. A fully refunded credit note reports zero
active outstanding after the native allocation or reconciliation completes. Its receivables
credit and the refund's receivables debit net to zero, while the refund credits the intended bank
or cash account. Require active receivables to contain neither an unallocated refund nor a
remaining customer credit. Confirm the original invoice remains submitted and settled; the
correction is a linked successor, not a rewrite of the original.

## Outputs

- A submitted credit note linked to the original Sales Invoice and its source rows.
- Reversed revenue and tax whose balanced ledger effect agrees with the credited amount.
- Either a visible customer credit for later allocation or a submitted refund Payment Entry.
- Zero credit-note outstanding after a full refund and an unchanged original invoice trail.

## Recovery

- If the map response is lost, resolve by `return_against` before mapping again.
- Correct a draft credit note in place. Correct a submitted one by reversing dependent
  allocations or refunds first, then cancelling and amending the credit note.
- Cancel a refund Payment Entry before cancelling its credit note. Verify the active receivable
  report and General Ledger return to their prior state and that the original invoice remains
  settled.
- A cancelled credit note may retain its historical negative `outstanding_amount` field. Its
  cancelled `docstatus`, inactive ledger effect, and absence from active receivables determine
  the operative state; do not treat that historical field as a live customer credit.
- Cancelling the ERP records does not recall a sent credit note or reverse money already sent.
  Preserve the correction trail and coordinate the real-world recovery explicitly.

## Discipline

A paid invoice is not corrected by editing history, and a negative invoice is not proven merely
because its total has a minus sign. Preserve the native return links, prove the reversal across
receivable, income, and tax accounts, and distinguish a retained credit from a refund that
actually occurred. Compensation runs in dependency order and is verified from active reports,
not inferred from remembered commands or a cancelled document's historical fields.

## See Also

- https://docs.frappe.io/erpnext/credit-note
- https://docs.frappe.io/erpnext/sales-return
- https://docs.frappe.io/erpnext/allocating-credit-note-and-payment
