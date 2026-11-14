---
description: Work that writes to a company's ERPNext is done by Sonnet 5.5 or a more capable model; the smallest model class is not used for it.
---

# Model Floor — Policy

## Rules

- **Work that writes to a company's ERPNext runs on Sonnet 5.5 or a more capable model.**
  This holds for a person's own agent and for an agent the platform dispatches.
- **The smallest class (Haiku) is not used for ERP work**, including reading work that
  leads to a decision about the books.
- **The floor is set where the model is chosen.** Whoever onboards a company tells its
  people which model to use for this work and has the organization's owner set it where the
  organization's settings offer the choice. Whoever dispatches an agent for ERP work names a
  model at or above the floor.
- **A model below the floor is admitted only after the same requests that qualified the
  floor have been run against it** and none ended in a wrong booking.

## Rationale

The ERP doctrine is prose a model follows; no tool enforces its sequence of reading,
drafting, previewing and reading back (`erp/policy/operating-disciplines`). Its protection
is therefore exactly as strong as the model's adherence to instructions it has loaded. A
model that departs from loaded instructions under ordinary requests posts wrong bookings
that look plausible, and more instructions do not correct a model that does not follow the
ones it has.

A person's agent is the model that person selected. Delegation to a stronger model is that
agent's own decision, so a weak first model cannot be relied on to hand the work up.

## Scope

Every agent that works in a company's ERPNext through the `erp` domain, wherever it runs.

## Enforcement

The onboarding procedure sets the floor for a company's people
(`local/projects/erpnext-ops/procedure/customer-onboarding`). Reviewers of a dispatch or an
agent definition for ERP work check the model it names.

## Exceptions

None.
