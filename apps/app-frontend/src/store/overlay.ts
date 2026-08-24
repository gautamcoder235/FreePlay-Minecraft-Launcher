import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { defineStore } from 'pinia'

import {
	hostingApi,
	type HostStatus,
	type PlayitTunnelEntry,
	type ServerEntry,
	type ServerTelemetry,
	type StartServerPayload,
	type TrackedPlayer,
} from '@/helpers/hosting'

export type OverlayMode = 'hidden' | 'passive_hud' | 'quick_panel' | 'full_operator'
export type OverlayTab = 'servers' | 'addons' | 'telemetry' | 'settings'

export type TelemetryState =
	| { status: 'unavailable' }
	| { status: 'loading' }
	| { status: 'live'; data: ServerTelemetry }
	| { status: 'error'; message: string }

export interface OverlayState {
	isOpen: boolean
	mode: OverlayMode
	activeTab: OverlayTab
	activeGamePid: number | null
	activeInstanceId: string | null
	activeInstanceName: string | null
	hotkey: string
	showServerWidget: boolean
	showAddonsWidget: boolean
	showTelemetryWidget: boolean
	showSettingsWidget: boolean
	showPlayerWidget: boolean
	sessionStartTime: number

	// Authoritative Server State
	server: {
		running: boolean
		status: HostStatus['serverStatus']
		version: string
		serverType: string
		ramMb: number
		port: number
		publicAddress: string | null
		claimUrl: string | null
		uptimeSeconds: number
		players: TrackedPlayer[]
		logs: string[]
		tunnels: PlayitTunnelEntry[]
	}

	// Local Server Switcher
	serverList: ServerEntry[]
	activeServerId: string | null

	// Real Telemetry State Machine
	telemetry: TelemetryState

	// Operator Controls
	commandHistory: string[]
	lastError: string | null
	isActionPending: boolean
	isTunnelConnecting: boolean
}

