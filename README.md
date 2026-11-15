# erpnext-ops

How agents run a small company's ERPNext for the people who work there.

People tell their own Claude what happened or what they need, in normal words: "the
customer paid", "I worked four hours at the Silva job", "who is late paying us?". Claude
does the work in the company's ERPNext, under that person's own login, following the
operating knowledge in this repository.

## What is here

| Path | What it is |
| --- | --- |
| `docs/global/erp/` | The operating knowledge: principles, rules, and procedures for the things a business does. Invoices and payments, credit notes and refunds, overdue debt, period close, work hours, material taken for a job, absence, getting a person started. |
| `docs/local/` | How this project itself works: how a new company is onboarded, and which models may do this work. |
| `skill/` | What goes into the plugin and how: which documents load at once and which on demand. |
| `tools/` | The builder that turns the documents into a Claude plugin, and the release tasks. |
| `plugins/erp/` | The released plugin, built by CI. Not edited by hand. |
| `image/` | The ERPNext image the sites run, with the staff-administration application added. |

## How it fits together

1. **ERPNext serves its own MCP server.** Claude reaches a company's site through it as a
   connector. Each person signs in with their own ERPNext account, so everything they do
   is recorded under their name and limited by their rights.
2. **The documents are the product.** They say how to work in ERPNext safely: search
   before creating, draft before posting, preview the ledger, read back what was stored.
   No tool enforces that sequence; the documents carry it.
3. **CI builds the plugin from the documents.** Every push builds it and runs Anthropic's
   plugin validator. A release builds it at a version, commits it under `plugins/`, tags
   it and publishes it.
4. **Claude loads it as a skill.** One short description is always in view; the rules load
   when the work is ERP work; each procedure loads when the request calls for it.

## Use the plugin

In Claude, open Customize, then Plugins, then Add, then Add marketplace, and enter
`InstruktAI/erpnext-ops`. Install the `erp` plugin that appears.

The plugin needs a connector to an ERPNext site that serves the MCP server: add the site's
`/api/v2/mcp` address as a custom connector and sign in.

To give it to a whole Claude organization at once, the organization follows a small
private catalog repository that lists this plugin; `docs/local/procedure/customer-onboarding.md`
describes that.

## Work on it

```sh
make install   # the pinned tools
make lint      # lint and type-check the builder
make test      # build the plugin and validate it
```

A release is cut by running the `release` workflow with a version such as `v1.2.3`.

## License

GPL-3.0, see `LICENSE`. The image recipe in `image/` derives from frappe/frappe_docker,
which is MIT licensed; its notice is in `image/LICENSE.frappe_docker`.
