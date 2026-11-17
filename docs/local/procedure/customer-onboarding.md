---
description: Add a company — the intake the provider answers, what each answer leads to, the setup of site, access, skill and connector, and the closing test that proves the access before the company's people start.
activation_trigger: Use when the provider wants to add a company, when an onboarded company's setup is to be brought in line with this procedure, or to find where a step of an onboarding stands.
---

# Customer Onboarding — Procedure

## Goal

A company's people work in the company's own ERPNext by talking to their own Claude. Each
person acts under their own ERPNext login, the whole organization holds the `erpnext` plugin
and receives its updates without anyone acting, and the provider holds no seat in the
company's Claude organization. What the company is and uses is on record as its facts, and
its access has been proven by test.

## Preconditions

- The provider is present to answer the intake.
- The company has a Claude organization with an owner.
- The provider administers the GitHub organization that publishes the plugin.

## Steps

### 1. Take the intake

Ask the provider these questions and take the answers as the company's facts.

| Question | Fact |
| --- | --- |
| What is the company called, who owns it, who organizes its work, and which e-mail address receives each one's mail? | Name, slug, owner, organizer |
| In which country does it operate? | Country |
| Does it have people who do its work and report it themselves? | Has workers |
| Do its workers read everything about people and jobs, or only their own work? | Workers' reach |
| Does it work on location? | Works on location |
| Which document storage does it use, and does it already have a structure there? | Document storage, document mapping |
| Which messaging app do its people use? | Messaging |
| Which system issues its legal invoices and carries its statutory books? | Legal invoicing |

### 2. Set up what each answer leads to

| Fact | What it leads to |
| --- | --- |
| Name, slug, owner, organizer | The site's name and the company on it (step 3); the two logins with owner access (step 5) |
| Country | The country, currency and formats the site is set up with (step 3) |
| Has workers | Staff administration is declared for the site; worker access is applied (`erpnext/spec/worker-access`) |
| Workers' reach | The reach worker access states. A company that needs another reach waits until that reach is specified as its own kind of access, with its rights table and the outcomes its acceptance expects |
| Works on location | Projects carry a place field; where the documents convention applies, the place is part of a folder's name (`erpnext/policy/documents-convention`) |
| Messaging | Recorded as a fact; nothing is set up from it |
| Document storage, document mapping | Customers and projects carry a link field to their documents folder; the outcome of the document mapping is recorded (`erpnext/policy/documents-convention`) |
| Legal invoicing in another system | ERPNext's standard chart of accounts; invoices and payments stay closed in the company's statement until how the two systems work together is settled |
| Legal invoicing in ERPNext | The chart is decided with the company's accountant (`local/projects/erpnext-ops/policy/chart-before-booking`) |

### 3. Create the site and the company

Create the company's site through the deployment that hosts ERPNext
(`apps/procedure/erpnext/create-site`), with the applications step 2 calls for in the same
site declaration, then onboard the company on it (`erpnext/procedure/company-onboarding`).

Read the chart of accounts the setup created before anything else depends on it: have
ERPNext accept its customer and supplier accounts as the company's defaults, and read how
it classifies customers, suppliers and stock. A chart that fails either is replaced before
the first booking (`local/projects/erpnext-ops/policy/chart-before-booking`).

### 4. Record the company's facts and statement

Write the company's own statement: what it does, how its people work, its facts from the
intake, which parts of the business are ready in ERPNext and which are not yet. Store it in
the site setting that ERPNext hands to every connecting agent. End the statement with where
it is kept, the agent instructions field of the site's system settings, and with who
changes it: the provider, on a wish that a person's own Claude records as a to-do assigned
to the provider (`general/principle/priming`).

Create the company's getting-started project in ERPNext with one open task per step still
ahead, in order (`erpnext/procedure/orient-a-person`).

### 5. Give the people their access

Onboard the owner and the organizer with owner access (`erpnext/spec/owner-access`,
`erpnext/procedure/person-onboarding`). The administrator role stays with the provider's
own login and the administrative agent login (`erpnext/principle/business-and-system`).

Each person receives ERPNext's own link to set a password. Set the lifetime of that link on
the site long enough for a person who reads mail once a day.

Apply each access by its specification's table. The means are the agent's own; what counts
is the closing test of step 9.

### 6. Give the organization the plugin

Each company has its own small private catalog repository in the provider's GitHub
organization, named for the company. It holds the catalog file that lists the `erpnext`
plugin from this repository at a released version, and the updater that moves that version
to the latest release.

Create the company's catalog by copying an existing one. Have the owner confirm their
GitHub account name, then invite that account to the catalog with administrator rights.

The owner then connects the catalog once, in the Claude organization's settings for
plugins and skills: sync from GitHub, the company's catalog repository, automatic sync on,
and the plugin's default access set to installed for everyone. Those settings are the
organization's admin pages on the web; a person's own plugin screen, which is all the
mobile app shows, installs for that person only. Where the dialog asks for the Claude
GitHub App on the repository, the provider approves it in the GitHub organization.

