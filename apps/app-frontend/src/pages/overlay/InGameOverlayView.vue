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
	UsersIcon,
	XIcon,
} from '@freeplay/assets'
import { Button, StyledInput } from '@freeplay/ui'
import { onMounted, onUnmounted, reactive, ref } from 'vue'

import OverlayPlayerRosterWidget from '@/components/ui/overlay/OverlayPlayerRosterWidget.vue'
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
// Normalized Multi-Resolution Layout Persistence & Drag Engine
// ------------------------------------------------------------------
interface NormalizedPosition {
	xRatio: number
	yRatio: number
	z: number
}

interface PixelPosition {
	x: number
	y: number
	z: number
}

const STORAGE_KEY = 'freeplay_overlay_layout_ratios'

const defaultRatios: Record<string, NormalizedPosition> = {
	topPill: { xRatio: 0.28, yRatio: 0.02, z: 20 },
	servers: { xRatio: 0.03, yRatio: 0.1, z: 10 },
	players: { xRatio: 0.38, yRatio: 0.1, z: 10 },
	addons: { xRatio: 0.68, yRatio: 0.1, z: 10 },
	settings: { xRatio: 0.35, yRatio: 0.2, z: 15 },
	telemetry: { xRatio: 0.25, yRatio: 0.82, z: 10 },
}

const positions = reactive<Record<string, PixelPosition>>({
	topPill: { x: 0, y: 20, z: 20 },
	servers: { x: 40, y: 100, z: 10 },
	players: { x: 550, y: 100, z: 10 },
	addons: { x: 990, y: 100, z: 10 },
	settings: { x: 400, y: 180, z: 15 },
	telemetry: { x: 300, y: 750, z: 10 },
})

function loadSavedLayout() {
	if (typeof window === 'undefined') return
	const vw = window.innerWidth
	const vh = window.innerHeight

	let saved: Record<string, NormalizedPosition> | null = null
	try {
		const raw = localStorage.getItem(STORAGE_KEY)
		if (raw) saved = JSON.parse(raw)
	} catch {
		// fallback to defaults
	}

	const ratios = saved || defaultRatios
	for (const key of Object.keys(positions)) {
		const ratio = ratios[key] || defaultRatios[key]
		if (ratio) {
			positions[key].x = Math.max(8, Math.min(vw - 120, Math.round(ratio.xRatio * vw)))
			positions[key].y = Math.max(8, Math.min(vh - 60, Math.round(ratio.yRatio * vh)))
			positions[key].z = ratio.z || 10
		}
	}
}

function saveCurrentLayout() {
	if (typeof window === 'undefined') return
	const vw = window.innerWidth
	const vh = window.innerHeight

	const ratiosToSave: Record<string, NormalizedPosition> = {}
	for (const [key, pos] of Object.entries(positions)) {
		ratiosToSave[key] = {
			xRatio: Math.max(0, Math.min(0.95, pos.x / vw)),
			yRatio: Math.max(0, Math.min(0.95, pos.y / vh)),
			z: pos.z,
		}
	}

	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(ratiosToSave))
	} catch {
		// ignore
	}
}

const isDragging = ref(false)
let currentDragKey: string | null = null
let dragStartX = 0
let dragStartY = 0
let initialPosX = 0
let initialPosY = 0
let highestZ = 30

function startDrag(e: PointerEvent, key: string) {
	if (e.button !== 0) return
	isDragging.value = true
	currentDragKey = key
	dragStartX = e.clientX
	dragStartY = e.clientY
	initialPosX = positions[key].x
	initialPosY = positions[key].y
	positions[key].z = ++highestZ

	window.addEventListener('pointermove', onPointerMove, { passive: true })
	window.addEventListener('pointerup', onPointerUp)
	window.addEventListener('pointercancel', onPointerUp)
}

