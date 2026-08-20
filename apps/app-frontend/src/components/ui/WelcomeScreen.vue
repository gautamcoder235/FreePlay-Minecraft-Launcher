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
	<div class="flex flex-col min-h-full px-6 py-8 select-none relative overflow-y-auto bg-[#090b0f]">
		<!-- Ambient Background Cyber Glows -->
		<div class="absolute inset-0 pointer-events-none -z-10 overflow-hidden">
			<div
				class="absolute top-1/6 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[350px] bg-gradient-to-tr from-sky-600/20 via-blue-600/20 to-cyan-500/20 rounded-full blur-[100px] opacity-70"
			></div>
			<div
				class="absolute bottom-10 right-1/4 w-72 h-72 bg-sky-500/10 rounded-full blur-[90px]"
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
						class="absolute -inset-2 rounded-3xl bg-gradient-to-r from-sky-500 via-blue-500 to-cyan-400 opacity-60 blur-xl group-hover:opacity-90 transition duration-500"
					></div>
					<div
						class="relative w-24 h-24 rounded-3xl bg-[#141923] border border-white/20 p-2 flex items-center justify-center shadow-2xl backdrop-blur-xl group-hover:scale-105 transition-transform duration-300"
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
									<stop offset="100%" stop-color="#38bdf8" />
								</linearGradient>
								<linearGradient id="fp-welcome-left" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="#38bdf8" />
									<stop offset="100%" stop-color="#0284c7" />
								</linearGradient>
								<linearGradient id="fp-welcome-right" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="#22d3ee" />
									<stop offset="100%" stop-color="#0891b2" />
								</linearGradient>
								<linearGradient id="fp-welcome-front-left" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="#0ea5e9" />
									<stop offset="100%" stop-color="#0369a1" />
								</linearGradient>
								<linearGradient id="fp-welcome-front-right" x1="0%" y1="0%" x2="100%" y2="100%">
									<stop offset="0%" stop-color="#06b6d4" />
									<stop offset="100%" stop-color="#0e7490" />
								</linearGradient>
							</defs>
							<g transform="translate(12, 12)">
								<polygon
									points="38,4 58,22 38,34 18,22"
									fill="url(#fp-welcome-top)"
									opacity="0.95"
								/>
								<polygon
									points="18,22 38,34 38,54 8,36"
									fill="url(#fp-welcome-left)"
									opacity="0.9"
								/>
								<polygon
									points="58,22 38,34 38,54 68,36"
									fill="url(#fp-welcome-right)"
									opacity="0.95"
								/>
								<polygon
									points="8,36 38,54 38,72"
									fill="url(#fp-welcome-front-left)"
									opacity="0.85"
								/>
								<polygon
									points="68,36 38,54 38,72"
									fill="url(#fp-welcome-front-right)"
									opacity="0.9"
								/>
								<polygon points="38,4 48,22 38,34" fill="#ffffff" opacity="0.45" />
								<polygon points="38,34 38,54 28,32" fill="#ffffff" opacity="0.25" />
								<line
									x1="38"
									y1="4"
									x2="38"
									y2="72"
									stroke="rgba(255,255,255,0.5)"
									stroke-width="1.5"
								/>
							</g>
						</svg>
					</div>
				</div>

				<!-- Header Titles -->
				<div class="flex flex-col items-center gap-2">
					<div
						class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-[#141923] border border-sky-500/30 text-xs font-bold text-sky-300 shadow-[0_0_15px_rgba(56,189,248,0.2)]"
					>
						<span class="w-2 h-2 rounded-full bg-sky-400 animate-pulse"></span>
						Next-Gen High Performance Minecraft Hub
					</div>

					<h1 class="m-0 text-3xl sm:text-4xl font-black tracking-tight text-white">
						Welcome to
						<span
							class="bg-gradient-to-r from-sky-400 via-cyan-300 to-blue-300 bg-clip-text text-transparent"
							>FreePlay Pro</span
						>
					</h1>
					<p class="m-0 text-sm sm:text-base text-zinc-400 max-w-lg leading-relaxed">
						Blazing fast modpack launching, zero-port-forward server hosting, and 1-click offline
						player profiles.
					</p>
				</div>
			</div>

			<!-- BENTO GATEWAY GRID (4 TACTICAL TILES) -->
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
				<!-- TILE 1: CREATE NEW INSTANCE (Sky Blue / Electric Cyan) -->
				<div
					class="group relative overflow-hidden rounded-3xl bg-[#141923] border border-white/10 hover:border-sky-500/40 p-6 flex flex-col justify-between gap-4 shadow-xl hover:shadow-[0_0_30px_rgba(56,189,248,0.2)] transition-all duration-300 cursor-pointer"
					@click="showCreationModal?.()"
				>
					<div class="flex items-start justify-between">
						<div class="flex items-center gap-3">
							<div
								class="p-3 rounded-2xl bg-sky-500/15 border border-sky-500/30 text-sky-400 group-hover:scale-110 transition-transform shadow-inner"
							>
								<PlusIcon class="w-6 h-6" />
							</div>
							<div class="flex flex-col">
								<h3 class="text-lg font-black text-white m-0">Create Instance</h3>
								<span class="text-xs text-zinc-400">Vanilla, Fabric, NeoForge, Forge, Quilt</span>
							</div>
						</div>
						<span
							class="px-2 py-0.5 rounded-md bg-sky-500/20 text-sky-300 border border-sky-500/30 font-mono text-[10px] font-bold"
						>
							FAST 1-CLICK
						</span>
					</div>

					<p class="text-xs text-zinc-400 m-0 leading-relaxed">
						Deploy an optimized Minecraft instance with automatic Java version matching and full
						shader support.
					</p>

					<div class="flex items-center justify-between pt-2 border-t border-white/5">
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
							class="px-4 py-2 rounded-xl bg-gradient-to-r from-sky-400 to-blue-500 hover:from-sky-300 hover:to-blue-400 text-zinc-950 font-black text-xs flex items-center gap-1.5 shadow-md shadow-sky-950/50 transition-all group-hover:shadow-[0_0_15px_rgba(56,189,248,0.5)] cursor-pointer border-none"
						>
							Launchpad &rarr;
						</button>
					</div>
				</div>

				<!-- TILE 2: ZERO-PORT SERVER CONTROL ROOM (Digital Violet / Purple) -->
				<div
					class="group relative overflow-hidden rounded-3xl bg-[#141923] border border-white/10 hover:border-violet-500/40 p-6 flex flex-col justify-between gap-4 shadow-xl hover:shadow-[0_0_30px_rgba(139,92,246,0.2)] transition-all duration-300 cursor-pointer"
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

					<div class="flex items-center justify-between pt-2 border-t border-white/5">
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
					class="group relative overflow-hidden rounded-3xl bg-[#141923] border border-white/10 hover:border-amber-500/40 p-6 flex flex-col justify-between gap-4 shadow-xl hover:shadow-[0_0_30px_rgba(245,158,11,0.2)] transition-all duration-300 cursor-pointer"
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

					<div class="flex items-center justify-between pt-2 border-t border-white/5">
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
					class="group relative overflow-hidden rounded-3xl bg-[#141923] border border-white/10 hover:border-emerald-500/40 p-6 flex flex-col justify-between gap-4 shadow-xl hover:shadow-[0_0_30px_rgba(16,185,129,0.2)] transition-all duration-300 cursor-pointer"
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

					<div class="flex items-center justify-between pt-2 border-t border-white/5">
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
				class="flex flex-wrap items-center justify-center gap-6 py-2 px-6 rounded-2xl bg-[#141923]/60 border border-white/5 text-xs text-zinc-400 font-mono"
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
