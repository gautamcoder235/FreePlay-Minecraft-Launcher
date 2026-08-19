# Perceived Performance & Latency Budgets

---

## 1. Overview

Perceived performance measures how fast an application feels to the user. A product with **0ms optimistic updates** feels instant even if remote server APIs require 400ms to acknowledge mutations.

---

## 2. Response Time Thresholds (Nielsen Norman Group)

- **0 - 100ms**: Perceived as instantaneous. Direct tactile response (press states, keypress).
- **100ms - 300ms**: User notices slight pause. Smooth spring animation bridges visual continuity.
- **300ms - 1000ms**: Task execution threshold. Skeleton UI or micro progress shimmer mandatory.
- **1000ms+**: Focus lost. Deterministic progress bar required.

---

## 3. Key References

- [Nielsen Norman Group - Response Times Limit](https://www.nngroup.com/articles/response-times-3-important-limits/)
- [MDN Web Performance Documentation](https://developer.mozilla.org/en-US/docs/Web/Performance)
