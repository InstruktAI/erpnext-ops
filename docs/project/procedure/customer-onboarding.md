---
description: Set ERPNext up for a company being onboarded — the intake ERPNext needs, what each answer leads to, the site, the company's statement, its people's access, the plugin for its Claude organization and the connector, and the closing test that proves the access before the company's people start.
activation_trigger: Use when a company that works with ERPNext is being onboarded, when an onboarded company's ERPNext setup is to be brought in line with this procedure, or to find where a step of that setup stands.
---

# Customer Onboarding — Procedure

## Goal

A company's people work in the company's own ERPNext, through their own Claude and
through the company's agent, each person under their own ERPNext login. The whole
organization holds the `erpnext` plugin and receives its updates without anyone acting,
and the provider holds no seat in the company's Claude organization. What the company is
and uses in ERPNext is on record as its facts, and its access has been proven by test.

The company itself is onboarded by `general/procedure/company-onboarding/overview`: its intake, its
home on the computer, its knowledge, its messaging connector, its services and the proof
that a person is answered. That procedure runs this one where the company works with
ERPNext, before ERPNext is connected as one of the company's services.

## Preconditions

- The company is being onboarded through `general/procedure/company-onboarding/overview`, whose
  intake gave the company's name, key, owner and organizer with their email addresses.
- The provider is present to answer the intake.
- The company has a Claude organization with an owner.
- The provider administers the GitHub organization that publishes the plugin.

## Steps

### 1. Take the intake

Ask the provider these questions and take the answers as the company's facts, beside
those the general intake gave.

| Question | Fact |
| --- | --- |
| In which country does it operate? | Country |
| Does it have people who do its work and report it themselves? | Has workers |
| Does it work on location? | Works on location |
| Which document storage does it use, and does it already have a structure there? | Document storage, document mapping |
| Which system issues its legal invoices and carries its statutory books? | Legal invoicing |

### 2. Set up what each answer leads to

| Fact | What it leads to |
| --- | --- |
| Name, key, owner, organizer (general intake) | The site's name and the company on it (step 3); the two logins with owner access (step 5) |
| Country | The country, currency and formats the site is set up with (step 3) |
| Has workers | Staff administration is declared for the site; worker access is applied (`erpnext/spec/worker-access`) |
| Works on location | Projects carry a place field; where the documents convention applies, the place is part of a folder's name (`erpnext/policy/documents-convention`); each new person is asked what their colleagues should be able to tell a medic (`erpnext/procedure/add-a-person`) |
| Document storage, document mapping | Customers and projects carry a link field to their documents folder; the outcome of the document mapping is recorded (`erpnext/policy/documents-convention`) |
| Legal invoicing in another system | ERPNext's standard chart of accounts; invoices and payments stay closed in the company's statement until how the two systems work together is settled |
| Legal invoicing in ERPNext | The chart is decided with the company's accountant (`project/erpnext-ops/policy/chart-before-booking`) |

### 3. Create the site and the company

Create the company's site (`project/erpnext-ops/procedure/create-site`), with the
applications step 2 calls for in the same site declaration, then set the company up on it
(`project/erpnext-ops/procedure/company-site-setup`).

Read the chart of accounts the setup created before anything else depends on it: have
ERPNext accept its customer and supplier accounts as the company's defaults, and read how
it classifies customers, suppliers and stock. A chart that fails either is replaced before
the first booking (`project/erpnext-ops/policy/chart-before-booking`).

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

Create the provider's own login on the site under the provider's one address, the same on
every site, and give it every enabled role of the site that can be given to a login. Read
those roles from the site at that moment; never choose them. The roles ERPNext assigns by
itself are not given, and ERPNext withholds the staff roles from a login that has no staff
record.

With those roles the login holds every right that a permission rule of the site gives to a
role it can hold. The comparison of step 9 still shows differences of three kinds, and they
are expected: a rule for the built-in Administrator login alone, a rule that holds only for
the owner of a record, such as another person's reminder, and a rule for a staff role
withheld from this login.

A difference in the rules is not yet a refusal, and a rule is not yet a right. ERPNext
decides some acts by a check of their own, which allows an act without a rule, as for
exporting and importing under the administrator role, or refuses it despite one, as for a
notification addressed to someone else.

The roles are those of the moment they were given, and nothing on the site renews them.
Whoever installs an application on the site, or adds or enables a role, by whatever path,
gives the provider's login its roles again and repeats the provider's row of the closing
test before that work is finished.

Switch off renaming for logins on the site with a property setter on the document type
`User`: property `allow_rename`, type `Check`, value `0`. A login's address is how a person
is known, also outside ERPNext, and as shipped a person can change their own.

Each person receives ERPNext's own link to set a password. Set the lifetime of that link on
the site long enough for a person who reads mail once a day.

Apply each access by its specification's table. The means are the agent's own; what counts
is the closing test of step 9.

### 6. Give the organization the plugin

Each company has one private repository in the provider's GitHub organization, named
`company-<name>`. It is the company's home, where what the company keeps as its own
knowledge goes, and the catalog its Claude organization reads. Its catalog file lists the
`erpnext` plugin from this repository at a released version, and its updater moves that
version to the latest release.

A company's own skill is a folder in the same repository, listed in the catalog file by its
path. This procedure creates no skill for the company; until the company has one, its
catalog lists the `erpnext` plugin alone.

Copy the catalog file and its updater from an existing company's repository into the one
the general onboarding created. Have the owner confirm their GitHub account name, then
invite that account to the repository with administrator rights.

The owner then connects the catalog once, in the Claude organization's settings for
plugins and skills: sync from GitHub, the company's repository, automatic sync on,
and the plugin's default access set to installed for everyone. Those settings are the
organization's admin pages on the web; a person's own plugin screen, which is all the
mobile app shows, installs for that person only. Where the dialog asks for the Claude
GitHub App on the repository, the provider approves it in the GitHub organization.

