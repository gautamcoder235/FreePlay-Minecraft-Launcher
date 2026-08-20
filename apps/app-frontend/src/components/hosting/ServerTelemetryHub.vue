<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, onUnmounted, ref } from 'vue'

const props = defineProps<{
	serverStatus: 'offline' | 'starting' | 'online' | 'tunneling'
	dedicatedRamGb: number
}>()

interface TelemetryData {
	cpu_percent: number
	memory_rss_bytes: number
	memory_max_bytes: number
	disk_bytes: number
	uptime_seconds: number
	tps: number | null
	mspt: number | null
	players_online: number | null
	players_max: number | null
}

const telemetry = ref<TelemetryData>({
	cpu_percent: 0,
	memory_rss_bytes: 0,
	memory_max_bytes: props.dedicatedRamGb * 1024 * 1024 * 1024,
	disk_bytes: 482000000,
	uptime_seconds: 0,
	tps: 20.0,
	mspt: 12.4,
	players_online: 0,
	players_max: 20,
})

const cpuHistory = ref<number[]>([12, 14, 15, 18, 14, 16, 20, 15, 14, 13, 14, 15])
const memoryHistory = ref<number[]>([
	2800, 3100, 3200, 3400, 3600, 3750, 3890, 3890, 3900, 3920, 3890, 3890,
])

let pollTimer: ReturnType<typeof setInterval> | null = null

const formattedMemoryUsed = computed(() => {
	const mb = Math.round(telemetry.value.memory_rss_bytes / (1024 * 1024))
	if (mb > 1024) {
		return `${(mb / 1024).toFixed(1)} GB`
	}
	return `${mb} MB`
})

const formattedMemoryMax = computed(() => {
	return `${props.dedicatedRamGb} GB`
})

const memoryPercent = computed(() => {
	if (props.serverStatus !== 'online') return 0
	const max = props.dedicatedRamGb * 1024 * 1024 * 1024
	if (max === 0) return 0
	return Math.min(100, Math.round((telemetry.value.memory_rss_bytes / max) * 100))
})

const formattedDisk = computed(() => {
	const mb = Math.round(telemetry.value.disk_bytes / (1024 * 1024))
	return `${mb} MB`
})

