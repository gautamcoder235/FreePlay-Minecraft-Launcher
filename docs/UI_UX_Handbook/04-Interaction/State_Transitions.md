# State Transitions & Cursor Latency Budgets

---

## 1. Overview

Input latency is the time elapsed between physical user input (mouse click, touch tap, keypress) and visual frame update on display hardware. Tier-1 applications target **< 16ms** input latency.

---

## 2. Latency Budget Allocation (16.6ms at 60Hz / 8.3ms at 120Hz)

```
+-------------------------------------------------------------------+
| JS Event Listener Processing  : ~3.0ms                           |
| React / Framework State Diff  : ~3.5ms                           |
| CSS Style & Layout Recalc     : ~2.0ms                           |
| GPU Paint & Layer Compositing : ~3.0ms                           |
| Buffer Swap & Display Refresh : ~5.1ms                           |
+-------------------------------------------------------------------+
| TOTAL FRAME TIME              : 16.6ms (60 FPS Target)           |
+-------------------------------------------------------------------+
```

---

## 3. Key References

- [MDN Web Performance - Anatomizing Frame Budgets](https://developer.mozilla.org/en-US/docs/Web/Performance)
- [W3C User Timing API](https://www.w3.org/TR/user-timing/)
