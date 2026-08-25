use tauri::{
	menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
	tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
	AppHandle, Emitter, Manager,
};

pub fn build_initial_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
	let show_item = MenuItem::with_id(app, "tray_show", "Show FreePlay Launcher", true, None::<&str>)?;
	let quit_item = MenuItem::with_id(app, "tray_quit", "Exit FreePlay", true, None::<&str>)?;

	Menu::with_items(
		app,
		&[
			&show_item,
			&PredefinedMenuItem::separator(app)?,
			&quit_item,
		],
	)
}

pub async fn build_tray_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
	let show_item = MenuItem::with_id(app, "tray_show", "Show FreePlay Launcher", true, None::<&str>)?;
	let quit_item = MenuItem::with_id(app, "tray_quit", "Exit FreePlay", true, None::<&str>)?;

	if !theseus::State::initialized() {
		return build_initial_menu(app);
	}

	// Instances submenu (with individual Start / Stop status for each created instance)
	let instances_submenu = Submenu::new(app, "Instances", true)?;
	let instances = theseus::instance::list().await.unwrap_or_default();
	let running_processes = theseus::process::get_all().await.unwrap_or_default();

	if instances.is_empty() {
		let empty_item = MenuItem::with_id(
			app,
			"no_instances",
			"No instances created",
			false,
			None::<&str>,
		)?;
		instances_submenu.append(&empty_item)?;
	} else {
		for instance in &instances {
			let running_proc = running_processes.iter().find(|p| {
				p.instance_id == instance.instance.id
					|| (!instance.instance.path.is_empty() && p.instance_path == instance.instance.path)
			});

			if let Some(proc) = running_proc {
				let item_id = format!("stop_instance:{}", proc.uuid);
				let title = format!("⏹  Stop {} (Running)", instance.instance.name);
				let item = MenuItem::with_id(app, &item_id, &title, true, None::<&str>)?;
				instances_submenu.append(&item)?;
			} else {
				let item_id = format!("run_instance:{}", instance.instance.id);
				let title = format!("▶  Start {}", instance.instance.name);
				let item = MenuItem::with_id(app, &item_id, &title, true, None::<&str>)?;
				instances_submenu.append(&item)?;
			}
		}
	}

	// Servers submenu (with individual Start / Stop status for each created server)
	let servers_submenu = Submenu::new(app, "Servers", true)?;
	let servers = theseus::server_address::host_list_servers().await.unwrap_or_default();
	let server_status = theseus::server_address::host_get_status().await.ok();
	let is_server_running = server_status
		.as_ref()
		.map(|s| s.server_running)
		.unwrap_or(false);

	let running_server_dir = if let Ok(state) = theseus::State::get().await {
		state.server_hosting.get_working_dir().await
	} else {
		None
	};

	if servers.is_empty() {
		let empty_item = MenuItem::with_id(
			app,
			"no_servers",
			"No servers created",
			false,
			None::<&str>,
		)?;
		servers_submenu.append(&empty_item)?;
	} else {
		for server in &servers {
			let is_this_server_running = is_server_running
				&& running_server_dir
					.as_ref()
					.map(|d| {
						let d_str = d.to_string_lossy().to_lowercase();
						let s_id_lower = server.id.to_lowercase();
						let s_path_lower = server.path.to_lowercase();
						d_str == s_path_lower
							|| d_str.ends_with(&s_id_lower)
							|| d_str.contains(&s_id_lower)
					})
					.unwrap_or(false);

			if is_this_server_running {
				let item_id = format!("stop_server:{}", server.id);
				let title = format!("⏹  Stop {} (Running)", server.name);
				let item = MenuItem::with_id(app, &item_id, &title, true, None::<&str>)?;
				servers_submenu.append(&item)?;
			} else {
				let item_id = format!("start_server:{}", server.id);
				let title = format!("⚡  Start {}", server.name);
				let item = MenuItem::with_id(app, &item_id, &title, true, None::<&str>)?;
				servers_submenu.append(&item)?;
			}
		}
	}

	let menu = Menu::with_items(
		app,
		&[
			&show_item,
			&PredefinedMenuItem::separator(app)?,
			&instances_submenu,
			&servers_submenu,
			&PredefinedMenuItem::separator(app)?,
			&quit_item,
		],
	)?;

	Ok(menu)
}

pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
	let initial_menu = build_initial_menu(app)?;

	let tray_builder = TrayIconBuilder::with_id("freeplay_tray")
		.tooltip("FreePlay Launcher")
		.menu(&initial_menu)
		.show_menu_on_left_click(false);

	let tray_builder = if let Some(icon) = app.default_window_icon() {
		tray_builder.icon(icon.clone())
	} else {
		tray_builder
	};

	let tray = tray_builder
		.on_tray_icon_event(move |tray, event| {
			if let TrayIconEvent::Click {
				button: MouseButton::Left,
				button_state: MouseButtonState::Up,
				..
			} = event
			{
				let app = tray.app_handle();
				if let Some(win) = app.get_webview_window("main") {
					let _ = win.unminimize();
					let _ = win.show();
					let _ = win.set_focus();
				}
			}
		})
		.on_menu_event(move |app, event| {
			let id_str = event.id.as_ref();
			if id_str == "tray_show" {
				if let Some(win) = app.get_webview_window("main") {
					let _ = win.unminimize();
					let _ = win.show();
					let _ = win.set_focus();
				}
			} else if id_str == "tray_quit" {
				if let Some(win) = app.get_webview_window("main") {
					let _ = win.unminimize();
					let _ = win.show();
					let _ = win.set_focus();
					let _ = win.emit("request-quit-app", true);
				}
				let _ = app.emit("request-quit-app", true);
			} else if let Some(uuid_str) = id_str.strip_prefix("stop_instance:") {
				if let Ok(uuid) = uuid::Uuid::parse_str(uuid_str) {
					let app_clone = app.clone();
					tauri::async_runtime::spawn(async move {
						tracing::info!("Stopping instance from tray: {uuid}");
						let _ = theseus::process::kill(uuid).await;
						let _ = refresh_tray_menu(app_clone).await;
					});
				}
			} else if let Some(instance_id) = id_str.strip_prefix("run_instance:") {
				let instance_id = instance_id.to_string();
				let app_clone = app.clone();
				tauri::async_runtime::spawn(async move {
					tracing::info!("Starting instance from tray: {instance_id}");
					let _ = theseus::instance::run(
						&instance_id,
						theseus::instance::QuickPlayType::None,
					)
					.await;
					let _ = refresh_tray_menu(app_clone).await;
				});
			} else if let Some(_server_id) = id_str.strip_prefix("stop_server:") {
				let app_clone = app.clone();
				tauri::async_runtime::spawn(async move {
					tracing::info!("Stopping server from tray");
					let _ = theseus::server_address::host_stop_server().await;
					let _ = app_clone.emit("server-stopped", ());
					tokio::time::sleep(std::time::Duration::from_millis(500)).await;
					let _ = refresh_tray_menu(app_clone).await;
				});
			} else if let Some(server_id) = id_str.strip_prefix("start_server:") {
				let server_id = server_id.to_string();
				let app_clone = app.clone();
				tauri::async_runtime::spawn(async move {
					tracing::info!("Starting server from tray: {server_id}");
					if let Ok(status) = theseus::server_address::host_get_status().await {
						if status.server_running {
							tracing::info!("Stopping current server before starting {server_id}");
							let _ = theseus::server_address::host_stop_server().await;
							tokio::time::sleep(std::time::Duration::from_millis(600)).await;
						}
					}
					let servers = theseus::server_address::host_list_servers().await.unwrap_or_default();
					if let Some(s) = servers.into_iter().find(|x| x.id == server_id) {
						let _ = theseus::server_address::host_select_server(s.id.clone()).await;
						let _ = app_clone.emit("server-switched", s.id.clone());
						let _ = theseus::server_address::host_start_server(
							s.version,
							s.engine,
							s.ram_gb * 1024,
							s.port,
						)
						.await;
						tokio::time::sleep(std::time::Duration::from_millis(600)).await;
						let _ = refresh_tray_menu(app_clone).await;
					}
				});
			}
		})
		.build(app)?;

	let app_for_async = app.clone();
	let tray_id = tray.id().clone();
	tauri::async_runtime::spawn(async move {
		if let Ok(menu) = build_tray_menu(&app_for_async).await {
			if let Some(tray) = app_for_async.tray_by_id(&tray_id) {
				let _ = tray.set_menu(Some(menu));
			}
		}
	});

	Ok(())
}

#[tauri::command]
pub async fn refresh_tray_menu(app: AppHandle) -> Result<(), String> {
	if let Ok(menu) = build_tray_menu(&app).await {
		if let Some(tray) = app.tray_by_id("freeplay_tray") {
			let _ = tray.set_menu(Some(menu));
		}
	}
	Ok(())
}
