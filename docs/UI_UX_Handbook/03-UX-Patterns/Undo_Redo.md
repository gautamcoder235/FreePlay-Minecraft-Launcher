# Undo / Redo History Stack Engine

---

## 1. Overview

Every destructive or state-mutating action in a Tier-1 app (Linear, Figma, VS Code) must be revertible via `Cmd+Z` / `Ctrl+Z`.

---

## 2. Command Pattern Implementation

```typescript
export interface Command {
  execute(): void;
  undo(): void;
}

export class HistoryManager {
  private undoStack: Command[] = [];
  private redoStack: Command[] = [];

  public execute(cmd: Command) {
    cmd.execute();
    this.undoStack.push(cmd);
    this.redoStack = [];
  }

  public undo() {
    const cmd = this.undoStack.pop();
    if (cmd) {
      cmd.undo();
      this.redoStack.push(cmd);
    }
  }

  public redo() {
    const cmd = this.redoStack.pop();
    if (cmd) {
      cmd.execute();
      this.undoStack.push(cmd);
    }
  }
}
```

---

## 3. Key References

- [Figma Undo Engine Engineering Post](https://www.figma.com/blog/engineering/)
- [Design Patterns: Command Pattern](https://refactoring.guru/design-patterns/command)
