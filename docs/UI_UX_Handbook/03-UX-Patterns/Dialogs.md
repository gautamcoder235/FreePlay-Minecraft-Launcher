# Dialogs, Modals, Drawers & Popovers

---

## 1. Overview

Modals disrupt user workflow by locking focus. Tier-1 apps strictly apply the overlay decision hierarchy: **Popover** (for contextual options), **Drawer** (for multi-field creation forms), and **Modal Dialog** (only for destructive confirmations or isolated multi-step flows).

---

## 2. Decision Matrix

| Overlay Pattern | Focus Trapping | Backdrop Dimming | Best Use Case |
| :--- | :--- | :--- | :--- |
| **Modal Dialog** | **Required** | High (`rgba(0,0,0,0.6)`) | Destructive confirm, Authentication |
| **Slide-over Drawer** | **Required** | Medium (`rgba(0,0,0,0.3)`) | Detail view, Workspace settings |
| **Popover** | Optional | None | Context menu, Date picker, Filter |
| **Tooltip** | None | None | Short inline clarification tag |

---

## 3. Key References

- [Radix UI Dialog Primitives](https://www.radix-ui.com/primitives/docs/components/dialog)
- [W3C ARIA APG Dialog (Modal) Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/)