export const useOverlayStore = defineStore('overlayStore', {
	state: (): OverlayState => ({
		isOpen: false,
		mode: 'hidden',
		activeTab: 'servers',
		activeGamePid: null,
		activeInstanceId: null,
		activeInstanceName: null,
		hotkey: 'Shift+Tab',
		showServerWidget: true,
		showAddonsWidget: true,
		showTelemetryWidget: true,
		showSettingsWidget: false,
		showPlayerWidget: true,
		sessionStartTime: Date.now(),

		serverList: [],
		activeServerId: null,

		server: {
			running: false,
			status: 'stopped',
			version: '1.21.1',
			serverType: 'Paper',
			ramMb: 4096,
			port: 25565,
			publicAddress: null,
			claimUrl: null,
			uptimeSeconds: 0,
			players: [],
			logs: [],
			tunnels: [],
		},

		telemetry: { status: 'unavailable' },
		commandHistory: [],
		lastError: null,
		isActionPending: false,
		isTunnelConnecting: false,
	}),

	getters: {
		sessionDurationFormatted: (state) => {
			const elapsedMs = Date.now() - state.sessionStartTime
			const totalMinutes = Math.floor(elapsedMs / 60000)
			const hours = Math.floor(totalMinutes / 60)
			const minutes = totalMinutes % 60
			if (hours > 0) {
				return `${hours}h ${minutes}m`
			}
			return `${minutes}m`
		},

		isServerOnline: (state) => state.server.running || state.server.status === 'running',
		onlinePlayerCount: (state) => state.server.players.length,
	},

	actions: {
		async init() {
			try {
				const state = (await invoke('plugin:overlay|overlay_get_state')) as {
					is_open: boolean
					active_game_pid: number | null
					active_instance_id: string | null
					active_instance_name: string | null
					hotkey: string
				}
				this.isOpen = state.is_open
				this.mode = state.is_open ? 'full_operator' : 'hidden'
				this.activeGamePid = state.active_game_pid
				this.activeInstanceId = state.active_instance_id
				this.activeInstanceName = state.active_instance_name
				if (state.hotkey) this.hotkey = state.hotkey
			} catch {
				// Non-tauri or mock environment
			}

			try {
				await listen<boolean>('overlay-state-changed', (event) => {
					this.isOpen = event.payload
					this.mode = event.payload ? 'full_operator' : 'hidden'
				})
			} catch {
				// ignore
			}

			await this.refreshHostStatus()
			await this.refreshTelemetry()
		},

		async toggle(forceState?: boolean) {
			try {
				const newState = (await invoke('plugin:overlay|overlay_toggle', {
					forceState,
				})) as boolean
				this.isOpen = newState
				this.mode = newState ? 'full_operator' : 'hidden'
			} catch {
				this.isOpen = forceState ?? !this.isOpen
				this.mode = this.isOpen ? 'full_operator' : 'hidden'
			}
		},

		async close() {
			await this.toggle(false)
		},

		async open(tab: OverlayTab = 'servers') {
			this.activeTab = tab
			await this.toggle(true)
		},

		setMode(newMode: OverlayMode) {
			this.mode = newMode
			if (newMode === 'hidden') {
				this.close()
			} else {
				this.isOpen = true
			}
		},

		toggleWidget(widget: 'servers' | 'addons' | 'telemetry' | 'settings' | 'players') {
			if (widget === 'servers') this.showServerWidget = !this.showServerWidget
			else if (widget === 'addons') this.showAddonsWidget = !this.showAddonsWidget
			else if (widget === 'telemetry') this.showTelemetryWidget = !this.showTelemetryWidget
			else if (widget === 'settings') this.showSettingsWidget = !this.showSettingsWidget
			else if (widget === 'players') this.showPlayerWidget = !this.showPlayerWidget
		},

		setGameContext(pid: number | null, instanceId: string | null, instanceName: string | null) {
			this.activeGamePid = pid
			this.activeInstanceId = instanceId
			this.activeInstanceName = instanceName
			this.sessionStartTime = Date.now()

			invoke('plugin:overlay|overlay_set_active_game', {
				pid,
				instanceId,
				instanceName,
			}).catch(() => {})
		},

		async refreshHostStatus() {
			try {
				const status = await hostingApi.getStatus()
				this.server.running = status.serverRunning
				this.server.status = status.serverStatus
				this.server.serverVersion = status.serverVersion || this.server.version
				this.server.serverType = status.serverType || this.server.serverType
				this.server.ramMb = status.serverRamMb || this.server.ramMb
				this.server.port = status.serverPort || this.server.port
				this.server.publicAddress = status.publicAddress ?? null
				this.server.claimUrl = status.claimUrl ?? null
				this.server.uptimeSeconds = status.uptimeSeconds
				this.server.players = status.players
				this.server.tunnels = status.tunnels

				if (status.serverLogs.length > 0) {
					this.server.logs = status.serverLogs
				}
				this.lastError = null
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.lastError = message || 'Failed to communicate with host supervisor'
			}
		},

		async refreshServerList() {
			try {
				const list = await hostingApi.listServers()
				this.serverList = list
				if (list.length > 0) {
					const savedId = typeof window !== 'undefined' ? localStorage.getItem('freeplay-active-server-id') : null
					const matching = list.find((s) => s.id === savedId) || list[0]
					if (matching && (!this.activeServerId || !list.some((s) => s.id === this.activeServerId))) {
						this.activeServerId = matching.id
					}
				}
			} catch (err) {
				console.debug('Failed to load server list in overlay', err)
			}
		},

		async switchServer(serverId: string) {
			this.isActionPending = true
			this.lastError = null
			try {
				await hostingApi.selectServer(serverId)
				this.activeServerId = serverId
				if (typeof window !== 'undefined') {
					localStorage.setItem('freeplay-active-server-id', serverId)
				}
				const s = this.serverList.find((srv) => srv.id === serverId)
				if (s) {
					this.server.version = s.version
					this.server.serverType = s.engine
					this.server.ramMb = (s.ram_gb || 4) * 1024
					this.server.port = s.port || 25565
				}
				await this.refreshHostStatus()
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.lastError = `Failed to switch server: ${message}`
			} finally {
				this.isActionPending = false
			}
		},

		async refreshTelemetry() {
			if (!this.server.running && this.server.status !== 'running') {
				this.telemetry = { status: 'unavailable' }
				return
			}

			try {
				const data = await hostingApi.getTelemetry()
				this.telemetry = { status: 'live', data }
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.telemetry = { status: 'error', message: message || 'Telemetry unreachable' }
			}
		},

		async startDedicatedServer(customPayload?: Partial<StartServerPayload>) {
			this.isActionPending = true
			this.lastError = null
			try {
				await hostingApi.startServer({
					version: customPayload?.version || this.server.version,
					serverType: customPayload?.serverType || this.server.serverType,
					ramMb: customPayload?.ramMb || this.server.ramMb,
					port: customPayload?.port || this.server.port,
				})
				await this.refreshHostStatus()
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.lastError = `Server start failed: ${message}`
				await this.refreshHostStatus()
			} finally {
				this.isActionPending = false
			}
		},

		async stopDedicatedServer() {
			this.isActionPending = true
			this.lastError = null
			try {
				await hostingApi.stopServer()
				await this.refreshHostStatus()
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.lastError = `Server stop failed: ${message}`
				await this.refreshHostStatus()
			} finally {
				this.isActionPending = false
			}
		},

		async sendConsoleCommand(cmd: string) {
			let clean = cmd.trim()
			if (!clean) return
			if (clean.startsWith('/')) {
				clean = clean.slice(1).trim()
			}
			if (!clean) return
			this.commandHistory.push(clean)

			try {
				await hostingApi.sendCommand(clean)
				this.server.logs.push(`> ${clean}`)
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.server.logs.push(`[Console Error] ${message}`)
			}
		},

		async startTunnel(port?: number) {
			this.isTunnelConnecting = true
			this.isActionPending = true
			this.lastError = null
			try {
				const status = await hostingApi.startTunnel(port || this.server.port)
				this.server.publicAddress = status.publicAddress
				this.server.claimUrl = status.claimUrl
				this.server.tunnels = status.tunnels

				// Poll until publicAddress or claimUrl is established (or up to 15 seconds)
				let retries = 0
				while (!this.server.publicAddress && !this.server.claimUrl && retries < 15) {
					await new Promise((resolve) => setTimeout(resolve, 1000))
					await this.refreshHostStatus()
					retries++
				}
				await this.refreshHostStatus()
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.lastError = `Failed to start tunnel: ${message}`
			} finally {
				this.isActionPending = false
				this.isTunnelConnecting = false
			}
		},

		async stopTunnel() {
			this.isActionPending = true
			this.lastError = null
			try {
				await hostingApi.stopTunnel()
				this.server.publicAddress = null
				this.server.claimUrl = null
				this.server.tunnels = []
				await this.refreshHostStatus()
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.lastError = `Failed to stop tunnel: ${message}`
			} finally {
				this.isActionPending = false
			}
		},

		async performPlayerAction(action: string, player: string, param?: string) {
			try {
				await hostingApi.executePlayerAction(action, player, param)
				await this.refreshHostStatus()
			} catch (err: unknown) {
				const message = err instanceof Error ? err.message : String(err)
				this.lastError = `Player action failed: ${message}`
			}
		},
	},
})
