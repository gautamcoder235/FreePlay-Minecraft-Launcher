# ADR 0003: Strict Path Containment and Hash Verification

## Status
Accepted

## Context
Third-party modpack archives (`.mrpack`, `.zip`) and downloads contain untrusted file paths and external resources. Malicious archives can execute `Zip Slip` attacks (directory traversal using `../` to overwrite system or launcher binaries), and corrupted or poisoned downloads can compromise user machines.

## Decision
1. **Bounded Extraction:** All archive extractors must parse entry paths, reject absolute paths, reject directory escapes (`..`), and enforce maximum byte and file count bounds.
2. **Path Containment:** Extractions and overrides are strictly confined to the targeted `instances/<id>` or `servers/<id>` directory.
3. **Pre-Commit Hash Verification:** Every downloaded artifact is downloaded to a staging file, verified against its SHA-1/SHA-512 manifest hash, and atomically moved to destination.

## Consequences
- Complete mitigation of Zip Slip and corrupted file execution vulnerabilities.
- Safe, deterministic modpack installation and rollback on verification failure.
