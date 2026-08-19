# Command Palette Component Specification

---

## 1. Overview

The Command Palette component provides global command search, hotkey display, recent items, and category grouping.

---

## 2. Component API Contract

```typescript
export interface CommandItemSpec {
  id: string;
  label: string;
  category: 'Navigation' | 'Actions' | 'Settings';
  shortcut?: string[]; // e.g. ['Cmd', 'K']
  onSelect: () => void;
}
```

---

## 3. Key References

- [cmdk Command Component Engine](https://cmdk.pavel.as/)
- [Raycast Product Architecture](https://www.raycast.com/)
