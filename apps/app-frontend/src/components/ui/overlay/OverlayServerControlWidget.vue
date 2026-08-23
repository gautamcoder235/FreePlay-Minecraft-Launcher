<script setup lang="ts">
import {
	AlertTriangleIcon,
	CheckIcon,
	ClipboardCopyIcon,
	GlobeIcon,
	GripVerticalIcon,
	PlayIcon,
	RefreshCwIcon,
	ServerIcon,
	SparklesIcon,
	StopCircleIcon,
	SunIcon,
	TerminalSquareIcon,
	XIcon,
} from '@freeplay/assets'
import { Button, StyledInput } from '@freeplay/ui'
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'

import { useOverlayStore } from '@/store/overlay'

const overlayStore = useOverlayStore()
const emit = defineEmits<{
	(e: 'drag-start', event: PointerEvent): void
}>()

const commandInput = ref('')
const copied = ref(false)
const terminalContainer = ref<HTMLElement | null>(null)
const autoScroll = ref(true)

// Command History (Up/Down Arrow Navigation)
const historyIndex = ref(-1)
const savedCurrentDraft = ref('')

const isRunning = computed(() => overlayStore.isServerOnline)
const serverStatusText = computed(() => {
	const status = overlayStore.server.status
	if (status === 'running') return 'Paper 1.21.1 • Active'
	if (status === 'starting' || status === 'preparing') return 'Booting Core Engine...'
	if (status === 'stopping') return 'Stopping Server...'
	if (status === 'crashed') return 'Server Crashed'
	return 'Server Offline'
})

const connectionAddress = computed(() => {
	return overlayStore.server.publicAddress || `127.0.0.1:${overlayStore.server.port || 25565}`
})

function scrollToBottom() {
	if (!autoScroll.value || !terminalContainer.value) return
	nextTick(() => {
		if (terminalContainer.value) {
			terminalContainer.value.scrollTop = terminalContainer.value.scrollHeight
		}
	})
}

watch(
	() => overlayStore.server.logs.length,
	() => {
		scrollToBottom()
	},
)

async function handleStart() {
	await overlayStore.startDedicatedServer()
}

async function handleStop() {
	await overlayStore.stopDedicatedServer()
}

async function handleSendCommand() {
	const cmd = commandInput.value.trim()
	if (!cmd) return
	commandInput.value = ''
	historyIndex.value = -1
	savedCurrentDraft.value = ''
	await overlayStore.sendConsoleCommand(cmd)
	scrollToBottom()
}

function handleKeyDown(e: KeyboardEvent) {
	const history = overlayStore.commandHistory
	if (history.length === 0) return

	if (e.key === 'ArrowUp') {
		e.preventDefault()
		if (historyIndex.value === -1) {
			savedCurrentDraft.value = commandInput.value
			historyIndex.value = history.length - 1
		} else if (historyIndex.value > 0) {
			historyIndex.value--
		}
		if (history[historyIndex.value]) {
			commandInput.value = history[historyIndex.value]
		}
	} else if (e.key === 'ArrowDown') {
		e.preventDefault()
		if (historyIndex.value !== -1) {
			if (historyIndex.value < history.length - 1) {
				historyIndex.value++
				commandInput.value = history[historyIndex.value]
			} else {
				historyIndex.value = -1
				commandInput.value = savedCurrentDraft.value
			}
		}
	}
}

function runMacro(command: string) {
	commandInput.value = command
	handleSendCommand()
}

function copyIp() {
	try {
		navigator.clipboard.writeText(connectionAddress.value)
		copied.value = true
		setTimeout(() => {
			copied.value = false
		}, 2000)
	} catch {
		// ignore
	}
}

let pollTimer: ReturnType<typeof setInterval> | null = null

onMounted(() => {
	overlayStore.refreshHostStatus()
	scrollToBottom()
	pollTimer = setInterval(() => {
		overlayStore.refreshHostStatus()
	}, 3000)
})

onUnmounted(() => {
	if (pollTimer) clearInterval(pollTimer)
})
</script>

