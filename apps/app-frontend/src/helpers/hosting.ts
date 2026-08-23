import { invoke } from '@tauri-apps/api/core'

export interface ServerTelemetry {
	cpuPercent: number
	memoryRssBytes: number
	memoryMaxBytes: number
	diskBytes: number
	uptimeSeconds: number
	tps: number | null
	mspt: number | null
	playersOnline: number | null
	playersMax: number | null
}

export interface TrackedPlayer {
	name: string
	uuid?: string
	isOp: boolean
	gamemode?: string
	pingMs?: number
}

export interface ModerationLists {
	ops: string[]
	whitelist: string[]
	whitelistEnabled: boolean
	bannedPlayers: string[]
}

export interface PlayitTunnelEntry {
	id?: string
	tunnelType?: string
	assignedDomain?: string
	assignedIp?: string
	port?: number
}

export interface HostStatus {
	serverRunning: boolean
	serverStatus: 'stopped' | 'preparing' | 'starting' | 'running' | 'stopping' | 'crashed'
	serverVersion?: string
	serverType?: string
	serverRamMb: number
	serverPort: number
	tunnelStatus: string
	publicAddress?: string
	claimUrl?: string
	tunnels: PlayitTunnelEntry[]
	uptimeSeconds: number
	players: TrackedPlayer[]
	serverLogs: string[]
	tunnelLogs: string[]
}

export interface StartServerPayload {
	version: string
	serverType: string
	ramMb: number
	port: number
}

interface RawHostStatusIpc {
	server_running?: boolean
	server_status?: string | { running?: boolean }
	server_version?: string
	server_type?: string
	server_ram_mb?: number
	server_port?: number
	tunnel_status?: string | Record<string, Record<string, string>>
	public_address?: string
	public_ip?: string
	claim_url?: string
	tunnels?: Array<{
		id?: string
		tunnel_type?: string
		assigned_domain?: string
		assigned_ip?: string
		port?: number
	}>
	uptime_seconds?: number
	players?: Array<{
		name: string
		uuid?: string
		is_op?: boolean
		gamemode?: string
		ping_ms?: number
	}>
	server_logs?: string[]
	tunnel_logs?: string[]
	logs?: string[]
}

interface RawTelemetryIpc {
	cpu_percent?: number
	memory_rss_bytes?: number
	memory_max_bytes?: number
	disk_bytes?: number
	uptime_seconds?: number
	tps?: number
	mspt?: number
	players_online?: number
	players_max?: number
}

interface RawModerationListsIpc {
	ops?: string[]
	whitelist?: string[]
	whitelist_enabled?: boolean
	banned_players?: string[]
}

function normalizeHostStatus(raw: RawHostStatusIpc): HostStatus {
	let statusStr: HostStatus['serverStatus'] = 'stopped'
	if (typeof raw.server_status === 'string') {
		const s = raw.server_status.toLowerCase()
		if (s === 'running' || s === 'online') statusStr = 'running'
		else if (s === 'starting' || s === 'preparing') statusStr = 'starting'
		else if (s === 'stopping') statusStr = 'stopping'
		else if (s === 'crashed') statusStr = 'crashed'
		else statusStr = 'stopped'
	} else if (raw.server_running) {
		statusStr = 'running'
	}

	let tunnelStatusLabel = 'stopped'
	if (typeof raw.tunnel_status === 'string') {
		tunnelStatusLabel = raw.tunnel_status
	} else if (raw.tunnel_status && typeof raw.tunnel_status === 'object') {
		if ('connected' in raw.tunnel_status) tunnelStatusLabel = 'connected'
		else if ('claiming' in raw.tunnel_status) tunnelStatusLabel = 'claiming'
		else if ('error' in raw.tunnel_status) tunnelStatusLabel = 'error'
	}

	const normalizedPlayers: TrackedPlayer[] = Array.isArray(raw.players)
		? raw.players.map((p) => ({
				name: p.name,
				uuid: p.uuid,
				isOp: Boolean(p.is_op),
				gamemode: p.gamemode,
				pingMs: p.ping_ms,
			}))
		: []

	const normalizedTunnels: PlayitTunnelEntry[] = Array.isArray(raw.tunnels)
		? raw.tunnels.map((t) => ({
				id: t.id,
				tunnelType: t.tunnel_type,
				assignedDomain: t.assigned_domain,
				assignedIp: t.assigned_ip,
				port: t.port,
			}))
		: []

	return {
		serverRunning: Boolean(raw.server_running),
		serverStatus: statusStr,
		serverVersion: raw.server_version,
		serverType: raw.server_type,
		serverRamMb: raw.server_ram_mb || 2048,
		serverPort: raw.server_port || 25565,
		tunnelStatus: tunnelStatusLabel,
		publicAddress: raw.public_address || raw.public_ip || undefined,
		claimUrl: raw.claim_url || undefined,
		tunnels: normalizedTunnels,
		uptimeSeconds: typeof raw.uptime_seconds === 'number' ? raw.uptime_seconds : 0,
		players: normalizedPlayers,
		serverLogs: Array.isArray(raw.server_logs)
			? raw.server_logs
			: Array.isArray(raw.logs)
				? raw.logs
				: [],
		tunnelLogs: Array.isArray(raw.tunnel_logs) ? raw.tunnel_logs : [],
	}
}

