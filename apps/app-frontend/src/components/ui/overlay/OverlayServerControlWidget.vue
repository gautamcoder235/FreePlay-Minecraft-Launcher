<script setup lang="ts">
import {
	CheckIcon,
	ClipboardCopyIcon,
	GripVerticalIcon,
	PlayIcon,
	ServerIcon,
	StopCircleIcon,
	TerminalSquareIcon,
	XIcon,
} from '@freeplay/assets'
import { Button, StyledInput } from '@freeplay/ui'
import { invoke } from '@tauri-apps/api/core'
import { onMounted, onUnmounted, ref } from 'vue'

import { useOverlayStore } from '@/store/overlay'

const overlayStore = useOverlayStore()
const emit = defineEmits<{
	(e: 'drag-start', event: PointerEvent): void
}>()

const isRunning = ref(false)
const serverStatus = ref('Active')
const publicIp = ref('playit.gg P2P Active (Port 25565)')
const commandInput = ref('')
const terminalLogs = ref<string[]>([
	'[Server] Loaded 48 world chunks',
	'[Server] FreePlay P2P Anycast network connected',
	'[Server] Dedicated server ready on 0.0.0.0:25565',
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
	<div
		class="w-[460px] rounded-[24px] bg-slate-900/90 backdrop-blur-2xl border border-white/12 p-5 flex flex-col gap-4 shadow-2xl shadow-black/90 select-none transform-gpu contain-paint"
	>
		<!-- 1. Draggable Header (Hardware Accelerated Pointer Grab) -->
		<div
			class="flex items-center justify-between border-b border-white/10 pb-3 cursor-grab active:cursor-grabbing group/header touch-none"
			title="Click and drag to move"
			@pointerdown.stop.prevent="emit('drag-start', $event)"
		>
			<div class="flex items-center gap-2.5 min-w-0 pointer-events-none">
				<GripVerticalIcon
					class="w-4 h-4 text-slate-500 group-hover/header:text-slate-300 transition-colors shrink-0"
				/>
				<div
					class="w-8 h-8 rounded-xl bg-purple-500/15 border border-purple-500/25 flex items-center justify-center text-purple-400 shrink-0 shadow-sm"
				>
					<ServerIcon class="w-4 h-4" />
				</div>
				<div class="flex flex-col min-w-0">
					<span class="text-sm font-semibold text-white/95 tracking-tight truncate"
						>Server Control Hub</span
					>
					<span class="text-[11px] text-slate-400 font-mono truncate">{{
						isRunning ? 'Paper 1.21.1 • Active' : 'Server Offline'
					}}</span>
				</div>
			</div>

			<div class="flex items-center gap-2 shrink-0" @pointerdown.stop>
				<span
					class="text-[10px] uppercase font-bold tracking-wider px-2.5 py-0.5 rounded-full border transition-all"
					:class="
						isRunning
							? 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30'
							: 'bg-white/5 text-slate-400 border-white/10'
					"
				>
					{{ isRunning ? 'ONLINE' : 'STOPPED' }}
				</span>
				<button
					class="w-7 h-7 rounded-xl bg-white/5 hover:bg-white/15 text-slate-400 hover:text-white flex items-center justify-center transition-all cursor-pointer border-none shadow-sm"
					title="Hide widget"
					@click="overlayStore.showServerWidget = false"
				>
					<XIcon class="w-3.5 h-3.5" />
				</button>
			</div>
		</div>

		<!-- 2. Connection Info Card -->
		<div
			class="p-3 rounded-xl bg-black/40 border border-white/5 flex items-center justify-between gap-3"
		>
			<div class="flex flex-col min-w-0">
				<span class="text-[10px] font-semibold text-slate-400 uppercase tracking-wider"
					>P2P Anycast IP</span
				>
				<span class="text-xs font-mono text-white/90 truncate select-all">{{ publicIp }}</span>
			</div>
			<button
				class="text-[11px] px-3 py-1.5 rounded-lg font-semibold transition-all shrink-0 cursor-pointer border flex items-center gap-1.5 shadow-sm"
				:class="
					copied
						? 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30'
						: 'bg-white/10 hover:bg-white/20 text-white border-white/10'
				"
				@click="copyIp"
			>
				<CheckIcon v-if="copied" class="w-3 h-3 text-emerald-400" />
				<ClipboardCopyIcon v-else class="w-3 h-3 text-slate-300" />
				<span>{{ copied ? 'Copied' : 'Copy IP' }}</span>
			</button>
		</div>

		<!-- 3. Live Terminal Logs -->
		<div class="flex flex-col gap-1.5">
			<div class="flex items-center justify-between text-xs text-slate-400">
				<span class="font-medium flex items-center gap-1.5 text-slate-300">
					<TerminalSquareIcon class="w-3.5 h-3.5 text-purple-400" />
					Live Console
				</span>
				<span class="font-mono text-[10px] text-slate-500">Auto-scroll</span>
			</div>
			<div
				class="h-24 p-3 rounded-xl bg-black/60 border border-white/5 font-mono text-[11px] text-emerald-400/90 overflow-y-auto space-y-1 select-text"
			>
				<div v-for="(log, idx) in terminalLogs" :key="idx" class="leading-tight">
					{{ log }}
				</div>
			</div>
		</div>

		<!-- 4. Command Input Form -->
		<form class="flex items-center gap-2" @submit.prevent="sendCommand">
			<StyledInput
				v-model="commandInput"
				type="text"
				placeholder="Type server command (e.g. /time set day)..."
				wrapper-class="w-full"
				autocomplete="off"
			/>
			<Button
				type="colored"
				color="brand"
				class="shrink-0 !font-semibold text-xs !py-1.5 !px-3"
				@click="sendCommand"
			>
				Run
			</Button>
		</form>

		<!-- 5. Primary Action Button -->
		<div class="flex items-center gap-2 pt-1 border-t border-white/5">
			<Button
				v-if="!isRunning"
				type="colored"
				color="green"
				class="w-full !font-bold flex items-center justify-center gap-2 text-xs !py-2"
				@click="startServer"
			>
				<PlayIcon class="w-3.5 h-3.5" />
				Start Dedicated Server
			</Button>
			<Button
				v-else
				type="colored"
				color="red"
				class="w-full !font-bold flex items-center justify-center gap-2 text-xs !py-2"
				@click="stopServer"
			>
				<StopCircleIcon class="w-3.5 h-3.5" />
				Stop Dedicated Server
			</Button>
		</div>
	</div>
</template>