<template>
	<div
		class="w-[490px] rounded-[24px] bg-slate-900/90 backdrop-blur-2xl border border-white/12 p-5 flex flex-col gap-3.5 shadow-2xl shadow-black/90 select-none transform-gpu contain-paint"
	>
		<!-- 1. Draggable Header -->
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
					<span class="text-[11px] text-slate-400 font-mono truncate">
						{{ serverStatusText }}
					</span>
				</div>
			</div>

			<div class="flex items-center gap-2 shrink-0" @pointerdown.stop>
				<span
					class="text-[10px] uppercase font-bold tracking-wider px-2.5 py-0.5 rounded-full border transition-all"
					:class="
						isRunning
							? 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30'
							: overlayStore.server.status === 'starting'
								? 'bg-amber-500/15 text-amber-300 border-amber-500/30 animate-pulse'
								: overlayStore.server.status === 'crashed'
									? 'bg-rose-500/15 text-rose-400 border-rose-500/30'
									: 'bg-white/5 text-slate-400 border-white/10'
					"
				>
					{{ isRunning ? 'ONLINE' : overlayStore.server.status.toUpperCase() }}
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

		<!-- Error Banner -->
		<div
			v-if="overlayStore.lastError"
			class="p-2.5 rounded-xl bg-rose-500/15 border border-rose-500/30 text-rose-300 text-xs flex items-center gap-2"
		>
			<AlertTriangleIcon class="w-4 h-4 shrink-0 text-rose-400" />
			<span class="truncate flex-1">{{ overlayStore.lastError }}</span>
			<button
				class="text-[10px] text-rose-300 underline cursor-pointer bg-transparent border-none"
				@click="overlayStore.lastError = null"
			>
				Dismiss
			</button>
		</div>

		<!-- 2. Connection Info Card with Tunnel Toggle -->
		<div
			class="p-3 rounded-xl bg-black/40 border border-white/5 flex items-center justify-between gap-3"
		>
			<div class="flex flex-col min-w-0 flex-1">
				<div class="flex items-center gap-1.5">
					<span class="text-[10px] font-semibold text-slate-400 uppercase tracking-wider">
						{{ overlayStore.server.publicAddress ? 'Online Anycast Tunnel' : 'Local Host Address' }}
					</span>
					<span
						v-if="overlayStore.server.publicAddress"
						class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"
					></span>
				</div>
				<span class="text-xs font-mono text-white/90 truncate select-all">{{
					connectionAddress
				}}</span>
			</div>
			<div class="flex items-center gap-1.5 shrink-0">
				<button
					class="text-[11px] px-2.5 py-1.5 rounded-lg font-semibold transition-all cursor-pointer border flex items-center gap-1 shadow-sm"
					:class="
						copied
							? 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30'
							: 'bg-white/10 hover:bg-white/20 text-white border-white/10'
					"
					@click="copyIp"
				>
					<CheckIcon v-if="copied" class="w-3 h-3 text-emerald-400" />
					<ClipboardCopyIcon v-else class="w-3 h-3 text-slate-300" />
					<span>{{ copied ? 'Copied' : 'Copy' }}</span>
				</button>

				<button
					v-if="overlayStore.server.publicAddress"
					class="text-[11px] px-2.5 py-1.5 rounded-lg font-semibold bg-rose-500/15 hover:bg-rose-500/25 text-rose-300 border border-rose-500/30 transition-all cursor-pointer flex items-center gap-1 shadow-sm"
					title="Disconnect Playit online IP (Send terminal q -> y)"
					:disabled="overlayStore.isActionPending"
					@click="overlayStore.stopTunnel()"
				>
					<GlobeIcon class="w-3 h-3 text-rose-400" />
					<span>Close Online IP</span>
				</button>
				<button
					v-else
					class="text-[11px] px-2.5 py-1.5 rounded-lg font-semibold bg-indigo-500/15 hover:bg-indigo-500/25 text-indigo-300 border border-indigo-500/30 transition-all cursor-pointer flex items-center gap-1 shadow-sm"
					title="Start Playit online Anycast tunnel"
					:disabled="overlayStore.isActionPending"
					@click="overlayStore.startTunnel()"
				>
					<GlobeIcon class="w-3 h-3 text-indigo-400" />
					<span>Go Online</span>
				</button>
			</div>
		</div>

		<!-- Quick Server Macro Presets -->
		<div v-if="isRunning" class="flex items-center gap-1.5 overflow-x-auto pb-0.5">
			<button
				class="px-2.5 py-1 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-[11px] font-medium text-slate-300 hover:text-white flex items-center gap-1.5 transition-all cursor-pointer shrink-0"
				title="Set time to day"
				@click="runMacro('/time set day')"
			>
				<SunIcon class="w-3 h-3 text-amber-400" />
				Day
			</button>
			<button
				class="px-2.5 py-1 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-[11px] font-medium text-slate-300 hover:text-white flex items-center gap-1.5 transition-all cursor-pointer shrink-0"
				title="Clear bad weather"
				@click="runMacro('/weather clear')"
			>
				<SparklesIcon class="w-3 h-3 text-sky-400" />
				Clear Weather
			</button>
			<button
				class="px-2.5 py-1 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-[11px] font-medium text-slate-300 hover:text-white flex items-center gap-1.5 transition-all cursor-pointer shrink-0"
				title="Save all chunks"
				@click="runMacro('/save-all')"
			>
				Save World
			</button>
			<button
				class="px-2.5 py-1 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-[11px] font-medium text-slate-300 hover:text-white flex items-center gap-1.5 transition-all cursor-pointer shrink-0"
				title="Reload datapacks and permissions"
				@click="runMacro('/reload')"
			>
				<RefreshCwIcon class="w-3 h-3 text-purple-400" />
				Reload
			</button>
		</div>

		<!-- 3. Live Terminal Logs -->
		<div class="flex flex-col gap-1.5">
			<div class="flex items-center justify-between text-xs text-slate-400">
				<span class="font-medium flex items-center gap-1.5 text-slate-300">
					<TerminalSquareIcon class="w-3.5 h-3.5 text-purple-400" />
					Live Console
				</span>
				<label
					class="flex items-center gap-1.5 cursor-pointer font-mono text-[10px] text-slate-400"
				>
					<input v-model="autoScroll" type="checkbox" class="rounded accent-purple-500" />
					<span>Auto-scroll</span>
				</label>
			</div>
			<div
				ref="terminalContainer"
				class="h-28 p-3 rounded-xl bg-black/60 border border-white/5 font-mono text-[11px] overflow-y-auto space-y-1 select-text"
			>
				<div
					v-for="(log, idx) in overlayStore.server.logs"
					:key="idx"
					class="leading-tight"
					:class="
						log.includes('[ERROR]') || log.includes('Exception')
							? 'text-rose-400'
							: log.includes('[WARN]')
								? 'text-amber-300'
								: log.startsWith('>')
									? 'text-sky-300 font-bold'
									: 'text-emerald-400/90'
					"
				>
					{{ log }}
				</div>
				<div v-if="overlayStore.server.logs.length === 0" class="text-slate-500 text-[11px] italic">
					No logs available. Start server to view console output.
				</div>
			</div>
		</div>

		<!-- 4. Command Input Form with Up/Down History -->
		<form class="flex items-center gap-2" @submit.prevent="handleSendCommand">
			<StyledInput
				v-model="commandInput"
				type="text"
				placeholder="Type command (/say, /tp, /gamemode)... [Up/Down for history]"
				wrapper-class="w-full"
				autocomplete="off"
				@keydown="handleKeyDown"
			/>
			<Button
				type="colored"
				color="brand"
				class="shrink-0 !font-semibold text-xs !py-1.5 !px-3"
				:disabled="!isRunning || overlayStore.isActionPending"
				@click="handleSendCommand"
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
				:disabled="overlayStore.isActionPending"
				@click="handleStart"
			>
				<PlayIcon class="w-3.5 h-3.5" />
				{{ overlayStore.isActionPending ? 'Starting Server...' : 'Start Dedicated Server' }}
			</Button>
			<Button
				v-else
				type="colored"
				color="red"
				class="w-full !font-bold flex items-center justify-center gap-2 text-xs !py-2"
				:disabled="overlayStore.isActionPending"
				@click="handleStop"
			>
				<StopCircleIcon class="w-3.5 h-3.5" />
				{{ overlayStore.isActionPending ? 'Stopping Server...' : 'Stop Dedicated Server' }}
			</Button>
		</div>
	</div>
</template>
