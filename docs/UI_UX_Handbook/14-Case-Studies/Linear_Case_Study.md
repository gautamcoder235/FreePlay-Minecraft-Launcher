# Product Breakdown: Linear

---

## 1. Overview

Linear is widely recognized as the benchmark for modern SaaS craft. It combines a local-first sync architecture, keyboard-driven navigation, sub-10ms UI responsiveness, and subtle micro-animations.

---

## 2. Reverse Engineering Linear's Craft Stack

```
+-------------------------------------------------------------------+
| FRONTEND     | React, TypeScript, Tailwind CSS, Custom UI Primitives|
| STATE / SYNC | IndexedDB Local SQLite Cache + WebSocket Delta Sync|
| LATENCY      | 0ms local mutation execution (Optimistic UI)       |
| NAVIGATION   | Cmd+K Command Palette + Single Keybindings (j, k, c)|
| ANIMATION    | Custom Spring Dynamics (stiffness: 400, damping: 30)|
+-------------------------------------------------------------------+
```

---

## 3. Key Takeaways for Tier-1 Builders

1. **Never block user input with network spinners**: Write edits to client local storage instantly.
2. **Keyboard velocity is primary**: Ensure every mouse action has a corresponding hotkey.
3. **Subtle contrast lines**: Replace thick dark borders with 1px semi-transparent inner highlights (`rgba(255,255,255,0.08)`).

---

## 4. Key References

- [Linear Engineering Blog](https://linear.app/blog)
- [Linear - Scaling the Sync Engine](https://linear.app/blog/scaling-the-linear-sync-engine)
