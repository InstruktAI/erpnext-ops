Read first, unless already read in this conversation: `references/procedure-apply-native-operation.md`.

# Record Material Taken — Procedure

## Goal

Material that leaves or changes its stock location is recorded when it happens, by the
person who moved it, with the job it was for, so stock and job cost can be settled from
facts instead of memory.

## Preconditions

- The agent is connected to the company's site.
- Each material resolves to one item, and the place it was taken from to one stock location.

## Steps

### 1. Resolve the job and the material

Resolve the project the material is for. Resolve each material the person names to one
existing item by its name or code. When several items fit, ask which one in the person's
terms. When none fits, record nothing for that material and leave a follow-up naming what
was taken; a new item is created by whoever maintains the company's materials.

### 2. Resolve where it came from and how much

Take the quantity in the item's own unit. Take the stock location from what the person
says; when the company keeps stock in one place, that place; otherwise ask.

### 3. Establish that it is not recorded yet

Search the person's material records for that day, job and item
(`resume-from-native-state`).

### 4. Record the movement as a draft

Record one stock movement for the report: material taken for a job leaves its location for
that project; material moved between locations names both; material brought back returns to
its location. Put the person's words in its remarks. Leave it as a draft.

### 5. Confirm

Read the draft back and confirm in one sentence: what, how many, from where, for which job.

### Settling the drafts

Submitting a movement changes stock and its value in the books. A person who administers
stock submits the drafts (`apply-native-operation`), after the stock preview
and the ledger preview show the expected effect.

## Outputs

- One draft stock movement per report, linked to the job, in the reporter's name.
- A follow-up for material that matched no item.

## Recovery

- The person corrects their own draft; a submitted movement is corrected by a person who
  administers stock.
- ERPNext refuses for lack of permission: hand it to the company's administrator
  (`erp/spec/worker-access`).

## See Also

- references/spec-worker-access.md