After a release of the plugin, start each catalog's updater; it also runs on its own
schedule.

### 7. Connect ERPNext

For the people's own Claude, the owner adds the company's site as a custom connector for
the organization, with the address of the site's MCP endpoint. Each person then connects
it once and signs in with their own ERPNext login.

For the company's agent, ERPNext is one of the company's services, connected by the
general onboarding once the site and its people's logins exist: its server is the site's
MCP endpoint (`project/erpnext-ops/adr/0001-frappe-mcp-server-is-the-erp-agent-surface`),
and each person signs in there with their own ERPNext login, the account whose email
address their messaging workspace shows for them.

### 8. Set the model

Tell the company's people which model to use for this work and have the owner set it
where the organization's settings offer the choice
(`project/erpnext-ops/policy/model-floor`).

### 9. Prove the access

Accept the site's access on the site itself (`erpnext/policy/operating-disciplines`). Create
one test login with owner access. Where the company has workers, have that login add two
staff records and create their logins. Under each login make the attempts below and compare
the outcome with the table. Remove the test logins and everything they created. An outcome
that differs is corrected and the test repeated before any person of the company starts.
The provider's row is read from the site's permission rules, changes nothing and creates
nothing.

| Login | Attempt | Outcome |
| --- | --- | --- |
| Owner | Add a customer, a material with a value, a stock location and a project | Allowed |
| Owner | Change a setting of a business area and add a tax category | Allowed |
| Owner | Add a staff record and create its login | Allowed |
| Owner | Change the company's statement in the system settings | Refused |
| Owner | Give a login a role, or change another person's login | Refused |
| Owner | Add a field to a form, a workflow or an integration, or start a bulk import | Refused |
| Provider | For every kind of record, compare what the roles of the provider's own login allow with what the site's permission rules allow to any enabled role, and list each difference with the kind of record, the action and why | Every difference is one of the expected kinds of step 5; none is unexplained |
| Worker | Record and submit own hours on a project, then cancel them | Allowed |
| Worker | Read another worker's hours, a project, a customer, a material and a stock level | Allowed |
| Worker | Record a material entry as a draft | Allowed |
| Worker | Read a colleague's staff record, a material and a leave request | Shown with the birthday, the emergency contact, the health details and the reason; without the bank details and without the material's value |
| Owner | Read the same staff record and material | Shown with the bank details and the value |
| Worker | Add a calendar entry and record a leave request; another worker reads the leave request | Allowed |
| Worker | Change, cancel or delete another worker's hours, draft, calendar entry or leave request | Refused |
| Worker | Submit a material entry, or approve own leave request | Refused |
| Worker | Rename own login to a valid address that no login holds | Refused, because renaming is switched off |
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
- **The provider's own login is refused something.** Read which check refused the act.
  Where a permission rule gives the right to a role the login does not hold, give the login
  its roles again (step 5) and repeat the provider's row of the closing test. Where the
  rule is of an expected kind of step 5, or the act's own check refuses it whatever roles
  the login holds, no role changes the outcome; report the act with the check that refused
  it.
- **A password link expired before it was used.** Send a new one after lengthening the
  link's lifetime on the site.
- **A person's address was wrong, or changes.** The agent carries this out and keeps the
  sides in line; it is not left to a person. Read every place that knows the person by
  the address: the login and the staff record in ERPNext, and the person's account in the
  company's messaging app as that app reports it. In ERPNext, under the administrative
  agent login: set the `allow_rename` property setter on `User` to `1`, rename the login,
  and set the property setter back to `0`. Set it back whatever the rename did, also after
  a failure, before anything else; where it cannot be set back, stop, report that
  renaming is left on, and keep the rename as unfinished work. A rename refused with a
  database deadlock is repeated. Read back that the setting is `0`, that a rename is
  refused again, and that the staff record and the work assigned to the person name the
  new login. Nothing is removed. Then the other sides: change the address where the
  agent's access reaches it, tell the person exactly what only they can change, and where
  the company's agent holds a sign-in for the person, have it ended so that the person
  signs in again under the new address. Finish by reading back that ERPNext and the
  messaging app hold the same address, and send a new link where the person has not
  signed in yet.

## See Also

- [project/erpnext-ops/policy/model-floor](../policy/model-floor.md)
- [project/erpnext-ops/policy/chart-before-booking](../policy/chart-before-booking.md)
- [project/erpnext-ops/adr/0001-frappe-mcp-server-is-the-erp-agent-surface](../adr/0001-frappe-mcp-server-is-the-erp-agent-surface.md)
- [project/erpnext-ops/adr/0002-plugin-reaches-an-organization-through-its-own-private-catalog](../adr/0002-plugin-reaches-an-organization-through-its-own-private-catalog.md)
- [project/erpnext-ops/procedure/create-site](create-site.md)
- [project/erpnext-ops/procedure/company-site-setup](company-site-setup.md)
- general/procedure/company-onboarding/overview
- general/procedure/company-onboarding/service-connection
- [erpnext/principle/business-and-system](../../domain/erpnext/principle/business-and-system.md)
- [erpnext/policy/documents-convention](../../domain/erpnext/policy/documents-convention.md)
- [erpnext/procedure/person-onboarding](../../domain/erpnext/procedure/person-onboarding.md)
- [erpnext/spec/worker-access](../../domain/erpnext/spec/worker-access.md)
- [erpnext/spec/owner-access](../../domain/erpnext/spec/owner-access.md)
- [erpnext/procedure/orient-a-person](../../domain/erpnext/procedure/orient-a-person.md)
- https://claude.com/docs/plugins/org-sync
- https://claude.com/docs/plugins/admin
