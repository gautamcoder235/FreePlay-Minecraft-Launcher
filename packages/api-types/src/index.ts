// Auto-generated IPC type definitions from Rust crates (freeplay-domain & freeplay-desktop-api)

export type DomainError =
  | { kind: "validation"; details: string }
  | { kind: "not_found"; details: { entity_type: string; id: string } }
  | { kind: "conflict"; details: string }
  | { kind: "invalid_state_transition"; details: { from: string; to: string; reason: string } }
  | { kind: "authentication"; details: string }
  | { kind: "security"; details: string }
  | { kind: "cancelled" }
  | { kind: "internal"; details: string };

export type AccountKind = "microsoft" | "offline";

export type EntitlementStatus = "entitled" | "not_entitled" | "offline";

export interface AccountIdentity {
  id: string;
  kind: AccountKind;
  minecraft_username: String;
  minecraft_uuid: string;
  skin_url: string | null;
  entitlement: EntitlementStatus;
  is_active: boolean;
  added_at: string;
  last_used_at: string | null;
}

export type LoaderType =
  | { vanilla: Record<string, never> }
  | { fabric: { version: string } }
  | { forge: { version: string } }
  | { neo_forge: { version: string } }
  | { quilt: { version: string } };

export type InstanceRunState =
  | "stopped"
  | "preparing"
  | "launching"
  | { running: { pid: number; started_at: string } }
  | "stopping"
  | { crashed: { exit_code: number | null } };

export interface InstanceSettings {
  min_memory_mb: number;
  max_memory_mb: number;
  java_path: string | null;
  jvm_args: string[];
  window_width: number;
  window_height: number;
}

export interface Instance {
  id: string;
  name: string;
  icon_path: string | null;
  game_version: string;
  loader: LoaderType;
  settings: InstanceSettings;
  state: InstanceRunState;
  total_play_time_seconds: number;
  last_played_at: string | null;
  created_at: string;
  updated_at: string;
}

export type ServerEngine =
  | "vanilla"
  | { paper: { build: number | null } }
  | { purpur: { build: number | null } }
  | { fabric: { loader_version: string } };

export interface EulaAgreement {
  agreed: boolean;
  agreed_at: string | null;
  eula_version: string;
}

export type ServerRunState =
  | "stopped"
  | "preparing"
  | "starting"
  | { ready: { port: number; online_players: number; max_players: number; started_at: string } }
  | "stopping"
  | { crashed: { exit_code: number | null } };

export interface ServerProfile {
  id: string;
  name: string;
  game_version: string;
  engine: ServerEngine;
  eula: EulaAgreement;
  state: ServerRunState;
  created_at: string;
  updated_at: string;
}

export type ProviderKind = "manual" | "playit" | "fake";

export type TunnelStatus =
  | "disabled"
  | { claiming: { claim_url: string } }
  | "connecting"
  | { online: { public_endpoint: string; connected_at: string } }
  | { degraded: { reason: string } }
  | "stopping"
  | "stopped";

export interface TunnelConfig {
  provider: ProviderKind;
  local_port: number;
  protocol: string;
  secret_key: string | null;
}

export interface AppStatusResponse {
  name: string;
  version: string;
  environment: string;
  os: string;
  architecture: string;
}
