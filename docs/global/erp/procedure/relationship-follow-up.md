---
description: Assess a missing reply or unmet external commitment, choose authorized outreach, continued waiting, or an owner handoff, and preserve the next expectation.
visibility: internal
activation_trigger: When an ERP commitment awaits a customer, supplier, contact, or owner contribution and its due condition or new evidence calls for reassessment.
---

# Relationship Follow-Up — Procedure

## Required reads

@~/.teleclaude/docs/erp/principle/operational-attention.md
@~/.teleclaude/docs/erp/principle/determinate-autonomy.md
@~/.teleclaude/docs/erp/concept/business-relationships.md
@~/.teleclaude/docs/erp/concept/business-activity-trail.md
@~/.teleclaude/docs/erp/policy/operating-disciplines.md

## Goal

Obtain the contribution needed to progress a business commitment, or preserve
appropriate waiting or owner involvement under the relationship's mandate.

## Preconditions

- A native work or follow-up record identifies the commitment and relevant party.
- The company mandate and available relationship evidence can be consulted.

## Steps

1. Read the expectation, its attention condition, affected business commitments,
   and activity since the last contact. Check whether a response or native event
   already satisfied it or whether another agent or the owner is handling it.
2. Read the applicable company rules, explicit contact preferences, agreements,
   and evidenced response patterns. Keep uncertain observations distinct from
   expressed preferences. Assess the consequence of waiting and whether another
   contact is useful now.
3. Choose the next contribution: continue waiting with a recorded attention
   condition, perform outreach within the delegated class, or prepare an owner
   handoff. Use `erp/procedure/confirm-business-intent` when the remaining business
   decision is the human's. Apply the existing freeform/templated communication
   boundary from `erp/principle/determinate-autonomy`.
4. For outreach, establish the intended recipient and a configured delivery
   surface appropriate to their preferences. Load that surface's contract.
   Request the specific missing information respectfully using the relevant
   business context. A preferred channel alone does not establish delivery
   capability; a failed send does not authorize switching channels silently.
5. For an owner handoff, provide the work reference, missing contribution,
   consequence or deadline, prior attempts, and the question a personal
   conversation needs to settle. Use the owner's configured communication route;
   do not assume the platform's customer-session escalation command applies to
   every business owner.
6. Record what was attempted and its observed result against the native record.
   Preserve delivery uncertainty. Update the expected response and next
   contributor, using the applicable reminder or periodic attention mechanism.
   Close the loop only when its actual commitment is satisfied; sending a request
   or handing it to the owner does not itself fulfill that commitment.
7. Record newly expressed preferences and relevant observations with their source
   and time. Revise the current relationship understanding without changing the
   company's authority or shared procedures through that record.

## Outputs

- A fulfilled expectation or an explicit remaining contribution and attention
  condition.
- An activity record linking the contact or handoff to its work and observed result.
- Source-backed relationship updates where the interaction supplies them.

## Recovery

- **Contradictory preferences:** resolve from the latest applicable explicit
  evidence or ask the relevant owner before selecting a consequential route.
- **Uncertain delivery:** inspect the delivery surface before resending; retain
  the uncertainty when delivery cannot be established.
- **Unavailable route:** retain the follow-up and surface the specific missing
  capability without claiming contact occurred.
- **No response:** reassess the expectation and consequences under the mandate;
  do not repeat outreach mechanically or abandon the commitment.

## See Also

- ~/.teleclaude/docs/erp/procedure/operating-cycle.md
- ~/.teleclaude/docs/general/policy/reminder.md
- ~/.teleclaude/docs/general/spec/tools/telec-cli.md
