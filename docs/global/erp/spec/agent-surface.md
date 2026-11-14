---
description: Which operation of an ERPNext site's MCP server realizes each operation the ERP doctrine names — resolve, draft, preview, submit, map, record activity, follow up, report.
---

# ERP Agent Surface — Spec

## What it is

Agents operate an ERPNext site through the MCP server the site itself serves. The server
offers generic tools over documents and whitelisted methods, with schema and method
discovery. One connection reaches one site as one authenticated ERPNext user, and every
call runs under that user's roles.

The ERP doctrine names business operations: resolve, draft, preview, submit, map, record
activity. This spec binds each of them to the server operation that realizes it, so a
procedure names the operation once and carries no tool mechanics.

## Canonical fields

| Doctrine operation | Server operation |
| --- | --- |
| Establish the connected site, user and company | The server's connection instructions name the user; `list_documents` on `Company` names the company and its defaults |
| Read live metadata of a document type | `get_doctype_schema` |
| Resolve a party, item or document | `list_documents` with filters on the naming fields; `count_documents` |
| Read a record | `get_document` |
| Create a draft, or a master record | `create_document`; a submittable document created without a `docstatus` is a draft |
| Change a draft or a master record | `update_document`, carrying the `modified` value last read |
| Preview the ledger effect of a draft | `call_method` on `erpnext.controllers.stock_controller.show_accounting_ledger_preview` with the company, document type and document name |
| Preview the stock effect of a draft | `call_method` on `erpnext.controllers.stock_controller.show_stock_ledger_preview` with the company, document type and document name |
| Submit | `call_method` on `frappe.client.submit` with the document as reviewed, which carries its `modified` value |
| Cancel | `call_document_method` with `cancel` |
| Amend a cancelled document | `create_document` for a new draft whose `amended_from` names the cancelled document |
| Map a source document to its successor | `call_method` on the native mapper with its source arguments; the mapper returns an unsaved document, which `create_document` stores |
| Find a method and its arguments | `search_methods`, `get_method_schema`, `list_document_methods` |
| Record a note on a business record | `call_method` on `frappe.desk.form.utils.add_comment` |
| Record an interaction that took place | `call_method` on `frappe.core.doctype.communication.email.make` with sending switched off |
| Record or close a follow-up | A `ToDo` document referencing the business record, opened and closed through its status |
| Run a native report | `call_method` on `frappe.desk.query_report.run` with the report name and filters |
| Read the ledger rows of one voucher | `list_documents` on `GL Entry` filtered by voucher and by not cancelled |
| Remove a draft | `delete_document` |

Arguments of `call_method` and `call_document_method` travel in `args`.

## Known caveats

Observed on Frappe 16.56 with ERPNext 16.55, as a user holding business roles and no
administrator role. On a site with other versions, or as a user with other roles, each is
re-established before it is relied on.

- A field name the document type does not have is dropped from a create or update without
  an error.
- A repeated create stores a second document; the server has no idempotent create.
- An update that carries no `modified` value replaces whatever another party stored since
  the record was read. `submit` through `call_document_method` and `cancel` accept no
  version at all.
- Only submittable documents have a draft state. A write to a master record takes effect
  at once.
- A document's change history and its deleted-document record are not readable.
- A report ERPNext prepares in the background returns no rows on the first call.
- ERPNext's bulk import is reserved to system administrators.
- A site that does not serve the MCP server cannot be operated under this doctrine.

## See Also

- ~/.teleclaude/docs/erp/policy/operating-disciplines.md
- ~/.teleclaude/docs/erp/procedure/apply-native-operation.md
- https://github.com/frappe/frappe/tree/develop/frappe/mcp
- https://docs.frappe.io/framework
