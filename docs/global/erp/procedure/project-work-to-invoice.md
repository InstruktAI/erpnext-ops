---
description: Playbook for service or project businesses moving from proposal and tasks to completed work, invoice, payment, and unresolved follow-up.
activation_trigger: Use when a service, production, tour, or custom project business needs to track work, obligations, billing, and payment without stock-centric assumptions.
---
# Project Work To Invoice — Procedure

## Required reads

@~/.teleclaude/docs/erp/concept/composable-business-procedures.md
@~/.teleclaude/docs/erp/concept/business-activity-trail.md
@~/.teleclaude/docs/erp/policy/operating-disciplines.md
@~/.teleclaude/docs/erp/procedure/resume-from-native-state.md
@~/.teleclaude/docs/erp/procedure/prove-native-effect.md

## Goal

Move project or service work from business intent to tracked tasks, completed work, invoice,
payment, and follow-up without forcing a stock-oriented workflow.

## Preconditions

- The agent is connected to the company's site.
- The customer and service item resolve unambiguously.
- The scope, price, delivery window, and billing trigger are known or carried as open loops.
- The company has an income account suitable for the service being billed.

## Steps

Every document below that is submitted is first reviewed as a draft, and submitted only
after its business effect, references, and amounts are correct
(`erp/procedure/apply-native-operation`).
A record without a draft state and an activity record are written directly.

### 1. Capture the work intent

Record the human source input, customer context, scope, deadline, price assumptions, and
uncertainty on the activity trail.

### 2. Resolve the business object

Resolve the Customer and any existing Project, Quotation, or Sales Order before creating
anything. Reuse the resolved Project when one represents the same work; otherwise create it
with the customer and delivery dates. Create only the missing Task records with their expected
dates. Store the outside reference of the work on each record created, so a later search
finds it.

### 3. Bind the commercial promise to the project

Create and submit the Quotation only when the offer is real. Convert it through ERPNext's
native mapper and set the Project on the resulting Sales Order before submission. Read the
stored Sales Order back and require both its source-quotation link and Project link; a detached
order cannot support project profitability or safe resume.

When the customer pays a deposit before work or milestone billing, compose
`customer-advance-to-invoice` from the submitted Sales Order. Keep the receipt distinct from
earned revenue until the company's configured billing event occurs.

### 4. Record actual work

Record work in a Timesheet whose rows link the Project and Task and carry the actual hours,
costing rate, and billing rate where applicable. Submit the Timesheet before relying on its
figures. Read the Project back and require its total costing and billable amounts to include the
submitted work.

### 5. Bill the agreed trigger

Bill only when the accepted proposal, milestone, completed work, or other agreed trigger is
true. Convert the Sales Order through ERPNext's native invoice mapper. For a partial or
milestone invoice, adjust quantities on the rows produced by the mapper. Preserve every row's
Sales Order link and never replace the item table with newly constructed rows. Before
submission, run the invoice's `add_timesheet_data` controller
method so the submitted Timesheet is linked to the invoice and the returned document is
persisted. Read the draft back and require its Project, Sales Order, Timesheet, and per-row
source links before submitting it. After submission, require the Sales Order's billed
percentage and each row's billed amount to reflect exactly what the invoice consumed; a posted
invoice with unchanged source-row billing state is a duplicate-billing blocker.

### 6. Close work and payment

Record the payment through the native Payment Entry mapper and submit it only when receipt is
true. Complete Tasks when their work is complete. Read back the invoice, Timesheet, Task, and
Project and require: zero invoice outstanding after full payment, billed Timesheet hours,
completed Task progress, and Project sales, cost, billed amount, and margin consistent with the
linked records. Close honoured follow-ups and keep the remaining ones visible as open
follow-ups.

## Outputs

- Native links from commercial promise through Project, Task, Timesheet, invoice, and payment.
- Project totals that expose contracted sales, actual work cost, billed revenue, and margin.
- Open-loop visibility for deadlines, decisions, customer follow-up, and unpaid amounts.

## Recovery

- If the billing trigger is unclear, record activity and follow-up instead of invoicing.
- On a lost create or mapper response, compose `resume-from-native-state`: the invoice resolves
  through the Sales Order's native links and remaining billed quantity or amount.
- If a controller method returns a changed document but saving it fails, preserve the partial
  result, inspect the stored draft, and retry from observed state.
- Correct submitted billing through cancel-and-amend and reverse dependent payments first;
  never edit a submitted invoice in place. After cancelling a partial invoice, read the Sales
  Order's per-row billed quantities back: the rows that invoice consumed are billable again
  only when their billed quantity says so, and the order's billed percentage may still carry
  other invoices.
- If the work belongs to an external booking or production system, capture the ERP trail only
  for the business outcome ERPNext actually owns.

## Discipline

The failure mode is correct-looking books with detached operational records: an order without
its Project, hours without their Task, an invoice without its Timesheet, or a payment reported
without settling the invoice. Track work, obligations, and billing triggers explicitly, then
verify the native links and derived totals that make project profitability trustworthy. Use
ERPNext for the business states it owns, and preserve activity around external systems when
ERPNext is only the accounting or overview layer.

## See Also

- ~/.teleclaude/docs/erp/procedure/resume-from-native-state.md
- ~/.teleclaude/docs/erp/procedure/prove-native-effect.md
- ~/.teleclaude/docs/erp/procedure/customer-invoice-and-payment.md
