# DOM Virtualization & Layout Thrashing Mitigation

---

## 1. Overview

DOM nodes consume browser memory and slow down CSS style recalculations. Virtualization keeps active DOM node counts below **1,000**, recycling DOM rows dynamically during scrolling.

---

## 2. Layout Thrashing Elimination Rule

- **BAD (Reflow Thrash)**: Interleaving DOM reads (`offsetHeight`) and DOM writes (`style.height`) in a loop.
- **GOOD (Batching)**: Perform all DOM reads first, compute dimensions, then batch all DOM writes inside `requestAnimationFrame`.

```javascript
// Batching DOM mutations
requestAnimationFrame(() => {
  element.style.height = `${computedHeight}px`;
  element.style.transform = `translateY(${topOffset}px)`;
});
```

---

## 3. Key References

- [MDN CSS Containment Specification (`contain`, `content-visibility`)](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_containment)
- [TanStack Virtual Windowing](https://tanstack.com/virtual/)
