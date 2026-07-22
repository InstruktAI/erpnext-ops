---
description: Detect overdue customer debt, progress it through native dunning and follow-up, and close the loop only when the receivable is actually settled.
---
# Overdue Collection — Procedure

## Goal

Turn an overdue customer balance into a traceable collection process without inventing a
payment, losing a promise to pay, or charging interest or fees that the business did not
approve.

## Preconditions

- The company context is active in `erpnext`.
- The Sales Invoice is submitted, has a positive current outstanding amount, and is overdue
  according to its stored payment schedule and the company's accepted terms.
- The customer, invoice, current receivable, prior reminders, disputes, promises, and existing
  Dunning records have been resolved from native state.
- Any interest, fee, escalation wording, and delivery channel are established business policy
  or pass through `confirm-business-intent` before use.

## Steps

### 1. Prove that collection is due

Run the native Accounts Receivable report as of the collection date, aged by due date. Require
the invoice row, due date, age, and outstanding amount to agree with the stored invoice and
payment schedule. A remembered due date, an invoice total, or an open follow-up is not enough.

Stop when the balance is paid, credited, disputed, promised within an accepted window, or not
yet due. Record the reason and the next commitment instead of escalating mechanically.

### 2. Resolve the current collection stage

Read the invoice's activity trail and resolve existing submitted or draft Dunning records that
contain its payment-schedule row. Reuse the current stage when it represents the same collection
attempt. Never map the same overdue invoice again merely because an earlier response was lost.

### 3. Build and verify the dunning

Create the Dunning through ERPNext's native Sales Invoice mapper. Read the stored draft and
require its customer, company, currency, invoice, payment-schedule row, due date, outstanding
amount, and overdue days to agree with the source.

A configured Dunning Type may supply wording, interest, a fee, income account, and cost center.
Verify every supplied value before submission. A first reminder may carry no fee or interest;
absence is not a reason to invent either.

### 4. Submit, communicate, and preserve the next commitment

Submit the verified Dunning. Sending a reminder is an outbound effect: restate the recipient,
invoice, amount, wording, channel, and any charge in plain business language before sending.
Record what was actually communicated on the Dunning or customer trail.

Create one dated open loop for the next observable commitment, such as checking a promised
payment, recording a reply, or advancing to the next accepted collection stage. A promise to
pay remains a commitment; it never becomes a Payment Entry until receipt is true.

### 5. Settle from the real receipt

When money arrives, compose `customer-receipt-allocation` against the overdue Sales Invoice.
Read the Payment Entry, invoice, Dunning, receivables report, and ledger state back. Require the
allocated receipt to reduce the invoice outstanding exactly and require ERPNext to mark the
Dunning resolved only when its overdue payment rows no longer remain outstanding.

Close the collection follow-up with the receipt or other accepted resolution that honoured it.
Manual Dunning status alone is not evidence that cash arrived or debt disappeared.

## Outputs

- Native receivables evidence that identifies the overdue balance and its age.
- One source-linked Dunning for the current collection stage, with approved fees and wording.
- A dated, traceable collection commitment until the balance is resolved.
- A linked receipt, corrected invoice, accepted dispute state, or other explicit resolution.
- Agreement among invoice outstanding, Dunning status, receivables, and ledger state.

## Recovery

- Resume from the Accounts Receivable report, stored invoice payment schedule, existing Dunning
  records, activity trail, and open loops. Do not infer the current stage from the last message
  an agent remembers sending.
- If a receipt is cancelled, require the invoice outstanding to return and the linked Dunning to
  become unresolved again. Create a new collection commitment when collection resumes; do not
  rewrite the historical closed loop.
- To compensate a clean test chain, cancel the invoice-linked Payment Entry first, then the
  Dunning, then the Sales Invoice. Prove that the receivable report no longer carries the test
  balance.
- Cancelling a Dunning does not recall a sent reminder, and cancelling a Payment Entry does not
  reverse an external bank transfer. Preserve and communicate those corrections explicitly.

## Discipline

Collection is driven by current debt, not by elapsed time alone. A Dunning records an escalation;
it does not prove delivery, agreement, or payment. Keep every promise visible, every charge
authorized, and every resolution tied back to native receivable and ledger state.

## See Also

- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/procedure/confirm-business-intent.md
- ~/.teleclaude/docs/erp/procedure/customer-invoice-and-payment.md
- ~/.teleclaude/docs/erp/procedure/customer-receipt-allocation.md
