<script setup lang="ts">
import {
	CpuIcon,
	GaugeIcon,
	GripVerticalIcon,
	SparklesIcon,
	UsersIcon,
	XIcon,
} from '@freeplay/assets'
import { computed, onMounted, onUnmounted } from 'vue'

import { useOverlayStore } from '@/store/overlay'

const overlayStore = useOverlayStore()
const emit = defineEmits<{
	(e: 'drag-start', event: PointerEvent): void
}>()

const isLive = computed(() => overlayStore.telemetry.status === 'live')
const telemetryData = computed(() => {
	if (overlayStore.telemetry.status === 'live') {
		return overlayStore.telemetry.data
	}
	return null
})

const tpsFormatted = computed(() => {
	if (!isLive.value || telemetryData.value?.tps == null) return '20.0'
	return telemetryData.value.tps.toFixed(1)
})

const msptFormatted = computed(() => {
	if (!isLive.value || telemetryData.value?.mspt == null) return '--'
	return `${telemetryData.value.mspt.toFixed(1)}ms`
})

const ramUsedMb = computed(() => {
	if (!isLive.value || !telemetryData.value?.memoryRssBytes) return 0
	return Math.round(telemetryData.value.memoryRssBytes / (1024 * 1024))
})

const ramMaxMb = computed(() => {
	if (!isLive.value || !telemetryData.value?.memoryMaxBytes) {
		return overlayStore.server.ramMb || 4096
	}
	return Math.round(telemetryData.value.memoryMaxBytes / (1024 * 1024))
})

const cpuPercentFormatted = computed(() => {
	if (!isLive.value || telemetryData.value?.cpuPercent == null) return 0
	return Math.round(telemetryData.value.cpuPercent)
})

const onlinePlayersCount = computed(() => {
	if (telemetryData.value?.playersOnline != null) {
		return telemetryData.value.playersOnline
	}
	return overlayStore.onlinePlayerCount
})

const maxPlayersCount = computed(() => {
	if (telemetryData.value?.playersMax != null) {
		return telemetryData.value.playersMax
	}
	return 20
})

let pollInterval: ReturnType<typeof setInterval> | null = null

onMounted(() => {
	overlayStore.refreshTelemetry()
	pollInterval = setInterval(() => {
		overlayStore.refreshTelemetry()
	}, 1500)
})

onUnmounted(() => {
	if (pollInterval) clearInterval(pollInterval)
})
</script>

<template>
	<div
		class="rounded-[24px] bg-slate-900/90 backdrop-blur-2xl border border-white/12 px-5 py-3.5 flex items-center justify-between gap-6 shadow-2xl shadow-black/90 select-none transform-gpu contain-paint"
	>
		<!-- Drag Handle -->
		<div
			class="flex items-center gap-2 cursor-grab active:cursor-grabbing text-slate-500 hover:text-slate-300 transition-colors pr-2 border-r border-white/10 touch-none"
			title="Click and drag to move HUD"
			@pointerdown.stop.prevent="emit('drag-start', $event)"
		>
			<GripVerticalIcon class="w-4 h-4 shrink-0 pointer-events-none" />
			<span
				class="text-[11px] font-bold font-lemon-milk tracking-wider text-slate-400 uppercase pointer-events-none"
				>HUD</span
			>
		</div>

		<!-- Real Metrics Deck -->
		<div class="flex items-center gap-7">
			<!-- TPS / MSPT -->
			<div class="flex items-center gap-2.5">
				<div
					class="w-7 h-7 rounded-xl bg-emerald-500/15 border border-emerald-500/25 flex items-center justify-center text-emerald-400"
				>
					<SparklesIcon class="w-3.5 h-3.5" />
				</div>
				<div class="flex flex-col">
					<span class="text-[10px] font-bold font-lemon-milk text-slate-400 uppercase tracking-wider"
						>Server TPS</span
					>
					<div class="flex items-baseline gap-1">
						<span
							class="text-base font-bold font-mono"
							:class="
								!isLive
									? 'text-slate-400'
									: Number(tpsFormatted) >= 19
										? 'text-emerald-400'
										: Number(tpsFormatted) >= 15
											? 'text-amber-300'
											: 'text-rose-400'
							"
						>
							{{ isLive ? tpsFormatted : 'N/A' }}
						</span>
						<span v-if="isLive" class="text-[10px] text-slate-400 font-mono"
							>({{ msptFormatted }})</span
						>
						<span v-else class="text-[10px] text-slate-500 font-mono">Offline</span>
					</div>
				</div>
			</div>

			<!-- Memory (JVM RSS) -->
			<div class="flex items-center gap-2.5">
				<div
					class="w-7 h-7 rounded-xl bg-sky-500/15 border border-sky-500/25 flex items-center justify-center text-sky-400"
				>
					<GaugeIcon class="w-3.5 h-3.5" />
				</div>
				<div class="flex flex-col">
					<span class="text-[10px] font-bold font-lemon-milk text-slate-400 uppercase tracking-wider"
						>Server RAM</span
					>
					<div class="flex items-baseline gap-1">
						<span class="text-base font-bold text-white/95 font-mono">
							{{ isLive ? ramUsedMb : 0 }}
						</span>
						<span class="text-[10px] text-slate-400 font-mono">/ {{ ramMaxMb }}MB</span>
					</div>
				</div>
			</div>

			<!-- CPU Usage -->
			<div class="flex items-center gap-2.5">
				<div
					class="w-7 h-7 rounded-xl bg-purple-500/15 border border-purple-500/25 flex items-center justify-center text-purple-400"
				>
					<CpuIcon class="w-3.5 h-3.5" />
				</div>
				<div class="flex flex-col">
					<span class="text-[10px] font-bold font-lemon-milk text-slate-400 uppercase tracking-wider"
						>CPU Load</span
					>
					<div class="flex items-baseline gap-1">
						<span class="text-base font-bold text-white/95 font-mono">
							{{ isLive ? cpuPercentFormatted : 0 }}%
						</span>
						<span class="text-[10px] text-slate-400">Core</span>
					</div>
				</div>
			</div>

			<!-- Online Players -->
			<div class="flex items-center gap-2.5">
				<div
					class="w-7 h-7 rounded-xl bg-amber-500/15 border border-amber-500/25 flex items-center justify-center text-amber-400"
				>
					<UsersIcon class="w-3.5 h-3.5" />
				</div>
				<div class="flex flex-col">
					<span class="text-[10px] font-medium text-slate-400 uppercase tracking-wider"
						>Players</span
					>
					<div class="flex items-baseline gap-1">
						<span class="text-base font-bold text-white/95 font-mono">
							{{ isLive ? onlinePlayersCount : 0 }}
						</span>
						<span class="text-[10px] text-slate-400 font-mono">/ {{ maxPlayersCount }}</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Hide HUD Button -->
		<button
			class="w-7 h-7 rounded-xl bg-white/5 hover:bg-white/15 text-slate-400 hover:text-white flex items-center justify-center transition-all cursor-pointer border-none shadow-sm ml-2"
			title="Hide HUD"
			@click="overlayStore.showTelemetryWidget = false"
		>
			<XIcon class="w-3.5 h-3.5" />
		</button>
	</div>
</template>
