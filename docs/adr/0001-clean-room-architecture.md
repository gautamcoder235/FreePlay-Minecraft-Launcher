# ADR 0001: Clean-Room Modular Architecture

## Status
Accepted

## Context
The project aims to build an open-source, high-performance Minecraft: Java Edition launcher with integrated hosting capabilities. Upstream open-source codebases (like Modrinth monorepo) provide valuable reference architecture, but direct copy-pasting of legacy monolithic modules introduces technical debt, legacy dependencies, and maintenance burden.

## Decision
We adopt a **Clean-Room, strictly modular Architecture**. Core domain logic, state machines, process supervisors, and IPC adapters are implemented with modern Rust (Tauri v2 + Tokio) and TypeScript (Vue 3 + Vite + Tailwind). Upstream code is used exclusively as a design and protocol reference.

## Consequences
- Clean, minimal dependency trees with zero bloat.
- 100% ownership and deep understanding of every component.
- Clear licensing boundaries and full compliance with GPLv3.
