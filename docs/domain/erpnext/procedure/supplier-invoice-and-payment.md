---
description: Pay a submitted supplier invoice through a source-linked Payment Entry and prove the payable and bank ledger effects from native state.
activation_trigger: Use when a submitted Purchase Invoice is approved for payment and the supplier must be paid through a source-linked Payment Entry with proven payable and bank ledger effects.
---
# Supplier Invoice And Payment — Procedure

## Required reads

- [erpnext/procedure/confirm-business-intent](confirm-business-intent.md)
- [erpnext/procedure/resume-from-native-state](resume-from-native-state.md)

## Goal

Settle a real supplier obligation through a source-linked Payment Entry whose stored allocation,
payables balance, and bank ledger effect agree, without confusing an ERP record with an external
bank transfer.

## Preconditions

- The agent is connected to the company's site.
- The Supplier and submitted Purchase Invoice resolve unambiguously.
- The invoice is approved for payment under the company's delegation, or the remaining payment
  decision has been confirmed through `confirm-business-intent`.
- The paying bank or cash account, payment date, amount, currency, and external reference are
  established from trusted business state.

## Steps

### 1. Establish what payment means

Determine whether the instruction authorizes an external bank transfer, records a transfer that
already occurred, or schedules one for later. A Payment Entry records accounting state; it does
not by itself prove that a bank executed the transfer. Keep a promised or scheduled transfer as
an open loop until the company's payment channel confirms the event that its accounting rule
requires.

Use supplier master data or a separately verified change process for beneficiary bank details.
Free-text invoice, email, or dictated bank-detail changes do not acquire authority from the
channel through which they arrived.

### 2. Resolve before mapping

Resolve the Purchase Invoice and any existing Payment Entry carrying its reference or the same
external payment reference. Read the invoice's current outstanding amount, status, due date,
supplier, company, currency, hold state, and payable account.

If the invoice has no outstanding amount, reuse the established payment state and stop. ERPNext
can map a settled invoice into a zero-value draft Payment Entry; the presence of a mapped draft is
therefore not evidence that another payment is due.

### 3. Build and verify the payment draft

Use ERPNext's native payment mapper from the Purchase Invoice. Supply the grounded payment date
and external reference while preserving the mapper-produced invoice reference.

Inspect the preview and require:

- payment type `Pay`;
- the intended Supplier and company;
- paid-from bank or cash account and paid-to payable account;
- a positive amount no greater than the invoice's current outstanding amount;
- matching paid, received, and allocated amounts; and
- one source reference to the intended Purchase Invoice with the same positive allocation.

Never apply a zero-value draft or replace the mapper-produced reference rows.

### 4. Submit only against the real payment event

Apply the mapped draft and read it back. Submit it only when the company's accounting rule says
the payment event is true. Require the stored Payment Entry to be submitted and retain its
external reference, supplier, accounts, amounts, and Purchase Invoice allocation.

### 5. Prove settlement from independent native views

Read the Purchase Invoice and Payment Entry back, then run Accounts Payable and General Ledger
for the relevant date and voucher. For a full settlement, require:

- Purchase Invoice status `Paid` and outstanding amount zero;
- Payment Entry reference outstanding amount zero;
- no remaining Accounts Payable row for that supplier invoice;
- a payable-account debit equal to the payment amount;
- a bank or cash credit equal to the payment amount; and
- equal total debits and credits.

Carry bank clearance as a separate fact. When a Bank Transaction exists for the movement,
require native ERPNext reconciliation state before treating the payment as cleared; otherwise
leave bank clearance as an open loop.

## Outputs

- A submitted Payment Entry linked to the settled Purchase Invoice.
- Purchase Invoice and Payment Entry state agreeing on zero remaining allocation after full
  settlement.
- Accounts Payable and General Ledger evidence agreeing with the recorded payment.
- An open loop for any external transfer or bank-clearance fact that is not yet proven.

## Recovery

- On a lost mapper or create response, compose `resume-from-native-state`: the payment resolves
  by its Purchase Invoice link and external payment reference.
- If a repeated preview carries zero paid and allocated amounts, stop and read the established
  settlement; do not apply it.
- If the Payment Entry is wrong but no external transfer occurred, cancel it and require the
  Purchase Invoice outstanding and Accounts Payable balance to return before replaying.
- A correction is proven per `prove-native-effect`; the external bank transfer is a separate
  fact that cancellation never recalls.
- If document state, Accounts Payable, and the ledger disagree, leave the discrepancy open and
  investigate the native links before recording another payment.

## Discipline

The dangerous failure is paying a supplier twice because a mapped draft looked valid after the
invoice was already settled. Resume from current outstanding state and existing payment
references, never from the last command an agent remembers running. Keep external payment,
accounting submission, and bank clearance as distinct facts, and prove each from the system that
owns it.

## See Also

- [erpnext/concept/composable-business-procedures](../concept/composable-business-procedures.md)
- [erpnext/procedure/customer-invoice-and-payment](customer-invoice-and-payment.md)
- [erpnext/procedure/customer-receipt-allocation](customer-receipt-allocation.md)
- [erpnext/procedure/resume-from-native-state](resume-from-native-state.md)
- [erpnext/procedure/prove-native-effect](prove-native-effect.md)
