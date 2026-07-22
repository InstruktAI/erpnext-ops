---
description: Playbook for service or project businesses moving from proposal and tasks to completed work, invoice, payment, and unresolved follow-up.
activation_trigger: Use when a service, production, tour, or custom project business needs to track work, obligations, billing, and payment without stock-centric assumptions.
---
# Project Work To Invoice — Procedure

## Required reads

@~/.teleclaude/docs/erp/concept/composable-business-procedures.md
@~/.teleclaude/docs/erp/concept/business-activity-trail.md
@~/.teleclaude/docs/erp/policy/operating-disciplines.md

## Goal

Move project or service work from business intent to tracked tasks, completed work, invoice,
payment, and follow-up without forcing a stock-oriented workflow.

## Preconditions

- The company context is active in `erpnext`.
- The customer and service item resolve unambiguously.
- The scope, price, delivery window, and billing trigger are known or carried as open loops.
- The company has an income account suitable for the service being billed.

## Steps

For every mutation below, inspect the CLI preview first and apply only after the proposed
business effect, references, and amounts are correct.

### 1. Capture the work intent

Record the human source input, customer context, scope, deadline, price assumptions, and
uncertainty with `erpnext activity capture`.

### 2. Resolve the business object

Resolve the Customer and any existing Project, Quotation, or Sales Order before creating
anything. Reuse the resolved Project when one represents the same work; otherwise create it
with the customer and delivery dates. Create only the missing Task records with their expected
dates. Use source tokens for series-named records so a repeated create returns the existing
record.

### 3. Bind the commercial promise to the project

Create and submit the Quotation only when the offer is real. Convert it through ERPNext's
native mapper and set the Project on the resulting Sales Order before submission. Read the
stored Sales Order back and require both its source-quotation link and Project link; a detached
order cannot support project profitability or safe resume.

### 4. Record actual work

Record work in a Timesheet whose rows link the Project and Task and carry the actual hours,
costing rate, and billing rate where applicable. Submit the Timesheet before relying on its
figures. Read the Project back and require its total costing and billable amounts to include the
submitted work.

### 5. Bill the agreed trigger

Bill only when the accepted proposal, milestone, completed work, or other agreed trigger is
true. Convert the Sales Order through ERPNext's native invoice mapper. Before submission, run
the invoice's `add_timesheet_data` controller method so the submitted Timesheet is linked to
the invoice and the returned document is persisted. Read the draft back and require its
Project, Sales Order, and Timesheet links before submitting it.

### 6. Close work and payment

Record the payment through the native Payment Entry mapper and submit it only when receipt is
true. Complete Tasks when their work is complete. Read back the invoice, Timesheet, Task, and
Project and require: zero invoice outstanding after full payment, billed Timesheet hours,
completed Task progress, and Project sales, cost, billed amount, and margin consistent with the
linked records. Close honoured follow-ups and keep the remaining ones visible through
`erpnext activity open-loops`.

## Outputs

- Native links from commercial promise through Project, Task, Timesheet, invoice, and payment.
- Project totals that expose contracted sales, actual work cost, billed revenue, and margin.
- Open-loop visibility for deadlines, decisions, customer follow-up, and unpaid amounts.

## Recovery

- If the billing trigger is unclear, record activity and follow-up instead of invoicing.
- If a create or mapper response is lost, inspect the source's native links and remaining billed
  quantity or amount before resuming; never map the same source blindly.
- If a controller method returns a changed document but saving it fails, preserve the partial
  result, inspect the stored draft, and retry from observed state.
- Correct submitted billing through cancel-and-amend and reverse dependent payments first;
  never edit a submitted invoice in place.
- If the work belongs to an external booking or production system, capture the ERP trail only
  for the business outcome ERPNext actually owns.

## Discipline

The failure mode is correct-looking books with detached operational records: an order without
its Project, hours without their Task, an invoice without its Timesheet, or a payment reported
without settling the invoice. Track work, obligations, and billing triggers explicitly, then
verify the native links and derived totals that make project profitability trustworthy. Use
ERPNext for the business states it owns, and preserve activity around external systems when
ERPNext is only the accounting or overview layer.
