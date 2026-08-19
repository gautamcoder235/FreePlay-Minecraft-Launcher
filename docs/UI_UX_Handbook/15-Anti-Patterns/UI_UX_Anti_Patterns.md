# Catalog of 15+ Fatal UI/UX Anti-Patterns

---

## 1. Overview

Anti-patterns degrade user trust, create visual confusion, and introduce latency debt. Tier-1 engineering teams actively enforce linters, automated tests, and design reviews to prevent these 16 fatal flaws.

---

## 2. Anti-Patterns Master Catalog Matrix

| Anti-Pattern Name | Root Cause | User Impact | Remediation Strategy |
| :--- | :--- | :--- | :--- |
| **1. Modal Spam** | Lazy routing / chained popups | High cognitive fatigue | Replace with slide-over drawers or inline views |
| **2. Layout Shift (CLS)** | Unsized images / late card loads | Mis-clicks, visual jumping | Reserve explicit CSS `aspect-ratio` & skeleton dimensions |
| **3. Spinner Abuse** | Generic fallback loading state | Perceived slow performance | Use Skeletons and Optimistic Local UI |
| **4. Dark Patterns** | Sneaky growth hacks / misleading CTAs | Loss of user trust | Clear contrast distinction between primary/secondary CTAs |
| **5. Notification Fatigue** | Unfiltered push/toast alerts | Banner blindness | Group notifications into digested trays |
| **6. Unanchored Scroll** | Content prepended above viewport | Reading position shifts | Apply CSS `scroll-anchoring: auto` |
| **7. Missing Focus Rings** | `outline: none` CSS reset | Keyboard users lose focus | Retain `:focus-visible` custom ring with 2px offset |
| **8. Infinite Scroll Trap** | Unpaginated list loading | Cannot reach footer or links | Provide explicit "Load More" pagination |
| **9. Silent Disabled Buttons** | Unresponsive disabled states | User confusion on block reason | Keep button active, display inline validation tooltip |
| **10. Mystery Meat Nav** | Icon-only buttons without tooltips | High cognitive parsing friction | Pair icons with labels or instantaneous tooltips |
| **11. Form Data Loss** | Form reset on validation fail | User rage quit after lost data | Preserve input state in local storage |
| **12. Blocking Animations** | Overslow CSS transitions (>350ms) | UI feels sluggish | Cap microinteraction transitions between 100ms-200ms |
| **13. Inaccessible Low Contrast**| Aesthetic choice over legibility | Unreadable text | Enforce WCAG AA 4.5:1 ratio linter checks in CI |
| **14. Toast Bombardment** | Global alert loops | Overlapped controls | Toast queue max limit of 3 visible toasts |
| **15. Hijacked Scrolling** | Custom JS smooth scroll scripts | Scroll lag & frame stutters | Use native CSS `scroll-behavior: smooth` |
| **16. Deep Settings Burial** | Unorganized settings hierarchy | Wasted navigation time | Command palette searchable settings index |

---

## 3. Key References

- [Laws of UX Anti-Patterns](https://lawsofux.com/)
- [W3C WCAG 2.1 Common Failures](https://www.w3.org/TR/WCAG21/)
- [Nielsen Norman Group - Dark Patterns](https://www.nngroup.com/articles/dark-patterns/)
