---
description: The default way a company's documents are laid out in its document storage and tied to its ERPNext records — split by role level at the top, a folder per customer and project, named from the record — and when a company's own mapping replaces it.
---

# Documents Convention — Policy

## Rules

### Which mapping applies

- **A company's own mapping comes first.** Where what the company states about itself
  holds a document mapping, the agent follows that mapping, whole, and this convention
  does not apply.
- **Both outcomes are recorded.** Establishing the mapping is a step of onboarding a
  company. Its result is written into what the company states about itself: its own
  mapping, or that it follows this convention.
- **No record means nobody has looked.** An agent that finds neither establishes the
  mapping with the company's owner before it creates the first folder.

### The convention

- **The top level is split by role level.** The storage has one top folder for each level
  of the company that sees different things. The default is two: one for the people who
  decide and one for the people who carry out the work. Access is granted on these top
  folders.
- **The same structure sits under each top folder.** A folder per customer holding a folder
  per project, and one folder for internal projects that have no customer.
- **A folder is named from its record.** The record's number first, then the customer when
  the project has one, then the place when the company works on location. A part that
  does not exist is left out.
- **The record holds the link.** The customer or project in ERPNext carries the link to
  its folder. Files are found from the record, not by searching the storage.
- **Nothing more is fixed.** Folders inside a project and the names of files are the
  company's to shape.

## Rationale

Where access is granted on a folder and reaches what the folder holds, the top of the
structure decides who sees what. A structure that grew without that split has to be
taken apart before anything in it can be shared with one group and kept from another, so
the split is made before the first file is stored, also in a company of one person.

The same structure under each level lets a person and an agent find the other level's
folder for a customer or project by its name alone.

A name that starts with the record's number stays unique and findable when a customer or
a project is renamed, and it ties the folder to one record without a lookup.

A company that already has a working structure keeps it: its people know where things
are, and another layout gives them nothing they asked for. Recording that the convention was
chosen, as well as that a company's own mapping exists, is what tells a later agent that
somebody looked. Without it the absence of a mapping reads as permission to lay the
default over a structure that is already there.

Every further fixed rule is one more thing each agent has to follow and each person can
break, so the convention fixes only what sharing and finding depend on.

## Scope

Every agent that creates, names, finds or shares folders for a company's customers and
projects, in any document storage.

## Enforcement

Reviewers of a company's onboarding look for the recorded outcome.

## Exceptions

A single customer's or project's folder is shared with a party outside the company, such
as that customer, when the company's owner asks for it.

## See Also

- [erpnext/principle/business-and-system](../principle/business-and-system.md)
- [erpnext/policy/operating-disciplines](operating-disciplines.md)
