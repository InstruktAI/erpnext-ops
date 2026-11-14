---
description: Playbook for a business moving from lead and proposal to procurement, installation, invoice, payment, and follow-up.
activation_trigger: Use when a business needs to turn a lead, site discussion, or accepted proposal into tracked ERP work through installation and payment.
---
# From Proposal To Installation — Procedure

## Required reads

@~/.teleclaude/docs/erp/concept/composable-business-procedures.md
@~/.teleclaude/docs/erp/concept/business-activity-trail.md
@~/.teleclaude/docs/erp/policy/operating-disciplines.md

## Goal

Move installation work from business input to tracked ERP state without losing source
intent, creating premature stock truth, or skipping follow-up.

## Preconditions

- The agent is connected to the company's site.
- The customer, lead, opportunity, or site contact is known or can be resolved.
- Product and service details are known enough to create a draft or activity trail.

## Steps

### 1. Capture the business input

Record on the activity trail the human's source input, site context, formulas,
constraints, photos, supplier remarks, or customer promises. Link the activity to the most
specific known business object.

### 2. Resolve or create the commercial object

Resolve, or create, the Lead, Customer, Opportunity, or Quotation target. Keep uncertain
work in draft state.

### 3. Build the proposal or quotation

Create or update the draft Proposal or Quotation with products, installation services, terms, and
assumptions. Review the preview for missing items, ambiguous parties, tax/currency context,
and linked follow-ups before applying.

### 4. Convert accepted work

When the customer accepts, use workflow commands to move from Quotation toward Sales Order or
Project work. Preserve the acceptance source and create follow-up for unresolved scheduling,
site access, supplier timing, or installation prerequisites.

When a received deposit secures the work, compose `customer-advance-to-invoice` from the
submitted Sales Order. A requested or promised deposit remains follow-up until receipt is true.

### 5. Handle procurement and stock carefully

Record supplier commitments and purchase intent without increasing actual stock before goods
arrive. Only receiving operations create received stock truth. Before the first receipt, verify
that the operating identity has the purchase-master capability and that the effective configured
inventory account resolution mode. For item-wise accounting, the item, item group, or brand
account must resolve to a non-group ledger account. Otherwise, the warehouse account hierarchy
and company default must resolve to a non-group ledger account. A chart group is not a
transaction account. When resuming or billing partially, preserve the
mapper-produced per-row source links and adjust quantities on those rows; replacing the item
list loses the links that ERPNext uses to track received, delivered, and billed quantities.

### 6. Complete, invoice, and collect

After installation completion is confirmed, create or submit the appropriate invoice through
preview/apply. Record payment when received and review open loops.

## Outputs

- Traceable activity trail for customer and supplier input.
- Draft or submitted commercial documents at the right lifecycle stage.
- Follow-ups for unresolved supplier, stock, customer, or installation obligations.
- Invoice and payment state when the work is complete.

## Recovery

- If a preview shows stock effects before goods are physically received, stop and use a draft
  procurement or activity record instead.
- If customer acceptance is unclear, record activity and follow-up rather than submitting.
- If duplicate customer, item, supplier, or quotation candidates appear, resolve them before
  applying the mutation.

## Discipline

The failure mode is treating an intuitive, verbal installation operation as clean ERP state too early.
Use the preview to keep stock, selling, installation, and accounting states distinct.
Record business memory immediately; mutate financial or stock truth only when the business
event is real.
