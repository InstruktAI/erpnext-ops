---
description: Meaningful human business input that affects or may later justify ERP state leaves a traceable activity record.
---
# Business Input Traceability — Principle

## Principle

Meaningful human business input that causes, influences, or may later justify ERP state
leaves a trace. The trace preserves the source input, the agent's interpretation, the
linked business objects, the action taken, and any unresolved follow-up.

Traceability is not a demand to record every utterance. It is the discipline of preserving
business-relevant input before it disappears into memory, chat history, or an irreversible
ERP mutation.

## Rationale

Business owners often speak from motion: a call in the car, a supplier deal at a job site,
a half-remembered customer promise, or a loose instruction about something due next week.
Those inputs may be too partial for a formal ERP document, but too valuable to lose.

ERPNext already carries timelines, communications, comments, tasks, and events. Agents need
a shared rule for when those primitives become the business memory layer. Without that rule,
agents either mutate ERP state too early or leave the human with no auditable trail for why
the later state exists.

## Implications

- Input that creates an ERP mutation, business commitment, or future obligation must be
  traceable to the human source.
- Ambiguous but business-relevant input is recorded as activity before it is promoted into
  stock, accounting, procurement, sales, or project truth.
- Verbatim input is preserved when practical; otherwise a faithful summary is recorded with
  the channel, source party, timestamp, and uncertainty.
- Unresolved commitments produce an open loop: a `ToDo`, `Event`, task, or equivalent
  follow-up linked to the relevant business object.
- Agents use activity trails before answering recall questions such as what happened last
  week, what is due tomorrow, or what remains unfinished for a party.

## Tensions

- **Traceability vs noise.** Recording every conversational fragment makes timelines useless.
  The trigger is business relevance, not raw speech.
- **Verbatim source vs privacy.** Preserve exact wording when it matters for auditability, but
  avoid storing sensitive or irrelevant personal material.
- **Draft memory vs ERP truth.** A trace can record intent or uncertainty. It must not imply
  received stock, posted accounting, accepted orders, or completed work unless the business
  event is actually true.
- **Autonomy vs noise.** When the trace is clearly needed, create it. When the business
  relevance cannot be determined, carry the uncertainty as an open loop or ask the focused
  question that resolves it.
