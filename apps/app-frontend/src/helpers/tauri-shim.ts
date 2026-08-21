/* eslint-disable */
import { generateOfflineUuid } from './offline-uuid.ts'

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

		const savedRam = localStorage.getItem('freeplay-server-ram-gb')
		const savedVersion = localStorage.getItem('freeplay-server-version')
		const savedEngine = localStorage.getItem('freeplay-server-engine')
		const savedPort = localStorage.getItem('freeplay-server-port')

		const hostingState = {
			status: 'online' as 'offline' | 'starting' | 'online' | 'tunneling',
			version: savedVersion || '1.21.4',
			engine: savedEngine || 'PaperMC',
			ram_gb: savedRam ? Number(savedRam) : 4,
			tunnel_enabled: false,
			public_ip: 'Not Active',
			local_port: savedPort ? Number(savedPort) : 25565,
			motd: '§a§lFreePlay Control Room §7- High-Speed PaperMC Server',
			uptime_seconds: 2540,
			cpu_percent: 14.2,
			ram_used_mb: 3890,
			players: [
				{
					name: 'Alex',
					uuid: '069a79f7-44e9-4726-a5be-fca90e38aaf5',
					latency: 24,
					is_op: true,
					online: true,
				},
				{
					name: 'Steve',
					uuid: '853c80ef-3c37-49fd-aa49-938b674adae6',
					latency: 38,
					is_op: false,
					online: true,
				},
				{
					name: 'CraftMaster',
					uuid: '616ab5b0-a12e-4512-86c4-3c0703f26000',
					latency: 45,
					is_op: false,
					online: true,
				},
			],
			logs: [
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
					return {
						...hostingState,
						server_running: hostingState.status === 'online',
						public_address: hostingState.tunnel_enabled ? hostingState.public_ip : undefined,
						public_ip: hostingState.tunnel_enabled ? hostingState.public_ip : 'Not Active',
					}
				}
				if (cmd === 'host_start_server') {
					hostingState.status = 'starting'
					hostingState.tunnel_enabled = true
					hostingState.public_ip = 'freeplay-game.gl.joinmc.link:25565'
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [FreePlay] Starting server core...`,
					)
					setTimeout(() => {
						hostingState.status = 'online'
						hostingState.cpu_percent = 14.2
						hostingState.ram_used_mb = 3890
						hostingState.tunnel_enabled = true
						hostingState.public_ip = 'freeplay-game.gl.joinmc.link:25565'
						hostingState.logs.push(
							`[${new Date().toLocaleTimeString()}] [Server thread/INFO]: [DONE] Server online and listening on :25565!`,
						)
						hostingState.logs.push(
							`[${new Date().toLocaleTimeString()}] [playit.gg]: Public Anycast tunnel routed -> freeplay-game.gl.joinmc.link:25565`,
						)
					}, 800)
					return { success: true, status: 'starting', public_ip: hostingState.public_ip }
				}
				if (cmd === 'host_stop_server') {
					hostingState.status = 'offline'
					hostingState.cpu_percent = 0
					hostingState.ram_used_mb = 0
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [FreePlay] Stopping server... saving world chunks`,
					)
					hostingState.logs.push(`[${time}] [Server thread/INFO]: Server stopped successfully.`)
					return { success: true, status: 'offline' }
				}
				if (cmd === 'host_kill_server') {
					hostingState.status = 'offline'
					hostingState.cpu_percent = 0
					hostingState.ram_used_mb = 0
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/WARN]: [FreePlay] Force Kill process SIGKILL sent. Process terminated.`,
					)
					return { success: true, status: 'offline' }
				}
				if (cmd === 'host_restart_server') {
					hostingState.status = 'starting'
					hostingState.tunnel_enabled = true
					hostingState.public_ip = 'freeplay-game.gl.joinmc.link:25565'
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [FreePlay] Restarting server instance...`,
					)
					setTimeout(() => {
						hostingState.status = 'online'
						hostingState.cpu_percent = 14.2
						hostingState.ram_used_mb = 3890
						hostingState.tunnel_enabled = true
						hostingState.public_ip = 'freeplay-game.gl.joinmc.link:25565'
						hostingState.logs.push(
							`[${new Date().toLocaleTimeString()}] [Server thread/INFO]: [DONE] Server restart complete!`,
						)
						hostingState.logs.push(
							`[${new Date().toLocaleTimeString()}] [playit.gg]: Public Anycast tunnel routed -> freeplay-game.gl.joinmc.link:25565`,
						)
					}, 1000)
					return { success: true, status: 'starting' }
				}
				if (cmd === 'host_start_tunnel') {
					hostingState.tunnel_enabled = true
					hostingState.public_ip = 'freeplay-game.gl.joinmc.link:25565'
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [playit.gg]: Tunnel routing enabled -> freeplay-game.gl.joinmc.link:25565`,
					)
					return { success: true, public_ip: hostingState.public_ip }
				}
				if (cmd === 'host_stop_tunnel') {
					hostingState.tunnel_enabled = false
					hostingState.public_ip = 'Not Active'
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
					} else if (lower.startsWith('/deop ')) {
						const target = command.substring(6).trim()
						output = `Made ${target} no longer a server operator`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/kick ')) {
						const target = command.substring(6).trim()
						output = `Kicked ${target} from the server`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/ban ')) {
						const target = command.substring(5).trim()
						output = `Banned player ${target}`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/gamemode ')) {
						const mode = command.substring(10).trim()
						output = `Set own game mode to ${mode} Mode`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/time set ')) {
						const t = command.substring(10).trim()
						output = `Set the time to ${t}`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/whitelist on')) {
						output = 'Enabled whitelist restriction'
						hostingState.logs.push(`[${time}] [Server thread/INFO]: Whitelist is now turned on`)
					} else if (lower.startsWith('/whitelist off')) {
						output = 'Removed whitelist restriction'
						hostingState.logs.push(`[${time}] [Server thread/INFO]: Whitelist is now turned off`)
					} else if (lower.startsWith('/whitelist add ')) {
						const target = command.substring(15).trim()
						output = `Added ${target} to the whitelist`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else if (lower.startsWith('/weather ')) {
						output = `Changed the weather to ${command.substring(9).trim()}`
						hostingState.logs.push(
							`[${time}] [Server thread/INFO]: Set weather to ${command.substring(9).trim()}`,
						)
					} else if (lower.startsWith('/say ')) {
						const msg = command.substring(5).trim()
						output = `[Server] ${msg}`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: [Server] ${msg}`)
					} else if (lower === '/tps') {
						output = 'TPS from last 1m, 5m, 15m: 20.00, 20.00, 19.98 (MSPT: 12.4ms)'
						hostingState.logs.push(`[${time}] [Server thread/INFO]: ${output}`)
					} else {
						output = `Executed: ${command}`
						hostingState.logs.push(`[${time}] [Server thread/INFO]: Command executed: ${command}`)
					}
					return { success: true, output }
				}
				if (cmd === 'host_get_capabilities') {
					const isPaper = hostingState.engine.toLowerCase().includes('paper')
					const isFabric = hostingState.engine.toLowerCase().includes('fabric')
					return {
						player_list: true,
						player_ping: isPaper || isFabric,
						player_gamemode: isPaper || isFabric,
						tps: isPaper,
						mspt: isPaper,
						console_commands: true,
						addon_install: isPaper || isFabric,
						world_reset: true,
					}
				}

				if (cmd === 'host_get_telemetry') {
					return {
						cpu_percent: hostingState.status === 'online' ? 14.2 + (Math.random() * 4 - 2) : 0,
						memory_rss_bytes:
							hostingState.status === 'online' ? hostingState.ram_used_mb * 1024 * 1024 : 0,
						memory_max_bytes: hostingState.ram_gb * 1024 * 1024 * 1024,
						disk_bytes: 482000000,
						uptime_seconds: hostingState.status === 'online' ? hostingState.uptime_seconds : 0,
						tps: hostingState.status === 'online' ? 20.0 : null,
						mspt: hostingState.status === 'online' ? 12.4 : null,
						players_online: hostingState.status === 'online' ? hostingState.players.length : 0,
						players_max: 20,
					}
				}

				if (cmd === 'host_get_players') {
					return hostingState.players
				}

				if (cmd === 'host_player_action') {
					const { action, player, param } = args
					const time = new Date().toLocaleTimeString()
					if (action === 'op') {
						const p = hostingState.players.find((pl) => pl.name === player)
						if (p) p.is_op = true
						hostingState.logs.push(
							`[${time}] [Server thread/INFO]: Made ${player} a server operator`,
						)
					} else if (action === 'deop') {
						const p = hostingState.players.find((pl) => pl.name === player)
						if (p) p.is_op = false
						hostingState.logs.push(
							`[${time}] [Server thread/INFO]: Made ${player} no longer a server operator`,
						)
					} else if (action === 'kick') {
						hostingState.players = hostingState.players.filter((pl) => pl.name !== player)
						hostingState.logs.push(
							`[${time}] [Server thread/INFO]: Kicked ${player}: ${param || 'Kicked by operator'}`,
						)
					} else if (action === 'ban') {
						hostingState.players = hostingState.players.filter((pl) => pl.name !== player)
						hostingState.logs.push(
							`[${time}] [Server thread/INFO]: Banned ${player}: ${param || 'Banned by operator'}`,
						)
					}
					return { success: true, players: hostingState.players }
				}

				if (cmd === 'host_get_server_properties') {
					return {
						difficulty: 'normal',
						gamemode: 'survival',
						hardcore: false,
						pvp: true,
						allow_flight: true,
						view_distance: 10,
						simulation_distance: 10,
						spawn_protection: 16,
						allow_nether: true,
						online_mode: false,
						white_list: false,
						enable_command_block: true,
						max_players: 20,
						spawn_monsters: true,
						spawn_animals: true,
						spawn_npcs: true,
						motd: hostingState.motd,
						server_port: hostingState.local_port,
					}
				}

				if (cmd === 'host_save_server_properties') {
					if (args.properties) {
						if (args.properties.motd) hostingState.motd = args.properties.motd
						if (args.properties.server_port) hostingState.local_port = args.properties.server_port
					}
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Config] Server properties updated and saved.`,
					)
					return { success: true }
				}

				// Backups Mock IPC
				if (cmd === 'host_get_backups') {
					return [
						{
							id: 'b-1',
							name: 'World Before Dragon Fight',
							created_at: new Date(Date.now() - 3600000).toISOString(),
							size_bytes: 48200000,
							type: 'Manual',
						},
						{
							id: 'b-2',
							name: 'Daily Auto-Backup',
							created_at: new Date(Date.now() - 86400000).toISOString(),
							size_bytes: 47800000,
							type: 'Automatic',
						},
						{
							id: 'b-3',
							name: 'Fresh World Spawn',
							created_at: new Date(Date.now() - 259200000).toISOString(),
							size_bytes: 42100000,
							type: 'Initial',
						},
					]
				}
				if (cmd === 'host_create_backup') {
					const name = args.name || `Backup #${Date.now()}`
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Backups] Created world backup snapshot: "${name}"`,
					)
					return {
						success: true,
						backup: {
							id: `b-${Date.now()}`,
							name,
							created_at: new Date().toISOString(),
							size_bytes: 48500000,
							type: 'Manual',
						},
					}
				}
				if (cmd === 'host_restore_backup') {
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Backups] Restored world backup snapshot ID: ${args.backup_id}`,
					)
					return { success: true }
				}
				if (cmd === 'host_delete_backup') {
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Backups] Deleted backup ID: ${args.backup_id}`,
					)
					return { success: true }
				}

				// Plugins & Addons Mock IPC
				if (cmd === 'host_get_plugins') {
					return [
						{
							id: 'essentialsx',
							name: 'EssentialsX',
							version: '2.20.1',
							enabled: true,
							category: 'Utility',
							description:
								'Essential server commands, teleportation, kits, economy, and chat formatting.',
						},
						{
							id: 'luckperms',
							name: 'LuckPerms',
							version: '5.4.102',
							enabled: true,
							category: 'Permissions',
							description:
								'Industry-standard permission management plugin with web editor support.',
						},
						{
							id: 'worldedit',
							name: 'WorldEdit',
							version: '7.3.0',
							enabled: true,
							category: 'Building',
							description: 'In-game Minecraft world editor and terraforming tool.',
						},
						{
							id: 'viaversion',
							name: 'ViaVersion',
							version: '4.9.2',
							enabled: true,
							category: 'Compatibility',
							description: 'Allows newer Minecraft client versions to connect to your server.',
						},
						{
							id: 'geyser',
							name: 'GeyserMC',
							version: '2.2.0',
							enabled: false,
							category: 'Crossplay',
							description: 'Enables Bedrock Edition players on mobile, Xbox, PS, Switch to join.',
						},
						{
							id: 'chunky',
							name: 'Chunky',
							version: '1.4.10',
							enabled: true,
							category: 'Performance',
							description: 'Pre-generates world chunks rapidly to eliminate exploration lag.',
						},
					]
				}
				if (cmd === 'host_toggle_plugin') {
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Plugins] Plugin "${args.id}" state updated to: ${args.enabled ? 'ENABLED' : 'DISABLED'}`,
					)
					return { success: true, id: args.id, enabled: args.enabled }
				}
				if (cmd === 'host_install_plugin') {
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Plugins] Installed plugin "${args.name}" successfully!`,
					)
					return {
						success: true,
						plugin: {
							id: args.id,
							name: args.name,
							version: args.version || '1.0.0',
							enabled: true,
							category: args.category || 'Utility',
							description: args.description || '',
						},
					}
				}
				if (cmd === 'host_delete_plugin') {
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Plugins] Removed plugin "${args.id}" from server`,
					)
					return { success: true }
				}

				// File Manager Mock IPC
				if (cmd === 'host_get_files') {
					return [
						{
							name: 'server.properties',
							path: '/server.properties',
							type: 'file',
							size: 1420,
							modified: new Date().toISOString(),
						},
						{
							name: 'eula.txt',
							path: '/eula.txt',
							type: 'file',
							size: 180,
							modified: new Date().toISOString(),
						},
						{
							name: 'ops.json',
							path: '/ops.json',
							type: 'file',
							size: 310,
							modified: new Date().toISOString(),
						},
						{
							name: 'whitelist.json',
							path: '/whitelist.json',
							type: 'file',
							size: 120,
							modified: new Date().toISOString(),
						},
						{
							name: 'banned-players.json',
							path: '/banned-players.json',
							type: 'file',
							size: 85,
							modified: new Date().toISOString(),
						},
						{
							name: 'plugins',
							path: '/plugins',
							type: 'directory',
							size: 0,
							modified: new Date().toISOString(),
						},
						{
							name: 'world',
							path: '/world',
							type: 'directory',
							size: 0,
							modified: new Date().toISOString(),
						},
						{
							name: 'logs',
							path: '/logs',
							type: 'directory',
							size: 0,
							modified: new Date().toISOString(),
						},
						{
							name: 'config',
							path: '/config',
							type: 'directory',
							size: 0,
							modified: new Date().toISOString(),
						},
					]
				}
				if (cmd === 'host_read_file') {
					if (args.path === '/server.properties') {
						return `# Minecraft server properties\nmotd=${hostingState.motd}\nserver-port=${hostingState.local_port}\nmax-players=20\ndifficulty=normal\ngamemode=survival\npvp=true\nonline-mode=false\nenforce-secure-profile=false\nview-distance=10\nallow-nether=true\nspawn-protection=0\n`
					}
					if (args.path === '/eula.txt') {
						return `# By changing the setting below to TRUE you are indicating your agreement to our EULA.\neula=true\n`
					}
					if (args.path === '/ops.json') {
						return JSON.stringify(
							hostingState.players
								.filter((p) => p.is_op)
								.map((p) => ({ uuid: p.uuid, name: p.name, level: 4, bypassesPlayerLimit: false })),
							null,
							2,
						)
					}
					if (args.path === '/whitelist.json') {
						return JSON.stringify(
							hostingState.players.map((p) => ({ uuid: p.uuid, name: p.name })),
							null,
							2,
						)
					}
					return `# File content for ${args.path}\n# Configured via FreePlay Server Control Room\n`
				}
				if (cmd === 'host_save_file') {
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Files] Saved changes to file: ${args.path}`,
					)
					return { success: true }
				}
				if (cmd === 'host_delete_file') {
					const time = new Date().toLocaleTimeString()
					hostingState.logs.push(
						`[${time}] [Server thread/INFO]: [Files] Deleted file: ${args.path}`,
					)
					return { success: true }
				}

				// Server Instances Multi-Host Mock IPC
				const savedServersStr = localStorage.getItem('freeplay-servers-list')
				let mockServers: any[] = []
				try {
					if (savedServersStr) mockServers = JSON.parse(savedServersStr)
				} catch {
					// ignore
				}
				if (!Array.isArray(mockServers) || mockServers.length === 0) {
					mockServers = [
						{
							id: 'main-server',
							name: 'Main Survival Server',
							path: 'C:\\Users\\FreePlay\\AppData\\Roaming\\.freeplay\\servers\\main-server',
							engine: 'PaperMC',
							version: '1.21.4',
							port: 25565,
							ram_gb: 4,
						},
						{
							id: 'creative-world',
							name: 'Creative Building Hub',
							path: 'C:\\Users\\FreePlay\\AppData\\Roaming\\.freeplay\\servers\\creative-world',
							engine: 'Fabric',
							version: '1.20.1',
							port: 25566,
							ram_gb: 6,
						},
					]
				}

				if (cmd === 'host_list_servers') {
					return mockServers
				}
				if (cmd === 'host_create_server') {
					const newServer = {
						id: args.name?.toLowerCase().replace(/\s+/g, '-') || `server-${Date.now()}`,
						name: args.name || 'New Minecraft Server',
						path:
							args.custom_path ||
							`C:\\Users\\FreePlay\\AppData\\Roaming\\.freeplay\\servers\\${args.name?.toLowerCase().replace(/\s+/g, '-') || 'new-server'}`,
						engine: args.engine || 'PaperMC',
						version: args.version || '1.21.4',
						port: args.port || 25565,
						ram_gb: args.ram_gb || 4,
					}
					mockServers.push(newServer)
					try {
						localStorage.setItem('freeplay-servers-list', JSON.stringify(mockServers))
					} catch {
						// ignore
					}
					return newServer
				}
				if (cmd === 'host_delete_server') {
					const idx = mockServers.findIndex((s) => s.id === args.server_id)
					if (idx !== -1) mockServers.splice(idx, 1)
					try {
						localStorage.setItem('freeplay-servers-list', JSON.stringify(mockServers))
					} catch {
						// ignore
					}
					return { success: true }
				}
				if (cmd === 'host_select_server') {
					const s = mockServers.find((s) => s.id === args.server_id)
					if (s) {
						hostingState.version = s.version
						hostingState.engine = s.engine
						hostingState.ram_gb = s.ram_gb
						hostingState.local_port = s.port
					}
					return { success: true }
				}
				if (cmd === 'host_update_config') {
					const ram = args.ram_gb || args.ramGb
					if (ram) {
						hostingState.ram_gb = ram
						try {
							localStorage.setItem('freeplay-server-ram-gb', String(ram))
						} catch {
							// ignore
						}
					}
					if (args.version) {
						hostingState.version = args.version
						try {
							localStorage.setItem('freeplay-server-version', args.version)
						} catch {
							// ignore
						}
					}
					if (args.engine) {
						hostingState.engine = args.engine
						try {
							localStorage.setItem('freeplay-server-engine', args.engine)
						} catch {
							// ignore
						}
					}
					if (args.port) {
						hostingState.local_port = args.port
						try {
							localStorage.setItem('freeplay-server-port', String(args.port))
						} catch {
							// ignore
						}
					}
					if (args.server_id || args.serverId) {
						const sid = args.server_id || args.serverId
						const s = mockServers.find((srv) => srv.id === sid)
						if (s) {
							if (ram) s.ram_gb = ram
							if (args.version) s.version = args.version
							if (args.engine) s.engine = args.engine
							if (args.port) s.port = args.port
							try {
								localStorage.setItem('freeplay-servers-list', JSON.stringify(mockServers))
							} catch {
								// ignore
							}
						}
					}
					return { success: true }
				}

				// Minecraft Skins IPC Commands
				if (cmd === 'plugin:minecraft-skins|get_available_skins' || cmd === 'get_available_skins') {
					return [
						{
							texture_key: 'steve',
							name: 'Steve',
							variant: 'CLASSIC',
							texture:
								'https://textures.minecraft.net/texture/1a4143fe1b5c92892994441584c6ef6e6c4331d2ffb511394b306b9b33a571',
							source: 'default',
							is_equipped: true,
						},
						{
							texture_key: 'alex',
							name: 'Alex',
							variant: 'SLIM',
							texture:
								'https://textures.minecraft.net/texture/68291410d2961d6bc080d9c4943f778d10ed8b1a8d0e703ff3245459ec784',
							source: 'default',
							is_equipped: false,
						},
					]
				}
				if (cmd === 'plugin:minecraft-skins|get_available_capes' || cmd === 'get_available_capes') {
					return [
						{
							id: 'minecon_2016',
							name: 'MINECON 2016',
							texture:
								'https://textures.minecraft.net/texture/b05b3590caf77d3e494b415b70256dcd5d3fd32335d37a8fed8e2d29ed48a2c',
							is_equipped: false,
						},
					]
				}
				if (cmd.includes('normalize_skin_texture')) {
					return new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10])
				}
				if (cmd.includes('add_and_equip_custom_skin') || cmd.includes('save_custom_skin')) {
					return {
						texture_key: 'custom_skin_' + Date.now(),
						name: 'Custom Skin',
						variant: args.variant || 'CLASSIC',
						texture:
							'https://textures.minecraft.net/texture/1a4143fe1b5c92892994441584c6ef6e6c4331d2ffb511394b306b9b33a571',
						source: 'custom',
						is_equipped: true,
					}
				}
				if (
					cmd.includes('equip_skin') ||
					cmd.includes('remove_custom_skin') ||
					cmd.includes('unequip_skin') ||
					cmd.includes('flush_pending_skin_change')
				) {
					return true
				}

				// JRE & Optimal Java IPC
				if (cmd.includes('get_optimal_jre_key') || cmd.includes('get_optimal_jre')) {
					return {
						parsed_version: 21,
						path: 'C:\\Program Files\\Java\\jdk-21\\bin\\java.exe',
					}
				}

				// Metadata / Loader Manifest IPC
				if (cmd.includes('get_loader_versions')) {
					return {
						gameVersions: [
							{ id: '1.21.4', loaders: [{ id: '1.0.0', stable: true }] },
							{ id: '1.20.1', loaders: [{ id: '0.15.0', stable: true }] },
							{ id: '${freeplay.gameVersion}', loaders: [{ id: '0.16.0', stable: true }] },
						],
					}
				}

				// Minecraft Auth IPC Commands
				if (
					cmd === 'plugin:auth|create_offline_account' ||
					cmd.includes('create_offline_account')
				) {
					const username = (args?.username || 'Player').trim().replace(/[^a-zA-Z0-9_]/g, '')
					const offlineUuid = generateOfflineUuid(username)

					const account = {
						profile: {
							id: offlineUuid,
							name: username,
							skins: [],
							capes: [],
						},
						access_token: '0',
						refresh_token: '',
						active: true,
					}
					try {
						const saved = localStorage.getItem('freeplay-offline-accounts') || '[]'
						const list = JSON.parse(saved)
						const profileObj = {
							id: offlineUuid,
							username: username,
							name: username,
							type: 'offline',
							avatar_url: `https://mc-heads.net/avatar/${encodeURIComponent(username)}/128`,
						}
						const idx = list.findIndex(
							(a: any) => (a.username || a.name)?.toLowerCase() === username.toLowerCase(),
						)
						if (idx >= 0) list[idx] = profileObj
						else list.unshift(profileObj)
						localStorage.setItem('freeplay-offline-accounts', JSON.stringify(list))
						localStorage.setItem('freeplay-active-player', JSON.stringify(profileObj))
						localStorage.setItem('freeplay-default-user', offlineUuid)
					} catch {
						// ignore
					}
					return account
				}

				if (cmd === 'plugin:auth|get_default_user' || cmd.includes('get_default_user')) {
					return localStorage.getItem('freeplay-default-user') || undefined
				}

				if (cmd === 'plugin:auth|set_default_user' || cmd.includes('set_default_user')) {
					if (args?.user) {
						localStorage.setItem('freeplay-default-user', args.user)
					}
					return true
				}

				if (cmd === 'plugin:auth|get_users' || cmd === 'plugin:auth|users') {
					try {
						const defaultUser = localStorage.getItem('freeplay-default-user')
						const saved = localStorage.getItem('freeplay-offline-accounts')
						if (saved) {
							const list = JSON.parse(saved)
							if (Array.isArray(list)) {
								return list.map((a: any) => {
									const isMatch =
										a.id === defaultUser ||
										(defaultUser &&
											a.id?.replace(/-/g, '').toLowerCase() ===
												defaultUser.replace(/-/g, '').toLowerCase()) ||
										(a.username || a.name)?.toLowerCase() === defaultUser?.toLowerCase()
									return {
										profile: {
											id: a.id,
											name: a.username || a.name,
										},
										access_token: '0',
										active: Boolean(isMatch),
										type: 'offline',
									}
								})
							}
						}
					} catch {
						// ignore
					}
					return []
				}

				if (cmd === 'plugin:auth|remove_user' || cmd.includes('remove_user')) {
					try {
						const saved = localStorage.getItem('freeplay-offline-accounts')
						if (saved) {
							const list = JSON.parse(saved)
							if (Array.isArray(list)) {
								const filtered = list.filter(
									(a: any) => a.id !== args?.user && a.username !== args?.user,
								)
								localStorage.setItem('freeplay-offline-accounts', JSON.stringify(filtered))
							}
						}
					} catch {
						// ignore
					}
					return true
				}
				// Instance Management IPC Mock
				function getStoredMockInstances() {
					try {
						const raw = localStorage.getItem('freeplay-mock-instances')
						if (raw) return JSON.parse(raw)
					} catch {
						// ignore
					}
					return [
						{
							id: 'test-1-instance',
							name: 'Test 1',
							game_version: '1.21.4',
							loader: 'fabric',
							icon_path: null,
							created: new Date().toISOString(),
							last_played: new Date().toISOString(),
							total_play_time: 120,
							groups: [],
						},
					]
				}

				function saveStoredMockInstances(list: unknown[]) {
					try {
						localStorage.setItem('freeplay-mock-instances', JSON.stringify(list))
					} catch {
						// ignore
					}
				}

				if (cmd === 'plugin:instance|instance_list' || cmd === 'instance_list') {
					return getStoredMockInstances()
				}

				if (cmd === 'plugin:instance|instance_get' || cmd === 'instance_get') {
					const list = getStoredMockInstances()
					return list.find((i: { id: string }) => i.id === args?.instanceId) || list[0] || null
				}

				if (cmd === 'plugin:instance|instance_get_many' || cmd === 'instance_get_many') {
					const list = getStoredMockInstances()
					const ids = args?.instanceIds || []
					return list.filter((i: { id: string }) => ids.includes(i.id))
				}

				if (cmd === 'plugin:instance|instance_remove' || cmd === 'instance_remove') {
					const list = getStoredMockInstances()
					const next = list.filter((i: { id: string }) => i.id !== args?.instanceId)
					saveStoredMockInstances(next)
					return true
				}

				// Array queries
				if (
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