export const hostingApi = {
	async getStatus(): Promise<HostStatus> {
		const raw = await invoke<RawHostStatusIpc>('host_get_status')
		return normalizeHostStatus(raw || {})
	},

	async startServer(payload: StartServerPayload): Promise<void> {
		await invoke('host_start_server', {
			version: payload.version,
			serverType: payload.serverType,
			server_type: payload.serverType,
			ramMb: payload.ramMb,
			ram_mb: payload.ramMb,
			port: payload.port,
		})
	},

	async stopServer(): Promise<void> {
		await invoke('host_stop_server')
	},

	async killServer(): Promise<void> {
		await invoke('host_kill_server')
	},

	async sendCommand(command: string): Promise<void> {
		await invoke('host_send_command', { command })
	},

	async getTelemetry(): Promise<ServerTelemetry> {
		const raw = await invoke<RawTelemetryIpc>('host_get_telemetry')
		return {
			cpuPercent: raw?.cpu_percent ?? 0,
			memoryRssBytes: raw?.memory_rss_bytes ?? 0,
			memoryMaxBytes: raw?.memory_max_bytes ?? 0,
			diskBytes: raw?.disk_bytes ?? 0,
			uptimeSeconds: raw?.uptime_seconds ?? 0,
			tps: typeof raw?.tps === 'number' ? raw.tps : null,
			mspt: typeof raw?.mspt === 'number' ? raw.mspt : null,
			playersOnline: typeof raw?.players_online === 'number' ? raw.players_online : null,
			playersMax: typeof raw?.players_max === 'number' ? raw.players_max : null,
		}
	},

	async getPlayers(): Promise<TrackedPlayer[]> {
		const raw = await invoke<
			Array<{
				name: string
				uuid?: string
				is_op?: boolean
				gamemode?: string
				ping_ms?: number
			}>
		>('host_get_players')
		if (!Array.isArray(raw)) return []
		return raw.map((p) => ({
			name: p.name,
			uuid: p.uuid,
			isOp: Boolean(p.is_op),
			gamemode: p.gamemode,
			pingMs: p.ping_ms,
		}))
	},

	async executePlayerAction(action: string, player: string, param?: string): Promise<void> {
		await invoke('host_player_action', { action, player, param })
	},

	async getModerationLists(): Promise<ModerationLists> {
		const raw = await invoke<RawModerationListsIpc>('host_get_moderation_lists')
		return {
			ops: Array.isArray(raw?.ops) ? raw.ops : [],
			whitelist: Array.isArray(raw?.whitelist) ? raw.whitelist : [],
			whitelistEnabled: Boolean(raw?.whitelist_enabled),
			bannedPlayers: Array.isArray(raw?.banned_players) ? raw.banned_players : [],
		}
	},

	async startTunnel(port: number): Promise<HostStatus> {
		const raw = await invoke<RawHostStatusIpc>('host_start_tunnel', { port })
		return normalizeHostStatus(raw || {})
	},

	async stopTunnel(): Promise<HostStatus> {
		const raw = await invoke<RawHostStatusIpc>('host_stop_tunnel')
		return normalizeHostStatus(raw || {})
	},

	async openServerDir(): Promise<void> {
		await invoke('host_open_server_dir')
	},
}
