---
description: The ERP activity trail is the queryable memory layer over communications, comments, tasks, events, notes, and linked business documents.
---
# Business Activity Trail — Concept

## Required reads

@~/.teleclaude/docs/erpnext/principle/business-input-traceability.md

## What

The business activity trail is the queryable memory layer over human input, agent actions,
follow-ups, and linked ERP documents. Agents read and write it as native records
(`erpnext/spec/agent-surface`). It lets agents answer questions such as what happened last week,
what is due tomorrow, what was said about a customer, or why a supplier commitment exists.

The trail consists of ERPNext and Frappe primitives:

- `Communication` records human interactions and messages with medium, sender, content,
  date, and optional document references.
- `Comment` records timeline notes attached to any referenced document.
- `ToDo` records open loops, due dates, assignees, and referenced documents.
- `Event` records calendar-style calls, meetings, reminders, and participants.
- `CRM Note` records notes inside Leads and Opportunities.
- Business documents such as Lead, Opportunity, Customer, Supplier, Quotation, Sales Order,
  Purchase Order, Project, Task, Invoice, and Payment Entry provide the objects the activity
  links to.

The trail connects source input to interpretation, native work references, and
material outcomes. Human inputs retain their source identity and time; automated
work names its standing responsibility and triggering observation rather than
inventing a new human instruction. A session reference identifies the agent's
execution separately from the ERP user under which the operation ran.

The current plan belongs in the work record; the trail explains its evolution.
It preserves partial results, uncertain effects, changed instructions, and who
holds the next contribution. A recorded completion is a business audit fact,
not a claim that the world remains unchanged or that native history is immutable.
Installed change tracking, permissions, and retention determine the site's
technical history guarantees.

Agents resolve the relevant source records when answering what a person asked
within a time window and which work addressed it. Related inputs and outcomes are
linked explicitly; temporal proximity alone does not establish fulfillment.

## Why

Humans do not keep a perfect operational overview. They trust the system to remember what was
said, what was planned, what is still open, and what should happen next. The activity
records give agents the cheap recall path needed to support that trust without
loading every ERP document in full.

The trail also separates memory from mutation. A verbal supplier promise can be captured as
activity and follow-up before it becomes a Purchase Order or received stock. A customer call
can be recorded before it becomes an Opportunity, Quotation, or support Issue. The activity
trail keeps business context visible while the stronger ERP documents remain reserved for
confirmed business state.

## See Also

- ~/.teleclaude/docs/erpnext/concept/operating-state.md
- ~/.teleclaude/docs/erpnext/concept/business-relationships.md
- ~/.teleclaude/docs/erpnext/concept/composable-business-procedures.md
- ~/.teleclaude/docs/erpnext/policy/operating-disciplines.md
- ~/.teleclaude/docs/erpnext/spec/agent-surface.md
