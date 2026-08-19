# ADR 0004: Provider-Neutral Tunnel Abstraction

## Status
Accepted

## Context
The launcher provides built-in online hosting to allow users to play with friends without port forwarding. However, relying solely on hardcoded integration with a single service (like playit.gg) creates tight coupling and fragility.

## Decision
We define a generic, provider-neutral port:
```rust
#[async_trait]
pub trait TunnelProviderPort: Send + Sync {
    async fn preflight(&self) -> Result<TunnelCapability, DomainError>;
    async fn start_tunnel(&self, config: TunnelConfig) -> Result<TunnelHandle, DomainError>;
    async fn observe_status(&self, handle: &TunnelHandle) -> Result<TunnelStatus, DomainError>;
    async fn stop_tunnel(&self, handle: TunnelHandle) -> Result<(), DomainError>;
}
```
We provide three initial implementations:
- `ManualTunnelAdapter`: User provides their own external IP / port forwarding.
- `PlayitTunnelAdapter`: Headless execution of `playit.exe` (`CREATE_NO_WINDOW` on Windows).
- `FakeTunnelAdapter`: For automated unit and integration tests.

## Consequences
- The launcher backend is not married to any single tunnel vendor.
- Headless, clean execution with zero console window popups.
- Easy to add alternative tunneling providers in the future.
