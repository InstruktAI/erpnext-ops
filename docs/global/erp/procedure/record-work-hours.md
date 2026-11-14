---
description: Record the hours a person worked on a job under their own staff record at the moment they report them, and keep them correctable by that person.
activation_trigger: Use when a person says how long they worked somewhere, asks which hours are logged for them or for a job, or wants to correct hours they logged.
---

# Record Work Hours — Procedure

## Required reads

- @~/.teleclaude/docs/erp/procedure/apply-native-operation.md

## Goal

Hours a person worked are recorded once, on the job they belong to, under that person's
staff record, when the person reports them.

## Preconditions

- The agent is connected to the company's site.
- The person reporting has a staff record linked to their login.
- The job resolves to one project, or the hours are recorded with what identifies the job
  and a follow-up.

## Steps

### 1. Establish whose hours these are

Hours are recorded for the signed-in person, on their own staff record. A person who
administers staff may report hours for someone else; the record then names that person and
its description says who reported them.

### 2. Resolve the job

Resolve the project from what the person says: its name, the customer, the place. When
several projects fit, ask which job in the person's own terms. When none fits, record the
hours with the customer and a description of the job, and leave a follow-up for whoever
plans the work to attach them to a project.

### 3. Establish the time worked

The person reports a duration with breaks already taken off; take it as given. Place it on
the day they name. When they give no start time and report as they leave, end the stretch
at the moment of reporting; otherwise start it at the beginning of the working day they
name. Say which time was used.

### 4. Establish that it is not recorded yet

Search the person's hours for that day and job (`erp/procedure/resume-from-native-state`).
ERPNext refuses a stretch that overlaps another of the same person; an overlap means the
hours are already there or the times need the person's correction.

### 5. Record and submit

Record one stretch per job: the staff record, the company, the project on the record and on
its time line, the activity, the start, the hours, and what was done in the person's words.
Submit it (`erp/procedure/apply-native-operation`). Hours post nothing to the books, so a
reported stretch is a determined action (`erp/principle/determinate-autonomy`).

### 6. Confirm

Read the record back and confirm in one sentence: the job, the day, the hours.

### Correcting and asking

A person corrects their own hours: a draft is changed, a submitted record is cancelled and
recorded again as its amendment. A question about hours (a person's week, who worked on a
job on a day) is answered from the submitted records and names any drafts separately.

## Outputs

- One submitted hours record per stretch of work, on the right job, under the right person.
- A follow-up where the job could not be resolved to a project.

## Recovery

- ERPNext refuses the record for lack of permission: the person's login has no worker
  access or no staff record; hand it to the company's administrator
  (`erp/spec/worker-access`).
- ERPNext refuses an overlap: read the existing stretch, tell the person what is already
  recorded, and record only what is missing.

## See Also

- ~/.teleclaude/docs/erp/spec/worker-access.md
- ~/.teleclaude/docs/erp/procedure/project-work-to-invoice.md
