# Confirm Business Intent — Procedure

## Goal

The person resolves a genuine business decision boundary before the agent applies the affected
ERP state, without being asked to interpret ERPNext fields, commands, or document mechanics.

## Preconditions

- Evidence, company configuration, party preferences, templates, ERPNext state, and standard
  ERP practice do not determine the full business outcome.
- A human instruction has been translated into the concrete outcome the agent believes was
  intended.
- The affected company, parties, amounts, quantities, dates, and commitments are known well
  enough to preview, or the remaining uncertainty is named.

## Steps

### 1. Choose one business approval boundary

First resolve the instruction through available evidence, configuration, preferences,
templates, ERPNext state, and standard ERP practice. If those determine the correct action,
review the clean preview and proceed autonomously; do not manufacture an approval boundary.

When human judgment remains necessary, group the ERP operations that realize one outcome. Ask
for approval of the outcome once, not once per document or field. Routine consequences
determined by the approved outcome remain autonomous.

### 2. Reframe the understood outcome

State in plain business language what changes, for whom, in which company's books, and the
important amounts, quantities, and dates. Name uncertainty instead of filling it silently.
Do not expose JSON, DocType names, API methods, or tool calls as the approval question.

### 3. State the lasting consequence

Name what becomes harder or impossible to reverse: submitted figures reach the books, stock
moves, deletion removes a record, and an outbound message leaves the system. Explain the
business consequence, not the implementation.

### 4. Ask for plain approval

End with: `If this is correct, say approve.` Approval binds to the stated outcome and its
reviewed preview. A substantive change to party, scope, amount, quantity, date, destination,
or lasting consequence invalidates that approval and requires a new reframe.

### 5. Apply and preserve the trace

After approval, apply the reviewed operation. Preserve the source
instruction, interpretation, approval, resulting records, and any unresolved follow-up in
the linked activity trail.

## Outputs

- One approved, human-readable statement of business intent.
- ERP records that realize exactly that intent.
- A trace linking source input, approval, applied records, and remaining commitments.

## Recovery

- If approval does not arrive, keep uncertain state as draft activity and carry any deadline
  as an open loop; do not apply the consequential mutation.
- If the approved intent changes, discard the old preview and reframe the changed outcome.
- If application partly succeeds, report what landed, resolve the native document links, and
  resume or compensate from observed ERPNext state rather than replaying blindly.

## Discipline

The failure mode is faithfully executing the wrong interpretation of a human instruction.
Confirmation happens only at an unresolved business-outcome boundary, not because work is
important or at every mechanical step. It catches semantic mistakes without turning
determinate operation into approval fatigue. The person supplies missing judgment; the agent
owns ERPNext mechanics.

## See Also

- references/concept-composable-business-procedures.md
- references/concept-business-activity-trail.md
- this skill's SKILL.md
