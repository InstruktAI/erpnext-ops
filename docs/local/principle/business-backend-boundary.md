---
description: This project operates third-party business backends for companies; their mechanics stay here and never enter the platform that builds software.
---

# Business Backend Boundary — Principle

## Principle

This project is the operating layer for the ERPNext sites that agents run on behalf of
companies: customer relations, sales, stock, accounting. It is a sibling of the TeleClaude
platform, not a part of it. The platform builds software and carries identity, activation
and delivery; this project holds the knowledge of operating a third-party business
application. The mechanics of that application stay on this side of the line.

## Rationale

A business application has its own versions, data model and failure modes. Knowledge of
them placed in the platform would make every platform change answer to a vendor's release
and would spread one application's quirks across projects that never touch it. Kept here,
that knowledge has one owner and one place to change when the vendor changes.

## Implications

- What an agent must know to operate ERPNext is published from this project as the `erp`
  domain.
- A need the platform cannot meet is raised as a platform capability in general terms,
  never met by teaching the platform about ERPNext.
- Deploying the sites belongs to the fleet that hosts them; this project operates sites
  that already exist.

## Tensions

- **Convenience vs. ownership.** A small ERPNext-specific shortcut inside the platform is
  often the fastest fix, and the start of a second home for the same knowledge.
