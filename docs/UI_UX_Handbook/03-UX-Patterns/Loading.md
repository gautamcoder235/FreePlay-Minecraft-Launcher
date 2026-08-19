# Loading Strategies & Skeleton Screens

---

## 1. Overview

Loading states directly dictate perceived performance. Standard web applications present full-page spinners, creating visual anxiety and freezing user action. Tier-1 applications utilize **Skeletons**, **Ghost Placeholders**, **Progressive Rendering**, and **Optimistic Local Mutations**.

---

## 2. Loading Strategy Decision Matrix

| Target Latency | Loading Strategy | Visual Implementation | Perceived Speed Impact |
| :--- | :--- | :--- | :--- |
| **0ms - 50ms** | Instant Local Update | Optimistic state mutation | **Instantaneous** |
| **50ms - 200ms** | Micro Shimmer Indicator | Subtle top progress bar (NProgress) | Unnoticeable delay |
| **200ms - 2000ms** | Skeleton Placeholder | Component-matching gray pulse | **High (+40% speed perception)** |
| **2000ms+** | Deterministic Progress Bar | Percentage progress (`45% complete`) | Reduces drop-off |

---

## 3. Cumulative Layout Shift (CLS = 0) Requirement

Skeleton loaders must reserve the exact bounding box height, width, line-height, and padding of the final content.

```css
/* Zero CLS Skeleton Container */
.skeleton-card {
  width: 100%;
  height: 120px; /* Matches exact rendered card height */
  border-radius: var(--radius-md);
  background: var(--bg-surface-elevated);
}
```

---

## 4. Key References

- [Nielsen Norman Group - Progress Indicators](https://www.nngroup.com/articles/progress-indicators/)
- [W3C Web Performance Working Group](https://www.w3.org/TR/navigation-timing/)
