---
description: Add a person to the company — a staff record, a login created from it, and what their colleagues need to know about them — when the company's owner or organizer asks for it.
activation_trigger: Use when an owner or organizer says a person joins the company, asks how to add an employee or a worker, or asks why a colleague cannot log in or log their work.
---

# Add a Person — Procedure

## Required reads

- @~/.teleclaude/docs/erpnext/procedure/apply-native-operation.md

## Goal

A person who joins the company has a staff record and their own login, holds worker
access, and their colleagues are shown what working together takes.

## Preconditions

- The agent is connected to the company's site under a login with owner access
  (`erpnext/spec/owner-access`).
- The company has workers, by what it states about itself.

## Steps

### 1. Establish who joins and that they are not there yet

Take the person's name, the e-mail address their mail goes to, their job and the day they
start. Search the staff records and the logins for that name and address
(`erpnext/procedure/resume-from-native-state`). A staff record that exists is continued from
where it stands.

### 2. Ask what the staff record needs and what colleagues need

Ask for what ERPNext requires on a staff record: the person's gender and their birthday.
Ask who should be called if something happens to them: the name, the phone number and how
that person is related. Where the company works on location, also ask whether there is
anything about their health that their colleagues should be able to tell a medic, such as an
allergy, a condition or their blood type, and take only what the person wants their
colleagues to know. Offer to record a few lines about themselves for their colleagues
(`erpnext/spec/worker-access`).

### 3. Add the staff record

Create the staff record for the company with what steps 1 and 2 gave, and read it back.

### 4. Create the login from the staff record

Create the person's login from the staff record with ERPNext's own method for it
(`erpnext/spec/owner-access`), and read back that the staff record names the login.

### 5. Confirm

Tell the owner or organizer that the person is added and what the person can do once signed
in: log their hours, the material they take and when they are away. Say that ERPNext mails
the person a link to set a password when it creates a login, that this login cannot see
whether that mail went out, and that the person confirms receiving it.

## Outputs

- A staff record for the person, linked to their own login.
- The owner or organizer knows the person still has to confirm the mail with the link.

## Recovery

- **ERPNext refuses to create the login.** The login asking does not hold owner access, or
  a login with that address exists. Read which, and continue from the existing login or
  record the wish for the system administrator (`erpnext/policy/operating-disciplines`).
- **The mail did not arrive.** Where the site lets people sign in with a password, tell the
  person to request a new link for their address on the site's sign-in page. Otherwise
  record the wish for a new link for the system administrator
  (`erpnext/policy/operating-disciplines`).
- **A person wants a different address on their login.** A login's address is changed by
  the system administrator, because it is how the person is recognized wherever the
  company's people reach the system from outside it, such as a messaging app. Record the
  wish for the system administrator (`erpnext/policy/operating-disciplines`).
- **The person leaves the company.** Mark their staff record as left; ERPNext switches
  their login off (`erpnext/spec/owner-access`).

## See Also

- ~/.teleclaude/docs/erpnext/spec/owner-access.md
- ~/.teleclaude/docs/erpnext/spec/worker-access.md
- ~/.teleclaude/docs/erpnext/procedure/resume-from-native-state.md
- ~/.teleclaude/docs/erpnext/policy/operating-disciplines.md
- https://github.com/frappe/frappe/blob/v16.56.0/frappe/core/doctype/user/user.py
- https://github.com/frappe/frappe/blob/v16.56.0/frappe/www/login.html
