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
    set_default_user_by_identifier(&user.to_string()).await
}

#[tracing::instrument]
pub async fn set_default_user_by_identifier(identifier: &str) -> crate::Result<()> {
    use sqlx::Row;

    let clean = identifier.trim();
    if clean.is_empty() {
        return Ok(());
    }

    let parsed_uuid = uuid::Uuid::parse_str(clean).ok();
    let offline_uuid = freeplay_domain::generate_offline_uuid(clean);

    let state = State::get().await?;
    let mut tx = state.pool.begin().await?;

    let mut matched_uuid: Option<String> = None;

    if let Some(u) = parsed_uuid {
        let u_hyphenated = u.as_hyphenated().to_string();
        let u_simple = u.simple().to_string();
        let row = sqlx::query("SELECT uuid FROM minecraft_users WHERE uuid = ? OR uuid = ?")
            .bind(&u_hyphenated)
            .bind(&u_simple)
            .fetch_optional(&mut *tx)
            .await?;
        if let Some(r) = row {
            matched_uuid = Some(r.get("uuid"));
        }
    }

    if matched_uuid.is_none() {
        let off_hyphenated = offline_uuid.as_hyphenated().to_string();
        let off_simple = offline_uuid.simple().to_string();
        let row = sqlx::query("SELECT uuid FROM minecraft_users WHERE uuid = ? OR uuid = ? OR LOWER(username) = LOWER(?)")
            .bind(&off_hyphenated)
            .bind(&off_simple)
            .bind(clean)
            .fetch_optional(&mut *tx)
            .await?;
        if let Some(r) = row {
            matched_uuid = Some(r.get("uuid"));
        }
    }

    if let Some(db_uuid) = matched_uuid {
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

    tx.commit().await?;
    crate::state::create_offline_account(clean.to_string(), &state.pool).await?;
    Ok(())
}

/// Remove a user account from the database
#[tracing::instrument]
pub async fn remove_user(uuid: uuid::Uuid) -> crate::Result<()> {
    remove_user_by_identifier(&uuid.to_string()).await
}

#[tracing::instrument]
pub async fn remove_user_by_identifier(identifier: &str) -> crate::Result<()> {
    let clean = identifier.trim();
    if clean.is_empty() {
        return Ok(());
    }

    let parsed_uuid = uuid::Uuid::parse_str(clean).ok();
    let offline_uuid = freeplay_domain::generate_offline_uuid(clean);
    let state = State::get().await?;

    let mut query = "DELETE FROM minecraft_users WHERE LOWER(username) = LOWER(?)".to_string();
    let mut args: Vec<String> = vec![clean.to_string()];

    if let Some(u) = parsed_uuid {
        query.push_str(" OR uuid = ? OR uuid = ?");
        args.push(u.as_hyphenated().to_string());
        args.push(u.simple().to_string());
    }
    query.push_str(" OR uuid = ? OR uuid = ?");
    args.push(offline_uuid.as_hyphenated().to_string());
    args.push(offline_uuid.simple().to_string());

    let mut q = sqlx::query(&query);
    for arg in args {
        q = q.bind(arg);
    }
    q.execute(&state.pool).await?;

    let all = Credentials::get_all(&state.pool).await?;
    let has_active = all.iter().any(|u| u.value().active);
    if !has_active {
        let target = all
            .iter()
            .find(|u| !u.value().offline_profile.name.eq_ignore_ascii_case("player"))
            .map(|u| *u.key())
            .or_else(|| all.iter().next().map(|u| *u.key()));

        if let Some(key) = target {
            if let Some((_, mut user)) = all.remove(&key) {
                user.active = true;
                let _ = user.upsert(&state.pool).await;
            }
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
