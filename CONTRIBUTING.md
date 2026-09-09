# Contributing to FreePlay

Thank you for your interest in contributing to **FreePlay**! We welcome contributions from developers, designers, and Minecraft community members of all skill levels.

---

## 🧭 Code of Conduct

All contributors and participants are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before participating in discussions or submitting code.

---

## 🛠️ Development Setup

### Prerequisites
- **Node.js**: `v24+` (managed via `nvm` / `.nvmrc` or direct install)
- **pnpm**: `v10+` (`corepack enable pnpm`)
- **Rust & Cargo**: Latest stable toolchain (`rustup default stable`)
- **C++ Build Tools**: Visual Studio C++ Build Tools (on Windows) or standard GCC/Clang on Unix

### 1. Clone & Install
```bash
git clone https://github.com/gautamcoder235/FreePlay-Minecraft-Launcher.git
cd FreePlay-Minecraft-Launcher
pnpm install
```

### 2. Running in Development Mode
- **Desktop Application (Tauri + Vue):**
  ```bash
  pnpm app:dev
  ```
- **Web Frontend (Nuxt):**
  ```bash
  pnpm web:dev
  ```
- **Shared UI Component Storybook:**
  ```bash
  pnpm storybook
  ```

---

## 🏗️ Monorepo Architecture

FreePlay is organized as a Turborepo monorepo with pnpm workspaces:

```
├── apps/
│   ├── app/              # Tauri v2 desktop application shell (Rust)
│   ├── app-frontend/     # Desktop UI (Vue 3 + Vite + Tailwind CSS)
│   ├── frontend/         # Web platform (Nuxt 3)
│   └── docs/             # Documentation site
├── crates/               # Core Rust domain & engine modules
│   ├── core/             # Domain logic and application services
│   └── adapters/         # Process supervisor, storage, and engine adapters
├── packages/             # Shared libraries and packages
│   ├── app-lib/          # Core launcher logic & Theseus state manager
│   ├── ui/               # Reusable Vue component library (@freeplay/ui)
│   ├── assets/           # Auto-generated icons, artwork, and styles
│   └── api-client/       # TypeScript API client
```

---

## 📐 Coding Guidelines & Standards

### Indentation & Formatting
- **Indentation:** Use **TAB** everywhere (never spaces).
- **Formatters:** Prettier & ESLint for frontend; `cargo fmt` & `cargo clippy` for Rust.

### Code Style & Comments
- Avoid "heading" comments like `=== Helper methods ===`.
- Write self-documenting code with clear variable and function names. Use doc comments (`/** ... */` or `/// ...`) when documenting public APIs.

### Pre-PR Verification Commands
Run these commands from the root before opening a pull request:
- **App Frontend:** `pnpm prepr:frontend:app`
- **Frontend Libraries:** `pnpm prepr:frontend:lib`
- **All Frontend:** `pnpm prepr`
- **Rust Checks:** `cargo check -p freeplay_launcher`

---

## 🔄 Pull Request Workflow

1. **Fork & Branch:** Create a feature branch from `main` (e.g., `feature/custom-tunnel-ports` or `fix/overlay-keybind`).
2. **Make Changes:** Keep commits focused and provide clear, descriptive commit messages.
3. **Verify:** Run the appropriate `prepr` or `cargo check` commands to ensure 0 lint errors and build cleanly.
4. **Submit PR:** Open a Pull Request against `main`. Fill in the PR template with a description of the change, related issues, and testing steps.

---

## 🐛 Reporting Bugs & Requesting Features

- **Bug Reports:** Search [existing issues](https://github.com/gautamcoder235/FreePlay-Minecraft-Launcher/issues) first. If reporting a new bug, use the [Bug Report Template](https://github.com/gautamcoder235/FreePlay-Minecraft-Launcher/issues/new/choose) and include detailed steps to reproduce.
- **Feature Requests:** Open a [Feature Request](https://github.com/gautamcoder235/FreePlay-Minecraft-Launcher/issues/new/choose) describing the proposed feature, user benefits, and any design ideas.

---

## 📜 License

By contributing to FreePlay, you agree that your contributions will be licensed under the [GNU General Public License v3.0](LICENSE).
