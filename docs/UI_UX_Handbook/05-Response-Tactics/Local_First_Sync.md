# Local-First UX & CRDT Sync Engine Basics

---

## 1. Overview

Local-first software treats the local client database (SQLite, IndexedDB) as the primary source of truth, synchronizing delta updates asynchronously with remote peers using Conflict-Free Replicated Data Types (CRDTs).

---

## 2. Ink & Switch: The 7 Local-First Principles

1. **No Delays**: Instant local reads and writes.
2. **Multi-device Sync**: Background delta synchronization.
3. **Network Independence**: Operates 100% offline.
4. **Seamless Collaboration**: Merges concurrent edits automatically.
5. **Longevity of Data**: Local files remain accessible forever.
6. **Security & Privacy**: End-to-end encryption compatible.
7. **User Ownership**: Physical ownership of data files.

---

## 3. Last-Write-Wins Register (LWW-Register) CRDT Implementation

```typescript
export interface LWWRegister<T> {
  value: T;
  timestamp: number;
  peerId: string;
}

export class LWWMap<V> {
  private store = new Map<string, LWWRegister<V>>();
  private peerId: string;

  constructor(peerId: string) {
    this.peerId = peerId;
  }

  public set(key: string, value: V, timestamp = Date.now()) {
    const existing = this.store.get(key);
    if (!existing || timestamp > existing.timestamp || (timestamp === existing.timestamp && this.peerId > existing.peerId)) {
      this.store.set(key, { value, timestamp, peerId: this.peerId });
    }
  }

  public get(key: string): V | undefined {
    return this.store.get(key)?.value;
  }
}
```

---

## 4. Key References

- [Ink & Switch - Local-First Software Essay](https://www.inkandswitch.com/local-first/)
- [Automerge Data Structure Library](https://automerge.org/)
- [Yjs CRDT Framework](https://yjs.dev/)
