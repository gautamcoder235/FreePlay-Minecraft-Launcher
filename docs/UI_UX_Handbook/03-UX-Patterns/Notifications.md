# Notification & Toast Queuing Architecture

---

## 1. Overview

Toast notifications acknowledge completed background operations. Tier-1 applications limit maximum visible toasts to **3**, automatically collapse identical alerts into counters (`"5 changes saved"`), and support inline **Undo** actions.

---

## 2. Queue Engine Architecture

```typescript
export interface Toast {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  message: string;
  action?: { label: string; onClick: () => void };
  duration?: number;
}

export class ToastQueue {
  private queue: Toast[] = [];
  private maxVisible = 3;

  public push(toast: Toast) {
    this.queue.push(toast);
    if (this.queue.length > 10) this.queue.shift(); // Evict oldest
  }

  public getVisible(): Toast[] {
    return this.queue.slice(-this.maxVisible);
  }
}
```

---

## 3. Key References

- [Sonner - Toast Library Architecture](https://sonner.emilkowal.ski/)
- [W3C ARIA Live Regions Specification](https://www.w3.org/WAI/ARIA/apg/practices/aria-live/)