After a release of the plugin, start each catalog's updater; it also runs on its own
schedule.

### 7. Connect ERPNext

The owner adds the company's site as a custom connector for the organization, with the
address of the site's MCP endpoint. Each person then connects it once and signs in with
their own ERPNext login.

### 8. Set the model

Tell the company's people which model to use for this work and have the owner set it
where the organization's settings offer the choice
(`local/projects/erpnext-ops/policy/model-floor`).

### 9. Prove the access

Accept the site's access on the site itself (`erpnext/policy/operating-disciplines`). Create
one test login with owner access. Where the company has workers, have that login add two
staff records and create their logins. Under each login make the attempts below and compare
the outcome with the table. Remove the test logins and everything they created. An outcome
that differs is corrected and the test repeated before any person of the company starts.

| Login | Attempt | Outcome |
| --- | --- | --- |
| Owner | Add a customer, a material with a value, a stock location and a project | Allowed |
| Owner | Change a setting of a business area and add a tax category | Allowed |
| Owner | Add a staff record and create its login | Allowed |
| Owner | Change the company's statement in the system settings | Refused |
| Owner | Give a login a role, or change another person's login | Refused |
| Owner | Add a field to a form, a workflow or an integration, or start a bulk import | Refused |
| Worker | Record and submit own hours on a project, then cancel them | Allowed |
| Worker | Read another worker's hours, a project, a customer, a material and a stock level | Allowed |
| Worker | Record a material entry as a draft | Allowed |
| Worker | Read a colleague's staff record and a material | Shown, without the birth date and without the material's value |
| Owner | Read the same staff record and material | Shown with both |
| Worker | Add a calendar entry and record a leave request; another worker reads the leave request | Allowed |
| Worker | Change, cancel or delete another worker's hours, draft, calendar entry or leave request | Refused |
| Worker | Submit a material entry, or approve own leave request | Refused |
| Worker | Read an invoice, a payment or a ledger entry | Refused |
| Worker | Read a record about another person, such as their attendance | Refused |
| Worker | Create a project or a task | Refused |

### 10. Check with the owner

With the owner present, ask their Claude what it can do for them and what comes next. The
answer names the company's own parts of the business and the next open step, and offers
nothing the statement marks as not ready. Then have one person record something small
under their own login and read it back as another.

## Outputs

- The company's site, with its company, its people's logins and its statement holding its
  facts.
- Access applied and proven by the closing test.
- The company's private catalog, connected to its Claude organization with automatic sync.
- The connector added for the organization and connected by each person.
- A getting-started project whose open tasks are the steps still ahead.

## Recovery

- **The owner finds only "add marketplace" and "upload plugin".** That is the personal
  plugin screen. The organization-wide install is in the organization's admin settings on
  the web. To keep working meanwhile, the owner adds this public repository there as a
  marketplace for themselves, and removes that copy once the catalog is connected.
- **The owner has no GitHub account and will not make one.** The owner uploads the
  released plugin file once for the organization and again for each release, or each
  person adds this public repository as a marketplace from their own plugin settings.
- **The owner's Claude does not mention the company.** The statement did not reach it
  through the connector: have the owner store the same text in the instructions of the
  company's Claude organization.
- **The chart fails the check after the company exists.** While the company has no ledger
  entries, replace the chart on the site's host in one transaction, after a site backup
  and a rehearsal on the sandbox with a throwaway company. Remove every record that points
  at the company's accounts and then the accounts, set the company's chart and save the
  company so that ERPNext creates the new accounts and tax templates, then have ERPNext
  set the company's default accounts. Afterwards read the company's defaults and the
  classification of customers, suppliers and stock, and bring the company's statement and
  its getting-started tasks in line.
- **A password link expired before it was used.** Send a new one after lengthening the
  link's lifetime on the site.
- **A person's address was wrong.** Rename the login to the right address and send a new
  link; do not create a second login.

## See Also

- docs/local/policy/model-floor.md
- docs/local/policy/chart-before-booking.md
- docs/local/adr/0002-plugin-reaches-an-organization-through-its-own-private-catalog.md
- ~/.teleclaude/docs/erpnext/principle/business-and-system.md
- ~/.teleclaude/docs/erpnext/policy/documents-convention.md
- ~/.teleclaude/docs/erpnext/procedure/company-onboarding.md
- ~/.teleclaude/docs/erpnext/procedure/person-onboarding.md
- ~/.teleclaude/docs/erpnext/spec/worker-access.md
- ~/.teleclaude/docs/erpnext/spec/owner-access.md
- ~/.teleclaude/docs/erpnext/procedure/orient-a-person.md
- https://claude.com/docs/plugins/org-sync
- https://claude.com/docs/plugins/admin
