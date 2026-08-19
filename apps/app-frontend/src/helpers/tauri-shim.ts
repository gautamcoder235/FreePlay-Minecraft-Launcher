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

		const hostingState = {
			status: 'online' as 'offline' | 'starting' | 'online' | 'tunneling',
			version: '1.21.4',
			engine: 'PaperMC',
			ram_gb: 4,
			tunnel_enabled: true,
			public_ip: 'freeplay-game.gl.joinmc.link:25565',
			local_port: 25565,
			motd: 'A FreePlay Minecraft Server - High Performance PaperMC',
			uptime_seconds: 9840,
			cpu_percent: 14.5,
			ram_used_mb: 2480,
			players: [
				{ name: 'Alex', uuid: '853c80ef3c3749fd8636b0d3a52912c7', latency: 18, is_op: true, online: true },
				{ name: 'Steve', uuid: 'c06f89064c8a49119c29ea1dbd1a1452', latency: 24, is_op: false, online: true },
				{ name: 'Notch', uuid: '069a79f444e94726a5befca90e38aaf5', latency: 45, is_op: true, online: false },
				{ name: 'CraftMaster', uuid: '61699b2e-d327-4a01-9f1e-0ea8c3f06bc6', latency: 32, is_op: false, online: true },
			],
			logs: [
				'[12:00:01] [Server thread/INFO]: Starting FreePlay PaperMC Server version 1.21.4-R0.1-SNAPSHOT (MC: 1.21.4)',
				'[12:00:02] [Server thread/INFO]: Loading properties from server.properties',
				'[12:00:03] [Server thread/INFO]: Generating keypair and initializing encryption...',
				'[12:00:04] [Server thread/INFO]: Starting Minecraft server on *:25565',
				'[12:00:05] [Server thread/INFO]: Using default channel type (epoll/nio)',
				'[12:00:06] [Server thread/INFO]: [playit.gg] Initializing freeplay tunnel daemon...',
				'[12:00:07] [Server thread/INFO]: [playit.gg] Tunnel established -> freeplay-game.gl.joinmc.link:25565 (Latency: 14ms)',
				'[12:00:08] [Server thread/INFO]: Preparing level "world"',
				'[12:00:09] [Server thread/INFO]: Preparing start region for dimension minecraft:overworld',
				'[12:00:10] [Server thread/INFO]: Time elapsed: 1420 ms',
				'[12:00:11] [Server thread/INFO]: [DONE] Done (10.421s)! For help, type "help"',
				'[12:00:15] [Server thread/INFO]: Alex[127.0.0.1:54321] logged in with entity id 124 at ([world]0.5, 64.0, 0.5)',
				'[12:00:20] [Server thread/INFO]: Steve[127.0.0.1:54322] logged in with entity id 125 at ([world]12.5, 65.0, -8.2)',
				'[12:01:05] [Server thread/INFO]: CraftMaster[127.0.0.1:54323] logged in with entity id 126 at ([world]-45.2, 70.0, 33.1)',
			],
		}

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

				// FreePlay Server Hosting IPC Commands
				if (cmd === 'host_get_status') {
					return { ...hostingState }
				}
				if (cmd === 'host_start_server') {
					hostingState.status = 'starting'
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(`[${time}] [Server thread/INFO]: [FreePlay] Starting server core...`)
					setTimeout(() => {
						hostingState.status = 'online'
						hostingState.logs.push(`[${new Date().toLocaleTimeString()}] [Server thread/INFO]: [DONE] Server online and listening on :25565!`)
					}, 800)
					return { success: true, status: 'starting' }
				}
				if (cmd === 'host_stop_server') {
					hostingState.status = 'offline'
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(`[${time}] [Server thread/INFO]: [FreePlay] Stopping server... saving world chunks`)
					hostingState.logs.push(`[${time}] [Server thread/INFO]: Server stopped successfully.`)
					return { success: true, status: 'offline' }
				}
				if (cmd === 'host_restart_server') {
					hostingState.status = 'starting'
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(`[${time}] [Server thread/INFO]: [FreePlay] Restarting server instance...`)
					setTimeout(() => {
						hostingState.status = 'online'
						hostingState.logs.push(`[${new Date().toLocaleTimeString()}] [Server thread/INFO]: [DONE] Server restart complete!`)
					}, 1000)
					return { success: true, status: 'starting' }
				}
				if (cmd === 'host_start_tunnel') {
					hostingState.tunnel_enabled = true
					hostingState.public_ip = 'freeplay-game.gl.joinmc.link:25565'
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(`[${time}] [playit.gg]: Tunnel routing enabled -> freeplay-game.gl.joinmc.link:25565`)
					return { success: true, public_ip: hostingState.public_ip }
				}
				if (cmd === 'host_stop_tunnel') {
					hostingState.tunnel_enabled = false
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(`[${time}] [playit.gg]: Tunnel disconnected.`)
					return { success: true }
				}
				if (cmd === 'host_send_command') {
					const command = args.command || ''
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(`[${time}] [Console]: > ${command}`)
					let output = ''

					const lower = command.toLowerCase().trim()
					if (lower.startsWith('/op ')) {
						const target = command.substring(4).trim()
						output = `Made ${target} a server operator`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/gamemode ')) {
						const mode = command.substring(10).trim()
						output = `Set own game mode to ${mode} Mode`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/time set ')) {
						const t = command.substring(10).trim()
						output = `Set the time to ${t}`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/whitelist off')) {
						output = 'Removed whitelist restriction'
						hostingState.logs.push(`[${time}] [Server thread/INFO]: Whitelist is now turned off`)
					} else if (lower.startsWith('/weather ')) {
						output = `Changed the weather to ${command.substring(9).trim()}`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: Set weather to ${command.substring(9).trim()}`)
					} else if (lower === '/tps') {
						output = 'TPS from last 1m, 5m, 15m: 20.00, 20.00, 19.98'
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else {
						output = `Executed: ${command}`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: Command executed: ${command}`)
					}
					return { success: true, output }
				}
				if (cmd === 'host_open_server_dir') {
					return { success: true, path: 'C:\\Users\\FreePlay\\AppData\\Roaming\\.freeplay\\server' }
				}
				if (cmd === 'host_update_config') {
					if (args.version) hostingState.version = args.version
					if (args.engine) hostingState.engine = args.engine
					if (args.ram_gb) hostingState.ram_gb = args.ram_gb
					if (typeof args.tunnel_enabled === 'boolean') hostingState.tunnel_enabled = args.tunnel_enabled
					return { success: true, config: hostingState }
				}

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