function onPointerMove(e: PointerEvent) {
	if (!isDragging.value || !currentDragKey || !positions[currentDragKey]) return

	const dx = e.clientX - dragStartX
	const dy = e.clientY - dragStartY
	const vw = window.innerWidth
	const vh = window.innerHeight

	let targetX = initialPosX + dx
	let targetY = initialPosY + dy

	// Smooth Magnetic Screen Edge Snapping
	if (targetX < 16) targetX = 16
	else if (targetX > vw - 300) targetX = vw - 280

	if (targetY < 16) targetY = 16
	else if (targetY > vh - 60) targetY = vh - 60

	positions[currentDragKey].x = Math.max(8, Math.min(vw - 80, targetX))
	positions[currentDragKey].y = Math.max(8, Math.min(vh - 40, targetY))
}

function onPointerUp(_e?: PointerEvent) {
	isDragging.value = false
	currentDragKey = null

	window.removeEventListener('pointermove', onPointerMove)
	window.removeEventListener('pointerup', onPointerUp)
	window.removeEventListener('pointercancel', onPointerUp)

	saveCurrentLayout()
}

// ------------------------------------------------------------------
// Quick Addons Mock Listing
// ------------------------------------------------------------------
interface QuickAddon {
	id: string
	name: string
	description: string
	icon: string
	downloads: string
	installed: boolean
}

const quickAddons = ref<QuickAddon[]>([
	{
		id: 'sodium',
		name: 'Sodium',
		description: 'Next-gen rendering engine and major FPS booster for modern Minecraft',
		icon: 'https://cdn.modrinth.com/data/AANobbMI/icon.png',
		downloads: '28.4M',
		installed: true,
	},
	{
		id: 'iris',
		name: 'Iris Shaders',
		description: 'Modern shaders mod for Minecraft compatible with Sodium shaders',
		icon: 'https://cdn.modrinth.com/data/YL57xq9U/icon.png',
		downloads: '19.2M',
		installed: true,
	},
	{
		id: 'lithium',
		name: 'Lithium',
		description: 'General-purpose server optimization mod for physics and chunk ticks',
		icon: 'https://cdn.modrinth.com/data/gvQqBUqZ/icon.png',
		downloads: '14.1M',
		installed: false,
	},
	{
		id: 'simple-voice-chat',
		name: 'Simple Voice Chat',
		description: 'Proximity voice chat in Minecraft with spatial audio positioning',
		icon: 'https://cdn.modrinth.com/data/9eGKb6K1/icon.png',
		downloads: '22.8M',
		installed: false,
	},
])

function installAddon(addon: QuickAddon) {
	addon.installed = true
}

function closeOverlay() {
	overlayStore.close()
}

function handleKeydown(e: KeyboardEvent) {
	if (e.key === 'Escape') {
		e.preventDefault()
		closeOverlay()
	} else if (e.key === 'Tab' && e.shiftKey) {
		e.preventDefault()
		closeOverlay()
	}
}

let clockTimer: ReturnType<typeof setInterval> | null = null

onMounted(() => {
	document.documentElement.classList.add('is-overlay-mode')
	if (document.body) {
		document.body.style.background = 'transparent'
		document.body.style.backgroundColor = 'transparent'
	}
	loadSavedLayout()
	overlayStore.init()
	updateTime()
	clockTimer = setInterval(updateTime, 1000)
	window.addEventListener('keydown', handleKeydown)
	window.addEventListener('resize', loadSavedLayout)
})

onUnmounted(() => {
	if (clockTimer) clearInterval(clockTimer)
	window.removeEventListener('keydown', handleKeydown)
	window.removeEventListener('resize', loadSavedLayout)
	window.removeEventListener('pointermove', onPointerMove)
	window.removeEventListener('pointerup', onPointerUp)
	window.removeEventListener('pointercancel', onPointerUp)
})
</script>

