<script setup lang="ts">
import {
	CheckIcon,
	ClockIcon,
	GaugeIcon,
	GripVerticalIcon,
	LayersIcon,
	PlusIcon,
	SearchIcon,
	ServerIcon,
	Settings2Icon,
	SparklesIcon,
	XIcon,
} from '@freeplay/assets'
import { Button, StyledInput } from '@freeplay/ui'
import { onMounted, onUnmounted, reactive, ref } from 'vue'

import OverlayServerControlWidget from '@/components/ui/overlay/OverlayServerControlWidget.vue'
import OverlayTelemetryWidget from '@/components/ui/overlay/OverlayTelemetryWidget.vue'
import { useOverlayStore } from '@/store/overlay'

const overlayStore = useOverlayStore()
const addonSearchQuery = ref('')
const currentTime = ref('')

function updateTime() {
	const now = new Date()
	currentTime.value = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

// ------------------------------------------------------------------
// High-Performance GPU Composited Drag Engine (rAF + Pointer Capture)
// ------------------------------------------------------------------
interface CapsulePosition {
	x: number
	y: number
	z: number
}

const positions = reactive<Record<string, CapsulePosition>>({
	topPill: { x: 0, y: 20, z: 20 },
	servers: { x: 40, y: 100, z: 10 },
	addons: { x: 530, y: 100, z: 10 },
	settings: { x: 280, y: 140, z: 15 },
	telemetry: { x: 0, y: 0, z: 10 },
})

const isDragging = ref(false)
let currentDragKey: string | null = null
let dragStartX = 0
let dragStartY = 0
let initialPosX = 0
let initialPosY = 0
let pendingX = 0
let pendingY = 0
let rafId: number | null = null
let highestZ = 30
let activePointerTarget: HTMLElement | null = null
let activePointerId = -1

function initDefaultPositions() {
	if (typeof window === 'undefined') return
	const vw = window.innerWidth
	const vh = window.innerHeight

	// Top Pill: Centered horizontally at top
	positions.topPill.x = Math.max(16, Math.floor((vw - 640) / 2))
	positions.topPill.y = 20

	// Server Card: Left side
	positions.servers.x = Math.max(24, Math.floor(vw / 2 - 490))
	positions.servers.y = 96

	// Addons Card: Right side
	positions.addons.x = Math.min(vw - 500, Math.floor(vw / 2 + 10))
	positions.addons.y = 96

	// Settings Card: Centered
	positions.settings.x = Math.max(24, Math.floor((vw - 420) / 2))
	positions.settings.y = 140

	// Telemetry Bar: Centered above bottom dock
	positions.telemetry.x = Math.max(16, Math.floor((vw - 680) / 2))
	positions.telemetry.y = Math.max(100, vh - 150)
}

function startDrag(e: PointerEvent, key: string) {
	if (e.button !== 0) return
	isDragging.value = true
	currentDragKey = key
	dragStartX = e.clientX
	dragStartY = e.clientY
	initialPosX = positions[key].x
	initialPosY = positions[key].y
	pendingX = initialPosX
	pendingY = initialPosY
	positions[key].z = ++highestZ

	activePointerId = e.pointerId
	activePointerTarget = (e.currentTarget || e.target) as HTMLElement
	try {
		activePointerTarget?.setPointerCapture(e.pointerId)
	} catch {
		// fallback
	}

	window.addEventListener('pointermove', onPointerMove, { passive: true })
	window.addEventListener('pointerup', onPointerUp)
	window.addEventListener('pointercancel', onPointerUp)
}

function onPointerMove(e: PointerEvent) {
	if (!isDragging.value || !currentDragKey) return

	const dx = e.clientX - dragStartX
	const dy = e.clientY - dragStartY
	const vw = window.innerWidth
	const vh = window.innerHeight

	// Constrain to screen boundary
	pendingX = Math.max(8, Math.min(vw - 120, initialPosX + dx))
	pendingY = Math.max(8, Math.min(vh - 80, initialPosY + dy))

	// Batch rendering via requestAnimationFrame (0 layout reflows, native 144Hz-240Hz frame rate)
	if (rafId === null) {
		rafId = requestAnimationFrame(updateFrame)
	}
}

function updateFrame() {
	rafId = null
	if (currentDragKey && positions[currentDragKey]) {
		positions[currentDragKey].x = pendingX
		positions[currentDragKey].y = pendingY
	}
}

function onPointerUp(_e?: PointerEvent) {
	if (rafId !== null) {
		cancelAnimationFrame(rafId)
		rafId = null
	}
	if (currentDragKey && positions[currentDragKey]) {
		positions[currentDragKey].x = pendingX
		positions[currentDragKey].y = pendingY
	}

	if (activePointerTarget && activePointerId !== -1) {
		try {
			activePointerTarget.releasePointerCapture(activePointerId)
		} catch {
			// ignore
		}
	}

	isDragging.value = false
	currentDragKey = null
	activePointerTarget = null
	activePointerId = -1

	window.removeEventListener('pointermove', onPointerMove)
	window.removeEventListener('pointerup', onPointerUp)
	window.removeEventListener('pointercancel', onPointerUp)
}

// ----------------------------------------------------
// Mock Quick Addons Browser Data
// ----------------------------------------------------
const quickAddons = ref([
	{
		id: 'sodium',
		name: 'Sodium',
		description: 'Modern rendering engine and performance optimization mod.',
		icon: 'https://cdn.modrinth.com/data/AANobbMI/icon.png',
		downloads: '32M',
		installed: true,
	},
	{
		id: 'iris',
		name: 'Iris Shaders',
		description: 'Modern shaders mod compatible with existing OptiFine shader packs.',
		icon: 'https://cdn.modrinth.com/data/YL57xq9U/icon.png',
		downloads: '21M',
		installed: false,
	},
	{
		id: 'lithium',
		name: 'Lithium',
		description: 'Physics, mob AI, and world-gen optimization mod.',
		icon: 'https://cdn.modrinth.com/data/gvQqBUqZ/icon.png',
		downloads: '18M',
		installed: true,
	},
	{
		id: 'complementary-reimagined',
		name: 'Complementary Reimagined',
		description: 'Stunning shader pack designed to preserve Minecraft aesthetic.',
		icon: 'https://cdn.modrinth.com/data/pZ2e09wV/icon.png',
		downloads: '15M',
		installed: false,
	},
])

function installAddon(addon: {
	id: string
	name: string
	description: string
	icon: string
	downloads: string
	installed: boolean
}) {
	addon.installed = true
}

function handleKeydown(e: KeyboardEvent) {
	if (e.key === 'Escape') {
		e.preventDefault()
		overlayStore.close()
	} else if (e.key === 'Tab' && e.shiftKey) {
		e.preventDefault()
		overlayStore.close()
	}
}

let clockTimer: ReturnType<typeof setInterval> | null = null

onMounted(() => {
	document.documentElement.classList.add('is-overlay-mode')
	if (document.body) {
		document.body.style.backgroundColor = 'transparent'
	}
	initDefaultPositions()
	overlayStore.init()
	updateTime()
	clockTimer = setInterval(updateTime, 1000)
	window.addEventListener('keydown', handleKeydown)
	window.addEventListener('resize', initDefaultPositions)
})

onUnmounted(() => {
	if (clockTimer) clearInterval(clockTimer)
	if (rafId !== null) cancelAnimationFrame(rafId)
	window.removeEventListener('keydown', handleKeydown)
	window.removeEventListener('resize', initDefaultPositions)
	window.removeEventListener('pointermove', onPointerMove)
	window.removeEventListener('pointerup', onPointerUp)
	window.removeEventListener('pointercancel', onPointerUp)
})
</script>

<template>
	<div
		class="fixed inset-0 z-50 bg-transparent text-contrast select-none overflow-hidden"
		@click.self="overlayStore.close"
	>
		<!-- 1. Floating Top Status Pill (GPU Hardware Composited) -->
		<div
			class="fixed top-0 left-0 flex items-center justify-between gap-3 px-4 py-2 rounded-full bg-slate-900/90 backdrop-blur-2xl border border-white/12 shadow-2xl shadow-black/90 cursor-grab active:cursor-grabbing group/top transition-shadow hover:border-white/20 transform-gpu contain-paint touch-none"
			:style="{
				transform: `translate3d(${positions.topPill.x}px, ${positions.topPill.y}px, 0)`,
				zIndex: positions.topPill.z,
				willChange: isDragging ? 'transform' : 'auto',
			}"
			title="Click and drag to reposition"
			@pointerdown.stop.prevent="startDrag($event, 'topPill')"
		>
			<div class="flex items-center gap-2 pointer-events-none">
				<GripVerticalIcon
					class="w-3.5 h-3.5 text-slate-500 group-hover/top:text-slate-300 transition-colors"
				/>
				<div
					class="w-6 h-6 rounded-lg bg-gradient-to-tr from-purple-500 to-indigo-500 p-0.5 flex items-center justify-center shadow-sm"
				>
					<div class="w-full h-full bg-slate-950 rounded-[6px] flex items-center justify-center">
						<SparklesIcon class="w-3.5 h-3.5 text-purple-400" />
					</div>
				</div>
				<span class="font-extrabold text-[11px] tracking-wider text-white/95 uppercase"
					>FreePlay HUD</span
				>
			</div>

			<div class="w-px h-3.5 bg-white/15"></div>

			<!-- Instance Title & Playtime -->
			<div class="flex items-center gap-2 text-xs pointer-events-none">
				<span class="font-semibold text-white/90 truncate max-w-[180px]">
					{{ overlayStore.activeInstanceName || 'Minecraft' }}
				</span>
				<span
					class="text-[10px] px-2 py-0.5 rounded-full bg-emerald-500/15 text-emerald-400 font-semibold border border-emerald-500/25"
				>
					Playing ({{ overlayStore.sessionDurationFormatted }})
				</span>
			</div>

			<div class="w-px h-3.5 bg-white/15"></div>

			<!-- Clock -->
			<div
				class="flex items-center gap-1.5 font-mono text-xs font-semibold text-slate-300 pointer-events-none"
			>
				<ClockIcon class="w-3 h-3 text-slate-400" />
				<span>{{ currentTime }}</span>
			</div>

			<div class="w-px h-3.5 bg-white/15"></div>

			<!-- Return Hint & Button -->
			<div
				class="flex items-center gap-1.5 pl-1 cursor-pointer"
				@pointerdown.stop
				@click="overlayStore.close"
			>
				<span class="text-[10px] text-slate-400 font-mono">
					<kbd class="px-1.5 py-0.5 rounded bg-white/10 text-white font-semibold text-[10px]">{{
						overlayStore.hotkey
					}}</kbd>
					or
					<kbd class="px-1.5 py-0.5 rounded bg-white/10 text-white font-semibold text-[10px]"
						>ESC</kbd
					>
				</span>
				<div
					class="w-5 h-5 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center text-slate-300 hover:text-white transition-all ml-1"
				>
					<XIcon class="w-3 h-3" />
				</div>
			</div>
		</div>

		<!-- 2. Floating Server Control Hub (GPU Hardware Composited) -->
		<div
			v-if="overlayStore.showServerWidget"
			class="fixed top-0 left-0 transform-gpu contain-paint"
			:style="{
				transform: `translate3d(${positions.servers.x}px, ${positions.servers.y}px, 0)`,
				zIndex: positions.servers.z,
				willChange: isDragging ? 'transform' : 'auto',
			}"
		>
			<OverlayServerControlWidget @drag-start="startDrag($event, 'servers')" />
		</div>

		<!-- 3. Floating Live Addons Installer Card (GPU Hardware Composited) -->
		<div
			v-if="overlayStore.showAddonsWidget"
			class="fixed top-0 left-0 w-[480px] rounded-[24px] bg-slate-900/90 backdrop-blur-2xl border border-white/12 p-5 flex flex-col gap-4 shadow-2xl shadow-black/90 select-none transition-shadow duration-200 hover:border-white/20 transform-gpu contain-paint"
			:style="{
				transform: `translate3d(${positions.addons.x}px, ${positions.addons.y}px, 0)`,
				zIndex: positions.addons.z,
				willChange: isDragging ? 'transform' : 'auto',
			}"
		>
			<!-- Draggable Header -->
			<div
				class="flex items-center justify-between border-b border-white/10 pb-3 cursor-grab active:cursor-grabbing group/header touch-none"
				title="Click and drag to move"
				@pointerdown.stop.prevent="startDrag($event, 'addons')"
			>
				<div class="flex items-center gap-2.5 min-w-0 pointer-events-none">
					<GripVerticalIcon
						class="w-4 h-4 text-slate-500 group-hover/header:text-slate-300 transition-colors shrink-0"
					/>
					<div
						class="w-8 h-8 rounded-xl bg-sky-500/15 border border-sky-500/25 flex items-center justify-center text-sky-400 shrink-0 shadow-sm"
					>
						<LayersIcon class="w-4 h-4" />
					</div>
					<div class="flex flex-col min-w-0">
						<span class="text-sm font-semibold text-white/95 tracking-tight truncate"
							>Live Addons Installer</span
						>
						<span class="text-[11px] text-slate-400 truncate"
							>1-click mods & shader packs browser</span
						>
					</div>
				</div>

				<button
					class="w-7 h-7 rounded-xl bg-white/5 hover:bg-white/15 text-slate-400 hover:text-white flex items-center justify-center transition-all cursor-pointer border-none shadow-sm shrink-0"
					title="Hide widget"
					@pointerdown.stop
					@click="overlayStore.showAddonsWidget = false"
				>
					<XIcon class="w-3.5 h-3.5" />
				</button>
			</div>

			<!-- Search Input -->
			<StyledInput
				v-model="addonSearchQuery"
				:icon="SearchIcon"
				placeholder="Search mods, shaders, resource packs..."
				autocomplete="off"
			/>

			<!-- Addons Listing (8-pt Rhythm) -->
			<div class="flex flex-col gap-2 max-h-56 overflow-y-auto pr-1">
				<div
					v-for="addon in quickAddons"
					:key="addon.id"
					class="p-3 rounded-xl bg-black/40 border border-white/5 hover:border-sky-500/30 transition-all flex items-center justify-between gap-3 shadow-sm"
				>
					<div class="flex items-center gap-3 min-w-0">
						<img
							:src="addon.icon"
							class="w-9 h-9 rounded-xl bg-surface-4 shrink-0 object-cover border border-white/10"
						/>
						<div class="flex flex-col min-w-0">
							<div class="flex items-center gap-1.5">
								<span class="text-xs font-semibold text-white/90 truncate">{{ addon.name }}</span>
								<span class="text-[10px] text-slate-400 font-mono">{{ addon.downloads }} dl</span>
							</div>
							<span class="text-[11px] text-slate-400 line-clamp-1">{{ addon.description }}</span>
						</div>
					</div>

					<Button
						v-if="!addon.installed"
						type="colored"
						color="brand"
						class="shrink-0 !font-semibold text-xs !py-1 !px-2.5 flex items-center gap-1"
						@click="installAddon(addon)"
					>
						<PlusIcon class="w-3 h-3" />
						Install
					</Button>
					<span
						v-else
						class="shrink-0 text-[10px] font-semibold text-emerald-400 px-2 py-0.5 rounded-lg bg-emerald-500/15 border border-emerald-500/25 flex items-center gap-1"
					>
						<CheckIcon class="w-3 h-3" />
						Installed
					</span>
				</div>
			</div>
		</div>

		<!-- 4. Floating Settings Card (GPU Hardware Composited) -->
		<div
			v-if="overlayStore.showSettingsWidget"
			class="fixed top-0 left-0 w-[400px] rounded-[24px] bg-slate-900/90 backdrop-blur-2xl border border-white/12 p-5 flex flex-col gap-4 shadow-2xl shadow-black/90 select-none transition-shadow duration-200 hover:border-white/20 transform-gpu contain-paint"
			:style="{
				transform: `translate3d(${positions.settings.x}px, ${positions.settings.y}px, 0)`,
				zIndex: positions.settings.z,
				willChange: isDragging ? 'transform' : 'auto',
			}"
		>
			<!-- Draggable Header -->
			<div
				class="flex items-center justify-between border-b border-white/10 pb-3 cursor-grab active:cursor-grabbing group/header touch-none"
				title="Click and drag to move"
				@pointerdown.stop.prevent="startDrag($event, 'settings')"
			>
				<div class="flex items-center gap-2.5 min-w-0 pointer-events-none">
					<GripVerticalIcon
						class="w-4 h-4 text-slate-500 group-hover/header:text-slate-300 transition-colors shrink-0"
					/>
					<div
						class="w-8 h-8 rounded-xl bg-amber-500/15 border border-amber-500/25 flex items-center justify-center text-amber-400 shrink-0 shadow-sm"
					>
						<Settings2Icon class="w-4 h-4" />
					</div>
					<div class="flex flex-col min-w-0">
						<span class="text-sm font-semibold text-white/95 tracking-tight truncate"
							>Overlay Settings</span
						>
						<span class="text-[11px] text-slate-400 truncate">Shortcut & HUD preferences</span>
					</div>
				</div>

				<button
					class="w-7 h-7 rounded-xl bg-white/5 hover:bg-white/15 text-slate-400 hover:text-white flex items-center justify-center transition-all cursor-pointer border-none shadow-sm shrink-0"
					title="Hide widget"
					@pointerdown.stop
					@click="overlayStore.showSettingsWidget = false"
				>
					<XIcon class="w-3.5 h-3.5" />
				</button>
			</div>

			<div
				class="p-3 rounded-xl bg-black/40 border border-white/5 flex items-center justify-between"
			>
				<div class="flex flex-col">
					<span class="text-xs font-semibold text-white/90">Global Toggle Hotkey</span>
					<span class="text-[11px] text-slate-400">Summon HUD anytime in Minecraft</span>
				</div>
				<span
					class="px-2.5 py-1 rounded-lg bg-white/10 border border-white/10 font-mono text-xs font-bold text-purple-400"
				>
					{{ overlayStore.hotkey }}
				</span>
			</div>

			<div
				class="p-3 rounded-xl bg-black/40 border border-white/5 flex items-center justify-between"
			>
				<div class="flex flex-col">
					<span class="text-xs font-semibold text-white/90">Window Background</span>
					<span class="text-[11px] text-slate-400">Transparent with modular glass capsules</span>
				</div>
				<span class="text-xs font-semibold text-emerald-400 flex items-center gap-1">
					<CheckIcon class="w-3 h-3" />
					Active
				</span>
			</div>
		</div>

		<!-- 5. Floating Telemetry Bar (GPU Hardware Composited) -->
		<div
			v-if="overlayStore.showTelemetryWidget"
			class="fixed top-0 left-0 transform-gpu contain-paint"
			:style="{
				transform: `translate3d(${positions.telemetry.x}px, ${positions.telemetry.y}px, 0)`,
				zIndex: positions.telemetry.z,
				willChange: isDragging ? 'transform' : 'auto',
			}"
		>
			<OverlayTelemetryWidget @drag-start="startDrag($event, 'telemetry')" />
		</div>

		<!-- 6. Bottom Floating Dock (Fixed Centered) -->
		<div
			class="fixed bottom-4 left-1/2 -translate-x-1/2 z-40 flex items-center gap-2 p-1.5 rounded-full bg-slate-900/90 backdrop-blur-2xl border border-white/15 shadow-2xl shadow-black/90 transform-gpu"
		>
			<!-- Servers Toggle -->
			<button
				class="flex items-center gap-2 px-4 py-2 rounded-full text-xs font-semibold transition-all cursor-pointer border-none shadow-sm"
				:class="
					overlayStore.showServerWidget
						? 'bg-purple-600 text-white shadow-purple-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('servers')"
			>
				<ServerIcon class="w-3.5 h-3.5" />
				<span>Servers</span>
			</button>

			<!-- Addons Toggle -->
			<button
				class="flex items-center gap-2 px-4 py-2 rounded-full text-xs font-semibold transition-all cursor-pointer border-none shadow-sm"
				:class="
					overlayStore.showAddonsWidget
						? 'bg-sky-600 text-white shadow-sky-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('addons')"
			>
				<LayersIcon class="w-3.5 h-3.5" />
				<span>Live Addons</span>
			</button>

			<!-- Telemetry Toggle -->
			<button
				class="flex items-center gap-2 px-4 py-2 rounded-full text-xs font-semibold transition-all cursor-pointer border-none shadow-sm"
				:class="
					overlayStore.showTelemetryWidget
						? 'bg-emerald-600 text-white shadow-emerald-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('telemetry')"
			>
				<GaugeIcon class="w-3.5 h-3.5" />
				<span>Telemetry</span>
			</button>

			<!-- Settings Toggle -->
			<button
				class="flex items-center gap-2 px-4 py-2 rounded-full text-xs font-semibold transition-all cursor-pointer border-none shadow-sm"
				:class="
					overlayStore.showSettingsWidget
						? 'bg-amber-600 text-white shadow-amber-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('settings')"
			>
				<Settings2Icon class="w-3.5 h-3.5" />
				<span>Settings</span>
			</button>

			<div class="w-px h-5 bg-white/15 mx-1"></div>

			<!-- Return to Game Button -->
			<button
				class="flex items-center gap-1.5 px-4 py-2 rounded-full text-xs font-semibold bg-white/10 hover:bg-rose-500/80 text-slate-300 hover:text-white transition-all cursor-pointer border-none"
				@click="overlayStore.close"
			>
				<XIcon class="w-3.5 h-3.5" />
				<span>Return to Game</span>
			</button>
		</div>
	</div>
</template>
