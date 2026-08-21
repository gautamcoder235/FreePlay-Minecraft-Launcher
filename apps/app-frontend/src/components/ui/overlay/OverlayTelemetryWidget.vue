<script setup lang="ts">
import { GaugeIcon, SparklesIcon } from '@freeplay/assets'
import { onMounted, onUnmounted } from 'vue'

import { useOverlayStore } from '@/store/overlay'

const overlayStore = useOverlayStore()

let telemetryInterval: ReturnType<typeof setInterval> | null = null

onMounted(() => {
	telemetryInterval = setInterval(() => {
		overlayStore.updateMockTelemetry()
	}, 1500)
})

onUnmounted(() => {
	if (telemetryInterval) clearInterval(telemetryInterval)
})
</script>

<template>
	<div class="grid grid-cols-2 md:grid-cols-4 gap-3.5">
		<!-- FPS Tile -->
		<div
			class="p-3.5 rounded-2xl bg-surface-2/80 border border-surface-4/70 backdrop-blur-md flex flex-col justify-between shadow-sm"
		>
			<div class="flex items-center justify-between">
				<span class="text-xs font-semibold text-secondary uppercase tracking-wider">Framerate</span>
				<span class="inline-block w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
			</div>
			<div class="mt-2 flex items-baseline gap-1.5">
				<span class="text-2xl font-black text-contrast font-mono tracking-tight">{{
					overlayStore.systemStats.fps
				}}</span>
				<span class="text-xs text-secondary font-medium">FPS</span>
			</div>
			<div class="mt-2 w-full bg-surface-4 rounded-full h-1.5 overflow-hidden">
				<div
					class="bg-brand h-full rounded-full transition-all duration-500"
					:style="{ width: `${Math.min(100, (overlayStore.systemStats.fps / 144) * 100)}%` }"
				></div>
			</div>
		</div>

		<!-- RAM Tile -->
		<div
			class="p-3.5 rounded-2xl bg-surface-2/80 border border-surface-4/70 backdrop-blur-md flex flex-col justify-between shadow-sm"
		>
			<div class="flex items-center justify-between">
				<span class="text-xs font-semibold text-secondary uppercase tracking-wider"
					>Memory (JVM)</span
				>
				<GaugeIcon class="w-3.5 h-3.5 text-secondary" />
			</div>
			<div class="mt-2 flex items-baseline gap-1.5">
				<span class="text-2xl font-black text-contrast font-mono tracking-tight">{{
					overlayStore.systemStats.ramUsedMb
				}}</span>
				<span class="text-xs text-secondary font-medium"
					>/ {{ overlayStore.systemStats.ramTotalMb }} MB</span
				>
			</div>
			<div class="mt-2 w-full bg-surface-4 rounded-full h-1.5 overflow-hidden">
				<div
					class="bg-cyan-500 h-full rounded-full transition-all duration-500"
					:style="{
						width: `${(overlayStore.systemStats.ramUsedMb / overlayStore.systemStats.ramTotalMb) * 100}%`,
					}"
				></div>
			</div>
		</div>

		<!-- CPU Tile -->
		<div
			class="p-3.5 rounded-2xl bg-surface-2/80 border border-surface-4/70 backdrop-blur-md flex flex-col justify-between shadow-sm"
		>
			<div class="flex items-center justify-between">
				<span class="text-xs font-semibold text-secondary uppercase tracking-wider">CPU Usage</span>
				<span class="text-[10px] px-1.5 py-0.5 rounded bg-surface-4 font-mono text-secondary"
					>Host</span
				>
			</div>
			<div class="mt-2 flex items-baseline gap-1.5">
				<span class="text-2xl font-black text-contrast font-mono tracking-tight"
					>{{ overlayStore.systemStats.cpuPercent }}%</span
				>
				<span class="text-xs text-secondary font-medium">Core load</span>
			</div>
			<div class="mt-2 w-full bg-surface-4 rounded-full h-1.5 overflow-hidden">
				<div
					class="bg-purple-500 h-full rounded-full transition-all duration-500"
					:style="{ width: `${overlayStore.systemStats.cpuPercent}%` }"
				></div>
			</div>
		</div>

		<!-- Latency Tile -->
		<div
			class="p-3.5 rounded-2xl bg-surface-2/80 border border-surface-4/70 backdrop-blur-md flex flex-col justify-between shadow-sm"
		>
			<div class="flex items-center justify-between">
				<span class="text-xs font-semibold text-secondary uppercase tracking-wider"
					>Server Ping</span
				>
				<SparklesIcon class="w-3.5 h-3.5 text-amber-400" />
			</div>
			<div class="mt-2 flex items-baseline gap-1.5">
				<span class="text-2xl font-black text-contrast font-mono tracking-tight">{{
					overlayStore.systemStats.pingMs
				}}</span>
				<span class="text-xs text-secondary font-medium">ms latency</span>
			</div>
			<div class="mt-2 w-full bg-surface-4 rounded-full h-1.5 overflow-hidden">
				<div
					class="bg-emerald-400 h-full rounded-full transition-all duration-500"
					:style="{ width: `${Math.max(10, 100 - overlayStore.systemStats.pingMs)}%` }"
				></div>
			</div>
		</div>
	</div>
</template>
