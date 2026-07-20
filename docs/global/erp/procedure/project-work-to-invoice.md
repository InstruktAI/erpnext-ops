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
- The customer, project, opportunity, event, or production context is known or can be resolved.
- The scope, price, deadline, or next action is known enough to capture.

## Steps

### 1. Capture the work intent

Record the human source input, customer context, scope, deadline, price assumptions, and
uncertainty with `erpnext activity capture`.

### 2. Resolve the business object

Use `erpnext resolve` to find or create the Customer, Opportunity, Project, Task, Quotation,
or Sales Order target. Keep uncertain commitments in draft state.

### 3. Track work and obligations

Create tasks, events, or linked project work for promised deliverables, approvals, deadlines,
and dependencies. Carry unresolved decisions as open loops.

### 4. Confirm completion before billing

Use the preview/apply flow to create or submit invoice state only when the billing event is
true: accepted proposal, milestone reached, work completed, or another agreed billing trigger.

### 5. Record payment and review open loops

Record payment when received and use `erpnext activity open-loops` or `erpnext ops open-loops`
to find unresolved project, customer, or billing work.

## Outputs

- Traceable project/service activity.
- Draft or submitted project, task, sales, invoice, and payment state as appropriate.
- Open-loop visibility for deadlines, decisions, and customer follow-up.

## Recovery

- If the billing trigger is unclear, record activity and follow-up instead of invoicing.
- If the preview shows accounting effects the human did not authorize, stop before applying.
- If the work belongs to an external booking or production system, capture the ERP trail only
  for the business outcome ERPNext actually owns.

## Discipline

The failure mode is forcing project or service businesses through a stock-heavy mental model.
Track work, obligations, and billing triggers explicitly. Use ERPNext for the business states
it owns, and preserve activity around external systems when ERPNext is only the accounting or
overview layer.
