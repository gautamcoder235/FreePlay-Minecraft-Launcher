# Neumorphism vs Flat vs Tactile Design

---

## 1. Overview

Neumorphism (Soft UI) attempted to replicate real-world physical extrusions using dual light/dark drop shadows (`box-shadow: 9px 9px 18px #bebebe, -9px -9px 18px #ffffff`). 

While trendy in concept mockups, **Neumorphism is deprecated in Tier-1 product design** due to critical accessibility failures and spatial hierarchy ambiguity.

---

## 2. Technical Comparison Matrix

| Design Style | Contrast Ratio Compliance | Spatial Hierarchy | Performance GPU Cost | Production Readiness |
| :--- | :--- | :--- | :--- | :--- |
| **Flat Design (Tier-3)** | High (Solid colors) | Low (Lacks depth) | Low (Zero blur) | Legacy SaaS |
| **Neumorphism** | **Fails WCAG AA (< 2.5:1)** | Low (Flat extrusion) | High (Dual shadows) | **Not Recommended** |
| **Glassmorphism (Tier-1)** | **High (WCAG AAA)** | **High (Z-axis stack)** | Medium (Backdrop blur) | **Industry Standard** |
| **Tactile Minimal (Tier-1)** | **High (WCAG AAA)** | **High (1px inner rim)** | **Low (Optimized)** | **Industry Standard** |

---

## 3. Key References

- [Nielsen Norman Group - Neumorphism & Accessibility Issues](https://www.nngroup.com/articles/neumorphism/)
- [W3C WCAG Non-Text Contrast Standards](https://www.w3.org/TR/WCAG21/#non-text-content)
