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
	EnumWindows, GetClientRect, GetForegroundWindow, GetWindowLongPtrW, GetWindowRect,
	GetWindowTextW, GetWindowThreadProcessId, IsIconic, IsWindowVisible, SetForegroundWindow,
	SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_TOPMOST, SWP_FRAMECHANGED,
	SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, WS_EX_LAYERED,
	WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
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
	active_game_pid: std::sync::atomic::AtomicU32,
	active_instance_id: Arc<RwLock<Option<String>>>,
	active_instance_name: Arc<RwLock<Option<String>>>,
	hotkey: Arc<RwLock<String>>,
}

impl Default for GlobalOverlayState {
	fn default() -> Self {
		Self {
			is_open: AtomicBool::new(false),
			active_game_pid: std::sync::atomic::AtomicU32::new(0),
			active_instance_id: Arc::new(RwLock::new(None)),
			active_instance_name: Arc::new(RwLock::new(None)),
			hotkey: Arc::new(RwLock::new("Shift+Tab".to_string())),
		}
	}
}

static OVERLAY_STATE: std::sync::OnceLock<GlobalOverlayState> = std::sync::OnceLock::new();
static IS_OVERLAY_RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

pub fn stop() {
	IS_OVERLAY_RUNNING.store(false, std::sync::atomic::Ordering::SeqCst);
}

