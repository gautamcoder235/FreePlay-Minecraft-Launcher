# Screen Reader Accessibility & ARIA Patterns

---

## 1. Overview

Screen readers (VoiceOver, NVDA, JAWS) rely on ARIA roles, states, and properties to announce dynamic UI changes.

---

## 2. ARIA APG Widget Reference Matrix

| Component | ARIA Role | Key ARIA States |
| :--- | :--- | :--- |
| **Combobox / Search** | `combobox` | `aria-expanded`, `aria-controls`, `aria-activedescendant` |
| **Modal Dialog** | `dialog` | `aria-modal="true"`, `aria-labelledby`, `aria-describedby` |
| **Treeview File Explorer**| `tree` -> `treeitem` | `aria-expanded`, `aria-level`, `aria-selected` |
| **Tabs Navigation** | `tablist` -> `tab` | `aria-selected="true|false"`, `aria-controls` |

---

## 3. Key References

- [W3C ARIA APG Design Patterns](https://www.w3.org/WAI/ARIA/apg/patterns/)
- [Apple VoiceOver Developer Guide](https://developer.apple.com/accessibility/)
