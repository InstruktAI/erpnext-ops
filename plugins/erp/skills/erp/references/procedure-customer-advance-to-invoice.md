Read first, unless already read in this conversation: `references/procedure-resume-from-native-state.md`, `references/procedure-prove-native-effect.md`.

# Customer Advance To Invoice — Procedure

## Goal

Record money received before final billing as a source-linked customer advance, carry it into
the eventual invoice, and prove the resulting settlement or reversal without counting the
same cash twice.

## Preconditions

- The agent is connected to the company's site.
- The submitted Sales Order, customer, currency, project, amount received, bank or cash
  account, and external payment reference are established.
- Receipt of the money is true. A promised deposit is an open loop, not a Payment Entry.
- The company's configured treatment of advances and any tax due on receipt is known. The
  procedure follows that configuration rather than inventing a receivable, liability, or tax
  treatment.

## Steps

### 1. Resolve the order and any existing receipt

Read the Sales Order and resolve submitted Payment Entries for the same customer, order, amount,
and external payment reference. Reuse the existing payment when it represents the same receipt.
Never create a second advance merely because a prior mapper response was lost.

### 2. Record and prove the advance

Create the Payment Entry through ERPNext's native payment mapper from the Sales Order. Preserve
the order and Project references. Verify the party, amount, allocation, bank or cash account,
currency, posting date, and external reference before submission.

After submission, read the stored Payment Entry and active ledger state. Require the bank or
cash movement and the configured customer-side receivable or advance-account movement to be
equal and balanced. Where the company's configuration recognizes tax on receipt, require the
native tax rows and ledger entries to agree with that treatment.

### 3. Build the final invoice from its source

Convert the Sales Order through ERPNext's native invoice mapper and request automatic advance
allocation. Preserve every mapper-produced row and its Sales Order row link. Read the stored
draft back before submission and require:

- an advance row naming the submitted Payment Entry;
- allocated advance no greater than either that Payment Entry's active remaining balance in
  the Advance Payment Ledger or the invoice total;
- `total_advance` equal to the sum of the stored allocation rows;
- the expected Project and per-row Sales Order links; and
- the remaining outstanding amount equal to invoice total less allocated advance.

An `advance_paid` value displayed on the Sales Order does not replace these checks.

### 4. Submit and prove settlement

Submit the invoice after its commercial and tax details are correct. Read the invoice, Sales
Order, Payment Entry, and native ledger state back. Require the invoice outstanding amount and
status to reflect the applied advance, the Sales Order's billed state to reflect the submitted
invoice, and the customer-side entries from receipt and invoice to reconcile under the company's
configured advance treatment.

Any unpaid remainder stays visible for normal collection. Any unconsumed advance remains an
active customer credit or advance rather than being silently treated as revenue.

## Outputs

- One submitted Payment Entry linked to the Sales Order and Project.
- One source-linked invoice carrying the stored advance allocation.
- Stored invoice, order, payment, and ledger state that agree on what was received, billed,
  allocated, and still outstanding.
- A visible open loop for any remaining collection, refund, or delivery obligation.

## Recovery

- Resume per `resume-from-native-state` from active Payment Entries, Advance Payment Ledger
  Entries, GL entries, invoice advance rows, and the source order's per-row billed state.
- If invoice creation or submission fails, leave the submitted advance intact and rebuild the
  invoice from the order only after resolving existing drafts and source links.
- An advance created by this procedure references the Sales Order, while the later invoice
  carries the allocation. Cancel the dependent invoice first
  so it releases the allocation, then cancel the Sales-Order-linked Payment Entry. A Payment
  Entry linked directly to an invoice follows that invoice's native payment dependency order
  instead. Verify that the invoice no longer consumes the order, the advance ledger entry is
  delinked, and no active GL cash or customer balance remains.
- A cancelled Payment Entry can leave the Sales Order's derived `advance_paid` display stale even
  when its advance ledger entry is delinked and its active GL effect is gone. A newly mapped
  invoice must then carry no advance row. Treat the active ledger, document status, and stored
  mapper result as authoritative; never repair the derived field by writing it directly.
- Returning the cash is a separate outbound payment event that cancellation never performs:
  preserve the original receipt, use ERPNext's native customer-credit or reverse-payment path
  for the active balance, and prove the bank and customer ledgers per `prove-native-effect`.
  Use `customer-credit-and-refund` only when a submitted invoice correction created the credit.

## Discipline

An order total is a commercial promise, an advance is cash received, and an invoice is the
billing event. ERPNext links them, but none is evidence for the others on its own. Prove the
active receipt before allocating it, prove the stored allocation before submitting the invoice,
and prove the ledger after either completion or compensation.

## See Also

- this skill's SKILL.md
- references/concept-composable-business-procedures.md
- references/procedure-customer-invoice-and-payment.md
- references/procedure-customer-credit-and-refund.md
- references/procedure-project-work-to-invoice.md
- references/procedure-resume-from-native-state.md
- references/procedure-prove-native-effect.md
