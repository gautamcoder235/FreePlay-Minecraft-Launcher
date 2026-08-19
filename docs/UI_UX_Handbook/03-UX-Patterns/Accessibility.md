# Core UX Accessibility Integration

---

## 1. Overview

Accessibility is embedded in the component lifecycle. Every input control, modal, tree node, and custom dropdown must satisfy ARIA APG standards and keyboard focus navigation.

---

## 2. Accessibility Checklist for Core UX

- [ ] All click handlers have matching `onKeyDown` handlers for `Enter` and `Space`.
- [ ] Modals trap keyboard focus and restore focus on dismiss.
- [ ] Dynamic updates (alerts, toasts, stream status) utilize `aria-live` regions.
- [ ] Form controls are bound to explicit `<label>` elements via `htmlFor` and `id`.

---

## 3. Key References

- [W3C ARIA Authoring Practices Guide (APG)](https://www.w3.org/WAI/ARIA/apg/)
- [Radix UI Accessible Primitives](https://www.radix-ui.com/)
