---
description: Apply one ERPNext change and act on its result as exactly one of a fixed set of outcomes, so every agent knows what each result means and what follows it.
activation_trigger: Use before any write to ERPNext — creating, changing, submitting, cancelling or deleting a record, or calling a method that changes one.
---

# Apply a Native Operation — Procedure

## Required reads

- [erpnext/procedure/resume-from-native-state](resume-from-native-state.md)

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

### 3. Establish the target and preview

Read the document type's live metadata and compose any write from fields it has
(`erpnext/policy/operating-disciplines`). What precedes the write depends on the operation:

- **Creating a document that is submitted later.** Create it as a draft and read the stored
  draft back. Require its party, company, dates, lines, amounts, taxes and links to match
  the expected effect.
- **Submitting a draft.** Read the draft as stored now. When submitting posts to the books,
  read its ledger preview and require the expected accounts and amounts. When it moves
  stock, read its stock preview and require the expected items, warehouses and quantities.
- **Cancelling a submitted document.** Read the document and the documents that depend on
  it, and establish that the governing procedure's dependency order is met.
- **Changing or deleting an existing record.** Read the record as stored now and state the
  intended change against it.
- **Writing a record that has no draft state.** State the target and the intended change;
  it takes effect when written.

The draft read-back, with the ledger and stock previews where they apply, is the preview
the procedures speak of. When anything does not match the expected effect, correct the
draft or stop and resolve the mismatch before going on.

### 4. Apply once

Write, submit, cancel or delete once. A change and a submission carry the version reviewed
in step 3, and a cancellation follows an immediate read of the record
(`erpnext/policy/operating-disciplines`). Never repeat a write to find out what happened.

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

- [erpnext/procedure/resume-from-native-state](resume-from-native-state.md)
- [erpnext/procedure/prove-native-effect](prove-native-effect.md)
- [erpnext/policy/operating-disciplines](../policy/operating-disciplines.md)
- [erpnext/spec/agent-surface](../spec/agent-surface.md)
