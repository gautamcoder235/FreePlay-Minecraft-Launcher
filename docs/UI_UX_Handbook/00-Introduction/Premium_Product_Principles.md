# Premium Product Principles

---

## 1. Overview

What creates the perception of "craft" and "premium quality" in software? Premium products communicate quality through spatial consistency, tactile feedback, sub-millisecond responsiveness, visual balance, and respect for user time.

---

## 2. The 10 Principles of Premium Product Craft

### Principle 1: Respect the Input Budget (16ms Rule)
All UI code must process user inputs within **16.6ms** (for 60Hz displays) or **8.3ms** (for 120Hz displays). Never run expensive synchronous array sorting, regex matching, or un-memoized layout calculations on the main event loop thread during user interaction.

### Principle 2: Optical Balance over Mathematical Centering
Text, icons, and badges often look misaligned when mathematically centered. Icons like play triangles (`▶`) or custom brand glyphs require optical adjustment (e.g. `transform: translateX(1px)`) to appear visually centered.

```css
/* Example: Optical centering for Play Button Icon */
.play-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  /* Optical shift for visual balance */
  padding-left: 2px; 
}
```

### Principle 3: Border Radius Nesting Consistency
Outer and inner border radiuses must remain concentric. Inner radius should equal outer radius minus outer padding:
$$R_{\text{inner}} = R_{\text{outer}} - P_{\text{padding}}$$

```css
:root {
  --card-outer-radius: 12px;
  --card-padding: 8px;
  --card-inner-radius: calc(var(--card-outer-radius) - var(--card-padding)); /* 4px */
}
```

### Principle 4: Layered Shadow & Depth Stack
Single harsh shadows look primitive. Premium depth relies on multi-layered ambient and key directional shadows.

```css
/* Premium Tier-1 Layered Elevation Shadow */
.tier1-card-elevation {
  box-shadow: 
    0 1px 2px 0 rgba(0, 0, 0, 0.05),
    0 4px 12px -2px rgba(0, 0, 0, 0.08),
    0 16px 32px -4px rgba(0, 0, 0, 0.12);
}
```

### Principle 5: Progressive Disclosure
Never overwhelm the user with 50 controls simultaneously. Show primary actions immediately; reveal secondary options in contextual drop downs, inspector sidebars, or keyboard shortcut modifiers.

---

## 3. Decision Matrix: Craft Tradeoffs

| Choice | Low Quality Approach | Premium Tier-1 Approach |
| :--- | :--- | :--- |
| **Loading State** | Full-page spinner modal blocking user | Skeleton UI + Optimistic local mutation |
| **Border Design** | 1px solid `#ccc` static line | 1px semi-transparent inner line + hover glow |
| **Animation** | `transition: all 0.3s ease` | Physics spring (`stiffness: 300, damping: 25`) |
| **Focus State** | Default browser blue outline | Custom 2px high-contrast ring + offset space |
| **Font Rendering** | Default system fallback | Subpixel antialiased custom typography stack |

---

## 4. Key References

- [Refactoring UI by Adam Wathan & Steve Schoger](https://www.refactoringui.com/)
- [Laws of UX - Jon Yablonski](https://lawsofux.com/)
- [Apple HIG - Color & Depth](https://developer.apple.com/design/human-interface-guidelines/color)
