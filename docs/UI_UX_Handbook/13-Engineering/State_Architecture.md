# Engineering State Architecture & Sync Engines

---

## 1. Overview

Front-end architecture in Tier-1 software decouples local view rendering from async sync pipelines using normalized state stores, immutable state reducers, and deterministic CRDT handlers.

---

## 2. Unidirectional Data Flow & Sync Engine Architecture

```mermaid
graph LR
    User[User Input] --> Action[Dispatch Action / Command]
    Action --> Reducer[State Reducer]
    Reducer --> LocalDB[Local Store / IndexedDB]
    LocalDB --> View[React Render View (0ms)]
    LocalDB --> Worker[Background Sync Engine]
    Worker --> Remote[Cloud API / WebSocket Server]
```

---

## 3. Key References

- [Linear Engineering - Local Sync Engine](https://linear.app/blog/scaling-the-linear-sync-engine)
- [Redux Toolkit / Zustand Architecture Patterns](https://zustand-demo.pmnd.rs/)
