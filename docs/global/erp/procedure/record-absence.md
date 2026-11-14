---
description: Record when a person is away or not available, as a leave request where the company tracks leave and as a calendar entry otherwise, so whoever plans the work can see it.
activation_trigger: Use when a person says when they are away or not available, asks who is away, or wants to change or withdraw an absence they reported.
---

# Record Absence — Procedure

## Required reads

- @~/.teleclaude/docs/erp/procedure/apply-native-operation.md

## Goal

Everyone who plans or shares the work can see who is away and when, from a record the
person made themselves when they knew.

## Preconditions

- The agent is connected to the company's site.
- The person reporting has a login; a leave request also needs their staff record.

## Steps

### 1. Establish whose absence and when

The absence is recorded for the signed-in person. Take the first and last day from what
they say, and whether a day is partial. A person who administers staff may record an
absence for someone else.

### 2. Choose the record

Where the site tracks leave and the person has a staff record, the absence is a leave
request with the company's leave type that fits what the person says: holiday, illness,
unpaid time. Otherwise it is a public calendar entry naming the person and the period.

### 3. Establish that it is not recorded yet

Search the person's leave requests and calendar entries for an overlapping period
(`erp/procedure/resume-from-native-state`).

### 4. Record

Record the leave request and leave it open: a person who approves leave decides it. Record
a calendar entry as public, covering the whole period.

### 5. Confirm

Read the record back and confirm in one sentence: who, from when to when, and whether it
still waits for approval.

### Asking who is away

Answer from the leave requests, open and approved, and the public calendar entries that
overlap the period asked about. Say which absences still wait for approval.

## Outputs

- One leave request or one public calendar entry per absence, in the person's name.

## Recovery

- ERPNext refuses the leave request for lack of leave balance or allocation: record a
  public calendar entry for the period and leave a follow-up for whoever administers
  staff.
- The person changes or withdraws their own open request or calendar entry; an approved
  leave request is changed by a person who approves leave.

## See Also

- ~/.teleclaude/docs/erp/spec/worker-access.md
