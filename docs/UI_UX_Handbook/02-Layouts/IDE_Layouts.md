# IDE & Code Editor Layout Architecture

---

## 1. Overview

IDE layouts (VS Code, Cursor, JetBrains) manage maximum data density without inducing visual cognitive overload. IDE layout engines handle tab splitting, active tab scrolling, treeview file explorers, and bottom output panels.

---

## 2. Split-View Grid Engine & Tab Architecture

```
+-------------------------------------------------------------------+
| Tab 1 (Active) | Tab 2          | Tab 3          | [ Split Right ]|
+---------------------------------+---------------------------------+
|                                 |                                 |
| Primary Editor Pane             | Secondary Editor Pane           |
| (Monaco / CodeMirror Instance) | (Diff / Reference Code)         |
|                                 |                                 |
+---------------------------------+---------------------------------+
```

---

## 3. Keyboard Shortcut Matrix for IDE Workspaces

| Action | macOS Shortcut | Windows / Linux Shortcut | Context |
| :--- | :--- | :--- | :--- |
| **Toggle Primary Sidebar** | `Cmd + B` | `Ctrl + B` | Global Workspace |
| **Toggle Bottom Terminal** | `Cmd + \`` | `Ctrl + \`` | Global Workspace |
| **Quick Open File** | `Cmd + P` | `Ctrl + P` | Command Palette |
| **Split Active Editor Right** | `Cmd + \` | `Ctrl + \` | Active Editor Pane |
| **Switch Active Tab** | `Cmd + 1..9` | `Ctrl + 1..9` | Tab Strip |

---

## 4. Key References

- [Monaco Editor API Specification](https://microsoft.github.io/monaco-editor/)
- [VS Code Grid Layout Engine](https://github.com/microsoft/vscode)
