# Docking Systems & Windowing Architecture

---

## 1. Overview

Desktop creative suites (**Figma**, **Photoshop**, **Blender**) require flexible docking systems where tool panels can detach into floating overlays or snap back into layout docks.

---

## 2. Docking Snap Targets & Drag Drops

```mermaid
flowchart TD
    Drag[User Drags Floating Panel] --> Intersect{Crosses Snap Zone Target?}
    Intersect -->|Yes| Highlight[Render Blue Highlight Snap Guide Box]
    Intersect -->|No| Float[Maintain Absolute Floating Coordinates]
    Highlight --> Drop{Mouse Release}
    Drop --> Attach[Attach Panel to Workspace Grid Column]
```

---

## 3. Key References

- [Golden Layout - Multi-Window Docking Engine](https://golden-layout.com/)
- [Figma Canvas Architecture](https://www.figma.com/blog/engineering/)