fn state() -> &'static GlobalOverlayState {
	OVERLAY_STATE.get_or_init(GlobalOverlayState::default)
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
	tauri::plugin::Builder::new("overlay")
		.setup(|app, _api| {
			#[cfg(windows)]
			{
				let app_handle = app.clone();
				std::thread::spawn(move || {
					use windows::Win32::UI::Input::KeyboardAndMouse::{
						GetAsyncKeyState, VK_F8, VK_LSHIFT, VK_RSHIFT, VK_SHIFT, VK_TAB,
					};

					let mut was_shift_tab_down = false;
					let mut was_f8_down = false;
					let mut last_fg_check = std::time::Instant::now();

					while IS_OVERLAY_RUNNING.load(std::sync::atomic::Ordering::Relaxed) {
						std::thread::sleep(std::time::Duration::from_millis(35));
						if !IS_OVERLAY_RUNNING.load(std::sync::atomic::Ordering::Relaxed) {
							break;
						}

						let is_shift = unsafe {
							((GetAsyncKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000) != 0)
								|| ((GetAsyncKeyState(VK_LSHIFT.0 as i32) as u16 & 0x8000) != 0)
								|| ((GetAsyncKeyState(VK_RSHIFT.0 as i32) as u16 & 0x8000) != 0)
						};
						let is_tab = unsafe { (GetAsyncKeyState(VK_TAB.0 as i32) as u16 & 0x8000) != 0 };
						let is_f8 = unsafe { (GetAsyncKeyState(VK_F8.0 as i32) as u16 & 0x8000) != 0 };

						let shift_tab_pressed = is_shift && is_tab;

						let is_overlay_open = state().is_open.load(std::sync::atomic::Ordering::Relaxed);
						let pid = state().active_game_pid.load(std::sync::atomic::Ordering::Relaxed);
						let is_game_active_foreground = if is_overlay_open {
							true
						} else if pid > 0 {
							let fg = unsafe { GetForegroundWindow() };
							if let Some(gh) = find_game_hwnd(pid) {
								fg == gh && !unsafe { IsIconic(gh).as_bool() }
							} else {
								false
							}
						} else {
							false
						};

						if shift_tab_pressed && !was_shift_tab_down && is_game_active_foreground {
							let app_clone = app_handle.clone();
							tauri::async_runtime::spawn(async move {
								let _ = overlay_toggle(app_clone, None).await;
							});
						}
						was_shift_tab_down = shift_tab_pressed;

						if is_f8 && !was_f8_down && is_game_active_foreground {
							let app_clone = app_handle.clone();
							tauri::async_runtime::spawn(async move {
								let _ = overlay_toggle(app_clone, None).await;
							});
						}
						was_f8_down = is_f8;

						// Monitor window focus: if overlay is active but user switches to another app or minimizes Minecraft, auto-hide overlay
						if last_fg_check.elapsed() >= std::time::Duration::from_millis(80) {
							last_fg_check = std::time::Instant::now();
							if state().is_open.load(std::sync::atomic::Ordering::Relaxed) {
								let pid = state().active_game_pid.load(std::sync::atomic::Ordering::Relaxed);
								if let Some(overlay_win) = app_handle.get_webview_window("overlay") {
									if let Ok(overlay_hwnd_res) = overlay_win.hwnd() {
										let overlay_hwnd = HWND(overlay_hwnd_res.0 as _);
										let fg = unsafe { GetForegroundWindow() };
										let game_hwnd = find_game_hwnd(pid);

										let is_game_or_overlay_focused = fg == overlay_hwnd
											|| (game_hwnd.is_some() && Some(fg) == game_hwnd);
										let is_game_minimized =
											game_hwnd.map_or(false, |gh| unsafe { IsIconic(gh).as_bool() });

										if (!is_game_or_overlay_focused || is_game_minimized) && fg.0 != 0 as _ {
											let app_clone = app_handle.clone();
											tauri::async_runtime::spawn(async move {
												let _ = overlay_toggle(app_clone, Some(false)).await;
											});
										} else if let Some(gh) = game_hwnd {
											let _ = gh;
											sync_bounds_to_game(overlay_hwnd.0 as isize, pid);
										}
									}
								}
							}
						}
					}
				});
			}
			Ok(())
		})
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
	if pid > 0 {
		let mut payload = (pid, None::<isize>);
		unsafe {
			let _ = EnumWindows(
				Some(enum_windows_callback),
				LPARAM(&mut payload as *mut _ as isize),
			);
		}
		if let Some(raw) = payload.1 {
			return Some(HWND(raw as _));
		}
	}

	// Fallback: Check if the active foreground window is Minecraft
	unsafe {
		let fg = GetForegroundWindow();
		if fg.0 != 0 as _ {
			let mut title_buf = [0u16; 512];
			let len = GetWindowTextW(fg, &mut title_buf);
			if len > 0 {
				let title = String::from_utf16_lossy(&title_buf[..len as usize]);
				if title.contains("Minecraft")
					|| title.contains("FreePlay")
					|| title.contains("Fabric")
					|| title.contains("Forge")
					|| title.contains("Paper")
				{
					return Some(fg);
				}
			}
		}
	}

	None
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
	let pid_val = state().active_game_pid.load(Ordering::SeqCst);
	let maybe_pid = if pid_val > 0 { Some(pid_val) } else { None };

	if let Some(overlay_win) = app.get_webview_window("overlay") {
		#[cfg(windows)]
		let hwnd_raw = overlay_win.hwnd().ok().map(|h| h.0 as isize);

		if next {
			#[cfg(windows)]
			if let Some(raw) = hwnd_raw {
				apply_click_through(raw, false);
				if let Some(pid) = maybe_pid {
					sync_bounds_to_game(raw, pid);
				} else {
					let _ = overlay_win.center();
				}
			}

			let _ = overlay_win.unminimize();
			let _ = overlay_win.show();
			let _ = overlay_win.set_focus();
			let _ = overlay_win.emit("overlay-state-changed", true);
		} else {
			let _ = overlay_win.emit("overlay-state-changed", false);

			#[cfg(windows)]
			if let Some(raw) = hwnd_raw {
				apply_click_through(raw, true);

				// Only restore foreground focus to the game if the overlay itself was focused
				// (i.e. intentional in-game dismissal via hotkey, ESC, or dock button).
				// If the user Alt+Tabbed or clicked another application, do NOT steal focus back!
				let fg = unsafe { GetForegroundWindow() };
				let overlay_hwnd = HWND(raw as _);
				let is_explicit_dismissal = fg == overlay_hwnd || fg.0 == 0 as _;

				if is_explicit_dismissal {
					if let Some(pid) = maybe_pid {
						if let Some(game_hwnd) = find_game_hwnd(pid) {
							unsafe {
								let _ = SetForegroundWindow(game_hwnd);
							}
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
	let pid_val = state().active_game_pid.load(Ordering::SeqCst);
	Ok(OverlayStateDto {
		is_open: state().is_open.load(Ordering::SeqCst),
		active_game_pid: if pid_val > 0 { Some(pid_val) } else { None },
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
	state().active_game_pid.store(pid.unwrap_or(0), Ordering::SeqCst);
	*state().active_instance_id.write().await = instance_id;
	*state().active_instance_name.write().await = instance_name;
	Ok(())
}

#[tauri::command]
pub async fn overlay_sync_geometry<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<()> {
	let pid_val = state().active_game_pid.load(Ordering::SeqCst);
	let maybe_pid = if pid_val > 0 { Some(pid_val) } else { None };
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
	let pid_val = state().active_game_pid.load(Ordering::SeqCst);
	let maybe_pid = if pid_val > 0 { Some(pid_val) } else { None };
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
