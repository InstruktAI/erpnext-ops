---
description: Recover ERP work from native Tasks, current briefs, expectations, assignment references, and attributable outcomes without rebuilding context from transcripts.
visibility: internal
---

# Operating State — Concept

## What

The current operating plan is the scoped set of native ERPNext work records and
their relationships. A Task represents a coherent outcome that an agent can own
and report. Its scope can require several actions; it is not a promise that those
actions form one transaction. Projects and group tasks organize work when useful.
Available native views expose this same state to humans and agents.

### Current work and retained history

The current brief answers what outcome is sought, which standing mandate or input
supports it, its constraints, the relevant business records, and what remains to
be done. It identifies the executor, any delegated work, and the next contribution
or waiting condition. A waiting expectation names what is expected, from whom or
which system, and when or under what condition it merits attention.

The business trail retains material input, changes of direction, partial effects,
handoffs, and completion reports with evidence references. Revising the current
brief does not erase the history that explains it. A sealed outcome means the
historical business account is preserved by operating discipline; it does not
assert that Task records are technically immutable or that external state cannot
change. Corrections add a trace rather than silently rewriting what happened.

### References and attribution

A work reference includes site, DocType, and native document name. ERPNext supplies
document identity; session identifiers identify executions rather than replace the
work ID. Reassignment changes the executor without changing the business item.
The recorded assignment includes the dispatched session reference and the scope
it received, so a later report can be associated with the right assignment.

Input references retain source identity, time, channel, and the original record or
conversation reference. One input may affect several tasks; one task may address
several inputs. Time windows help find related interactions, but explicit links
establish which work addresses them. An ERP service user or session lineage does
not establish who authored a human instruction.

### Native representation

Agents use the site's actual Task fields and statuses, native dependencies,
activity records, follow-ups, and attachments. The description carries the current
brief; linked activity carries material history. Existing fields and record links
are used where they express the information. A missing structured field is not
silently assumed to exist, and prose references do not claim referential integrity.

`erpnext doc` operations maintain native work records, `erpnext activity` records
and recalls business activity, and `erpnext evidence` attaches supporting files.
The live CLI surface and site metadata determine available operations. This work
model supplies no additional command, custom DocType, automatic session-stamping
hook, or unified event-log service.

### Outcomes and recovery

A worker records its material result, relevant references, and remaining
obligations before marking its native Task complete. Waiting and failed attempts
retain their reason, partial effects, and next required contribution. The
coordinator trusts a complete outcome attributable to the assignment; it reads
deeper evidence only for missing information, contradictions, or a request for
detail. The executor owns native effect verification.

An interrupted request has an uncertain outcome until native state resolves it.
Neither an absent local success record nor a new session authorizes repeating the
effect. A replacement agent reads current work and unresolved outcomes first,
then expands the relevant trail and native records. Raw session transcripts remain
supporting evidence rather than the only home of a commitment.

## Why

A compact current plan makes resumption inexpensive. Retained history explains
changes without forcing every reader to replay them. Native work identity keeps
the board, business records, and delegated execution connected across retries and
session replacement. Explicit input links let the human inspect what their
contribution led to without mistaking temporal proximity for causation.

## See Also

- ~/.teleclaude/docs/erp/principle/operational-attention.md
- ~/.teleclaude/docs/erp/principle/business-input-traceability.md
- ~/.teleclaude/docs/erp/concept/business-activity-trail.md
- ~/.teleclaude/docs/erp/procedure/resume-from-native-state.md
- ~/.teleclaude/docs/erp/procedure/prove-native-effect.md
- ~/.teleclaude/docs/erp/spec/cli-surface.md
- ~/.teleclaude/docs/general/concept/lineage-vs-authorship.md
- https://docs.frappe.io/erpnext/tasks
- https://docs.frappe.io/erpnext/project-views
