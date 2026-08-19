# Keyboard Navigation & Focus Trapping

---

## 1. Overview

Tier-1 applications permit 100% operation without mouse or touch input. This requires a focus trapping engine for modals and roving tabindex implementations for menus and tab bars.

---

## 2. Focus Trap Engine Implementation

```typescript
import { useEffect, useRef } from 'react';

const TABBABLE_SELECTOR = 'a[href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), button:not([disabled]), [tabindex]:not([tabindex="-1"])';

export function useFocusTrap(isActive = true) {
  const containerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!isActive) return;
    const container = containerRef.current;
    if (!container) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key !== 'Tab') return;
      const elements = Array.from(container.querySelectorAll<HTMLElement>(TABBABLE_SELECTOR));
      if (elements.length === 0) return;

      const first = elements[0];
      const last = elements[elements.length - 1];

      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isActive]);

  return containerRef;
}
```

---

## 3. Key References

- [W3C ARIA APG Keyboard Navigation Specs](https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/)
- [Radix UI Focus Scope](https://www.radix-ui.com/primitives/docs/utilities/focus-scope)
