# Frame Budgets: 60 FPS vs 120 FPS Architecture

---

## 1. Overview

High-refresh displays (ProMotion 120Hz on macOS/iOS) require main-thread execution to complete within **8.33ms per frame**.

---

## 2. Frame Budget Comparison

```
+-------------------------------------------------------------------+
| 60 FPS Target (Standard Displays)   : 16.67ms Total Budget         |
| Main Thread JS Work Threshold      : ~10.0ms                      |
+-------------------------------------------------------------------+
| 120 FPS Target (ProMotion Displays) : 8.33ms Total Budget          |
| Main Thread JS Work Threshold      : ~4.0ms                       |
+-------------------------------------------------------------------+
```

---

## 3. GPU Layer Promotion Rules

```css
/* Promote high-frequency animation elements to GPU hardware compositor layers */
.gpu-accelerated-panel {
  will-change: transform, opacity;
  transform: translateZ(0);
  backface-visibility: hidden;
}
```

---

## 4. Key References

- [MDN Web Performance Anatomy](https://developer.mozilla.org/en-US/docs/Web/Performance)
- [W3C Prioritized Task Scheduling API](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)
