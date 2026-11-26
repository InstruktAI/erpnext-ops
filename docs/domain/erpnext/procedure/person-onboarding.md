---
description: Onboard a person onto a company's ERPNext site with permission-scoped access.
activation_trigger: Use when a human user needs ERPNext login access, roles, and permissions on a company's site.
---

# ERP Person Onboarding — Procedure

## Goal

A person can log into the company's ERPNext site with access scoped to the roles and data
they need.

## Preconditions

- The company's site is provisioned and the company is onboarded.
- The person's name and email address are known.
- The active operator or administrator identity has authority to create and scope users.

## Steps

### 1. Create the ERPNext user

Create an ERPNext user for the person on the company's site.

### 2. Scope permissions

Assign only the roles and permission rules the person needs. When the site hosts multiple
companies of the business group, scope the user to the intended company's data.

Where the company's own access mandate gives the people who report their work on site
this reach, such a person receives worker access. The company's owner or organizer adds the
person (`erpnext/procedure/add-a-person`); the login created that way holds worker access
without anything further being assigned (`erpnext/spec/worker-access`).

The company's owner and its organizer receive owner access: the business roles without the
administrator role (`erpnext/spec/owner-access`).

### 3. Send the login email

Trigger ERPNext's welcome or password-reset email so the person can set credentials and sign
in. Verify the message reaches the outgoing queue and the recipient; a successful User write
does not prove mail delivery.

### 4. Verify access

Confirm the user can access the intended data and holds no roles beyond those assigned.

## Outputs

- Permission-scoped ERPNext user for the person on the company's site.
- Login email sent through ERPNext.
- Verified permission scope.

## Recovery

- If the email does not arrive, inspect the outgoing queue and site error log, confirm the
  resolved sender is a plain email address with its display name configured separately, and
  resend the native password-reset message after repair.
- If access is too broad, disable the user or revoke roles until the permission scope is
  corrected.

## Discipline

The failure mode is over-broad human access to company financial data. Create the user only
with the roles needed, verify the scope, and use ERPNext's mail path for login delivery. Do
not send passwords or secrets through chat.

## See Also

- [erpnext/concept/multi-tenancy](../concept/multi-tenancy.md)
- [erpnext/spec/worker-access](../spec/worker-access.md)
- [erpnext/spec/owner-access](../spec/owner-access.md)
