---
visibility: internal
number: 0002
date: 2026-11-14
description: The erp plugin is published from this public repository, and each served company's Claude organization receives it through a small private catalog repository of its own, connected once by the company's owner.
---

# Plugin Reaches an Organization Through Its Own Private Catalog — ADR

## Context

The `erp` plugin is built from this repository's documents and released by its CI. The
human wants this repository public, as a showcase others can use, and wants each served
company to hold the plugin organization-wide, with updates arriving without anyone acting.
The provider serves several companies and takes no seat in their Claude organizations.

Claude's organization-wide install has rules that pull against that:

- An organization follows only a private or internal repository as its plugin catalog. A
  public one is refused for this route.
- A catalog may list a plugin whose source is a public repository.
- Automatic sync is switched on by a person who administers the followed repository,
  because Claude creates a webhook on it. That person also has to be an owner of the
  Claude organization.
- A person can add a public repository as a marketplace for themselves, with automatic
  sync, but that installs for that person only.

These rules are Anthropic's, as documented on 2026-11-14 in the pages under See Also.

## Decision

This repository stays public and is the plugin's source.

Each served company has its own private catalog repository in the provider's GitHub
organization. It holds a catalog file that lists the `erpnext` plugin from this repository at
a released version, and an updater that moves that version to the latest release.

The company's owner is invited to their catalog with administrator rights and connects it
once in their Claude organization, with automatic sync on and the plugin installed for
everyone. The provider holds no seat in that organization.

## Consequences

Gained:

- The source is public, and a company's people receive the plugin and its updates without
  acting and without GitHub accounts of their own.
- The provider governs every catalog and pays for no seat.
- A catalog is the natural home for a company's own plugin later.

Given up or newly owed:

- One private repository per company, created at onboarding.
- The company's owner needs a GitHub account and performs the connection, on the
  organization's admin pages on the web.
- The owner administers their own catalog and could break it. It affects that company
  alone, and the provider can restore it.
- A release reaches an organization only after the catalog's updater has run. It is
  started after a release and also runs on the schedule its own workflow sets.
- The owner-side connection had not been performed when this was decided. The catalog,
  its updater and installing through it were exercised from the command line only.

## Alternatives Considered

- **Each person adds this public repository as a marketplace.** No private repository and
  no owner action, and automatic sync works. Rejected as the default because it is per
  person, and the human wants each new employee to have the plugin without acting. It
  remains the fallback for an owner who will not make a GitHub account.
- **Make this repository private and follow it directly.** One repository, but not public,
  which the human wants.
- **One private catalog shared by all companies.** The connecting owner needs
  administrator rights on the followed repository, so every company's owner would
  administer the others' catalog.
- **The provider as an owner in each company's Claude organization.** The provider could
  then connect a shared catalog. Rejected by the human: a paid seat for one-time
  configuration the provider otherwise never uses.
- **The owner uploads the plugin file.** Organization-wide at once and needs no GitHub,
  but every release needs another upload.

## See Also

- https://claude.com/docs/plugins/org-sync
- https://claude.com/docs/plugins/admin
- https://claude.com/docs/plugins/overview
- https://code.claude.com/docs/en/plugins/host-marketplace
