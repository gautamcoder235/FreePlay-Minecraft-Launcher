# Theme Design Tokens Engine

---

## 1. Overview

Theme engines allow users to switch seamlessly between Light, Dark, OLED, and Custom Brand themes. Components reference semantic CSS variables (`var(--bg-surface)`), which map dynamically based on the active `[data-theme]` attribute.

---

## 2. Dynamic Theme Schema Definition

```css
/* Light Theme Mapping */
:root[data-theme="light"] {
  --bg-app: #f8fafc;
  --bg-surface: #ffffff;
  --text-primary: #0f172a;
  --text-secondary: #64748b;
  --border-subtle: #e2e8f0;
}

/* Dark Theme Mapping */
:root[data-theme="dark"] {
  --bg-app: #090d16;
  --bg-surface: #0f172a;
  --text-primary: #f8fafc;
  --text-secondary: #94a3b8;
  --border-subtle: #1e293b;
}
```

---

## 3. Key References

- [GitHub Primer Design System - Theme Engine](https://primer.style/primitives/colors)
- [W3C Design Tokens Community Group (DTCG)](https://www.designtokens.org/)
