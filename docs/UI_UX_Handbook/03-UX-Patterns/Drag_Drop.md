# Drag & Drop State Management

---

## 1. Overview

Drag and Drop (DnD) in Tier-1 applications (kanban boards, file trees, canvas layers) requires physics preview previews, accessible keyboard alternatives, and **Fractional Indexing** to prevent $O(N)$ array reordering overhead.

---

## 2. Fractional Indexing Math

When inserting a dragged item between Item A (key `'a0'`) and Item B (key `'a1'`), the system computes the lexicographical midpoint key `'a0V'`, avoiding re-indexing all existing database rows.

```
Item A: 'a0'
Dragged Item: 'a0V'  <-- Fractional Index Midpoint Key
Item B: 'a1'
```

---

## 3. Key References

- [dnd kit - React Drag and Drop Library](https://dndkit.com/)
- [Figma Fractional Indexing Architecture](https://www.figma.com/blog/engineering/)
