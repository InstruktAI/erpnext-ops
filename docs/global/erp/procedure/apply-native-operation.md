---
description: Apply one ERPNext change through the erpnext CLI and act on its result as exactly one of a fixed set of outcomes, so every agent knows what each result means and what follows it.
activation_trigger: Use before running any erpnext command with --apply.
---

# Apply a Native Operation — Procedure

## Required reads

- @~/.teleclaude/docs/erp/procedure/resume-from-native-state.md
- @~/.teleclaude/docs/erp/procedure/prove-native-effect.md

## Goal

Make every change an agent applies to ERPNext follow one loop with a closed set of
outcomes, so the result of any apply has one meaning and one next move.

## Preconditions

- The company context is active in `erpnext`.
- A governing procedure or assignment names the business outcome this change serves.

## Steps

### 1. State the expected effect

From the governing procedure, state the effect in business terms: which document,
which status it reaches, which amounts and links it carries.

### 2. Establish whether it is already recorded

Follow `erp/procedure/resume-from-native-state`. When the event is already recorded,
continue from what exists instead of applying again.

### 3. Preview

Run the command without `--apply`. Require the preview's target, its status, the
company, and the payload to match the expected effect. When they do not, stop and
resolve the mismatch before applying.

### 4. Apply once

Apply the previewed command once. Never repeat an apply to find out what happened.

### 5. Classify the result

Every result is exactly one of these outcomes:

| Result | Meaning | Next move |
| --- | --- | --- |
| Success envelope | The site accepted the change | Prove the effect with `erp/procedure/prove-native-effect` before reporting it done |
| Failure before anything was sent (invalid payload or configuration) | Nothing reached the site | Correct the input and return to step 3 |
| Failure the site returned for the request itself (authentication, permission, missing record, validation), or a transition the site answered without taking effect | The site refused; nothing changed | Read current state; never work around the refusal; follow the governing procedure's decision point or carry an open loop |
| Failure reported after the site accepted part of the change (a created document whose verification failed, a follow-up save the site refused) | Part of the change may be stored | Read the stored state; continue from the stored partial result with `erp/procedure/resume-from-native-state`; never apply the same change again |
| Refusal because the effect already exists | The change was made before | Read back; when the stored state matches the expected effect, treat the step as done |
| Failure whose outcome is unknown (transport failure, unexpected response, failed read-back) | The change may or may not have landed | Apply nothing further until `erp/procedure/resume-from-native-state` has settled the state |
| Success on an operation the governing procedure says completes in the background | Accepted, not yet done | Record the expectation, set a reminder to re-check, and never repeat the apply |

A failure's `kind`, `message`, and any document reference it carries identify its row.

### 6. Record the outcome

Record the outcome and its evidence on the work item: the command, the result, and
the read-back that established the stored state.

## Outputs

- One classified outcome per apply, with its evidence on the work item.
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
