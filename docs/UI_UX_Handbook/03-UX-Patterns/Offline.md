# Offline Mode & Network Reconnection Queues

---

## 1. Overview

A network disconnection should never block user interaction. Tier-1 applications queue mutations offline into IndexedDB/SQLite outbox queues and flush updates automatically upon reconnection.

---

## 2. Reconnection Outbox Queue Diagram

```mermaid
sequenceDiagram
    participant User
    participant LocalDB as Local IndexedDB
    participant Outbox as Reconnection Queue
    participant Server as Remote API

    User->>LocalDB: Perform Action (Offline)
    LocalDB->>Outbox: Append Delta Event to Outbox
    Note over Outbox: Device Offline - Queue Persisted
    Note over Outbox: Network Connectivity Restored
    Outbox->>Server: POST Outbox Deltas (Exponential Backoff)
    Server-->>Outbox: ACK Sync Success
    Outbox->>LocalDB: Clear Flushed Outbox Items
```

---

## 3. Key References

- [Ink & Switch - Local-First Software](https://www.inkandswitch.com/local-first/)
- [MDN Network Information API](https://developer.mozilla.org/en-US/docs/Web/API/Network_Information_API)
