//! Authentication flow interface

use reqwest::StatusCode;

use crate::State;
use crate::state::{Credentials, MinecraftLoginFlow};
use crate::util::fetch::INSECURE_REQWEST_CLIENT;

#[tracing::instrument]
pub async fn check_reachable() -> crate::Result<()> {
    let resp = INSECURE_REQWEST_CLIENT
        .get("https://sessionserver.mojang.com/session/minecraft/hasJoined")
        .send()
        .await?;
    if resp.status() == StatusCode::NO_CONTENT {
        return Ok(());
    }
    resp.error_for_status()?;
    Ok(())
}

#[tracing::instrument]
pub async fn begin_login() -> crate::Result<MinecraftLoginFlow> {
    let state = State::get().await?;

    crate::state::login_begin(&state.pool).await
}

#[tracing::instrument]
pub async fn finish_login(
    code: &str,
    flow: MinecraftLoginFlow,
) -> crate::Result<Credentials> {
    let state = State::get().await?;

    let credentials =
        crate::state::login_finish(code, flow, &state.pool).await?;

    if let Err(error) =
        crate::onboarding_checklist::mark_logged_into_minecraft().await
    {
        tracing::warn!(
            "Failed to mark Minecraft login in onboarding checklist: {error}"
        );
    }

    Ok(credentials)
}

#[tracing::instrument]
pub async fn create_offline_account(username: String) -> crate::Result<Credentials> {
    let state = State::get().await?;
    let credentials = crate::state::create_offline_account(username, &state.pool).await?;

    if let Err(error) = crate::onboarding_checklist::mark_logged_into_minecraft().await {
        tracing::warn!("Failed to mark Minecraft login in onboarding checklist: {error}");
    }

    Ok(credentials)
}

#[tracing::instrument]
pub async fn get_default_user() -> crate::Result<Option<uuid::Uuid>> {
    let state = State::get().await?;
    let user = Credentials::get_default_credential(&state.pool).await?;
    Ok(user.map(|user| user.offline_profile.id))
}

#[tracing::instrument]
pub async fn set_default_user(user: uuid::Uuid) -> crate::Result<()> {
    use sqlx::Row;

    let state = State::get().await?;
    let uuid_hyphenated = user.as_hyphenated().to_string();
    let uuid_simple = user.simple().to_string();

    let mut tx = state.pool.begin().await?;

    // 1. Direct check by hyphenated or simple UUID
    let row = sqlx::query(
        "SELECT uuid FROM minecraft_users WHERE uuid = ? OR uuid = ?",
    )
    .bind(&uuid_hyphenated)
    .bind(&uuid_simple)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(r) = row {
        let db_uuid: String = r.get("uuid");
        sqlx::query("UPDATE minecraft_users SET active = FALSE")
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE minecraft_users SET active = TRUE WHERE uuid = ?")
            .bind(&db_uuid)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        return Ok(());
    }

    // 2. Fallback: check if any stored user has a matching calculated offline UUID
    let rows = sqlx::query("SELECT uuid, username FROM minecraft_users")
        .fetch_all(&mut *tx)
        .await?;

    for r in rows {
        let db_username: String = r.get("username");
        let db_uuid: String = r.get("uuid");
        let expected = freeplay_domain::generate_offline_uuid(&db_username);
        if expected == user {
            sqlx::query("UPDATE minecraft_users SET active = FALSE")
                .execute(&mut *tx)
                .await?;
            sqlx::query(
                "UPDATE minecraft_users SET active = TRUE, uuid = ? WHERE uuid = ?",
            )
            .bind(&uuid_hyphenated)
            .bind(&db_uuid)
            .execute(&mut *tx)
            .await?;
            tx.commit().await?;
            return Ok(());
        }
    }

    // 3. Fallback: if username matches directly or user was just created
    tx.commit().await?;
    Ok(())
}

/// Remove a user account from the database
#[tracing::instrument]
pub async fn remove_user(uuid: uuid::Uuid) -> crate::Result<()> {
    let state = State::get().await?;
    let uuid_hyphenated = uuid.as_hyphenated().to_string();
    let uuid_simple = uuid.simple().to_string();

    sqlx::query("DELETE FROM minecraft_users WHERE uuid = ? OR uuid = ?")
        .bind(&uuid_hyphenated)
        .bind(&uuid_simple)
        .execute(&state.pool)
        .await?;

    let all = Credentials::get_all(&state.pool).await?;
    let has_active = all.iter().any(|u| u.value().active);
    if !has_active {
        if let Some((_, mut first)) = all.into_iter().next() {
            first.active = true;
            let _ = first.upsert(&state.pool).await;
        }
    }

    Ok(())
}

/// Get a copy of the list of all user credentials
#[tracing::instrument]
pub async fn users() -> crate::Result<Vec<Credentials>> {
    let state = State::get().await?;
    let users = Credentials::get_all(&state.pool).await?;
    Ok(users.into_iter().map(|x| x.1).collect())
}
