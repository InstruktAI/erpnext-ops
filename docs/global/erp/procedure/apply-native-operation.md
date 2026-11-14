---
description: Apply one ERPNext change and act on its result as exactly one of a fixed set of outcomes, so every agent knows what each result means and what follows it.
activation_trigger: Use before any write to ERPNext — creating, changing, submitting, cancelling or deleting a record, or calling a method that changes one.
---

# Apply a Native Operation — Procedure

## Required reads

- @~/.teleclaude/docs/erp/procedure/resume-from-native-state.md
- @~/.teleclaude/docs/erp/procedure/prove-native-effect.md

## Goal

Make every change an agent applies to ERPNext follow one loop with a closed set of
outcomes, so the result of any write has one meaning and one next move.

## Preconditions

- The agent is connected to the company's site and has established the user and company
  it acts as.
- A governing procedure, an assignment, or a person's request names the business outcome
  this change serves.

## Steps

### 1. State the expected effect

From the governing procedure, state the effect in business terms: which document,
which status it reaches, which amounts and links it carries.

### 2. Establish whether it is already recorded

Follow `resume-from-native-state`. When the event is already recorded, continue from
what exists instead of writing again.

### 3. Draft and preview

Read the document type's live metadata and compose the write from fields it has
(`erp/policy/operating-disciplines`).

For a submittable document, create it as a draft and read the stored draft back. Require
its party, company, dates, lines, amounts, taxes and links to match the expected effect.
When submitting it posts to the books, read its ledger preview and require the expected
accounts and amounts. This read-back, with the ledger preview where one applies, is the
preview the procedures speak of.

A record without a draft state takes effect when written. State the target and the
intended change first.

When anything does not match, correct the draft or stop and resolve the mismatch before
going on.

### 4. Apply once

Submit, cancel, write or delete once. A change to an existing record carries the
modification marker last read (`erp/policy/operating-disciplines`). Never repeat a write
to find out what happened.

### 5. Classify the result

Every result is exactly one of these outcomes:

| Result | Meaning | Next move |
| --- | --- | --- |
| The server reports success | The site accepted the change | Prove the effect with `prove-native-effect` before reporting it done |
| The server rejects the call's arguments | Nothing reached the site | Correct the input and return to step 3 |
| The site refuses the request itself (authentication, permission, missing record, validation), or answers a transition without taking effect | The site refused; nothing changed | Read current state; never work around the refusal; follow the governing procedure's decision point or carry an open loop |
| The site refuses because the record changed since it was read | Another party wrote in between; nothing changed | Read the record again and re-establish the expected effect from step 1 |
| A failure after the site stored part of the change (a created document whose follow-up write was refused) | Part of the change is stored | Read the stored state; continue from the stored partial result with `resume-from-native-state`; never apply the same change again |
| Refusal because the effect already exists | The change was made before | Read back; when the stored state matches the expected effect, treat the step as done |
| A failure whose outcome is unknown (timeout, transport failure, unexpected response, failed read-back) | The change may or may not have landed | Write nothing further until `resume-from-native-state` has settled the state |
| Success on an operation the governing procedure says completes in the background | Accepted, not yet done | Record the expectation as a follow-up to re-check, and never repeat the write |

The result's error text and any document reference it carries identify its row.

### 6. Record the outcome

Record the outcome and its evidence on the work item, or on the business record's
activity trail when no work item exists: the change, the result, and the read-back that
established the stored state.

## Outputs

- One classified outcome per write, with its evidence recorded.
- No change reported done without proof of its stored effect.

## Recovery

- A result that fits no row is handled as unknown: no further writes until the
  stored state is settled from native records.
- A refusal that blocks the business outcome is carried as an open loop or surfaced
  as the missing decision; it is never retried with altered input to get past it.

## See Also

- ~/.teleclaude/docs/erp/procedure/resume-from-native-state.md
- ~/.teleclaude/docs/erp/procedure/prove-native-effect.md
- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/spec/agent-surface.md
