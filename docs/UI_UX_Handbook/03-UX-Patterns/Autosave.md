# Autosave & Background Synchronization

---

## 1. Overview

Tier-1 applications eliminate manual "Save" buttons. Edits save automatically to local storage within **100ms** of keypress, debouncing network updates to remote servers while guaranteeing zero data loss.

---

## 2. Debounced Autosave Pipeline

```typescript
import { useEffect, useRef } from 'react';

export function useAutosave<T>(data: T, onSave: (data: T) => Promise<void>, delayMs = 1000) {
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    // Write instant copy to LocalStorage immediately
    localStorage.setItem('draft_cache', JSON.stringify(data));

    // Debounce network save API request
    if (timeoutRef.current) clearTimeout(timeoutRef.current);
    timeoutRef.current = setTimeout(() => {
      onSave(data);
    }, delayMs);

    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, [data, onSave, delayMs]);
}
```

---

## 3. Key References

- [Linear Sync Engine Architecture](https://linear.app/blog/scaling-the-linear-sync-engine)
- [Ink & Switch - Local-First Software](https://www.inkandswitch.com/local-first/)
