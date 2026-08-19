// Browser development and Playwright inspection shim for Tauri IPC and Plugins
if (typeof window !== 'undefined') {
	if (!(window as any).__TAURI_OS_PLUGIN_INTERNALS__) {
		;(window as any).__TAURI_OS_PLUGIN_INTERNALS__ = {
			eol: '\r\n',
			platform: 'windows',
			version: '10.0.19045',
			family: 'windows',
			os_type: 'windows',
			arch: 'x86_64',
			exe_extension: 'exe',
		}
	}

	if (!(window as any).__TAURI_INTERNALS__) {
		const defaultSettings: Record<string, any> = {
			native_decorations: false,
			theme: 'dark',
			locale: 'en-US',
			telemetry: false,
			collapsed_navigation: false,
			hide_nametag_skins_page: false,
			advanced_rendering: false,
			toggle_sidebar: false,
			developer_mode: false,
			feature_flags: {},
			pending_update_toast_for_version: null,
			max_memory: 4096,
			min_memory: 2048,
			java_args: '',
		}

		const listeners = new Map<string, Set<Function>>()

		;(window as any).__TAURI_INTERNALS__ = {
			metadata: {
				currentWindow: { label: 'main' },
				currentWebview: { label: 'main' },
			},
			transformCallback: (callback: any, once = false) => {
				const id = Math.floor(Math.random() * 1000000)
				;(window as any)[`_${id}`] = (data: any) => {
					if (once) {
						delete (window as any)[`_${id}`]
					}
					if (typeof callback === 'function') {
						callback(data)
					}
				}
				return id
			},
			invoke: async (cmd: string, args: any = {}) => {
				console.log(`[Tauri IPC Mock] ${cmd}`, args)

				// Array queries
				if (
					cmd.includes('get_users') ||
					cmd.includes('users') ||
					cmd.includes('instance') ||
					cmd.includes('tags') ||
					cmd.includes('process') ||
					cmd.includes('skins') ||
					cmd.includes('capes') ||
					cmd.includes('install_job') ||
					cmd.includes('jobs') ||
					cmd.includes('progress_bars')
				) {
					return []
				}

				if (cmd.includes('get_default_user')) {
					return undefined
				}
				if (cmd.includes('settings') || cmd.includes('get_settings')) {
					return defaultSettings
				}
				if (cmd.includes('onboarding-checklist') || cmd.includes('checklist')) {
					return { logged_in: false, created_instance: false }
				}
				if (cmd.includes('os') || cmd.includes('get_os')) {
					return 'Windows'
				}
				if (cmd.includes('is_dev') || cmd.includes('dev')) {
					return true
				}
				if (cmd.includes('version')) {
					return '1.0.0-freeplay'
				}
				if (cmd.includes('theme')) {
					return 'dark'
				}
				if (cmd.includes('locale')) {
					return 'en-US'
				}
				if (cmd.includes('is_fullscreen') || cmd.includes('is_maximized')) {
					return false
				}
				if (cmd.includes('should_disable_mouseover')) {
					return false
				}
				if (cmd.includes('are_updates_enabled')) {
					return false
				}
				if (cmd.includes('is_network_metered')) {
					return false
				}

				return defaultSettings[cmd] ?? null
			},
			convertFileSrc: (filePath: string) => filePath,
		}

		;(window as any).__TAURI__ = {
			core: {
				invoke: (window as any).__TAURI_INTERNALS__.invoke,
			},
			event: {
				listen: async (event: string, handler: Function) => {
					if (!listeners.has(event)) listeners.set(event, new Set())
					listeners.get(event)?.add(handler)
					return () => listeners.get(event)?.delete(handler)
				},
				emit: async (event: string, payload: any) => {
					listeners.get(event)?.forEach((fn) => fn({ event, payload }))
				},
			},
			window: {
				getCurrentWindow: () => ({
					label: 'main',
					listen: async () => () => {},
					once: async () => () => {},
					emit: async () => {},
					setTitle: async () => {},
					maximize: async () => {},
					unmaximize: async () => {},
					minimize: async () => {},
					close: async () => {},
					isMaximized: async () => false,
					isFullscreen: async () => false,
					setFocus: async () => {},
					show: async () => {},
					hide: async () => {},
					setDecorations: async () => {},
					onResized: async (cb: any) => () => {},
				}),
			},
		}
	}
}
