# Split Views & Resizable Panel Engines

---

## 1. Overview

Split view engines allow users to adjust relative panel sizes dynamically. Implementing resizable split views without layout bugs requires explicit pointer capture handling and constraint boundaries.

---

## 2. Pointer Capture & Boundary Constraints Implementation

```typescript
export function usePanelResize(minPx = 180, maxPx = 500, initialPx = 260) {
  const [width, setWidth] = useState(initialPx);
  const [isResizing, setIsResizing] = useState(false);

  const startResize = (e: React.PointerEvent<HTMLDivElement>) => {
    e.currentTarget.setPointerCapture(e.pointerId);
    setIsResizing(true);
  };

  const resize = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!isResizing) return;
    const clamped = Math.max(minPx, Math.min(maxPx, e.clientX));
    setWidth(clamped);
  };

  const stopResize = (e: React.PointerEvent<HTMLDivElement>) => {
    e.currentTarget.releasePointerCapture(e.pointerId);
    setIsResizing(false);
  };

  return { width, isResizing, startResize, resize, stopResize };
}
```

---

## 3. Key References

- [React Panel Resize Library (react-resizable-panels)](https://github.com/bvaughn/react-resizable-panels)
- [MDN Pointer Events Specification](https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events)
