import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { defineStore } from 'pinia'

export type OverlayTab = 'servers' | 'addons' | 'telemetry' | 'settings'

export interface OverlayState {
	isOpen: boolean
	activeTab: OverlayTab
	activeGamePid: number | null
	activeInstanceId: string | null
	activeInstanceName: string | null
	hotkey: string
	showServerWidget: boolean
	showAddonsWidget: boolean
	showTelemetryWidget: boolean
	showSettingsWidget: boolean
	sessionStartTime: number
	systemStats: {
		fps: number
		ramUsedMb: number
		ramTotalMb: number
		cpuPercent: number
		pingMs: number
	}
}

export const useOverlayStore = defineStore('overlayStore', {
	state: (): OverlayState => ({
		isOpen: false,
		activeTab: 'servers',
		activeGamePid: null,
		activeInstanceId: null,
		activeInstanceName: null,
		hotkey: 'Shift+Tab',
		showServerWidget: true,
		showAddonsWidget: true,
		showTelemetryWidget: true,
		showSettingsWidget: false,
		sessionStartTime: Date.now(),
		systemStats: {
			fps: 60,
			ramUsedMb: 2048,
			ramTotalMb: 4096,
			cpuPercent: 18,
			pingMs: 24,
		},
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
				})
			} catch {
				// ignore
			}
		},

		async toggle(forceState?: boolean) {
			try {
				const newState = (await invoke('plugin:overlay|overlay_toggle', {
					forceState,
				})) as boolean
				this.isOpen = newState
			} catch {
				this.isOpen = forceState ?? !this.isOpen
			}
		},

		async close() {
			await this.toggle(false)
		},

		async open(tab: OverlayTab = 'servers') {
			this.activeTab = tab
			await this.toggle(true)
		},

		toggleWidget(widget: 'servers' | 'addons' | 'telemetry' | 'settings') {
			if (widget === 'servers') this.showServerWidget = !this.showServerWidget
			else if (widget === 'addons') this.showAddonsWidget = !this.showAddonsWidget
			else if (widget === 'telemetry') this.showTelemetryWidget = !this.showTelemetryWidget
			else if (widget === 'settings') this.showSettingsWidget = !this.showSettingsWidget
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

		updateMockTelemetry() {
			this.systemStats.fps = Math.floor(58 + Math.random() * 6)
			this.systemStats.cpuPercent = Math.floor(14 + Math.random() * 12)
			this.systemStats.pingMs = Math.floor(22 + Math.random() * 8)
		},
	},
})
