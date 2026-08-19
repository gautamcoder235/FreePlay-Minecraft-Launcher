# ADR 0005: OS Keyring Vault for OAuth2 Token Storage

## Status
Accepted

## Context
Microsoft OAuth2 tokens and tunnel secret keys grant access to user accounts. Storing these sensitive credentials in plain text in JSON, SQLite, or configuration files presents a significant security risk from local malware or accidental log exports.

## Decision
All sensitive tokens (Microsoft refresh tokens, Xbox Live user tokens, playit secret keys) must be stored exclusively in the operating system's native secure credential vault:
- **Windows:** Windows Credential Manager (DPAPI) via the `keyring` crate.
- **macOS:** Apple Keychain Services.
- **Linux:** Secret Service API (Freedesktop / libsecret).

SQLite and disk configurations will store only opaque account IDs and public metadata (username, UUID, skin URL), never raw authentication tokens or secrets.

## Consequences
- Protects user accounts against credential theft.
- Secrets are automatically redacted from error reports, disk logs, and UI exports.
