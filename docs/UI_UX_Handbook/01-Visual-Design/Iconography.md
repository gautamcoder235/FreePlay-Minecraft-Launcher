# Iconography & Symbol Systems

---

## 1. Overview

Icons in Tier-1 software function as visual anchors to speed up scanning. Bad iconography introduces visual noise and ambiguity. Tier-1 systems enforce a 24px bounding box grid, consistent optical stroke weights (1.5px/2px), and accessibility labeling.

---

## 2. Icon Bounding Grid & Stroke Architecture

```css
:root {
  /* Standard Icon Sizes */
  --icon-size-xs: 12px; /* Inline metadata tags */
  --icon-size-sm: 16px; /* Buttons, Dense list items */
  --icon-size-md: 20px; /* Standard navigation menu */
  --icon-size-lg: 24px; /* Primary action toolbar */
  --icon-size-xl: 32px; /* Empty state highlight */

  /* Uniform Stroke Widths */
  --icon-stroke-thin: 1.25px;
  --icon-stroke-normal: 1.5px;
  --icon-stroke-bold: 2.0px;
}
```

---

## 3. Optical Alignment & ARIA Accessibility Rules

```html
<!-- Decorative Icon (Hidden from Screen Reader) -->
<svg class="icon-sm" aria-hidden="true" focusable="false" viewBox="0 0 24 24">
  <path d="..." />
</svg>

<!-- Interactive Icon Button (Requires Accessible Title / Label) -->
<button aria-label="Search Repository" class="icon-button">
  <svg class="icon-md" aria-hidden="true" viewBox="0 0 24 24">
    <path d="..." />
  </svg>
</button>
```

---

## 4. Key References

- [Lucide Icons System](https://lucide.dev/)
- [Apple SF Symbols Guidelines](https://developer.apple.com/sf-symbols/)
- [W3C ARIA Non-Text Content](https://www.w3.org/TR/WCAG21/#non-text-content)
