# Optimistic UI & Predictive Rendering

---

## 1. Overview

Optimistic UI updates local client state instantly upon user input without waiting for server network roundtrips.

---

## 2. Optimistic Mutation Flowchart

```mermaid
sequenceDiagram
    participant User
    participant LocalState as React State / Local DB
    participant API as Remote Server

    User->>LocalState: Click Checkbox / Create Task
    LocalState-->>User: Render Instantly in 0ms (Optimistic Update)
    LocalState->>API: POST Mutation Request
    alt Server Success
        API-->>LocalState: Confirm ACK (Reconcile UUID)
    else Server Failure
        API-->>LocalState: Rollback Mutation & Show Retry Toast
        LocalState-->>User: Revert UI State + Alert
    end
```

---

## 3. Key References

- [Linear Blog - Optimistic UI and Sync Engines](https://linear.app/blog/optimistic-ui-and-sync-engine)
- [TanStack Query Optimistic Updates Guide](https://tanstack.com/query/latest/docs/framework/react/guides/optimistic-updates)
