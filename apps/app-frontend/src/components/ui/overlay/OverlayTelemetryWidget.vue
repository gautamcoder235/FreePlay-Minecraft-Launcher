<script setup lang="ts">
import {
	CpuIcon,
	GaugeIcon,
	GripVerticalIcon,
	SignalIcon,
	SparklesIcon,
	XIcon,
} from '@freeplay/assets'
import { onMounted, onUnmounted } from 'vue'

import { useOverlayStore } from '@/store/overlay'

const overlayStore = useOverlayStore()
const emit = defineEmits<{
	(e: 'drag-start', event: PointerEvent): void
}>()

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
	<div
		class="rounded-[24px] bg-slate-900/90 backdrop-blur-2xl border border-white/12 px-5 py-3.5 flex items-center justify-between gap-6 shadow-2xl shadow-black/90 select-none transform-gpu contain-paint"
	>
		<!-- Drag Handle (Hardware Accelerated Pointer Grab) -->
		<div
			class="flex items-center gap-2 cursor-grab active:cursor-grabbing text-slate-500 hover:text-slate-300 transition-colors pr-2 border-r border-white/10 touch-none"
			title="Click and drag to move HUD"
			@pointerdown.stop.prevent="emit('drag-start', $event)"
		>
			<GripVerticalIcon class="w-4 h-4 shrink-0 pointer-events-none" />
			<span
				class="text-[11px] font-semibold tracking-wider text-slate-400 uppercase font-mono pointer-events-none"
				>HUD</span
			>
		</div>

		<!-- Metrics Deck (8-pt Spacing Grid) -->
		<div class="flex items-center gap-8">
			<!-- Framerate FPS -->
			<div class="flex items-center gap-2.5">
				<div
					class="w-7 h-7 rounded-xl bg-emerald-500/15 border border-emerald-500/25 flex items-center justify-center text-emerald-400"
				>
					<SparklesIcon class="w-3.5 h-3.5" />
				</div>
				<div class="flex flex-col">
					<span class="text-[10px] font-medium text-slate-400 uppercase tracking-wider"
						>Framerate</span
					>
					<div class="flex items-baseline gap-1">
						<span class="text-base font-bold text-white/95 font-mono">{{
							overlayStore.systemStats.fps
						}}</span>
						<span class="text-[10px] text-emerald-400 font-semibold">FPS</span>
					</div>
				</div>
			</div>

			<!-- Memory (JVM) -->
			<div class="flex items-center gap-2.5">
				<div
					class="w-7 h-7 rounded-xl bg-sky-500/15 border border-sky-500/25 flex items-center justify-center text-sky-400"
				>
					<GaugeIcon class="w-3.5 h-3.5" />
				</div>
				<div class="flex flex-col">
					<span class="text-[10px] font-medium text-slate-400 uppercase tracking-wider"
						>Memory (JVM)</span
					>
					<div class="flex items-baseline gap-1">
						<span class="text-base font-bold text-white/95 font-mono">{{
							overlayStore.systemStats.ramUsedMb
						}}</span>
						<span class="text-[10px] text-slate-400 font-mono"
							>/ {{ overlayStore.systemStats.ramTotalMb }}MB</span
						>
					</div>
				</div>
			</div>

			<!-- CPU Load -->
			<div class="flex items-center gap-2.5">
				<div
					class="w-7 h-7 rounded-xl bg-purple-500/15 border border-purple-500/25 flex items-center justify-center text-purple-400"
				>
					<CpuIcon class="w-3.5 h-3.5" />
				</div>
				<div class="flex flex-col">
					<span class="text-[10px] font-medium text-slate-400 uppercase tracking-wider"
						>Core Load</span
					>
					<div class="flex items-baseline gap-1">
						<span class="text-base font-bold text-white/95 font-mono"
							>{{ overlayStore.systemStats.cpuPercent }}%</span
						>
						<span class="text-[10px] text-slate-400">Usage</span>
					</div>
				</div>
			</div>

			<!-- Server Ping / Latency -->
			<div class="flex items-center gap-2.5">
				<div
					class="w-7 h-7 rounded-xl bg-amber-500/15 border border-amber-500/25 flex items-center justify-center text-amber-400"
				>
					<SignalIcon class="w-3.5 h-3.5" />
				</div>
				<div class="flex flex-col">
					<span class="text-[10px] font-medium text-slate-400 uppercase tracking-wider"
						>Latency</span
					>
					<div class="flex items-baseline gap-1">
						<span class="text-base font-bold text-white/95 font-mono">{{
							overlayStore.systemStats.pingMs
						}}</span>
						<span class="text-[10px] text-amber-400 font-semibold">ms</span>
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
