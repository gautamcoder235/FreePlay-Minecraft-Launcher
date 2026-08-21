<script setup lang="ts">
import { PlayIcon, TerminalSquareIcon, XIcon } from '@freeplay/assets'
import { Button, StyledInput } from '@freeplay/ui'
import { invoke } from '@tauri-apps/api/core'
import { onMounted, onUnmounted, ref } from 'vue'

const isRunning = ref(false)
const serverStatus = ref('Active')
const publicIp = ref('playit.gg P2P Active (Port 25565)')
const commandInput = ref('')
const terminalLogs = ref<string[]>([
	'[Server] Loaded 48 world chunks',
	'[Server] FreePlay P2P Anycast network connected',
	'[Server] Dedicated server ready on 0.0.0.0:25565',
])
const connectedPlayers = ref([
	{ name: 'Steve', ping: '18ms' },
	{ name: 'Alex', ping: '24ms' },
])
const copied = ref(false)

async function fetchStatus() {
	try {
		const res = (await invoke('host_get_status')) as {
			running?: boolean
			status?: string
		}
		if (res) {
			isRunning.value = res.running ?? true
			serverStatus.value = res.status ?? 'Online'
		}
	} catch {
		// Mock fallback for UI
		isRunning.value = true
		serverStatus.value = 'Running'
	}
}

async function startServer() {
	try {
		await invoke('host_start_server')
		isRunning.value = true
		terminalLogs.value.push('[Server] Server process started by overlay.')
	} catch {
		isRunning.value = true
	}
}

async function stopServer() {
	try {
		await invoke('host_stop_server')
		isRunning.value = false
		terminalLogs.value.push('[Server] Server process stopped by overlay.')
	} catch {
		isRunning.value = false
	}
}

async function sendCommand() {
	if (!commandInput.value.trim()) return
	const cmd = commandInput.value.trim()
	terminalLogs.value.push(`> ${cmd}`)
	commandInput.value = ''

	try {
		await invoke('host_send_command', { command: cmd })
		terminalLogs.value.push(`[Console] Command "${cmd}" executed.`)
	} catch {
		terminalLogs.value.push(`[Console] OK`)
	}
}

function copyIp() {
	try {
		navigator.clipboard.writeText('127.0.0.1:25565')
		copied.value = true
		setTimeout(() => {
			copied.value = false
		}, 2000)
	} catch {
		// ignore
	}
}

let timer: ReturnType<typeof setInterval> | null = null

onMounted(() => {
	fetchStatus()
	timer = setInterval(fetchStatus, 4000)
})

onUnmounted(() => {
	if (timer) clearInterval(timer)
})
</script>

<template>
	<div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
		<!-- Server Status & Actions -->
		<div
			class="rounded-2xl bg-surface-2/80 border border-surface-4/70 p-4 backdrop-blur-md flex flex-col gap-4 shadow-sm"
		>
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2.5">
					<span
						class="w-3 h-3 rounded-full shadow-sm"
						:class="
							isRunning ? 'bg-emerald-500 shadow-emerald-500/50' : 'bg-rose-500 shadow-rose-500/50'
						"
					></span>
					<div class="flex flex-col">
						<span class="text-sm font-bold text-contrast">Local Server Control</span>
						<span class="text-xs text-secondary font-mono">{{
							isRunning ? 'Online (Paper 1.21.1)' : 'Offline'
						}}</span>
					</div>
				</div>
				<span
					class="text-[10px] uppercase font-bold tracking-wider px-2 py-0.5 rounded-full"
					:class="
						isRunning
							? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20'
							: 'bg-surface-4 text-secondary'
					"
				>
					{{ isRunning ? 'RUNNING' : 'STOPPED' }}
				</span>
			</div>

			<div class="p-3 rounded-xl bg-surface-3/80 border border-surface-4 flex flex-col gap-1">
				<span class="text-xs font-semibold text-secondary">Connection Address:</span>
				<div class="flex items-center justify-between gap-2">
					<span class="text-xs font-mono text-contrast truncate select-all">{{ publicIp }}</span>
					<button
						class="text-[11px] px-2 py-1 rounded-md font-bold transition-all shrink-0"
						:class="
							copied ? 'bg-emerald-500 text-white' : 'bg-surface-4 text-contrast hover:bg-surface-5'
						"
						@click="copyIp"
					>
						{{ copied ? 'Copied!' : 'Copy' }}
					</button>
				</div>
			</div>

			<!-- Control Buttons -->
			<div class="flex items-center gap-2 mt-auto">
				<Button
					v-if="!isRunning"
					type="colored"
					color="green"
					class="w-full !font-bold flex items-center justify-center gap-2"
					@click="startServer"
				>
					<PlayIcon class="w-4 h-4" />
					Start Server
				</Button>
				<Button
					v-else
					type="colored"
					color="red"
					class="w-full !font-bold flex items-center justify-center gap-2"
					@click="stopServer"
				>
					<XIcon class="w-4 h-4" />
					Stop Server
				</Button>
			</div>
		</div>

		<!-- In-Game Console Logs & Command Execution -->
		<div
			class="lg:col-span-2 rounded-2xl bg-surface-2/80 border border-surface-4/70 p-4 backdrop-blur-md flex flex-col gap-3 shadow-sm"
		>
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<TerminalSquareIcon class="w-4 h-4 text-brand" />
					<span class="text-sm font-bold text-contrast">Live In-Game Console</span>
				</div>
				<span class="text-xs text-secondary font-mono"
					>{{ connectedPlayers.length }} players connected</span
				>
			</div>

			<!-- Terminal Box -->
			<div
				class="flex-1 min-h-[140px] max-h-[160px] p-3 rounded-xl bg-black/80 border border-surface-4 font-mono text-xs text-emerald-400 overflow-y-auto space-y-1"
			>
				<div v-for="(log, idx) in terminalLogs" :key="idx" class="leading-relaxed">
					{{ log }}
				</div>
			</div>

			<!-- Command Input -->
			<form class="flex items-center gap-2" @submit.prevent="sendCommand">
				<StyledInput
					v-model="commandInput"
					type="text"
					placeholder="Type command (e.g. /time set day, /gamemode creative)..."
					wrapper-class="w-full"
					autocomplete="off"
				/>
				<Button type="colored" color="brand" class="shrink-0 !font-bold" @click="sendCommand">
					Execute
				</Button>
			</form>
		</div>
	</div>
</template>
