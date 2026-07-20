---
description: Playbook for custom product businesses moving from configured customer order to production batch, delivery, invoice, and follow-up.
activation_trigger: Use when a business sells custom or batch-produced goods that must be tracked from order intake through production, delivery, invoice, and customer follow-up.
---
# Custom Order To Fulfillment — Procedure

## Required reads

@~/.teleclaude/docs/erp/concept/composable-business-procedures.md
@~/.teleclaude/docs/erp/concept/business-activity-trail.md
@~/.teleclaude/docs/erp/policy/operating-disciplines.md

## Goal

Track a custom product order from customer intent through production and delivery without
confusing configured demand, raw material availability, finished goods, and invoice state.

## Preconditions

- The company context is active in `erpnext`.
- The customer or order source is known or can be resolved.
- The product configuration, measurements, quantity, or batch details are known enough to
  record as draft commercial or activity state.

## Steps

### 1. Capture configured demand

Record the customer's configuration, dimensions, quality requirements, deadline, delivery
expectations, and source input with `erpnext activity capture` or a draft commercial document.

### 2. Resolve master data

Use `erpnext resolve` and `erpnext doc` preview to identify the Customer, Item, variant,
production target, or project/order document. Avoid creating duplicate customer or item
records.

### 3. Create draft order state

Create or update the draft Quotation, Sales Order, Project, or Task structure that best
matches the company's operating model. Let the preview surface missing fields, links, and
side effects.

### 4. Track production and delivery obligations

Carry production, batching, courier, delivery, and customer communication as tasks, events,
or linked activity. Do not represent goods as finished or delivered before the business event
is confirmed.

### 5. Invoice and close

Create or submit invoice state only when the company's billing rule is satisfied. Record
payment and close remaining open loops after fulfillment.

## Outputs

- Traceable customer order context.
- Draft or submitted sales/project documents at the correct lifecycle stage.
- Production and delivery follow-ups.
- Invoice and payment state when appropriate.

## Recovery

- If configuration details are incomplete, capture activity and follow-up instead of forcing a
  formal order.
- If the preview indicates stock or accounting effects earlier than intended, stop and keep the
  operation in draft/activity state.
- If delivery is custom or courier-driven, preserve the delivery promise and follow-up rather
  than relying on implicit stock movement alone.

## Discipline

The failure mode is collapsing custom intent, production work, delivered goods, and invoicing
into one premature ERP action. Keep each business state explicit. Use the CLI preview to see
what will become real before applying it.
