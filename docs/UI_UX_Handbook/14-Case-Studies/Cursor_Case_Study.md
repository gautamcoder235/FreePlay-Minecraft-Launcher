# Product Breakdown: Cursor IDE

---

## 1. Overview

Cursor redefined AI pair programming by embedding real-time token streaming, ghost text inline diffs, and shadow workspace AST parsers into an Electron VS Code fork.

---

## 2. Key UX Architectural Features

- **Inline Ghost Text Diffs**: Rendered via low-level Monaco Editor `ICursorDecoration` layers without text layout jumps.
- **Fast IPC Channel**: Binary buffers between C++ language server host and Electron renderer process.
- **Composer Workspace Panel**: Multi-file edit preview with non-blocking stream cancellation.

---

## 3. Key References

- [Cursor Product Architecture](https://www.cursor.com/)
- [Monaco Editor API Specification](https://microsoft.github.io/monaco-editor/)
