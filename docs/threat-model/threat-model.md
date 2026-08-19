# FreePlay Minecraft Launcher — Threat Model & Security Policy

## 1. Executive Summary
This document defines the threat landscape, security boundaries, and attack mitigation strategies for the FreePlay Minecraft Launcher.

---

## 2. Trust Boundaries

```
[ Untrusted World / Internet ]
   │
   ▼ (HTTPS Only, Hash Checked)
[ Download & Content Staging Engine ]
   │
   ▼ (Strict Path Validation, No Traversal)
[ Instance / Server Filesystem Storage ]
   ▲
   │ (Controlled IPC, Narrow Capabilities)
[ Webview Frontend (Untrusted Context) ]
   ▲
   │ (Nonced Child Processes, No Shell)
[ Java Client / Server & playit Daemon ]
```

---

## 3. Threat Vectors & Mitigations

### 3.1 Zip Slip & Archive Traversal
- **Threat:** Malicious `.mrpack` or `.zip` modpacks containing filenames like `../../launcher.exe` or `/etc/passwd`.
- **Mitigation:**
  - Strict path sanitization on all archive entries.
  - Reject absolute paths, drive roots (`C:`), and parent directory traversals (`..`).
  - All writes must reside strictly within the resolved target instance directory.

### 3.2 Poisoned or Tampered Downloads
- **Threat:** Man-in-the-middle attacks or CDN compromises serving modified binaries or malicious `.jar` files.
- **Mitigation:**
  - Enforce HTTPS for all external downloads.
  - Maintain a strict domain allowlist for `.mrpack` downloads (`cdn.modrinth.com`, `github.com`, `raw.githubusercontent.com`, `gitlab.com`).
  - Pre-commit SHA-1 and SHA-512 cryptographic verification against trusted manifests.

### 3.3 Credential Theft (OAuth2 Tokens & Secrets)
- **Threat:** Malware reading plain-text tokens or accidental leakage in log exports.
- **Mitigation:**
  - Tokens and secret keys are stored exclusively in the OS Keyring (Windows Credential Manager / DPAPI).
  - All token references in SQLite and application memory use opaque IDs.
  - Automatic regex-based redaction of tokens, authorization headers, and claim URLs from all logs.

### 3.4 Command Injection via Process Arguments
- **Threat:** Unsanitized user inputs passed into shell commands when launching Minecraft or servers.
- **Mitigation:**
  - Zero usage of shell wrappers (`cmd.exe`, `sh`, `powershell`).
  - Process execution uses direct argument arrays (`std::process::Command` / `tokio::process::Command`).
  - Windows background daemons are spawned with `CREATE_NO_WINDOW` (0x08000000) flags.

### 3.5 WebView / Tauri IPC Escalation
- **Threat:** Compromised frontend dependencies calling sensitive backend APIs.
- **Mitigation:**
  - Strict Content Security Policy (CSP).
  - Fine-grained Tauri v2 capabilities granting only explicitly required command IDs.
  - Zero broad filesystem or shell plugin exposure to the webview.
