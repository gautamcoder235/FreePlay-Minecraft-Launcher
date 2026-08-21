<script setup lang="ts">
import {
	FolderIcon,
	GaugeIcon,
	SearchIcon,
	ServerIcon,
	Settings2Icon,
	SparklesIcon,
	XIcon,
} from '@freeplay/assets'
import { Button, StyledInput } from '@freeplay/ui'
import { onMounted, onUnmounted, ref } from 'vue'

import OverlayServerControlWidget from '@/components/ui/overlay/OverlayServerControlWidget.vue'
import OverlayTelemetryWidget from '@/components/ui/overlay/OverlayTelemetryWidget.vue'
import { useOverlayStore } from '@/store/overlay'

const overlayStore = useOverlayStore()
const activeTab = ref<'servers' | 'addons' | 'telemetry' | 'settings'>('servers')
const addonSearchQuery = ref('')

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

function installAddon(addon: QuickAddon) {
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

onMounted(() => {
	overlayStore.init()
	window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
	window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
	<div
		class="fixed inset-0 z-50 flex flex-col bg-slate-950/80 backdrop-blur-2xl text-base text-contrast select-none overflow-hidden transition-all duration-200"
		@click.self="overlayStore.close"
	>
		<!-- Top Bar -->
		<header
			class="h-16 px-6 border-b border-surface-4/70 bg-surface-1/60 backdrop-blur-md flex items-center justify-between shrink-0"
		>
			<!-- Logo & Status -->
			<div class="flex items-center gap-3.5">
				<div
					class="w-9 h-9 rounded-xl bg-gradient-to-tr from-brand to-emerald-400 p-0.5 shadow-lg shadow-brand/20 flex items-center justify-center"
				>
					<div class="w-full h-full bg-surface-1 rounded-[10px] flex items-center justify-center">
						<SparklesIcon class="w-4 h-4 text-brand" />
					</div>
				</div>
				<div class="flex flex-col">
					<div class="flex items-center gap-2">
						<span class="font-extrabold text-sm tracking-wide text-contrast"
							>FreePlay In-Game HUD</span
						>
						<span
							class="text-[10px] uppercase font-bold tracking-widest px-1.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
						>
							Active
						</span>
					</div>
					<span class="text-xs text-secondary font-medium">
						Playing: {{ overlayStore.activeInstanceName || 'Minecraft Instance' }}
					</span>
				</div>
			</div>

			<!-- Center Tab Navigation -->
			<nav
				class="flex items-center gap-1.5 p-1 rounded-xl bg-surface-2/90 border border-surface-4/80"
			>
				<button
					class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all"
					:class="
						activeTab === 'servers'
							? 'bg-surface-4 text-contrast shadow-sm'
							: 'text-secondary hover:text-contrast hover:bg-surface-3'
					"
					@click="activeTab = 'servers'"
				>
					<ServerIcon class="w-3.5 h-3.5" />
					Servers
				</button>
				<button
					class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all"
					:class="
						activeTab === 'addons'
							? 'bg-surface-4 text-contrast shadow-sm'
							: 'text-secondary hover:text-contrast hover:bg-surface-3'
					"
					@click="activeTab = 'addons'"
				>
					<FolderIcon class="w-3.5 h-3.5" />
					Live Addons
				</button>
				<button
					class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all"
					:class="
						activeTab === 'telemetry'
							? 'bg-surface-4 text-contrast shadow-sm'
							: 'text-secondary hover:text-contrast hover:bg-surface-3'
					"
					@click="activeTab = 'telemetry'"
				>
					<GaugeIcon class="w-3.5 h-3.5" />
					Performance
				</button>
				<button
					class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all"
					:class="
						activeTab === 'settings'
							? 'bg-surface-4 text-contrast shadow-sm'
							: 'text-secondary hover:text-contrast hover:bg-surface-3'
					"
					@click="activeTab = 'settings'"
				>
					<Settings2Icon class="w-3.5 h-3.5" />
					Overlay Settings
				</button>
			</nav>

			<!-- Close Hint & Button -->
			<div class="flex items-center gap-3">
				<span class="text-xs text-secondary font-mono"
					>Press
					<kbd
						class="px-1.5 py-0.5 rounded bg-surface-3 border border-surface-4 text-contrast font-bold"
						>ESC</kbd
					>
					or
					<kbd
						class="px-1.5 py-0.5 rounded bg-surface-3 border border-surface-4 text-contrast font-bold"
						>{{ overlayStore.hotkey }}</kbd
					>
					to return</span
				>
				<button
					class="w-8 h-8 rounded-lg bg-surface-3 hover:bg-surface-4 text-secondary hover:text-contrast flex items-center justify-center transition-all border border-surface-4/60 cursor-pointer"
					@click="overlayStore.close"
				>
					<XIcon class="w-4 h-4" />
				</button>
			</div>
		</header>

		<!-- Main Overlay Content Area -->
		<main class="flex-1 p-6 overflow-y-auto max-w-6xl w-full mx-auto flex flex-col gap-6">
			<!-- Telemetry Bar -->
			<OverlayTelemetryWidget />

			<!-- Tab 1: Server Control Room -->
			<section v-if="activeTab === 'servers'" class="flex flex-col gap-4">
				<OverlayServerControlWidget />
			</section>

			<!-- Tab 2: Live Addons Installer -->
			<section v-else-if="activeTab === 'addons'" class="flex flex-col gap-4">
				<div
					class="rounded-2xl bg-surface-2/80 border border-surface-4/70 p-5 backdrop-blur-md flex flex-col gap-4 shadow-sm"
				>
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col">
							<h3 class="m-0 text-base font-bold text-contrast">In-Game Addon Installer</h3>
							<p class="m-0 text-xs text-secondary mt-0.5">
								Search and 1-click install mods, shaders, and texture packs directly into your
								active playing session.
							</p>
						</div>
						<div class="w-72">
							<StyledInput
								v-model="addonSearchQuery"
								:icon="SearchIcon"
								placeholder="Search mods, shaders, resource packs..."
								autocomplete="off"
							/>
						</div>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-2 gap-3.5 mt-2">
						<div
							v-for="addon in quickAddons"
							:key="addon.id"
							class="p-4 rounded-xl bg-surface-3/80 border border-surface-4 hover:border-brand/40 transition-all flex items-center justify-between gap-4 shadow-sm"
						>
							<div class="flex items-center gap-3.5 min-w-0">
								<img
									:src="addon.icon"
									class="w-10 h-10 rounded-xl bg-surface-4 shrink-0 object-cover"
								/>
								<div class="flex flex-col min-w-0">
									<div class="flex items-center gap-2">
										<span class="text-sm font-bold text-contrast truncate">{{ addon.name }}</span>
										<span class="text-[10px] text-secondary font-mono"
											>{{ addon.downloads }} dl</span
										>
									</div>
									<span class="text-xs text-secondary line-clamp-1 mt-0.5">{{
										addon.description
									}}</span>
								</div>
							</div>

							<Button
								v-if="!addon.installed"
								type="colored"
								color="brand"
								class="shrink-0 !font-bold text-xs"
								@click="installAddon(addon)"
							>
								Install
							</Button>
							<span
								v-else
								class="shrink-0 text-xs font-bold text-emerald-400 px-2.5 py-1 rounded-md bg-emerald-500/10 border border-emerald-500/20"
							>
								Installed
							</span>
						</div>
					</div>
				</div>
			</section>

			<!-- Tab 3: Detailed Telemetry -->
			<section v-else-if="activeTab === 'telemetry'" class="flex flex-col gap-4">
				<div
					class="rounded-2xl bg-surface-2/80 border border-surface-4/70 p-5 backdrop-blur-md flex flex-col gap-4 shadow-sm"
				>
					<h3 class="m-0 text-base font-bold text-contrast">Real-Time Performance Diagnostics</h3>
					<p class="m-0 text-xs text-secondary">
						Diagnostics streamed out-of-process without interfering with Minecraft graphics
						pipelines or shaders.
					</p>

					<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-2">
						<div class="p-4 rounded-xl bg-surface-3/80 border border-surface-4 flex flex-col gap-2">
							<span class="text-xs font-bold text-secondary uppercase">Graphics & Shaders</span>
							<span class="text-sm font-semibold text-contrast">OpenGL Composited Flip Model</span>
							<span class="text-xs text-emerald-400">Zero In-Process Hook Interference</span>
						</div>
						<div class="p-4 rounded-xl bg-surface-3/80 border border-surface-4 flex flex-col gap-2">
							<span class="text-xs font-bold text-secondary uppercase">Process Supervisor</span>
							<span class="text-sm font-semibold text-contrast font-mono"
								>PID: {{ overlayStore.activeGamePid || 'Attached' }}</span
							>
							<span class="text-xs text-emerald-400">Low-overhead Async Log Consumer</span>
						</div>
						<div class="p-4 rounded-xl bg-surface-3/80 border border-surface-4 flex flex-col gap-2">
							<span class="text-xs font-bold text-secondary uppercase">Input Coordinator</span>
							<span class="text-sm font-semibold text-contrast">Win32 Pass-Through Click Lock</span>
							<span class="text-xs text-emerald-400">Instant Cursor Release on Toggle</span>
						</div>
					</div>
				</div>
			</section>

			<!-- Tab 4: Overlay Settings -->
			<section v-else-if="activeTab === 'settings'" class="flex flex-col gap-4">
				<div
					class="rounded-2xl bg-surface-2/80 border border-surface-4/70 p-5 backdrop-blur-md flex flex-col gap-4 shadow-sm"
				>
					<h3 class="m-0 text-base font-bold text-contrast">Overlay & Shortcut Configuration</h3>
					<p class="m-0 text-xs text-secondary">
						Customize keybindings and overlay behavior during gameplay.
					</p>

					<div
						class="flex items-center justify-between p-4 rounded-xl bg-surface-3/80 border border-surface-4"
					>
						<div class="flex flex-col gap-0.5">
							<span class="text-sm font-semibold text-contrast">Overlay Toggle Shortcut</span>
							<span class="text-xs text-secondary"
								>Global shortcut to summon and dismiss in-game overlay.</span
							>
						</div>
						<div
							class="px-3 py-1.5 rounded-lg bg-surface-4 border border-surface-5 font-mono text-sm font-bold text-brand"
						>
							{{ overlayStore.hotkey }}
						</div>
					</div>
				</div>
			</section>
		</main>
	</div>
</template>
