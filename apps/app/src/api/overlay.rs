use crate::api::Result;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Emitter, Manager};
use tokio::sync::RwLock;

#[cfg(windows)]
use windows::core::BOOL;
#[cfg(windows)]
use windows::Win32::Foundation::{HWND, LPARAM, RECT};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
	EnumWindows, GetClientRect, GetWindowLongPtrW, GetWindowRect, GetWindowThreadProcessId,
	IsWindowVisible, SetForegroundWindow, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE,
	HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
	WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OverlayStateDto {
	pub is_open: bool,
	pub active_game_pid: Option<u32>,
	pub active_instance_id: Option<String>,
	pub active_instance_name: Option<String>,
	pub hotkey: String,
}

struct GlobalOverlayState {
	is_open: AtomicBool,
	active_game_pid: Arc<RwLock<Option<u32>>>,
	active_instance_id: Arc<RwLock<Option<String>>>,
	active_instance_name: Arc<RwLock<Option<String>>>,
	hotkey: Arc<RwLock<String>>,
}

impl Default for GlobalOverlayState {
	fn default() -> Self {
		Self {
			is_open: AtomicBool::new(false),
			active_game_pid: Arc::new(RwLock::new(None)),
			active_instance_id: Arc::new(RwLock::new(None)),
			active_instance_name: Arc::new(RwLock::new(None)),
			hotkey: Arc::new(RwLock::new("Shift+Tab".to_string())),
		}
	}
}

static OVERLAY_STATE: std::sync::OnceLock<GlobalOverlayState> = std::sync::OnceLock::new();

fn state() -> &'static GlobalOverlayState {
	OVERLAY_STATE.get_or_init(GlobalOverlayState::default)
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
	tauri::plugin::Builder::new("overlay")
		.invoke_handler(tauri::generate_handler![
			overlay_toggle,
			overlay_set_visible,
			overlay_get_state,
			overlay_set_active_game,
			overlay_sync_geometry,
			overlay_focus_game,
		])
		.build()
}

#[cfg(windows)]
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
	unsafe {
		let payload = lparam.0 as *mut (u32, Option<isize>);
		let target_pid = (*payload).0;
		let mut process_id = 0u32;
		GetWindowThreadProcessId(hwnd, Some(&mut process_id));

		if process_id == target_pid && IsWindowVisible(hwnd).as_bool() {
			let mut rect = RECT::default();
			if GetClientRect(hwnd, &mut rect).is_ok()
				&& (rect.right - rect.left) > 100
				&& (rect.bottom - rect.top) > 100
			{
				(*payload).1 = Some(hwnd.0 as isize);
				return BOOL(0);
			}
		}
		BOOL(1)
	}
}

#[cfg(windows)]
fn find_game_hwnd(pid: u32) -> Option<HWND> {
	let mut payload = (pid, None::<isize>);
	unsafe {
		let _ = EnumWindows(
			Some(enum_windows_callback),
			LPARAM(&mut payload as *mut _ as isize),
		);
	}
	payload.1.map(|raw| HWND(raw as _))
}

#[cfg(windows)]
fn apply_click_through(hwnd_raw: isize, enable: bool) {
	unsafe {
		let hwnd = HWND(hwnd_raw as _);
		let current_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
		let new_style = if enable {
			current_style
				| WS_EX_LAYERED.0
				| WS_EX_TRANSPARENT.0
				| WS_EX_TOPMOST.0
				| WS_EX_TOOLWINDOW.0
				| WS_EX_NOACTIVATE.0
		} else {
			(current_style & !(WS_EX_TRANSPARENT.0 | WS_EX_NOACTIVATE.0))
				| WS_EX_LAYERED.0
				| WS_EX_TOPMOST.0
				| WS_EX_TOOLWINDOW.0
		};

		let _ = SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style as isize);
		let _ = SetWindowPos(
			hwnd,
			Some(HWND_TOPMOST),
			0,
			0,
			0,
			0,
			SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED,
		);
	}
}

