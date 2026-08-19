# Modal, Drawer & Popover Component Specs

---

## 1. Overview

Modals, Drawers, and Popovers share underlying portal, focus trap, and escape key event listeners while catering to distinct layout levels.

---

## 2. Shared Portal Root Architecture

```tsx
import ReactDOM from 'react-dom';

export function OverlayPortal({ children }: { children: React.ReactNode }) {
  const portalRoot = document.getElementById('portal-root') || document.body;
  return ReactDOM.createPortal(children, portalRoot);
}
```

---

## 3. Key References

- [Radix UI Primitives Repository](https://www.radix-ui.com/)
- [W3C ARIA Dialog (Modal) Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/)
