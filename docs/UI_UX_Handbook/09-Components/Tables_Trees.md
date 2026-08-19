# Data Tables & Tree View Components

---

## 1. Overview

Data tables and file treeviews handle multi-level nested hierarchies, sticky header rows, virtual scrolling, multi-select rows, and inline cell editing.

---

## 2. ARIA APG Treeview Role Hierarchy

```
role="tree"
 ├── role="treeitem" (aria-expanded="true", aria-level="1")
 │    └── role="group"
 │         ├── role="treeitem" (aria-level="2")
 │         └── role="treeitem" (aria-level="2")
 └── role="treeitem" (aria-level="1")
```

---

## 3. Key References

- [W3C ARIA APG Treeview Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/treeview/)
- [TanStack Table V8 Documentation](https://tanstack.com/table/v8)
