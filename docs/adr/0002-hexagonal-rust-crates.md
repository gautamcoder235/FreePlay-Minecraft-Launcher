# ADR 0002: Hexagonal Architecture (Ports and Adapters) in Rust Crates

## Status
Accepted

## Context
The launcher interacts with numerous external systems: Microsoft Auth, Mojang CDN, Modrinth API, PaperMC, SQLite, OS Keyring, Java runtimes, and reverse proxy tunnels (playit.gg / manual). Tight coupling between these services and the UI would make the system fragile, difficult to test, and hard to extend.

## Decision
We organize the Rust backend using **Hexagonal Architecture (Ports and Adapters)**:
1. `crates/core/domain`: Pure entities and state machines with zero I/O.
2. `crates/core/application`: Use cases and abstract `Port` traits.
3. `crates/adapters/*`: Concrete implementations of ports (HTTP, SQLite, Keyring, Playit, etc.).
4. `crates/desktop-api`: Thin Tauri IPC controller exposing use cases to the frontend.

## Consequences
- Every domain rule and state transition is 100% unit-testable without mocking networks or filesystems.
- New providers (e.g. NeoForge, Purpur, Cloudflare Tunnel) can be added as isolated adapters without modifying existing code.
