<script setup lang="ts">
import { CompassIcon, ImportIcon, PlusIcon, ServerStackIcon } from '@freeplay/assets'
import { inject, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const showCreationModal = inject<() => void>('showCreationModal')
const showImportModal = inject<() => void>('showImportModal')

const offline = ref(!navigator.onLine)

function handleOffline() {
	offline.value = true
}

function handleOnline() {
	offline.value = false
}

function handleQuickCreate(event: KeyboardEvent) {
	const target = event.target as HTMLElement | null
	if (
		event.key.toLowerCase() !== 'n' ||
		event.repeat ||
		event.metaKey ||
		event.ctrlKey ||
		event.altKey ||
		target?.isContentEditable ||
		['INPUT', 'TEXTAREA', 'SELECT'].includes(target?.tagName ?? '')
	) {
		return
	}

	if (!offline.value) {
		event.preventDefault()
		showCreationModal?.()
	}
}

onMounted(() => {
	window.addEventListener('offline', handleOffline)
	window.addEventListener('online', handleOnline)
	window.addEventListener('keydown', handleQuickCreate)
})

onUnmounted(() => {
	window.removeEventListener('offline', handleOffline)
	window.removeEventListener('online', handleOnline)
	window.removeEventListener('keydown', handleQuickCreate)
})
</script>

<template>
	<div
		class="flex flex-col min-h-full px-6 py-8 select-none relative overflow-y-auto bg-[var(--surface-1)]"
	>
		<!-- Ambient Background Cyber Glows -->
		<div class="absolute inset-0 pointer-events-none -z-10 overflow-hidden">
			<div
				class="absolute top-1/6 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[350px] bg-gradient-to-tr from-[var(--color-brand)]/20 via-[var(--color-brand-highlight)]/15 to-transparent rounded-full blur-[100px] opacity-70"
			></div>
			<div
				class="absolute bottom-10 right-1/4 w-72 h-72 bg-[var(--color-brand)]/10 rounded-full blur-[90px]"
			></div>
		</div>

		<div
			class="relative flex grow flex-col items-center justify-center max-w-4xl mx-auto w-full gap-8"
		>
			<!-- TOP HERO SECTION: 3D DIAMOND JEWEL GEM & TITLE -->
			<div class="flex flex-col items-center text-center gap-4">
				<!-- 3D Diamond Jewel Gem Container -->
				<div class="relative group cursor-pointer" @click="showCreationModal?.()">
					<div
						class="absolute -inset-2 rounded-3xl bg-[var(--color-brand-gradient)] opacity-60 blur-xl group-hover:opacity-90 transition duration-500"
					></div>
					<div
						class="relative w-24 h-24 rounded-3xl bg-[var(--surface-2)] border border-[var(--border-default)] p-2 flex items-center justify-center shadow-2xl backdrop-blur-xl group-hover:scale-105 transition-transform duration-300"
					>
						<svg
							viewBox="0 0 100 100"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
							class="w-full h-full"
						>
							<defs>
								<linearGradient id="fp-welcome-top" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="#a5f3fc" />
									<stop offset="100%" stop-color="var(--color-brand-highlight)" />
								</linearGradient>
								<linearGradient id="fp-welcome-left" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="var(--color-brand-highlight)" />
									<stop offset="100%" stop-color="var(--color-brand)" />
								</linearGradient>
								<linearGradient id="fp-welcome-right" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="var(--color-brand-highlight)" />
									<stop offset="100%" stop-color="var(--color-brand)" />
								</linearGradient>
								<linearGradient id="fp-welcome-front-left" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="var(--color-brand)" />
									<stop offset="100%" stop-color="var(--color-brand)" />
								</linearGradient>
								<linearGradient id="fp-welcome-front-right" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="var(--color-brand)" />
									<stop offset="100%" stop-color="var(--color-brand)" />
								</linearGradient>
							</defs>
							<g transform="translate(12, 12)">
								<polygon
									points="38,4 58,22 38,34 18,22"
									fill="url(#fp-welcome-top)"
									stroke="rgba(255,255,255,0.4)"
									stroke-width="1"
								/>
								<polygon
									points="18,22 38,34 38,62 18,50"
									fill="url(#fp-welcome-left)"
									stroke="rgba(255,255,255,0.2)"
									stroke-width="1"
								/>
								<polygon
									points="38,34 58,22 58,50 38,62"
									fill="url(#fp-welcome-right)"
									stroke="rgba(255,255,255,0.2)"
									stroke-width="1"
								/>
								<polygon
									points="18,50 38,62 38,72 18,60"
									fill="url(#fp-welcome-front-left)"
									opacity="0.9"
								/>
								<polygon
									points="38,62 58,50 58,60 38,72"
									fill="url(#fp-welcome-front-right)"
									opacity="0.9"
								/>
							</g>
						</svg>
					</div>
				</div>

				<!-- Header Titles -->
				<div class="flex flex-col items-center gap-2">
					<div
						class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-[var(--surface-2)] border border-[var(--color-brand-shadow)] text-xs font-bold text-[var(--color-brand-highlight,var(--color-brand))] shadow-[var(--accent-glow)]"
					>
						<span
							class="w-2 h-2 rounded-full bg-[var(--color-brand-highlight,var(--color-brand))] animate-pulse"
						></span>
						Next-Gen High Performance Minecraft Hub
					</div>

					<h1 class="m-0 text-3xl sm:text-4xl font-black tracking-tight text-white">
						Welcome to
						<span class="text-accent-gradient">FreePlay Pro</span>
					</h1>
					<p class="m-0 text-sm sm:text-base text-zinc-400 max-w-lg leading-relaxed">
						Blazing fast modpack launching, zero-port-forward server hosting, and 1-click offline
						player profiles.
					</p>
				</div>
			</div>

			<!-- BENTO GATEWAY GRID (4 TACTICAL TILES) -->
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
				<!-- TILE 1: CREATE NEW INSTANCE (Dynamic Accent) -->
				<div
					class="group relative overflow-hidden rounded-3xl bg-[var(--surface-2)] border border-[var(--border-default)] hover:border-[var(--color-brand-shadow)] p-6 flex flex-col justify-between gap-4 shadow-xl hover:shadow-[var(--accent-glow)] transition-all duration-300 cursor-pointer"
					@click="showCreationModal?.()"
				>
					<div class="flex items-start justify-between">
						<div class="flex items-center gap-3">
							<div
								class="p-3 rounded-2xl bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] text-[var(--color-brand-highlight,var(--color-brand))] group-hover:scale-110 transition-transform shadow-inner"
							>
								<PlusIcon class="w-6 h-6" />
							</div>
							<div class="flex flex-col">
								<h3 class="text-lg font-black text-white m-0">Create Instance</h3>
								<span class="text-xs text-zinc-400">Vanilla, Fabric, NeoForge, Forge, Quilt</span>
							</div>
						</div>
						<span
							class="px-2 py-0.5 rounded-md bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border border-[var(--color-brand-shadow)] font-mono text-[10px] font-bold"
						>
							FAST 1-CLICK
						</span>
					</div>

					<p class="text-xs text-zinc-400 m-0 leading-relaxed">
						Deploy an optimized Minecraft instance with automatic Java version matching and full
						shader support.
					</p>

					<div
						class="flex items-center justify-between pt-2 border-t border-[var(--border-subtle)]"
					>
						<span class="text-xs text-zinc-500 font-mono flex items-center gap-1">
							Press
							<kbd
								class="px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-300 border border-white/10 font-bold"
								>N</kbd
							>
							anywhere
						</span>
						<button
							type="button"
							class="px-4 py-2 rounded-xl btn-accent-primary font-black text-xs flex items-center gap-1.5 cursor-pointer border-none"
						>
							Launchpad &rarr;
						</button>
					</div>
				</div>

				<!-- TILE 2: ZERO-PORT SERVER CONTROL ROOM (Digital Violet / Purple) -->
				<div
					class="group relative overflow-hidden rounded-3xl bg-[var(--surface-2)] border border-[var(--border-default)] hover:border-violet-500/40 p-6 flex flex-col justify-between gap-4 shadow-xl hover:shadow-[0_0_30px_rgba(139,92,246,0.2)] transition-all duration-300 cursor-pointer"
					@click="router.push('/hosting/manage')"
				>
					<div class="flex items-start justify-between">
						<div class="flex items-center gap-3">
							<div
								class="p-3 rounded-2xl bg-violet-500/15 border border-violet-500/30 text-violet-400 group-hover:scale-110 transition-transform shadow-inner"
							>
								<ServerStackIcon class="w-6 h-6" />
							</div>
							<div class="flex flex-col">
								<h3 class="text-lg font-black text-white m-0">Server Control Room</h3>
								<span class="text-xs text-zinc-400"
									>Zero port-forwarding LAN & Anycast tunnels</span
								>
							</div>
						</div>
						<span
							class="px-2 py-0.5 rounded-md bg-violet-500/20 text-violet-300 border border-violet-500/30 font-mono text-[10px] font-bold"
						>
							14ms ROUTE
						</span>
					</div>

					<p class="text-xs text-zinc-400 m-0 leading-relaxed">
						Host multiplayer worlds directly from your PC with a shareable secure invite link for
						friends.
					</p>

					<div
						class="flex items-center justify-between pt-2 border-t border-[var(--border-subtle)]"
					>
						<span class="text-xs text-violet-300 font-mono flex items-center gap-1">
							<span class="w-1.5 h-1.5 rounded-full bg-violet-400 animate-pulse"></span>
							Global Anycast Edge Ready
						</span>
						<button
							type="button"
							class="px-4 py-2 rounded-xl bg-gradient-to-r from-violet-500 to-purple-600 hover:from-violet-400 hover:to-purple-500 text-white font-bold text-xs flex items-center gap-1.5 shadow-md shadow-violet-950/50 transition-all group-hover:shadow-[0_0_15px_rgba(139,92,246,0.5)] cursor-pointer border-none"
						>
							Control Room &rarr;
						</button>
					</div>
				</div>

				<!-- TILE 3: DISCOVER MODPACKS & MODS (Warm Amber / Gold) -->
				<div
					class="group relative overflow-hidden rounded-3xl bg-[var(--surface-2)] border border-[var(--border-default)] hover:border-amber-500/40 p-6 flex flex-col justify-between gap-4 shadow-xl hover:shadow-[0_0_30px_rgba(245,158,11,0.2)] transition-all duration-300 cursor-pointer"
					@click="router.push('/browse/modpack')"
				>
					<div class="flex items-start justify-between">
						<div class="flex items-center gap-3">
							<div
								class="p-3 rounded-2xl bg-amber-500/15 border border-amber-500/30 text-amber-400 group-hover:scale-110 transition-transform shadow-inner"
							>
								<CompassIcon class="w-6 h-6" />
							</div>
							<div class="flex flex-col">
								<h3 class="text-lg font-black text-white m-0">Discover Content</h3>
								<span class="text-xs text-zinc-400">Modpacks, shaders, optimization packs</span>
							</div>
						</div>
						<span
							class="px-2 py-0.5 rounded-md bg-amber-500/20 text-amber-300 border border-amber-500/30 font-mono text-[10px] font-bold"
						>
							OVER 100K+
						</span>
					</div>

					<p class="text-xs text-zinc-400 m-0 leading-relaxed">
						Explore community-crafted content with 1-click automatic dependency resolution.
					</p>

					<div
						class="flex items-center justify-between pt-2 border-t border-[var(--border-subtle)]"
					>
						<span class="text-xs text-zinc-400 font-mono">Fabulously Optimized • Prominence</span>
						<button
							type="button"
							class="px-4 py-2 rounded-xl bg-gradient-to-r from-amber-400 to-orange-500 hover:from-amber-300 hover:to-orange-400 text-zinc-950 font-black text-xs flex items-center gap-1.5 shadow-md shadow-amber-950/50 transition-all group-hover:shadow-[0_0_15px_rgba(245,158,11,0.5)] cursor-pointer border-none"
						>
							Explore &rarr;
						</button>
					</div>
				</div>

				<!-- TILE 4: FAST LAUNCHER IMPORTER (Mint Emerald / Neon Green) -->
				<div
					class="group relative overflow-hidden rounded-3xl bg-[var(--surface-2)] border border-[var(--border-default)] hover:border-emerald-500/40 p-6 flex flex-col justify-between gap-4 shadow-xl hover:shadow-[0_0_30px_rgba(16,185,129,0.2)] transition-all duration-300 cursor-pointer"
					@click="showImportModal?.()"
				>
					<div class="flex items-start justify-between">
						<div class="flex items-center gap-3">
							<div
								class="p-3 rounded-2xl bg-emerald-500/15 border border-emerald-500/30 text-emerald-400 group-hover:scale-110 transition-transform shadow-inner"
							>
								<ImportIcon class="w-6 h-6" />
							</div>
							<div class="flex flex-col">
								<h3 class="text-lg font-black text-white m-0">Import Launcher Data</h3>
								<span class="text-xs text-zinc-400">Prism, CurseForge, GDLauncher, Modrinth</span>
							</div>
						</div>
						<span
							class="px-2 py-0.5 rounded-md bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 font-mono text-[10px] font-bold"
						>
							ZERO LOSS
						</span>
					</div>

					<p class="text-xs text-zinc-400 m-0 leading-relaxed">
						Seamlessly migrate your existing instances, world saves, screenshots, and custom
						resource packs.
					</p>

					<div
						class="flex items-center justify-between pt-2 border-t border-[var(--border-subtle)]"
					>
						<span class="text-xs text-zinc-400 font-mono">100% Automatic Detection</span>
						<button
							type="button"
							class="px-4 py-2 rounded-xl bg-gradient-to-r from-emerald-400 to-teal-500 hover:from-emerald-300 hover:to-teal-400 text-zinc-950 font-black text-xs flex items-center gap-1.5 shadow-md shadow-emerald-950/50 transition-all group-hover:shadow-[0_0_15px_rgba(16,185,129,0.5)] cursor-pointer border-none"
						>
							Import &rarr;
						</button>
					</div>
				</div>
			</div>

			<!-- BOTTOM TACTICAL KEYBOARD HUD -->
			<div
				class="flex flex-wrap items-center justify-center gap-6 py-2 px-6 rounded-2xl bg-[var(--surface-2)]/60 border border-[var(--border-subtle)] text-xs text-zinc-400 font-mono"
			>
				<span class="flex items-center gap-1.5">
					<kbd
						class="px-2 py-0.5 rounded bg-sky-500/15 text-sky-300 border border-sky-500/30 font-bold"
						>N</kbd
					>
					New Instance
				</span>
				<span class="flex items-center gap-1.5">
					<kbd
						class="px-2 py-0.5 rounded bg-violet-500/15 text-violet-300 border border-violet-500/30 font-bold"
						>⌘K</kbd
					>
					Universal Search
				</span>
				<span class="flex items-center gap-1.5">
					<kbd
						class="px-2 py-0.5 rounded bg-amber-500/15 text-amber-300 border border-amber-500/30 font-bold"
						>Ctrl+B</kbd
					>
					Toggle Right HUD
				</span>
			</div>
		</div>
	</div>
</template>