#[cfg(windows)]
fn sync_bounds_to_game(overlay_raw: isize, game_pid: u32) {
	if let Some(game_hwnd) = find_game_hwnd(game_pid) {
		unsafe {
			let overlay_hwnd = HWND(overlay_raw as _);
			let mut rect = RECT::default();
			if GetWindowRect(game_hwnd, &mut rect).is_ok() {
				let width = rect.right - rect.left;
				let height = rect.bottom - rect.top;
				if width > 50 && height > 50 {
					let _ = SetWindowPos(
						overlay_hwnd,
						Some(HWND_TOPMOST),
						rect.left,
						rect.top,
						width,
						height,
						SWP_NOACTIVATE | SWP_SHOWWINDOW,
					);
				}
			}
		}
	}
}

#[tauri::command]
pub async fn overlay_toggle<R: tauri::Runtime>(
	app: tauri::AppHandle<R>,
	force_state: Option<bool>,
) -> Result<bool> {
	let current = state().is_open.load(Ordering::SeqCst);
	let next = force_state.unwrap_or(!current);
	state().is_open.store(next, Ordering::SeqCst);
	let maybe_pid = *state().active_game_pid.read().await;

	if let Some(overlay_win) = app.get_webview_window("overlay") {
		#[cfg(windows)]
		let hwnd_raw = overlay_win.hwnd().ok().map(|h| h.0 as isize);

		if next {
			#[cfg(windows)]
			if let Some(raw) = hwnd_raw {
				apply_click_through(raw, false);
				if let Some(pid) = maybe_pid {
					sync_bounds_to_game(raw, pid);
				}
			}

			let _ = overlay_win.show();
			let _ = overlay_win.set_focus();
			let _ = overlay_win.emit("overlay-state-changed", true);
		} else {
			let _ = overlay_win.emit("overlay-state-changed", false);

			#[cfg(windows)]
			if let Some(raw) = hwnd_raw {
				apply_click_through(raw, true);
				if let Some(pid) = maybe_pid {
					if let Some(game_hwnd) = find_game_hwnd(pid) {
						unsafe {
							let _ = SetForegroundWindow(game_hwnd);
						}
					}
				}
			}

			let _ = overlay_win.hide();
		}
	}

	Ok(next)
}

#[tauri::command]
pub async fn overlay_set_visible<R: tauri::Runtime>(
	app: tauri::AppHandle<R>,
	visible: bool,
) -> Result<()> {
	overlay_toggle(app, Some(visible)).await?;
	Ok(())
}

#[tauri::command]
pub async fn overlay_get_state() -> Result<OverlayStateDto> {
	Ok(OverlayStateDto {
		is_open: state().is_open.load(Ordering::SeqCst),
		active_game_pid: *state().active_game_pid.read().await,
		active_instance_id: state().active_instance_id.read().await.clone(),
		active_instance_name: state().active_instance_name.read().await.clone(),
		hotkey: state().hotkey.read().await.clone(),
	})
}

#[tauri::command]
pub async fn overlay_set_active_game(
	pid: Option<u32>,
	instance_id: Option<String>,
	instance_name: Option<String>,
) -> Result<()> {
	*state().active_game_pid.write().await = pid;
	*state().active_instance_id.write().await = instance_id;
	*state().active_instance_name.write().await = instance_name;
	Ok(())
}

#[tauri::command]
pub async fn overlay_sync_geometry<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<()> {
	let maybe_pid = *state().active_game_pid.read().await;
	if let Some(overlay_win) = app.get_webview_window("overlay") {
		#[cfg(windows)]
		if let Ok(hwnd) = overlay_win.hwnd() {
			if let Some(pid) = maybe_pid {
				sync_bounds_to_game(hwnd.0 as isize, pid);
			}
		}
	}
	Ok(())
}

#[tauri::command]
pub async fn overlay_focus_game<R: tauri::Runtime>(_app: tauri::AppHandle<R>) -> Result<()> {
	let maybe_pid = *state().active_game_pid.read().await;
	#[cfg(windows)]
	if let Some(pid) = maybe_pid {
		if let Some(game_hwnd) = find_game_hwnd(pid) {
			unsafe {
				let _ = SetForegroundWindow(game_hwnd);
			}
		}
	}
	Ok(())
}
