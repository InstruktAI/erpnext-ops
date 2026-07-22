---
description: Reframe a consequential human business instruction once, obtain plain-language approval, and apply only the approved intent.
activation_trigger: Use when a human instruction would create, submit, cancel, delete, send, or materially change business state and a wrong interpretation would matter.
---
# Confirm Business Intent — Procedure

## Required reads

@~/.teleclaude/docs/erp/principle/determinate-autonomy.md
@~/.teleclaude/docs/erp/principle/business-input-traceability.md
@~/.teleclaude/docs/erp/policy/operating-disciplines.md

## Goal

The person confirms the business outcome the agent understood before consequential ERP state
changes, without being asked to interpret ERPNext fields, commands, or document mechanics.

## Preconditions

- A human instruction has been translated into a concrete business outcome.
- The affected company, parties, amounts, quantities, dates, and commitments are known well
  enough to preview, or the remaining uncertainty is named.

## Steps

### 1. Choose one business approval boundary

Group the ERP operations that realize one outcome. Ask for approval of the outcome once, not
once per document or field. Routine consequences determined by the approved outcome remain
autonomous.

### 2. Reframe the understood outcome

State in plain business language what changes, for whom, in which company's books, and the
important amounts, quantities, and dates. Name uncertainty instead of filling it silently.
Do not expose JSON, DocType names, API methods, or CLI instructions as the approval question.

### 3. State the lasting consequence

Name what becomes harder or impossible to reverse: submitted figures reach the books, stock
moves, deletion removes a record, and an outbound message leaves the system. Explain the
business consequence, not the implementation.

### 4. Ask for plain approval

End with: `If this is correct, say approve.` Approval binds to the stated outcome and its
reviewed preview. A substantive change to party, scope, amount, quantity, date, destination,
or lasting consequence invalidates that approval and requires a new reframe.

### 5. Apply and preserve the trace

After approval, apply the reviewed operation through `erpnext`. Preserve the source
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
Confirmation happens at the business-outcome boundary, not at every mechanical step, so it
catches semantic mistakes without turning autonomous operation into approval fatigue. The
person approves meaning; the agent owns ERPNext mechanics.

## See Also

- ~/.teleclaude/docs/erp/concept/composable-business-procedures.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
