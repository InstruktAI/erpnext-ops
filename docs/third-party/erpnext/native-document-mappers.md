---
description: ERPNext native document-mapper request conventions.
---

# ERPNext Native Document Mappers

ERPNext native document mappers build target documents from source documents; the caller stores the returned document. Selling-chain mappers accept `source_name`; Payment Entry mapping accepts `dt` and `dn`. Mapper-specific inputs belong in the mapper request, while target-document field overrides apply only after the mapper response is received.

## Sources

- https://docs.frappe.io/erpnext/sales-integration
- https://github.com/frappe/erpnext/blob/develop/erpnext/accounts/doctype/payment_entry/payment_entry.py
- https://docs.frappe.io/framework/user/en/basics/doctypes/fieldtypes
- https://docs.frappe.io/erpnext/dynamic-link-fields
- https://github.com/frappe/frappe/blob/version-15/frappe/model/base_document.py
- https://github.com/frappe/erpnext/blob/version-15/erpnext/accounts/doctype/payment_entry_reference/payment_entry_reference.json
