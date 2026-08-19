# Skeleton Design Systems

---

## 1. Overview

Skeletons replace traditional spinning loaders by rendering a visual preview of the layout structure before real network data arrives.

---

## 2. Skeleton Shimmer CSS Engine

```css
.skeleton-shimmer {
  background: linear-gradient(
    90deg,
    rgba(255, 255, 255, 0.03) 0%,
    rgba(255, 255, 255, 0.08) 50%,
    rgba(255, 255, 255, 0.03) 100%
  );
  background-size: 200% 100%;
  animation: skeleton-wave 1.8s infinite cubic-bezier(0.4, 0, 0.6, 1);
  border-radius: 4px;
}

@keyframes skeleton-wave {
  0% { background-position: -200% 0; }
  100% { background-position: 200% 0; }
}
```

---

## 3. Key References

- [Linear UI Loading Patterns](https://linear.app/blog)
- [Nielsen Norman Group - Skeleton Screens](https://www.nngroup.com/articles/skeleton-screens/)
