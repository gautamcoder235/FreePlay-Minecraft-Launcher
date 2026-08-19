use std::path::Path;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use tokio::sync::Mutex;
use uuid::Uuid;

use freeplay_application::StoragePort;
use freeplay_domain::{
    AccountId, AccountIdentity, AccountKind, DomainError, EntitlementStatus, Instance, InstanceId,
    InstanceRunState, InstanceSettings, LoaderType, ServerEngine, ServerId, ServerProfile,
    ServerRunState, ServerSettings, TunnelId, TunnelRun, TunnelStatus,
};

pub struct SqliteStorage {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStorage {
    /// Opens or creates a SQLite database at the specified path and runs migrations.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DomainError> {
        let conn = Connection::open(path).map_err(|e| DomainError::Internal(format!("Failed to open SQLite db: {e}")))?;
        Self::migrate_conn(&conn)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Opens an in-memory SQLite database (ideal for unit and integration testing).
    pub fn open_in_memory() -> Result<Self, DomainError> {
        let conn = Connection::open_in_memory().map_err(|e| DomainError::Internal(format!("Failed to open in-memory SQLite db: {e}")))?;
        Self::migrate_conn(&conn)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    fn migrate_conn(conn: &Connection) -> Result<(), DomainError> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS instances (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                icon_path TEXT,
                game_version TEXT NOT NULL,
                loader_json TEXT NOT NULL,
                settings_json TEXT NOT NULL,
                state_json TEXT NOT NULL,
                total_play_time_seconds INTEGER NOT NULL,
                last_played_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                username TEXT NOT NULL,
                uuid TEXT NOT NULL,
                skin_url TEXT,
                entitlement TEXT NOT NULL,
                is_active INTEGER NOT NULL DEFAULT 0,
                added_at TEXT NOT NULL,
                last_used_at TEXT
            );

            CREATE TABLE IF NOT EXISTS servers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                game_version TEXT NOT NULL,
                engine_json TEXT NOT NULL,
                eula_json TEXT NOT NULL,
                settings_json TEXT NOT NULL,
                state_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tunnels (
                id TEXT PRIMARY KEY,
                provider TEXT NOT NULL,
                status_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .map_err(|e| DomainError::Internal(format!("Failed to run SQLite schema migrations: {e}")))?;
        Ok(())
    }
}

#[async_trait]
impl StoragePort for SqliteStorage {
    // -------------------------------------------------------------------------
    // Instances
    // -------------------------------------------------------------------------

