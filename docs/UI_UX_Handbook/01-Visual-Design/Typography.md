# Typography Systems & Font Scaling

---

## 1. Overview

Typography is the structural backbone of software interfaces. Over 80% of any desktop or web interface consists of textual data. A Tier-1 typography system establishes visual hierarchy, guarantees vertical rhythm, optimizes legibility across screen pixel densities, and reduces cognitive fatigue.

---

## 2. Mathematical Type Scaling Models

Tier-1 systems generate font sizes dynamically using modular scale ratios rather than hand-picked arbitrary values.

### Modular Scale Ratios

$$S_n = S_{\text{base}} \times r^n$$

Where:
- $S_{\text{base}} = 16\text{px}$ (Standard root interface body font size)
- $r = \text{Scale Ratio}$
- $n = \text{Scale Step } (-2, -1, 0, 1, 2, 3, 4, 5)$

| Scale Name | Ratio ($r$) | Best Suited Application |
| :--- | :--- | :--- |
| **Minor Third** | `1.200` | Dense IDEs, Data Grids (VS Code, Cursor) |
| **Major Third** | `1.250` | Standard Desktop SaaS (Linear, Notion) |
| **Perfect Fourth** | `1.333` | Marketing & Documentation (Apple HIG) |
| **Augmented Fourth** | `1.414` | High-Impact Canvas Apps |
| **Golden Ratio** | `1.618` | Editorial & Editorial Layouts |

---

## 3. Major Third Scale Concrete Implementation (1.250 Ratio)

```css
:root {
  /* Base Typography Tokens */
  --font-family-sans: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  --font-family-mono: "JetBrains Mono", "SF Mono", Fira Code, Menlo, monospace;

  /* Font Size Scale (Major Third - r = 1.250) */
  --text-xs:   0.64rem;  /* 10.24px - Micro Badges / Timestamps */
  --text-sm:   0.80rem;  /* 12.80px - Table Headers, Secondary Metadata */
  --text-base: 1.00rem;  /* 16.00px - Primary Body & Inputs */
  --text-md:   1.25rem;  /* 20.00px - H4 Titles, Section Headers */
  --text-lg:   1.56rem;  /* 25.00px - H3 Modal Titles */
  --text-xl:   1.95rem;  /* 31.25px - H2 Page Headers */
  --text-2xl:  2.44rem;  /* 39.06px - H1 Hero Headers */
  --text-3xl:  3.05rem;  /* 48.83px - Display Metric Numbers */

  /* Strict Line Height Rhythms */
  --leading-none: 1.00;
  --leading-tight: 1.25;
  --leading-snug: 1.375;
  --leading-normal: 1.50;
  --leading-relaxed: 1.625;

  /* Font Weight Tokens */
  --font-weight-regular: 400;
  --font-weight-medium: 500;
  --font-weight-semibold: 600;
  --font-weight-bold: 700;
}
```

---

## 4. Vertical Rhythm & Baseline Alignment (4px Grid Integration)

Vertical rhythm requires every font size's computed `line-height` to be an exact integer multiple of **4px** or **8px**.

$$\text{LineHeight}_{\text{px}} = \lceil \text{FontSize}_{\text{px}} \times \text{Multiplier} \rceil \quad \text{rounded up to nearest } 4\text{px}$$

### Typography Rhythm Table

| Token | Font Size | Line Height (Ratio) | Computed Line Height | 4px Grid Alignment |
| :--- | :--- | :--- | :--- | :--- |
| `--text-xs` | 10px | 1.60 | **16px** | $16 = 4 \times 4$ ✓ |
| `--text-sm` | 13px | 1.538 | **20px** | $20 = 4 \times 5$ ✓ |
| `--text-base` | 16px | 1.500 | **24px** | $24 = 4 \times 6$ ✓ |
| `--text-md` | 20px | 1.400 | **28px** | $28 = 4 \times 7$ ✓ |
| `--text-lg` | 25px | 1.280 | **32px** | $32 = 4 \times 8$ ✓ |
| `--text-xl` | 31px | 1.290 | **40px** | $40 = 4 \times 10$ ✓ |
| `--text-2xl` | 39px | 1.230 | **48px** | $48 = 4 \times 12$ ✓ |

---

## 5. Font Smoothing & Variable Font Performance

```css
/* High-DPI Font Rendering Optimization */
body {
  font-family: var(--font-family-sans);
  font-size: var(--text-base);
  line-height: var(--leading-normal);
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
  font-feature-settings: "cv02", "cv03", "cv04", "cv11";
}
```

---

## 6. UX Checklist for Typography

- [ ] Base body size is defined in relative `rem` units (never fixed `px` on root body).
- [ ] Every heading line-height aligns strictly to an 8px or 4px baseline grid.
- [ ] Tabular data (numbers, dates, code metrics) uses `font-variant-numeric: tabular-nums` or monospace font stacks.
- [ ] Contrast ratio between text color and background meets WCAG AAA standards (min 7:1 for normal text).
- [ ] Maximum line length for body paragraph text is capped between 45 to 75 characters (`ch` unit).

---

## 7. Key References

- [Apple Human Interface Guidelines - Typography](https://developer.apple.com/design/human-interface-guidelines/typography)
- [W3C WCAG 2.1 Contrast Ratios](https://www.w3.org/TR/WCAG21/#contrast-minimum)
- [Inter Font Family Design Docs by Rasmus Andersson](https://rsms.me/inter/)
