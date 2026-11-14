Read first, unless already read in this conversation: `references/concept-business-activity-trail.md`.

# Custom Order To Fulfillment — Procedure

## Goal

Track a custom product order from customer intent through production and delivery without
confusing configured demand, raw material availability, finished goods, and invoice state.

## Preconditions

- The agent is connected to the company's site.
- The customer or order source is known or can be resolved.
- The product configuration, measurements, quantity, or batch details are known enough to
  record as draft commercial or activity state.

## Steps

### 1. Capture configured demand

Record the customer's configuration, dimensions, quality requirements, deadline, delivery
expectations, and source input on the activity trail or in a draft commercial document.

### 2. Resolve master data

Resolve the Customer, Item, variant,
production target, or project/order document. Avoid creating duplicate customer or item
records.

### 3. Create draft order state

Create or update the draft Quotation, Sales Order, Project, or Task structure that best
matches the company's operating model. Let the preview surface missing fields, links, and
side effects.

When a received deposit confirms the custom order, compose `customer-advance-to-invoice`
from the submitted Sales Order. Keep a promised deposit as follow-up rather than recording
cash that has not arrived.

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
into one premature ERP action. Keep each business state explicit. Use the preview to see
what will become real before applying it.
