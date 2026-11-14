Read first, unless already read in this conversation: `references/procedure-prove-native-effect.md`.

# Resume From Native State — Procedure

## Goal

Continue an ERP operation from what the site actually holds, so a lost response,
an interrupted run, or a repeated instruction never records the same business
event twice or skips a step that already succeeded.

## Preconditions

- The agent is connected to the company's site.
- The business event has at least one identity the site can be searched by: an
  external reference, a source document and its native links, or a natural key.

## Steps

### 1. Resolve by the strongest identity the event carries

Search native state before every create (`erp/policy/operating-disciplines`). A
document resolves by the
outside reference of its event, stored on it when it was created: the
customer's order number, the supplier's invoice number, the bank reference. A
mapped document resolves through the source document's native links and
reference rows. A master resolves by its natural key. A lost response is not
evidence of absence.

### 2. Read the current state, not the remembered one

Read the resolved documents back and take their current `docstatus`, status,
outstanding amounts, allocations, and per-row links as the starting point.
Native reports for the relevant party and date supply the same facts from a
second view. An earlier observation, a cached total, or the last command an
agent remembers running is not current state.

### 3. Decide from what exists

- The event is fully recorded: reuse it and stop.
- The event is partly recorded: continue from the stored partial result. A
  controller method that changed a document but failed to save leaves the
  stored draft as the point to continue from. A mapped draft is continued, not
  mapped again.
- The event is not recorded: proceed with the operation, carrying the same
  identity so the next resumption resolves it.

### 4. Refuse the false continuation

A mapper may produce a zero-value draft from a source that has nothing left to
settle; such a draft is not evidence that a payment is due. A second conversion
of the same source creates a second document. A create repeated under a changed
reference creates a parallel record. None of these is a way to resume.

## Outputs

- Exactly one native record per real business event, resolved before any write.
- An operation continued from stored state, with the identity that lets the
  next resumption find it.

## Recovery

- If no identity resolves and the response is lost, search by every identity
  the event carries and by the native timeline of the source document before
  concluding the event is unrecorded.
- If two records represent the same event, stop; correcting the duplicate is a
  reversal with its own dependency order, proven per `prove-native-effect`.

## Discipline

The failure this guards against is a second invoice, payment, or advance created
because the first one's response was lost or the run was resumed from memory.
Native state is the only starting point; the agent's recollection of what it
sent is not.

## See Also

- references/procedure-prove-native-effect.md
- this skill's SKILL.md
- this skill's SKILL.md
