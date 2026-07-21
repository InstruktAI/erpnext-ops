<!-- LOCKED_FILE: DO NOT EDIT. Human-maintained thin wrapper. Knowledge additions/modifications MUST go to the source docs/** snippets, never here. Agents edit only on explicit human override. Rule: project/policy/agent-artifact-governance. -->

# AGENTS.md

## Purpose

ERPNext Business Operations — the operating layer for the ERPNext instances that InstruktAI
agents run on behalf of companies (CRM, sales, accounting). This project is a sibling of the
TeleClaude SDLC platform, not part of it: TeleClaude builds software; this project operates a
third-party business backend. Third-party mechanics never get plumbed into the mother project.

## Required reads

- @docs/project/baseline.md

## Credentials

Per-instance credentials live in a gitignored `.env` (`ERPNEXT_URL`, `ERPNEXT_API_KEY`,
`ERPNEXT_API_SECRET`); see `.env.example`. One `.env` per customer instance. The token
belongs to the dedicated `agent@instrukt.ai` (System Manager) user on that instance.

## Reference

- `reference/frappe-openapi-v2.json` — the generic Frappe v2 REST envelope (six operations,
  doctype-parameterized). Reference only; the per-instance schema is read at runtime via
  `/api/v2/doctype/{doctype}/meta`.

<!-- /LOCKED_FILE -->