<template>
	<div class="fixed inset-0 z-50 bg-transparent text-contrast select-none overflow-hidden">
		<!-- 1. Floating Top Status Pill -->
		<div
			class="fixed top-0 left-0 flex items-center justify-between gap-3 px-4 py-2 rounded-full bg-slate-900/90 backdrop-blur-md border border-white/12 shadow-2xl shadow-black/90 cursor-grab active:cursor-grabbing group/top transition-shadow hover:border-white/20 transform-gpu touch-none"
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
					<div class="w-2 h-2 rounded-full bg-white shadow-xs"></div>
				</div>
				<div class="flex items-center gap-1.5 font-bold font-lemon-milk tracking-wide text-xs text-white">
					<span>FREEPLAY</span>
					<span
						class="text-[10px] text-purple-400 font-semibold px-1.5 py-0.5 rounded-full bg-purple-500/15 border border-purple-500/30"
					>
						HUD
					</span>
				</div>
			</div>

			<div class="w-[1px] h-3.5 bg-white/15 mx-0.5"></div>

			<!-- Instance & Game Status -->
			<div class="flex items-center gap-2 text-xs">
				<span class="font-bold font-lemon-milk tracking-wide text-slate-300">
					{{ overlayStore.activeInstanceName || 'Minecraft Dedicated Session' }}
				</span>
				<div class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></div>
			</div>

			<div class="w-[1px] h-3.5 bg-white/15 mx-0.5"></div>

			<!-- Live Clock -->
			<div class="flex items-center gap-1.5 text-xs text-slate-300 font-mono">
				<span>{{ currentTime }}</span>
			</div>
		</div>

		<!-- 2. Floating Server Control Hub -->
		<div
			v-if="overlayStore.showServerWidget"
			class="fixed top-0 left-0 transform-gpu"
			:style="{
				transform: `translate3d(${positions.serverControl.x}px, ${positions.serverControl.y}px, 0)`,
				zIndex: positions.serverControl.z,
				willChange: isDragging ? 'transform' : 'auto',
			}"
		>
			<OverlayServerControlWidget @drag-start="startDrag($event, 'serverControl')" />
		</div>

		<!-- 3. Floating Player Roster Card -->
		<div
			v-if="overlayStore.showPlayerWidget"
			class="fixed top-0 left-0 transform-gpu"
			:style="{
				transform: `translate3d(${positions.players.x}px, ${positions.players.y}px, 0)`,
				zIndex: positions.players.z,
				willChange: isDragging ? 'transform' : 'auto',
			}"
		>
			<OverlayPlayerRosterWidget @drag-start="startDrag($event, 'players')" />
		</div>

		<!-- 4. Floating Quick Addons Card -->
		<div
			v-if="overlayStore.showAddonsWidget"
			class="fixed top-0 left-0 w-[420px] rounded-[24px] bg-slate-900/90 backdrop-blur-md border border-white/12 p-5 flex flex-col gap-4 shadow-2xl shadow-black/90 select-none transition-shadow duration-200 hover:border-white/20 transform-gpu"
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
						<span class="text-sm font-bold font-lemon-milk tracking-wide text-white/95 truncate">
							Live Addons Installer
						</span>
						<span class="text-[11px] text-slate-400 truncate">
							1-click mods & shader packs browser
						</span>
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

			<!-- Addons Listing -->
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

		<!-- 5. Floating Settings Card -->
		<div
			v-if="overlayStore.showSettingsWidget"
			class="fixed top-0 left-0 w-[400px] rounded-[24px] bg-slate-900/90 backdrop-blur-md border border-white/12 p-5 flex flex-col gap-4 shadow-2xl shadow-black/90 select-none transition-shadow duration-200 hover:border-white/20 transform-gpu"
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
						<span class="text-sm font-bold font-lemon-milk tracking-wide text-white/95 truncate">
							Overlay Settings
						</span>
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
					<span class="text-[11px] text-slate-400"
						>Opaque dark surface for reliable in-game input</span
					>
				</div>
				<span class="text-xs font-semibold text-emerald-400 flex items-center gap-1">
					<CheckIcon class="w-3 h-3" />
					Active
				</span>
			</div>
		</div>

		<!-- 6. Floating Telemetry Bar -->
		<div
			v-if="overlayStore.showTelemetryWidget"
			class="fixed top-0 left-0 transform-gpu"
			:style="{
				transform: `translate3d(${positions.telemetry.x}px, ${positions.telemetry.y}px, 0)`,
				zIndex: positions.telemetry.z,
				willChange: isDragging ? 'transform' : 'auto',
			}"
		>
			<OverlayTelemetryWidget @drag-start="startDrag($event, 'telemetry')" />
		</div>

		<!-- 7. Bottom Floating Dock -->
		<div
			class="fixed bottom-5 left-1/2 -translate-x-1/2 z-40 flex items-center flex-nowrap gap-1.5 p-1.5 rounded-full bg-slate-900/95 backdrop-blur-md border border-white/15 shadow-2xl shadow-black/90 transform-gpu select-none max-w-max shrink-0"
		>
			<!-- Servers Toggle -->
			<button
				class="whitespace-nowrap flex items-center justify-center gap-1.5 px-3.5 py-1.5 rounded-full text-[11px] font-bold font-lemon-milk tracking-wide transition-all cursor-pointer border-none shadow-sm shrink-0 active:scale-95"
				:class="
					overlayStore.showServerWidget
						? 'bg-purple-600 text-white shadow-purple-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('servers')"
			>
				<ServerIcon class="w-3.5 h-3.5 shrink-0" />
				<span>Server</span>
			</button>

			<!-- Players Toggle -->
			<button
				class="whitespace-nowrap flex items-center justify-center gap-1.5 px-3.5 py-1.5 rounded-full text-[11px] font-bold font-lemon-milk tracking-wide transition-all cursor-pointer border-none shadow-sm shrink-0 active:scale-95"
				:class="
					overlayStore.showPlayerWidget
						? 'bg-indigo-600 text-white shadow-indigo-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('players')"
			>
				<UsersIcon class="w-3.5 h-3.5 shrink-0" />
				<span>Players</span>
			</button>

			<!-- Addons Toggle -->
			<button
				class="whitespace-nowrap flex items-center justify-center gap-1.5 px-3.5 py-1.5 rounded-full text-[11px] font-bold font-lemon-milk tracking-wide transition-all cursor-pointer border-none shadow-sm shrink-0 active:scale-95"
				:class="
					overlayStore.showAddonsWidget
						? 'bg-sky-600 text-white shadow-sky-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('addons')"
			>
				<LayersIcon class="w-3.5 h-3.5 shrink-0" />
				<span>Addons</span>
			</button>

			<!-- Telemetry Toggle -->
			<button
				class="whitespace-nowrap flex items-center justify-center gap-1.5 px-3.5 py-1.5 rounded-full text-[11px] font-bold font-lemon-milk tracking-wide transition-all cursor-pointer border-none shadow-sm shrink-0 active:scale-95"
				:class="
					overlayStore.showTelemetryWidget
						? 'bg-emerald-600 text-white shadow-emerald-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('telemetry')"
			>
				<GaugeIcon class="w-3.5 h-3.5 shrink-0" />
				<span>Telemetry</span>
			</button>

			<!-- Settings Toggle -->
			<button
				class="whitespace-nowrap flex items-center justify-center gap-1.5 px-3.5 py-1.5 rounded-full text-[11px] font-bold font-lemon-milk tracking-wide transition-all cursor-pointer border-none shadow-sm shrink-0 active:scale-95"
				:class="
					overlayStore.showSettingsWidget
						? 'bg-amber-600 text-white shadow-amber-600/30'
						: 'bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white'
				"
				@click="overlayStore.toggleWidget('settings')"
			>
				<Settings2Icon class="w-3.5 h-3.5 shrink-0" />
				<span>Settings</span>
			</button>

			<div class="w-px h-4 bg-white/20 mx-1 shrink-0"></div>

			<!-- Return to Game Button -->
			<button
				class="whitespace-nowrap flex items-center justify-center gap-1.5 px-3.5 py-1.5 rounded-full text-[11px] font-bold font-lemon-milk tracking-wide bg-rose-500/20 hover:bg-rose-500 text-rose-300 hover:text-white border border-rose-500/30 transition-all cursor-pointer shadow-sm shrink-0 active:scale-95"
				title="Close overlay and return to Minecraft"
				@click="closeOverlay"
			>
				<XIcon class="w-3.5 h-3.5 shrink-0" />
				<span>Return to Game</span>
			</button>
		</div>
	</div>
</template>
