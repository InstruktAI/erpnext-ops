---
description: Bring a new company from nothing to its people working in their own ERPNext through their own Claude — site, company, people, the plugin for the whole organization, the connector, and the first check with the owner.
activation_trigger: Use when a new company is to be onboarded, when an onboarded company adds people or needs the plugin or the connector set up, or to find where a step of an onboarding stands.
---

# Customer Onboarding — Procedure

## Goal

A company's people work in the company's own ERPNext by talking to their own Claude. Each
person acts under their own ERPNext login, the whole organization holds the `erp` plugin
and receives its updates without anyone acting, and the provider holds no seat in the
company's Claude organization.

## Preconditions

- The company's slug and the name of its owner are known, and the owner's e-mail address
  is one that receives mail.
- The company has a Claude organization with an owner.
- The provider administers the GitHub organization that publishes the plugin.

## Steps

### 1. Create the site and onboard the company

Create the company's site through the deployment that hosts ERPNext
(`apps/procedure/erpnext/create-site`), then onboard the company on it
(`erp/procedure/company-onboarding`). Declare the applications the company needs beyond
ERPNext itself, such as staff administration, in the same site declaration.

Read the chart of accounts the setup created before anything else depends on it: have
ERPNext accept its customer and supplier accounts as the company's defaults, and read how
it classifies customers, suppliers and stock. A chart that fails either is decided with
the company's accountant before the first booking, because a chart cannot be replaced once
anything is booked.

### 2. State how the company works and what is ready

Write the company's own statement: what it does, how its people work, which parts of the
business are ready in ERPNext and which are not yet. Store it in the site setting that
ERPNext hands to every connecting agent, and have the owner store the same text in the
instructions of the company's Claude organization.

Create the company's getting-started project in ERPNext with one open task per step still
ahead, in order (`erp/procedure/orient-a-person`).

### 3. Give the people their logins

Onboard the owner first, then the other people (`erp/procedure/person-onboarding`). Each
receives ERPNext's own link to set a password. Set the lifetime of that link on the site
long enough for a person who reads mail once a day.

Where the company's mandate gives its workers that reach, apply worker access on the site
and to each worker's login (`erp/spec/worker-access`).

### 4. Give the organization the plugin

The plugin is published from this repository, which is public. A Claude organization
follows only a private repository for an organization-wide install, and that repository
may list a plugin from a public one. Each company therefore has its own small private
catalog repository in the provider's GitHub organization, named for the company. It holds
the catalog file that lists the `erp` plugin from this repository at a released version,
and the updater that moves that version to the latest release.

Create the company's catalog by copying an existing one. Invite the GitHub account of the
company's owner to it with administrator rights: Claude installs the update notification
on the repository, which needs those rights, and the catalog affects that company alone.

The owner then connects the catalog once, in the Claude organization's settings for
plugins and skills: sync from GitHub, the company's catalog repository, automatic sync on,
and the plugin's default access set to installed for everyone. Where the dialog asks for
the Claude GitHub App on the repository, the provider approves it in the GitHub
organization.

After a release of the plugin, start each catalog's updater; it also runs daily.

### 5. Connect ERPNext

The owner adds the company's site as a custom connector for the organization, with the
address of the site's MCP endpoint. Each person then connects it once and signs in with
their own ERPNext login.

### 6. Set the model

Tell the company's people which model to use for this work and have the owner set it
where the organization's settings offer the choice
(`local/projects/erpnext-ops/policy/model-floor`).

### 7. Check with the owner

With the owner present, ask their Claude what it can do for them and what comes next. The
answer names the company's own parts of the business and the next open step, and offers
nothing the statement marks as not ready. Then have one person record something small
under their own login and read it back as another.

## Outputs

- The company's site, with its company, its people's logins and its statement.
- The company's private catalog, connected to its Claude organization with automatic sync.
- The connector added for the organization and connected by each person.
- A getting-started project whose open tasks are the steps still ahead.

## Recovery

- **The owner has no GitHub account and will not make one.** The owner uploads the
  released plugin file once for the organization and again for each release, or each
  person adds this public repository as a marketplace from their own plugin settings.
- **The owner's Claude does not mention the company.** The statement did not reach it:
  confirm it is stored in the organization's instructions as well as on the site.
- **A password link expired before it was used.** Send a new one after lengthening the
  link's lifetime on the site.
- **A person's address was wrong.** Rename the login to the right address and send a new
  link; do not create a second login.

## See Also

- docs/local/policy/model-floor.md
- ~/.teleclaude/docs/erp/procedure/company-onboarding.md
- ~/.teleclaude/docs/erp/procedure/person-onboarding.md
- ~/.teleclaude/docs/erp/spec/worker-access.md
- ~/.teleclaude/docs/erp/procedure/orient-a-person.md
- https://claude.com/docs/plugins/org-sync
- https://claude.com/docs/plugins/admin
