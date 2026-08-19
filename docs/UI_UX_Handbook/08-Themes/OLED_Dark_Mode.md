# OLED Dark Mode & Visual Smearing Mitigation

---

## 1. Overview

Pure `#000000` (true black) on OLED screens causes visual **haloing / blooming** when bright white text sits directly adjacent to it, alongside sub-pixel motion smearing during rapid scrolling.

Tier-1 applications employ deep charcoal surface stacks (`#09090B` canvas, `#121215` sidebar, `#18181B` card surface) rather than raw pure black.

---

## 2. Elevation Surface Stack (Dark Mode)

```
Surface 0 (Canvas Background)  : #09090B (Deep Charcoal)
Surface 1 (Sidebar / Pane)     : #121215 (2% White Tint)
Surface 2 (Card Container)     : #18181B (4% White Tint)
Surface 3 (Popover / Menu)     : #202024 (7% White Tint)
Surface 4 (Modal Dialog)       : #27272A (10% White Tint)
```

---

## 3. Key References

- [Google Material Design 3 Dark Theme Guidance](https://m3.material.io/styles/color/dark-theme/overview)
- [Apple HIG Color Guidelines](https://developer.apple.com/design/human-interface-guidelines/color)
