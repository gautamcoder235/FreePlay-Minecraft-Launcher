<script setup lang="ts">
import { ImportIcon, PlusIcon } from '@modrinth/assets'
import { Button, defineMessages, IntlFormatted, useVIntl } from '@modrinth/ui'
import { inject, onMounted, onUnmounted, ref } from 'vue'

import modrinthSocialIcon from '../../assets/welcome/modrinth-social-icon.png'

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
	<div class="flex flex-col min-h-full px-6 pb-8 pt-12 select-none relative overflow-hidden">
		<!-- Background Cyber Gradient Mesh -->
		<div class="absolute inset-0 pointer-events-none -z-10 overflow-hidden">
			<div class="absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[400px] bg-gradient-to-tr from-indigo-600/15 via-purple-600/15 to-cyan-500/15 rounded-full blur-3xl opacity-70"></div>
			<div class="absolute bottom-10 right-1/4 w-72 h-72 bg-cyan-500/10 rounded-full blur-2xl"></div>
		</div>

		<div class="relative flex grow items-center justify-center py-6">
			<div class="relative isolate flex flex-col items-center gap-6 max-w-2xl text-center">
				<!-- FreePlay 3D Hero Badge -->
				<div class="relative group">
					<div class="absolute -inset-1 rounded-3xl bg-gradient-to-r from-indigo-500 via-purple-500 to-cyan-400 opacity-60 blur-lg group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt"></div>
					<div class="relative size-24 rounded-3xl bg-surface-2 border border-white/20 flex items-center justify-center shadow-2xl backdrop-blur-xl">
						<div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-[#6366f1] via-[#8b5cf6] to-[#06b6d4] flex items-center justify-center shadow-[0_0_25px_rgba(99,102,241,0.6)]">
							<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" class="w-8 h-8">
								<path d="M4 6.5C4 5.11929 5.11929 4 6.5 4H17.5C18.8807 4 20 5.11929 20 6.5V11C20 15.4183 16.4183 19 12 19C7.58172 19 4 15.4183 4 11V6.5Z" fill="white" fill-opacity="0.3"/>
								<path d="M8.5 7.5L17 12L8.5 16.5V7.5Z" fill="white"/>
							</svg>
						</div>
					</div>
				</div>

				<!-- Titles & Subtitles -->
				<div class="flex flex-col items-center gap-3">
					<div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-surface-3/80 border border-surface-4 text-xs font-semibold text-cyan-300 shadow-inner">
						<span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse"></span>
						Next-Gen Minecraft Launcher
					</div>

					<h1 class="m-0 text-3xl sm:text-4xl font-black tracking-tight text-contrast">
						Welcome to <span class="bg-gradient-to-r from-indigo-400 via-purple-300 to-cyan-300 bg-clip-text text-transparent">FreePlay</span>
					</h1>
					<p class="m-0 text-center text-sm sm:text-base leading-relaxed text-secondary max-w-lg">
						Experience blazing fast modpack loading, instant one-click server hosting with zero port-forwarding, and next-level customization.
					</p>
				</div>

				<!-- Feature Pills -->
				<div class="flex flex-wrap items-center justify-center gap-2.5 my-1">
					<div class="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-surface-3/60 border border-surface-4 text-xs font-medium text-primary">
						<span class="text-indigo-400">⚡</span> Ultra-Fast Engine
					</div>
					<div class="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-surface-3/60 border border-surface-4 text-xs font-medium text-primary">
						<span class="text-cyan-400">🌐</span> Built-in Server Hosting
					</div>
					<div class="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-surface-3/60 border border-surface-4 text-xs font-medium text-primary">
						<span class="text-purple-400">🎨</span> Full Mod & Shader Support
					</div>
				</div>

				<!-- Main Action Buttons -->
				<div class="flex flex-col sm:flex-row items-center gap-3 w-full sm:w-auto">
					<Button
						type="colored"
						color="brand"
						size="lg"
						class="!shadow-[0_0_25px_rgba(99,102,241,0.4)] !font-bold !px-6 !py-3 w-full sm:w-auto hover:!scale-105 transition-transform"
						:disabled="offline"
						@click="showCreationModal?.()"
					>
						<PlusIcon />
						{{ formatMessage(messages.createInstance) }}
					</Button>

					<router-link
						to="/hosting/manage"
						class="inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl bg-surface-3 hover:bg-surface-4 border border-surface-4 text-contrast font-bold text-sm transition-all w-full sm:w-auto"
					>
						<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-cyan-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<rect width="20" height="8" x="2" y="2" rx="2" ry="2"/>
							<rect width="20" height="8" x="2" y="14" rx="2" ry="2"/>
							<line x1="6" x2="6.01" y1="6" y2="6"/>
							<line x1="6" x2="6.01" y1="18" y2="18"/>
						</svg>
						Server Hosting
					</router-link>
				</div>

				<!-- Quick Shortcut Hint -->
				<span class="flex items-center gap-1 text-xs text-secondary">
					<IntlFormatted :message-id="messages.quickCreateHint">
						<template #shortcut="{ children }">
							<kbd
								class="inline-flex h-5 min-w-5 items-center justify-center rounded-md border border-solid border-surface-5 bg-button-bg px-1.5 text-xs font-mono font-bold text-contrast"
							>
								<component :is="() => children" />
							</kbd>
						</template>
					</IntlFormatted>
				</span>
			</div>
		</div>

		<!-- Footer Import Prompt -->
		<div class="flex flex-col sm:flex-row items-center justify-center gap-3 text-sm text-secondary pt-4 border-t border-surface-4/40">
			<span>{{ formatMessage(messages.importPrompt) }}</span>
			<Button size="md" class="!font-semibold !px-4" :disabled="offline" @click="showImportModal?.()">
				<ImportIcon />
				{{ formatMessage(messages.importFromLauncher) }}
			</Button>
		</div>
	</div>
</template>
