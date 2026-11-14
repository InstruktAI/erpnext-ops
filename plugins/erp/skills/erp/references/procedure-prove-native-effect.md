# Prove Native Effect — Procedure

## Goal

Establish that a mutation produced the intended business state by reading it
back from three independent native views, and report only what those views
agree on.

## Preconditions

- The mutation has been applied and its response, or its absence, is known.
- The expected effect is stated in business terms: which document reaches which
  state, which amounts move between which accounts, which links must exist.

## Steps

### 1. Stored document state

Read the document back and require the stored `docstatus`, status, and amounts
to match the expected effect. A command that reported success while the stored
state did not change is a failed operation.

### 2. Native links

Read the links the effect depends on: reference rows on a payment, per-row
source links on a mapped document, advance rows on an invoice, the source's own
consumed quantities or outstanding amount. The link is the evidence; a total
that looks right without the link is not.

### 3. Ledger and reports

Read the native General Ledger entries or the report that owns the effect for
the exact voucher and date. Require each expected debit and credit, require
total debits to equal total credits, and require the party's outstanding state
to reflect the mutation. A balanced voucher that encodes the wrong accounts is
not proof.

### 4. Effects outside ERPNext are separate facts

A submitted Payment Entry records accounting state; whether a bank executed the
transfer is the bank's fact. A cancelled invoice does not recall a document the
customer received. Whether goods arrived is the warehouse's fact, not the
purchase record's. Each such fact is proven by the system that owns it or
carried as an open loop until it is.

### 5. Proving a correction

A correction is proven the same way, in dependency order: cancel or amend the
dependent document first, then the one it depended on, and after each step read
back that exactly the intended contribution was removed. Other documents may
still hold the same party or source; verify the contribution, not a zero total.
Cancellation never recalls an effect outside ERPNext; that effect is compensated
separately and proven by its owner.

## Outputs

- Stored state, native links, and ledger entries that agree on the effect.
- Any effect outside ERPNext named as a proven fact or as an open loop.

## Recovery

- If the three views disagree, stop and leave the discrepancy open; investigate
  the native links before recording another mutation on top of it.
- If a correction leaves a dependent document holding a stale link or
  allocation, the correction is incomplete; continue in dependency order before
  reporting it.

## Discipline

The failure this guards against is reporting an invoice, payment, or close as
done because one view looked right: a plausible grand total, a mapper that
returned a document, a submit that returned success. Proof crosses the three
views, and anything outside ERPNext is proven by its owner or stays open.

## See Also

- references/procedure-resume-from-native-state.md
- this skill's SKILL.md
