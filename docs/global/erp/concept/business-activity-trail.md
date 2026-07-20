---
description: The ERP activity trail is the queryable memory layer over communications, comments, tasks, events, notes, and linked business documents.
---
# Business Activity Trail — Concept

## Required reads

@~/.teleclaude/docs/erp/principle/business-input-traceability.md

## What

The business activity trail is the queryable memory layer over human input, agent actions,
follow-ups, and linked ERP documents. Agents access it through `erpnext activity` and related
CLI commands. It lets agents answer questions such as what happened last week, what is due
tomorrow, what was said about a customer, or why a supplier commitment exists.

The trail is assembled behind the CLI from ERPNext and Frappe primitives:

- `Communication` records human interactions and messages with medium, sender, content,
  date, and optional document references.
- `Comment` records timeline notes attached to any referenced document.
- `ToDo` records open loops, due dates, assignees, and referenced documents.
- `Event` records calendar-style calls, meetings, reminders, and participants.
- `CRM Note` records notes inside Leads and Opportunities.
- Business documents such as Lead, Opportunity, Customer, Supplier, Quotation, Sales Order,
  Purchase Order, Project, Task, Invoice, and Payment Entry provide the objects the activity
  links to.

## Why

Humans do not keep a perfect operational overview. They trust the system to remember what was
said, what was planned, what is still open, and what should happen next. The `erpnext`
activity surface gives agents the cheap recall path needed to support that trust without
loading every ERP document in full.

The trail also separates memory from mutation. A verbal supplier promise can be captured as
activity and follow-up before it becomes a Purchase Order or received stock. A customer call
can be recorded before it becomes an Opportunity, Quotation, or support Issue. The activity
trail keeps business context visible while the stronger ERP documents remain reserved for
confirmed business state.

## See Also

- ~/.teleclaude/docs/erp/concept/composable-business-procedures.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
