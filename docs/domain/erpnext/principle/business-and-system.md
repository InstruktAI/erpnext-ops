---
description: The company's people own everything about their business in the system, its configuration included; the system administrator owns the system they work in; ERPNext holds the line through each person's own login.
---

# Business and System — Principle

## Principle

**The business is the company's.** The company's people decide and do everything about
their own business in the system. They record it, organize it and configure it: customers,
suppliers, materials and their variants, prices, projects, hours, stock, their staff and
who works there, tax and payment setup, the settings of each business area, their
accounts.

**The system is its administrator's.** The system administrator builds and keeps the
system the company works in: the company's statement that tells agents how this company
works, the kinds of access that exist and what each allows, the administrator role, the
shape of forms and fields, workflows, print layouts, integrations, system-wide settings,
and the skill agents work by.

**Where the two meet.** When the business needs the system to change, the company says
what it needs and the administrator builds it. What the company's statement says is the
company's to decide and the administrator's to write.

**How it is held.** ERPNext enforces the line through each person's own login. It is not
left to an agent's judgment in a conversation.

## Rationale

- A company that cannot configure its own business in its own system depends on someone
  else for its daily decisions, and that someone is made the judge of business choices
  that are not theirs.
- The parts of the system are what every person's agent works by. A change to one of them
  alters the work of everyone, so it is made by the one party who answers for the system.
- An agent carries out what it is asked. A line that rests on the agent declining a
  request does not hold; a right the person's login lacks does.
- Whose something is can be answered for a case nobody listed. How much harm a mistake in
  it could do cannot, and a line drawn there takes the business's own configuration away
  from it.

## Implications

- A record type or a setting is placed on one side by asking whose it is.
- Where ERPNext puts an act of the business behind the administrator role, the
  administrator opens that act to the company's people.
- An agent working under a company login that is asked for a change to the system records
  the wish for the administrator (`erpnext/policy/operating-disciplines`).
- A company's own configuration is changed by its people with the care every write gets
  (`erpnext/policy/operating-disciplines`).

## Tensions

- **The company's choice, the administrator's kinds.** Giving a person access is the
  company's act; which kinds of access exist and what each allows is the system. The
  company chooses among kinds it did not define.
- **A costly business mistake.** A wrong tax rate or account is the company's to make and
  to correct. The administrator advises and does not take the setting away.
- **ERPNext's own grouping.** ERPNext bundles rights by its roles, which cross this line
  in places. The rights are adjusted to the line; the line is not moved to the roles.

## See Also

- [erpnext/spec/owner-access](../spec/owner-access.md)
- [erpnext/spec/worker-access](../spec/worker-access.md)
- [erpnext/policy/operating-disciplines](../policy/operating-disciplines.md)