    async fn get_instance(&self, id: &InstanceId) -> Result<Option<Instance>, DomainError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare(
                "SELECT id, name, icon_path, game_version, loader_json, settings_json, state_json, \
                 total_play_time_seconds, last_played_at, created_at, updated_at \
                 FROM instances WHERE id = ?1",
            )
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let id_str = id.0.to_string();
        let mut rows = stmt
            .query_map(params![id_str], |row| {
                let id_raw: String = row.get(0)?;
                let name: String = row.get(1)?;
                let icon_path: Option<String> = row.get(2)?;
                let game_version: String = row.get(3)?;
                let loader_json: String = row.get(4)?;
                let settings_json: String = row.get(5)?;
                let state_json: String = row.get(6)?;
                let total_play_time: u64 = row.get(7)?;
                let last_played_raw: Option<String> = row.get(8)?;
                let created_raw: String = row.get(9)?;
                let updated_raw: String = row.get(10)?;

                Ok((
                    id_raw,
                    name,
                    icon_path,
                    game_version,
                    loader_json,
                    settings_json,
                    state_json,
                    total_play_time,
                    last_played_raw,
                    created_raw,
                    updated_raw,
                ))
            })
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some(row_res) = rows.next() {
            let (
                id_raw,
                name,
                icon_path,
                game_version,
                loader_json,
                settings_json,
                state_json,
                total_play_time,
                last_played_raw,
                created_raw,
                updated_raw,
            ) = row_res.map_err(|e| DomainError::Internal(e.to_string()))?;

            let id_uuid = Uuid::parse_str(&id_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let loader: LoaderType = serde_json::from_str(&loader_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt loader JSON: {e}")))?;
            let settings: InstanceSettings = serde_json::from_str(&settings_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt settings JSON: {e}")))?;
            let state: InstanceRunState = serde_json::from_str(&state_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt state JSON: {e}")))?;
            let last_played_at = last_played_raw
                .map(|s| s.parse::<DateTime<Utc>>())
                .transpose()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let created_at = created_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let updated_at = updated_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            Ok(Some(Instance {
                id: InstanceId(id_uuid),
                name,
                icon_path,
                game_version,
                loader,
                settings,
                state,
                total_play_time_seconds: total_play_time,
                last_played_at,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn list_instances(&self) -> Result<Vec<Instance>, DomainError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare(
                "SELECT id, name, icon_path, game_version, loader_json, settings_json, state_json, \
                 total_play_time_seconds, last_played_at, created_at, updated_at \
                 FROM instances ORDER BY updated_at DESC",
            )
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let id_raw: String = row.get(0)?;
                let name: String = row.get(1)?;
                let icon_path: Option<String> = row.get(2)?;
                let game_version: String = row.get(3)?;
                let loader_json: String = row.get(4)?;
                let settings_json: String = row.get(5)?;
                let state_json: String = row.get(6)?;
                let total_play_time: u64 = row.get(7)?;
                let last_played_raw: Option<String> = row.get(8)?;
                let created_raw: String = row.get(9)?;
                let updated_raw: String = row.get(10)?;

                Ok((
                    id_raw,
                    name,
                    icon_path,
                    game_version,
                    loader_json,
                    settings_json,
                    state_json,
                    total_play_time,
                    last_played_raw,
                    created_raw,
                    updated_raw,
                ))
            })
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut instances = Vec::new();
        for r in rows {
            let (
                id_raw,
                name,
                icon_path,
                game_version,
                loader_json,
                settings_json,
                state_json,
                total_play_time,
                last_played_raw,
                created_raw,
                updated_raw,
            ) = r.map_err(|e| DomainError::Internal(e.to_string()))?;

            let id_uuid = Uuid::parse_str(&id_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let loader: LoaderType = serde_json::from_str(&loader_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt loader JSON: {e}")))?;
            let settings: InstanceSettings = serde_json::from_str(&settings_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt settings JSON: {e}")))?;
            let state: InstanceRunState = serde_json::from_str(&state_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt state JSON: {e}")))?;
            let last_played_at = last_played_raw
                .map(|s| s.parse::<DateTime<Utc>>())
                .transpose()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let created_at = created_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let updated_at = updated_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            instances.push(Instance {
                id: InstanceId(id_uuid),
                name,
                icon_path,
                game_version,
                loader,
                settings,
                state,
                total_play_time_seconds: total_play_time,
                last_played_at,
                created_at,
                updated_at,
            });
        }
        Ok(instances)
    }

    async fn save_instance(&self, instance: &Instance) -> Result<(), DomainError> {
        let conn = self.conn.lock().await;
        let id_str = instance.id.0.to_string();
        let loader_json = serde_json::to_string(&instance.loader)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let settings_json = serde_json::to_string(&instance.settings)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let state_json = serde_json::to_string(&instance.state)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let last_played_str = instance.last_played_at.map(|d| d.to_rfc3339());
        let created_str = instance.created_at.to_rfc3339();
        let updated_str = instance.updated_at.to_rfc3339();

        conn.execute(
            r#"
            INSERT INTO instances (
                id, name, icon_path, game_version, loader_json, settings_json, state_json,
                total_play_time_seconds, last_played_at, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                icon_path = excluded.icon_path,
                game_version = excluded.game_version,
                loader_json = excluded.loader_json,
                settings_json = excluded.settings_json,
                state_json = excluded.state_json,
                total_play_time_seconds = excluded.total_play_time_seconds,
                last_played_at = excluded.last_played_at,
                updated_at = excluded.updated_at
            "#,
            params![
                id_str,
                instance.name,
                instance.icon_path,
                instance.game_version,
                loader_json,
                settings_json,
                state_json,
                instance.total_play_time_seconds,
                last_played_str,
                created_str,
                updated_str
            ],
        )
        .map_err(|e| DomainError::Internal(format!("Failed to save instance: {e}")))?;

        Ok(())
    }

    async fn delete_instance(&self, id: &InstanceId) -> Result<(), DomainError> {
        let conn = self.conn.lock().await;
        let id_str = id.0.to_string();
        conn.execute("DELETE FROM instances WHERE id = ?1", params![id_str])
            .map_err(|e| DomainError::Internal(format!("Failed to delete instance: {e}")))?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Accounts
    // -------------------------------------------------------------------------

    async fn get_account(&self, id: &AccountId) -> Result<Option<AccountIdentity>, DomainError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare(
                "SELECT id, kind, username, uuid, skin_url, entitlement, is_active, added_at, last_used_at \
                 FROM accounts WHERE id = ?1",
            )
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let id_str = id.0.to_string();
        let mut rows = stmt
            .query_map(params![id_str], |row| {
                let id_raw: String = row.get(0)?;
                let kind_raw: String = row.get(1)?;
                let username: String = row.get(2)?;
                let uuid_raw: String = row.get(3)?;
                let skin_url: Option<String> = row.get(4)?;
                let entitlement_raw: String = row.get(5)?;
                let is_active_raw: i32 = row.get(6)?;
                let added_raw: String = row.get(7)?;
                let last_used_raw: Option<String> = row.get(8)?;

                Ok((
                    id_raw,
                    kind_raw,
                    username,
                    uuid_raw,
                    skin_url,
                    entitlement_raw,
                    is_active_raw,
                    added_raw,
                    last_used_raw,
                ))
            })
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some(row_res) = rows.next() {
            let (
                id_raw,
                kind_raw,
                username,
                uuid_raw,
                skin_url,
                entitlement_raw,
                is_active_raw,
                added_raw,
                last_used_raw,
            ) = row_res.map_err(|e| DomainError::Internal(e.to_string()))?;

            let id_uuid = Uuid::parse_str(&id_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let mc_uuid = Uuid::parse_str(&uuid_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let kind: AccountKind = serde_json::from_str(&format!("\"{}\"", kind_raw))
                .map_err(|e| DomainError::Internal(format!("Corrupt account kind: {e}")))?;
            let entitlement: EntitlementStatus = serde_json::from_str(&format!("\"{}\"", entitlement_raw))
                .map_err(|e| DomainError::Internal(format!("Corrupt entitlement: {e}")))?;
            let added_at = added_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let last_used_at = last_used_raw
                .map(|s| s.parse::<DateTime<Utc>>())
                .transpose()
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            Ok(Some(AccountIdentity {
                id: AccountId(id_uuid),
                kind,
                minecraft_username: username,
                minecraft_uuid: mc_uuid,
                skin_url,
                entitlement,
                is_active: is_active_raw == 1,
                added_at,
                last_used_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_active_account(&self) -> Result<Option<AccountIdentity>, DomainError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare(
                "SELECT id, kind, username, uuid, skin_url, entitlement, is_active, added_at, last_used_at \
                 FROM accounts WHERE is_active = 1 LIMIT 1",
            )
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut rows = stmt
            .query_map([], |row| {
                let id_raw: String = row.get(0)?;
                let kind_raw: String = row.get(1)?;
                let username: String = row.get(2)?;
                let uuid_raw: String = row.get(3)?;
                let skin_url: Option<String> = row.get(4)?;
                let entitlement_raw: String = row.get(5)?;
                let is_active_raw: i32 = row.get(6)?;
                let added_raw: String = row.get(7)?;
                let last_used_raw: Option<String> = row.get(8)?;

                Ok((
                    id_raw,
                    kind_raw,
                    username,
                    uuid_raw,
                    skin_url,
                    entitlement_raw,
                    is_active_raw,
                    added_raw,
                    last_used_raw,
                ))
            })
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some(row_res) = rows.next() {
            let (
                id_raw,
                kind_raw,
                username,
                uuid_raw,
                skin_url,
                entitlement_raw,
                is_active_raw,
                added_raw,
                last_used_raw,
            ) = row_res.map_err(|e| DomainError::Internal(e.to_string()))?;

            let id_uuid = Uuid::parse_str(&id_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let mc_uuid = Uuid::parse_str(&uuid_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let kind: AccountKind = serde_json::from_str(&format!("\"{}\"", kind_raw))
                .map_err(|e| DomainError::Internal(format!("Corrupt account kind: {e}")))?;
            let entitlement: EntitlementStatus = serde_json::from_str(&format!("\"{}\"", entitlement_raw))
                .map_err(|e| DomainError::Internal(format!("Corrupt entitlement: {e}")))?;
            let added_at = added_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let last_used_at = last_used_raw
                .map(|s| s.parse::<DateTime<Utc>>())
                .transpose()
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            Ok(Some(AccountIdentity {
                id: AccountId(id_uuid),
                kind,
                minecraft_username: username,
                minecraft_uuid: mc_uuid,
                skin_url,
                entitlement,
                is_active: is_active_raw == 1,
                added_at,
                last_used_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn list_accounts(&self) -> Result<Vec<AccountIdentity>, DomainError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare(
                "SELECT id, kind, username, uuid, skin_url, entitlement, is_active, added_at, last_used_at \
                 FROM accounts ORDER BY is_active DESC, added_at DESC",
            )
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let id_raw: String = row.get(0)?;
                let kind_raw: String = row.get(1)?;
                let username: String = row.get(2)?;
                let uuid_raw: String = row.get(3)?;
                let skin_url: Option<String> = row.get(4)?;
                let entitlement_raw: String = row.get(5)?;
                let is_active_raw: i32 = row.get(6)?;
                let added_raw: String = row.get(7)?;
                let last_used_raw: Option<String> = row.get(8)?;

                Ok((
                    id_raw,
                    kind_raw,
                    username,
                    uuid_raw,
                    skin_url,
                    entitlement_raw,
                    is_active_raw,
                    added_raw,
                    last_used_raw,
                ))
            })
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut accounts = Vec::new();
        for r in rows {
            let (
                id_raw,
                kind_raw,
                username,
                uuid_raw,
                skin_url,
                entitlement_raw,
                is_active_raw,
                added_raw,
                last_used_raw,
            ) = r.map_err(|e| DomainError::Internal(e.to_string()))?;

            let id_uuid = Uuid::parse_str(&id_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let mc_uuid = Uuid::parse_str(&uuid_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let kind: AccountKind = serde_json::from_str(&format!("\"{}\"", kind_raw))
                .map_err(|e| DomainError::Internal(format!("Corrupt account kind: {e}")))?;
            let entitlement: EntitlementStatus = serde_json::from_str(&format!("\"{}\"", entitlement_raw))
                .map_err(|e| DomainError::Internal(format!("Corrupt entitlement: {e}")))?;
            let added_at = added_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let last_used_at = last_used_raw
                .map(|s| s.parse::<DateTime<Utc>>())
                .transpose()
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            accounts.push(AccountIdentity {
                id: AccountId(id_uuid),
                kind,
                minecraft_username: username,
                minecraft_uuid: mc_uuid,
                skin_url,
                entitlement,
                is_active: is_active_raw == 1,
                added_at,
                last_used_at,
            });
        }
        Ok(accounts)
    }

    async fn save_account(&self, account: &AccountIdentity) -> Result<(), DomainError> {
        let conn = self.conn.lock().await;
        let id_str = account.id.0.to_string();
        let uuid_str = account.minecraft_uuid.to_string();
        let kind_str = match account.kind {
            AccountKind::Microsoft => "microsoft",
            AccountKind::Offline => "offline",
        };
        let entitlement_str = match account.entitlement {
            EntitlementStatus::Entitled => "entitled",
            EntitlementStatus::NotEntitled => "not_entitled",
            EntitlementStatus::Offline => "offline",
        };
        let added_str = account.added_at.to_rfc3339();
        let last_used_str = account.last_used_at.map(|d| d.to_rfc3339());

        conn.execute(
            r#"
            INSERT INTO accounts (
                id, kind, username, uuid, skin_url, entitlement, is_active, added_at, last_used_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                username = excluded.username,
                skin_url = excluded.skin_url,
                entitlement = excluded.entitlement,
                is_active = excluded.is_active,
                last_used_at = excluded.last_used_at
            "#,
            params![
                id_str,
                kind_str,
                account.minecraft_username,
                uuid_str,
                account.skin_url,
                entitlement_str,
                if account.is_active { 1 } else { 0 },
                added_str,
                last_used_str
            ],
        )
        .map_err(|e| DomainError::Internal(format!("Failed to save account: {e}")))?;

        Ok(())
    }

    async fn set_active_account(&self, id: &AccountId) -> Result<(), DomainError> {
        let conn = self.conn.lock().await;
        let id_str = id.0.to_string();
        conn.execute("UPDATE accounts SET is_active = 0", [])
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        conn.execute("UPDATE accounts SET is_active = 1 WHERE id = ?1", params![id_str])
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn delete_account(&self, id: &AccountId) -> Result<(), DomainError> {
        let conn = self.conn.lock().await;
        let id_str = id.0.to_string();
        conn.execute("DELETE FROM accounts WHERE id = ?1", params![id_str])
            .map_err(|e| DomainError::Internal(format!("Failed to delete account: {e}")))?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Servers
    // -------------------------------------------------------------------------

    async fn get_server(&self, id: &ServerId) -> Result<Option<ServerProfile>, DomainError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare(
                "SELECT id, name, game_version, engine_json, eula_json, settings_json, state_json, created_at, updated_at \
                 FROM servers WHERE id = ?1",
            )
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let id_str = id.0.to_string();
        let mut rows = stmt
            .query_map(params![id_str], |row| {
                let id_raw: String = row.get(0)?;
                let name: String = row.get(1)?;
                let game_version: String = row.get(2)?;
                let engine_json: String = row.get(3)?;
                let eula_json: String = row.get(4)?;
                let settings_json: String = row.get(5)?;
                let state_json: String = row.get(6)?;
                let created_raw: String = row.get(7)?;
                let updated_raw: String = row.get(8)?;

                Ok((
                    id_raw,
                    name,
                    game_version,
                    engine_json,
                    eula_json,
                    settings_json,
                    state_json,
                    created_raw,
                    updated_raw,
                ))
            })
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some(row_res) = rows.next() {
            let (
                id_raw,
                name,
                game_version,
                engine_json,
                eula_json,
                settings_json,
                state_json,
                created_raw,
                updated_raw,
            ) = row_res.map_err(|e| DomainError::Internal(e.to_string()))?;

            let id_uuid = Uuid::parse_str(&id_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let engine: ServerEngine = serde_json::from_str(&engine_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt server engine JSON: {e}")))?;
            let eula = serde_json::from_str(&eula_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt EULA JSON: {e}")))?;
            let settings: ServerSettings = serde_json::from_str(&settings_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt server settings JSON: {e}")))?;
            let state: ServerRunState = serde_json::from_str(&state_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt server state JSON: {e}")))?;
            let created_at = created_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let updated_at = updated_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            Ok(Some(ServerProfile {
                id: ServerId(id_uuid),
                name,
                game_version,
                engine,
                eula,
                settings,
                state,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn list_servers(&self) -> Result<Vec<ServerProfile>, DomainError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare(
                "SELECT id, name, game_version, engine_json, eula_json, settings_json, state_json, created_at, updated_at \
                 FROM servers ORDER BY updated_at DESC",
            )
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let id_raw: String = row.get(0)?;
                let name: String = row.get(1)?;
                let game_version: String = row.get(2)?;
                let engine_json: String = row.get(3)?;
                let eula_json: String = row.get(4)?;
                let settings_json: String = row.get(5)?;
                let state_json: String = row.get(6)?;
                let created_raw: String = row.get(7)?;
                let updated_raw: String = row.get(8)?;

                Ok((
                    id_raw,
                    name,
                    game_version,
                    engine_json,
                    eula_json,
                    settings_json,
                    state_json,
                    created_raw,
                    updated_raw,
                ))
            })
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut servers = Vec::new();
        for r in rows {
            let (
                id_raw,
                name,
                game_version,
                engine_json,
                eula_json,
                settings_json,
                state_json,
                created_raw,
                updated_raw,
            ) = r.map_err(|e| DomainError::Internal(e.to_string()))?;

            let id_uuid = Uuid::parse_str(&id_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let engine: ServerEngine = serde_json::from_str(&engine_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt server engine JSON: {e}")))?;
            let eula = serde_json::from_str(&eula_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt EULA JSON: {e}")))?;
            let settings: ServerSettings = serde_json::from_str(&settings_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt server settings JSON: {e}")))?;
            let state: ServerRunState = serde_json::from_str(&state_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt server state JSON: {e}")))?;
            let created_at = created_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let updated_at = updated_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            servers.push(ServerProfile {
                id: ServerId(id_uuid),
                name,
                game_version,
                engine,
                eula,
                settings,
                state,
                created_at,
                updated_at,
            });
        }
        Ok(servers)
    }

    async fn save_server(&self, server: &ServerProfile) -> Result<(), DomainError> {
        let conn = self.conn.lock().await;
        let id_str = server.id.0.to_string();
        let engine_json = serde_json::to_string(&server.engine)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let eula_json = serde_json::to_string(&server.eula)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let settings_json = serde_json::to_string(&server.settings)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let state_json = serde_json::to_string(&server.state)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let created_str = server.created_at.to_rfc3339();
        let updated_str = server.updated_at.to_rfc3339();

        conn.execute(
            r#"
            INSERT INTO servers (
                id, name, game_version, engine_json, eula_json, settings_json, state_json, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                game_version = excluded.game_version,
                engine_json = excluded.engine_json,
                eula_json = excluded.eula_json,
                settings_json = excluded.settings_json,
                state_json = excluded.state_json,
                updated_at = excluded.updated_at
            "#,
            params![
                id_str,
                server.name,
                server.game_version,
                engine_json,
                eula_json,
                settings_json,
                state_json,
                created_str,
                updated_str
            ],
        )
        .map_err(|e| DomainError::Internal(format!("Failed to save server: {e}")))?;

        Ok(())
    }

    async fn delete_server(&self, id: &ServerId) -> Result<(), DomainError> {
        let conn = self.conn.lock().await;
        let id_str = id.0.to_string();
        conn.execute("DELETE FROM servers WHERE id = ?1", params![id_str])
            .map_err(|e| DomainError::Internal(format!("Failed to delete server: {e}")))?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Tunnels
    // -------------------------------------------------------------------------

    async fn get_tunnel(&self, id: &TunnelId) -> Result<Option<TunnelRun>, DomainError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT id, provider, status_json, created_at, updated_at FROM tunnels WHERE id = ?1")
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let id_str = id.0.to_string();
        let mut rows = stmt
            .query_map(params![id_str], |row| {
                let id_raw: String = row.get(0)?;
                let provider_raw: String = row.get(1)?;
                let status_json: String = row.get(2)?;
                let created_raw: String = row.get(3)?;
                let updated_raw: String = row.get(4)?;

                Ok((id_raw, provider_raw, status_json, created_raw, updated_raw))
            })
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some(row_res) = rows.next() {
            let (id_raw, provider_raw, status_json, created_raw, updated_raw) =
                row_res.map_err(|e| DomainError::Internal(e.to_string()))?;

            let id_uuid = Uuid::parse_str(&id_raw).map_err(|e| DomainError::Internal(e.to_string()))?;
            let provider = serde_json::from_str(&format!("\"{}\"", provider_raw))
                .map_err(|e| DomainError::Internal(format!("Corrupt provider: {e}")))?;
            let status: TunnelStatus = serde_json::from_str(&status_json)
                .map_err(|e| DomainError::Internal(format!("Corrupt tunnel status: {e}")))?;
            let created_at = created_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            let updated_at = updated_raw
                .parse::<DateTime<Utc>>()
                .map_err(|e| DomainError::Internal(e.to_string()))?;

            Ok(Some(TunnelRun {
                id: TunnelId(id_uuid),
                provider,
                status,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn save_tunnel(&self, tunnel: &TunnelRun) -> Result<(), DomainError> {
        let conn = self.conn.lock().await;
        let id_str = tunnel.id.0.to_string();
        let provider_str = match tunnel.provider {
            freeplay_domain::ProviderKind::Manual => "manual",
            freeplay_domain::ProviderKind::Playit => "playit",
            freeplay_domain::ProviderKind::Fake => "fake",
        };
        let status_json = serde_json::to_string(&tunnel.status)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let created_str = tunnel.created_at.to_rfc3339();
        let updated_str = tunnel.updated_at.to_rfc3339();

        conn.execute(
            r#"
            INSERT INTO tunnels (id, provider, status_json, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                status_json = excluded.status_json,
                updated_at = excluded.updated_at
            "#,
            params![id_str, provider_str, status_json, created_str, updated_str],
        )
        .map_err(|e| DomainError::Internal(format!("Failed to save tunnel: {e}")))?;

        Ok(())
    }
}
