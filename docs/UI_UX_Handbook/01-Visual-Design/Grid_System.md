# Grid Systems & Resizable Grids

---

## 1. Overview

Tier-1 web and desktop software relies on fluid 12-column or multi-panel CSS Grid systems combined with resizable split-view architectures.

---

## 2. 12-Column Responsive Grid Spec

```css
.tier1-grid-container {
  display: grid;
  grid-template-columns: repeat(12, minmax(0, 1fr));
  gap: var(--space-4); /* 16px default gap */
  padding-left: var(--space-6);
  padding-right: var(--space-6);
  max-width: 1600px;
  margin-left: auto;
  margin-right: auto;
}

/* Breakpoint Multi-Grid Adapter */
@media (max-width: 1024px) {
  .tier1-grid-container {
    grid-template-columns: repeat(8, minmax(0, 1fr));
    gap: var(--space-3);
  }
}

@media (max-width: 640px) {
  .tier1-grid-container {
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: var(--space-2);
  }
}
```

---

## 3. Multi-Panel Resizable Layout Architecture

For IDEs and desktop tools (VS Code, Cursor, Figma), fixed column grids are replaced with **CSS Subgrid** or **Flexbox split view handles**.

```html
<div class="workspace-layout">
  <aside class="sidebar-panel" style="width: 260px; min-width: 180px; max-width: 450px;">
    <!-- Sidebar Content -->
  </aside>
  <div class="resize-handle" role="separator" tabindex="0" aria-label="Resize Sidebar"></div>
  <main class="editor-canvas-panel">
    <!-- Editor / Canvas Content -->
  </main>
</div>
```

---

## 4. Key References

- [MDN CSS Grid Layout Specification](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_grid_layout)
- [Figma Layout Grid System Documentation](https://help.figma.com/hc/en-us/articles/360040450513-Create-layout-grids-with-auto-layout)
