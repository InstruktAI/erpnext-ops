---
description: ERPNext native document-mapper request conventions for the CLI adapter.
---

# ERPNext Native Document Mappers

ERPNext native document mappers build target documents from source documents before the CLI can apply target-document overrides. Selling-chain mappers accept `source_name`; Payment Entry mapping accepts `dt` and `dn`. Mapper-specific inputs belong in the mapper request, while target-document field overrides apply only after the mapper response is received.

## Sources

- https://docs.frappe.io/erpnext/sales-integration
- https://github.com/frappe/erpnext/blob/develop/erpnext/accounts/doctype/payment_entry/payment_entry.py