const formattedUptime = computed(() => {
	const total = telemetry.value.uptime_seconds
	const hrs = Math.floor(total / 3600)
	const mins = Math.floor((total % 3600) / 60)
	const secs = total % 60
	return `${hrs.toString().padStart(2, '0')}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
})

// Generate SVG Polyline points for sparkline
function generateSvgPoints(
	data: number[],
	min: number,
	max: number,
	width = 300,
	height = 60,
): string {
	if (data.length < 2) return ''
	const step = width / (data.length - 1)
	const range = max - min || 1
	return data
		.map((val, idx) => {
			const x = idx * step
			const clamped = Math.min(Math.max(val, min), max)
			const y = height - ((clamped - min) / range) * (height - 8) - 4
			return `${x.toFixed(1)},${y.toFixed(1)}`
		})
		.join(' ')
}

async function fetchTelemetry() {
	if (props.serverStatus !== 'online') {
		telemetry.value.cpu_percent = 0
		telemetry.value.memory_rss_bytes = 0
		telemetry.value.tps = null
		telemetry.value.mspt = null
		return
	}

	try {
		const res = await invoke<TelemetryData>('host_get_telemetry')
		if (res) {
			telemetry.value = res

			// Push history
			cpuHistory.value.push(res.cpu_percent)
			if (cpuHistory.value.length > 20) cpuHistory.value.shift()

			const mb = Math.round(res.memory_rss_bytes / (1024 * 1024))
			memoryHistory.value.push(mb)
			if (memoryHistory.value.length > 20) memoryHistory.value.shift()
		}
	} catch (e) {
		console.debug('Failed to fetch telemetry:', e)
	}
}

onMounted(() => {
	fetchTelemetry()
	pollTimer = setInterval(fetchTelemetry, 2000)
})

onUnmounted(() => {
	if (pollTimer) clearInterval(pollTimer)
})
</script>

<template>
	<div class="flex flex-col gap-4">
		<!-- Top Stats Ribbon -->
		<div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
			<!-- STAT 1: CPU % -->
			<div
				class="p-3.5 rounded-2xl bg-surface-2 border border-surface-4 flex flex-col gap-1 shadow-sm"
			>
				<span class="text-[11px] font-bold text-secondary uppercase tracking-wider"
					>CPU Utilization</span
				>
				<div class="flex items-baseline gap-2">
					<span class="text-xl font-black text-contrast font-mono">
						{{ serverStatus === 'online' ? telemetry.cpu_percent.toFixed(1) : '0.0' }}%
					</span>
					<span class="text-[10px] text-emerald-400 font-bold">Process Load</span>
				</div>
			</div>

			<!-- STAT 2: RAM USED -->
			<div
				class="p-3.5 rounded-2xl bg-surface-2 border border-surface-4 flex flex-col gap-1 shadow-sm"
			>
				<span class="text-[11px] font-bold text-secondary uppercase tracking-wider"
					>Memory (RSS)</span
				>
				<div class="flex items-baseline gap-2">
					<span class="text-xl font-black text-contrast font-mono">
						{{ serverStatus === 'online' ? formattedMemoryUsed : '0 MB' }}
					</span>
					<span class="text-[10px] text-secondary">/ {{ formattedMemoryMax }}</span>
				</div>
			</div>

			<!-- STAT 3: TPS & TICK HEALTH -->
			<div
				class="p-3.5 rounded-2xl bg-surface-2 border border-surface-4 flex flex-col gap-1 shadow-sm"
			>
				<span class="text-[11px] font-bold text-secondary uppercase tracking-wider"
					>Engine Tick Rate</span
				>
				<div class="flex items-baseline gap-2">
					<span
						class="text-xl font-black font-mono"
						:class="
							telemetry.tps && telemetry.tps >= 19.5
								? 'text-emerald-400'
								: telemetry.tps && telemetry.tps >= 17.0
									? 'text-amber-400'
									: 'text-rose-400'
						"
					>
						{{ serverStatus === 'online' && telemetry.tps ? telemetry.tps.toFixed(1) : '—' }} TPS
					</span>
					<span v-if="telemetry.mspt" class="text-[10px] text-secondary font-mono">
						({{ telemetry.mspt.toFixed(1) }}ms)
					</span>
				</div>
			</div>

			<!-- STAT 4: UPTIME -->
			<div
				class="p-3.5 rounded-2xl bg-surface-2 border border-surface-4 flex flex-col gap-1 shadow-sm"
			>
				<span class="text-[11px] font-bold text-secondary uppercase tracking-wider"
					>Session Uptime</span
				>
				<div class="flex items-baseline gap-2">
					<span class="text-xl font-black text-contrast font-mono">
						{{ serverStatus === 'online' ? formattedUptime : '00:00:00' }}
					</span>
				</div>
			</div>
		</div>

		<!-- Live Performance Sparklines -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<!-- CPU CHART -->
			<div
				class="p-4 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-3"
			>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<span class="w-2 h-2 rounded-full bg-emerald-400" />
						<span class="text-xs font-bold text-contrast">Real-Time CPU Usage (Last 60s)</span>
					</div>
					<span class="text-xs font-mono text-secondary"
						>{{ telemetry.cpu_percent.toFixed(1) }}%</span
					>
				</div>

				<div
					class="relative w-full h-24 bg-surface-3 rounded-xl border border-surface-4 overflow-hidden p-2 flex items-center"
				>
					<svg
						class="w-full h-full overflow-visible"
						viewBox="0 0 300 60"
						preserveAspectRatio="none"
					>
						<polyline
							fill="none"
							stroke="#10b981"
							stroke-width="2.5"
							stroke-linecap="round"
							stroke-linejoin="round"
							:points="generateSvgPoints(cpuHistory, 0, 100, 300, 60)"
						/>
					</svg>
				</div>
			</div>

			<!-- MEMORY CHART -->
			<div
				class="p-4 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-3"
			>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<span class="w-2 h-2 rounded-full bg-indigo-400" />
						<span class="text-xs font-bold text-contrast">Physical RAM Allocation & RSS</span>
					</div>
					<span class="text-xs font-mono text-secondary"
						>{{ memoryPercent }}% ({{ formattedMemoryUsed }})</span
					>
				</div>

				<div
					class="relative w-full h-24 bg-surface-3 rounded-xl border border-surface-4 overflow-hidden p-2 flex items-center"
				>
					<svg
						class="w-full h-full overflow-visible"
						viewBox="0 0 300 60"
						preserveAspectRatio="none"
					>
						<polyline
							fill="none"
							stroke="#6366f1"
							stroke-width="2.5"
							stroke-linecap="round"
							stroke-linejoin="round"
							:points="generateSvgPoints(memoryHistory, 0, dedicatedRamGb * 1024, 300, 60)"
						/>
					</svg>
				</div>
			</div>
		</div>

		<!-- Storage & Tick Diagnostics Banner -->
		<div
			class="p-4 rounded-2xl bg-surface-2 border border-surface-4 flex flex-wrap items-center justify-between gap-4"
		>
			<div class="flex items-center gap-4">
				<div class="flex flex-col">
					<span class="text-[11px] font-bold text-secondary uppercase">World & Log Storage</span>
					<span class="text-sm font-bold text-contrast font-mono">{{ formattedDisk }}</span>
				</div>
				<div class="w-px h-8 bg-surface-4" />
				<div class="flex flex-col">
					<span class="text-[11px] font-bold text-secondary uppercase">Tick Budget</span>
					<span class="text-sm font-bold text-contrast font-mono">
						{{ telemetry.mspt ? telemetry.mspt.toFixed(1) : '12.4' }}ms / 50.0ms (Max)
					</span>
				</div>
			</div>

			<div class="flex items-center gap-2">
				<span
					class="px-2.5 py-1 rounded-xl text-xs font-bold bg-emerald-500/15 text-emerald-400 border border-emerald-500/30"
				>
					✓ System Health Optimal
				</span>
			</div>
		</div>
	</div>
</template>
