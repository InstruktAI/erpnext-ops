# Orient a Person — Procedure

## Goal

The person knows, in their own terms, what they can ask for now and which step comes next,
without being handed a list of everything the system holds.

## Preconditions

- The agent is connected to the company's site.

## Steps

### 1. Establish who is asking

Read the signed-in user and their roles. What they may read and write decides what is
offered: a person with worker access hears about their hours, their jobs and the material
they take; a person who runs the business hears about the business, and that changes to
the system itself go to its administrator (`erpnext/spec/owner-access`).

### 2. Establish what the company uses and what is ready

Read the company's own statement of the parts of the business it runs in ERPNext and which
of them are ready, where the connection or the company's knowledge supplies one. Read the
company's getting-started project: its open tasks are the steps still ahead, in order.

### 3. Answer in a few sentences

Group the answer by the parts of the business this person can act on, in words they use:
customers and jobs, material and stock, hours, invoices. Say what is ready now and what is
not yet. Name no record types and no procedures.

### 4. Lead to the next step

Propose the next open step of the getting-started project when there is one. Otherwise ask
one question: which part they want to start with.

### 5. Start

When the person names a part, load the procedure for it and begin.

## Outputs

- A person who knows what to ask now and what comes next.

## Recovery

- No statement of readiness and no getting-started project exist: offer only what the
  person's roles allow and the site's records show is in use, and say that the company's
  setup is not described yet.

## Discipline

Offer only what is ready for this company and within this person's reach.
