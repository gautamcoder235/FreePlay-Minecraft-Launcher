use crate::api::Result;
use chrono::{Duration, Utc};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime, UserAttentionType};
use theseus::prelude::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("auth")
        .invoke_handler(tauri::generate_handler![
            check_reachable,
            login,
            create_offline_account,
            remove_user,
            get_default_user,
            set_default_user,
            get_users,
        ])
        .build()
}

/// Creates an offline account with a custom nickname and standard offline UUID (SKlauncher/TLauncher style)
#[tauri::command]
pub async fn create_offline_account(username: String) -> Result<Credentials> {
    Ok(minecraft_auth::create_offline_account(username).await?)
}

/// Checks if the authentication servers are reachable.
#[tauri::command]
pub async fn check_reachable() -> Result<()> {
    minecraft_auth::check_reachable().await?;
    Ok(())
}

/// Authenticate a user with Hydra - part 1
/// This begins the authentication flow quasi-synchronously, returning a URL to visit (that the user will sign in at)
#[tauri::command]
pub async fn login<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<Option<Credentials>> {
    let flow = minecraft_auth::begin_login().await?;

    let start = Utc::now();

    if let Some(window) = app.get_webview_window("signin") {
        window.close()?;
    }

    let window = tauri::WebviewWindowBuilder::new(
        &app,
        "signin",
        tauri::WebviewUrl::External(flow.auth_request_uri.parse().map_err(
            |_| {
                theseus::ErrorKind::OtherError(
                    "Error parsing auth redirect URL".to_string(),
                )
                .as_error()
            },
        )?),
    )
    .title("Sign into FreePlay")
    .always_on_top(true)
    .min_inner_size(500.0, 500.0)
    .inner_size(1000.0, 700.0)
    .focused(true)
    .center()
    .build()?;

    window.request_user_attention(Some(UserAttentionType::Critical))?;

    while (Utc::now() - start) < Duration::minutes(10) {
        if window.title().is_err() {
            // user closed window, cancelling flow
            return Ok(None);
        }

        if window
            .url()?
            .as_str()
            .starts_with("https://login.live.com/oauth20_desktop.srf")
            && let Some((_, code)) =
                window.url()?.query_pairs().find(|x| x.0 == "code")
        {
            window.close()?;
            let val = minecraft_auth::finish_login(&code.clone(), flow).await?;

            return Ok(Some(val));
        }

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    window.close()?;
    Ok(None)
}

#[tauri::command]
pub async fn remove_user(user: serde_json::Value) -> Result<()> {
    let user_str = match user {
        serde_json::Value::String(s) => s,
        serde_json::Value::Object(o) => {
            o.get("id")
                .or_else(|| o.get("uuid"))
                .or_else(|| o.get("username"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        }
        _ => user.to_string().trim_matches('"').to_string(),
    };
    Ok(minecraft_auth::remove_user_by_identifier(&user_str).await?)
}

#[tauri::command]
pub async fn get_default_user() -> Result<Option<uuid::Uuid>> {
    Ok(minecraft_auth::get_default_user().await?)
}

#[tauri::command]
pub async fn set_default_user(user: serde_json::Value) -> Result<()> {
    let user_str = match user {
        serde_json::Value::String(s) => s,
        serde_json::Value::Object(o) => {
            o.get("id")
                .or_else(|| o.get("uuid"))
                .or_else(|| o.get("username"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        }
        _ => user.to_string().trim_matches('"').to_string(),
    };
    Ok(minecraft_auth::set_default_user_by_identifier(&user_str).await?)
}

/// Get a copy of the list of all user credentials
#[tauri::command]
pub async fn get_users() -> Result<Vec<Credentials>> {
    Ok(minecraft_auth::users().await?)
}
