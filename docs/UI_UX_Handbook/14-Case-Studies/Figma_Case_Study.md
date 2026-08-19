# Product Breakdown: Figma

---

## 1. Overview

Figma achieves desktop-class graphic performance in web browsers by executing core vector operations in C++ compiled to WebAssembly (Wasm) and rendering directly to WebGL canvases.

---

## 2. Key Engineering Pillars

- **Wasm Scene Graph**: Bypasses browser HTML DOM entirely for canvas node management.
- **WebGL Rendering Loop**: Renders 50,000+ vector shapes at locked 60/120 FPS frame rates.
- **Multi-user Real-time Sync**: Fractional indexing and custom binary multiplayer server protocols.

---

## 3. Key References

- [Figma Engineering Blog](https://www.figma.com/blog/engineering/)
