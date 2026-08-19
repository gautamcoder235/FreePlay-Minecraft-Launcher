<script setup lang="ts">
import { ImportIcon, PlusIcon } from '@freeplay/assets'
import { Button, defineMessages, IntlFormatted, useVIntl } from '@freeplay/ui'
import { inject, onMounted, onUnmounted, ref } from 'vue'

import freeplaySocialIcon from '../../assets/welcome/freeplay-social-icon.png'

const showCreationModal = inject<() => void>('showCreationModal')
const showImportModal = inject<() => void>('showImportModal')

const { formatMessage } = useVIntl()

const messages = defineMessages({
	welcomeTitle: {
		id: 'app.welcome-screen.title',
		defaultMessage: 'Welcome to FreePlay',
	},
	welcomeDescription: {
		id: 'app.welcome-screen.description',
		defaultMessage: 'Ready to start playing?',
	},
	createInstance: {
		id: 'app.welcome-screen.create-instance',
		defaultMessage: 'Create an instance',
	},
	quickCreateHint: {
		id: 'app.welcome-screen.quick-create-hint',
		defaultMessage: 'Press <shortcut>N</shortcut> to quick create an instance',
	},
	importPrompt: {
		id: 'app.welcome-screen.import-prompt',
		defaultMessage: 'Escaping another launcher?',
	},
	importFromLauncher: {
		id: 'app.welcome-screen.import-from-launcher',
		defaultMessage: 'Import from launcher',
	},
})

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
	<div class="flex flex-col min-h-full px-6 pb-8 pt-8 select-none relative overflow-hidden bg-zinc-950">
		<!-- Background Cyber Gradient Mesh -->
		<div class="absolute inset-0 pointer-events-none -z-10 overflow-hidden">
			<div class="absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[700px] h-[450px] bg-gradient-to-tr from-indigo-600/20 via-purple-600/20 to-cyan-500/20 rounded-full blur-3xl opacity-80 animate-pulse"></div>
			<div class="absolute bottom-10 right-1/4 w-80 h-80 bg-cyan-500/10 rounded-full blur-3xl"></div>
			<div class="absolute top-10 left-10 w-72 h-72 bg-emerald-500/10 rounded-full blur-3xl"></div>
		</div>

		<div class="relative flex grow items-center justify-center py-6">
			<div class="relative isolate flex flex-col items-center gap-6 max-w-2xl text-center bg-zinc-950/70 backdrop-blur-xl border border-white/10 p-8 sm:p-10 rounded-3xl shadow-2xl">
				<!-- FreePlay 3D Hero Badge -->
				<div class="relative group cursor-pointer">
					<div class="absolute -inset-1 rounded-3xl bg-gradient-to-r from-indigo-500 via-purple-500 to-cyan-400 opacity-70 blur-xl group-hover:opacity-100 transition duration-500"></div>
					<div class="relative size-24 rounded-3xl bg-zinc-900 border border-white/20 flex items-center justify-center shadow-2xl backdrop-blur-xl">
						<div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-indigo-600 via-purple-600 to-cyan-500 flex items-center justify-center shadow-[0_0_30px_rgba(99,102,241,0.7)]">
							<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" class="w-8 h-8">
								<path d="M12 2L2 9L12 22L22 9L12 2Z" fill="url(#hero-diamond-grad)" fill-opacity="0.95" />
								<path d="M12 2L2 9H22L12 2Z" fill="white" fill-opacity="0.35" />
								<path d="M12 2L7 9L12 22L17 9L12 2Z" fill="white" fill-opacity="0.25" />
								<defs>
									<linearGradient id="hero-diamond-grad" x1="2" y1="2" x2="22" y2="22" gradientUnits="userSpaceOnUse">
										<stop stop-color="#818CF8"/>
										<stop offset="0.5" stop-color="#C084FC"/>
										<stop offset="1" stop-color="#22D3EE"/>
									</linearGradient>
								</defs>
							</svg>
						</div>
					</div>
				</div>

				<!-- Titles & Subtitles -->
				<div class="flex flex-col items-center gap-3">
					<div class="inline-flex items-center gap-2 px-3.5 py-1 rounded-full bg-zinc-900/90 border border-cyan-500/30 text-xs font-bold text-cyan-300 shadow-[0_0_15px_rgba(34,211,238,0.2)]">
						<span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse"></span>
						Next-Gen Minecraft Launcher
					</div>

					<h1 class="m-0 text-3xl sm:text-4xl font-black tracking-tight text-white">
						Welcome to <span class="bg-gradient-to-r from-indigo-400 via-purple-300 to-cyan-300 bg-clip-text text-transparent">FreePlay</span>
					</h1>
					<p class="m-0 text-center text-sm sm:text-base leading-relaxed text-zinc-400 max-w-lg">
						Experience blazing fast modpack loading, instant one-click server hosting with zero port-forwarding, and next-level customization.
					</p>
				</div>

				<!-- Feature Badges with Inline SVGs -->
				<div class="flex flex-wrap items-center justify-center gap-3 my-2">
					<div class="flex items-center gap-2 px-3.5 py-1.5 rounded-xl bg-zinc-900/90 border border-emerald-500/30 text-xs font-semibold text-zinc-200 shadow-[0_0_12px_rgba(16,185,129,0.15)]">
						<svg class="w-4 h-4 text-emerald-400 fill-current" viewBox="0 0 24 24"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
						Ultra-Fast Engine
					</div>
					<div class="flex items-center gap-2 px-3.5 py-1.5 rounded-xl bg-zinc-900/90 border border-indigo-500/30 text-xs font-semibold text-zinc-200 shadow-[0_0_12px_rgba(99,102,241,0.15)]">
						<svg class="w-4 h-4 text-indigo-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
						Built-in Server Hosting
					</div>
					<div class="flex items-center gap-2 px-3.5 py-1.5 rounded-xl bg-zinc-900/90 border border-cyan-500/30 text-xs font-semibold text-zinc-200 shadow-[0_0_12px_rgba(34,211,238,0.15)]">
						<svg class="w-4 h-4 text-cyan-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="13.5" cy="6.5" r=".5" fill="currentColor"/><circle cx="17.5" cy="10.5" r=".5" fill="currentColor"/><circle cx="8.5" cy="7.5" r=".5" fill="currentColor"/><circle cx="6.5" cy="12.5" r=".5" fill="currentColor"/><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.92 0 1.7-.72 1.7-1.65 0-.41-.15-.81-.44-1.12-.29-.31-.44-.72-.44-1.16 0-.93.75-1.67 1.68-1.67h2.2C19.55 16.4 22 13.95 22 10.9 22 6 17.5 2 12 2z"/></svg>
						Full Mod & Shader Support
					</div>
				</div>

				<!-- High-Contrast Main Action Buttons -->
				<div class="flex flex-col sm:flex-row items-center gap-3.5 w-full sm:w-auto">
					<button
						class="inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl bg-emerald-600 hover:bg-emerald-500 active:scale-[0.98] text-white font-bold text-sm shadow-lg shadow-emerald-950/60 cursor-pointer transition-all duration-200 border border-emerald-400/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-400 w-full sm:w-auto"
						:disabled="offline"
						@click="showCreationModal?.()"
					>
						<PlusIcon class="w-5 h-5" />
						{{ formatMessage(messages.createInstance) }}
					</button>

					<router-link
						to="/hosting/manage"
						class="inline-flex items-center justify-center gap-2.5 px-6 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 active:scale-[0.98] text-white font-bold text-sm shadow-lg shadow-indigo-950/60 cursor-pointer transition-all duration-200 border border-indigo-400/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400 w-full sm:w-auto"
					>
						<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-indigo-200" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<rect width="20" height="8" x="2" y="2" rx="2" ry="2"/>
							<rect width="20" height="8" x="2" y="14" rx="2" ry="2"/>
							<line x1="6" x2="6.01" y1="6" y2="6"/>
							<line x1="6" x2="6.01" y1="18" y2="18"/>
						</svg>
						Server Hosting
					</router-link>
				</div>

				<!-- Quick Shortcut Hint -->
				<span class="flex items-center gap-1 text-xs text-zinc-400">
					<IntlFormatted :message-id="messages.quickCreateHint">
						<template #shortcut="{ children }">
							<kbd
								class="inline-flex h-5 min-w-5 items-center justify-center rounded-md border border-white/20 bg-zinc-800 px-1.5 text-xs font-mono font-bold text-white shadow"
							>
								<component :is="() => children" />
							</kbd>
						</template>
					</IntlFormatted>
				</span>
			</div>
		</div>

		<!-- Footer Import Prompt -->
		<div class="flex flex-col sm:flex-row items-center justify-center gap-3 text-sm text-zinc-400 pt-4 border-t border-white/10">
			<span>{{ formatMessage(messages.importPrompt) }}</span>
			<Button size="md" class="!font-semibold !px-4 cursor-pointer hover:!bg-white/10 transition-colors" :disabled="offline" @click="showImportModal?.()">
				<ImportIcon />
				{{ formatMessage(messages.importFromLauncher) }}
			</Button>
		</div>
	</div>
</template>
