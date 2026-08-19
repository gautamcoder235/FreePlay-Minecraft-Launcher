# Artifact Panels & Workspace UI

---

## 1. Overview

Applications like **Claude Artifacts** and **Cursor Composer** separate raw conversational text from persistent outputs (code files, SVG previews, HTML prototypes) using a split workspace layout.

---

## 2. Dual-Pane Workspace Layout Architecture

```
+-----------------------------------+-----------------------------------+
| CONVERSATION PANE (35-40% Width)  | ARTIFACT WORKSPACE (60-65% Width) |
+-----------------------------------+-----------------------------------+
| User: Create a React chart component| [ Code Editor ] [ Live Preview ]  |
| Assistant: Here is the code.      |                                   |
| [ View Artifact: ChartWrapper ]   | export function Chart() { ... }   |
+-----------------------------------+-----------------------------------+
```

---

## 3. Key References

- [Anthropic Claude Artifacts Feature Announcement](https://www.anthropic.com/news/artifacts)
- [Cursor Composer Product Overview](https://www.cursor.com/)
