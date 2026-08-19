# WCAG 2.1 AA & AAA Compliance Standards

---

## 1. Overview

Accessibility is a non-negotiable requirement for Tier-1 software. Complying with **WCAG 2.1 AA / AAA** guarantees that software is accessible to users with visual, auditory, motor, or cognitive impairments.

---

## 2. Key WCAG Specification Table

| WCAG Criterion | Level | Specification | Implementation Strategy |
| :--- | :--- | :--- | :--- |
| **1.4.3 Contrast (Min)** | AA | **4.5:1** (Normal text), **3.0:1** (Large text) | Automate linter checks in CI/CD pipeline |
| **1.4.6 Contrast (Enhanced)** | AAA | **7.0:1** (Normal text), **4.5:1** (Large text) | High Contrast theme support |
| **1.4.11 Non-text Contrast** | AA | **3.0:1** for active UI borders, focus rings | Ensure focus outlines meet 3:1 contrast |
| **2.4.7 Focus Visible** | AA | Clear visual focus indicator on element | Custom `:focus-visible` ring with 2px offset |
| **2.5.8 Target Size** | AA | **24x24px** minimum touch/click target size | Use pseudo-element hit-target expansion |

---

## 3. Key References

- [W3C WCAG 2.1 Full Guidelines](https://www.w3.org/TR/WCAG21/)
- [W3C Accessibility Conformance Matrix](https://www.w3.org/WAI/WCAG21/quickref/)
