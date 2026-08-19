<script setup lang="ts">
import { HomeIcon, PlusIcon } from '@modrinth/assets'
import { defineMessages, injectNotificationManager, useVIntl } from '@modrinth/ui'
import dayjs from 'dayjs'
import { computed, inject, onActivated, ref } from 'vue'

import ContextMenu from '@/components/ui/ContextMenu.vue'
import LibrarySection from '@/components/ui/library/index.vue'
import WelcomeScreen from '@/components/ui/WelcomeScreen.vue'
import RecentWorldsList from '@/components/ui/world/RecentWorldsList.vue'
import { useAppEvent } from '@/composables/use-app-event'
import { toError } from '@/helpers/errors'
import { list } from '@/helpers/instance'
import type { GameInstance } from '@/helpers/types'
import { useRootBreadcrumb } from '@/providers/breadcrumbs'
import { injectOnboardingChecklist } from '@/providers/onboarding-checklist'
import { useTheming } from '@/store/theme.ts'

defineOptions({
	name: 'LibraryPage',
})

const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()
const { hasCreatedInstance, isReady } = injectOnboardingChecklist()
const showCreationModal = inject<() => void>('showCreationModal')
const pageOptions = ref<InstanceType<typeof ContextMenu>>()
const themeStore = useTheming()

const messages = defineMessages({
	home: {
		id: 'app.navigation.home',
		defaultMessage: 'Home',
	},
	newInstance: {
		id: 'app.library.context-menu.create-instance',
		defaultMessage: 'New instance',
	},
})

const homeBreadcrumb = useRootBreadcrumb({
	slot: 'root',
	id: 'home',
	label: formatMessage(messages.home),
	to: '/',
	visual: { type: 'icon', component: HomeIcon },
})
onActivated(homeBreadcrumb.reset)

const instances = ref<GameInstance[]>([])
let latestInstanceFetch = 0

const recentInstances = computed(() =>
	instances.value
		.slice()
		.sort((a, b) => dayjs(b.last_played ?? b.created).diff(dayjs(a.last_played ?? a.created))),
)

async function fetchInstances() {
	const fetchId = ++latestInstanceFetch
	try {
		const nextInstances = await list()
		if (fetchId === latestInstanceFetch) {
			instances.value = nextInstances
		}
	} catch (error: unknown) {
		if (fetchId === latestInstanceFetch) {
			handleError(toError(error))
		}
	}
}

if (hasCreatedInstance.value) {
	await fetchInstances()
}

useAppEvent('instance', fetchInstances)
useAppEvent('instance_groups_changed', fetchInstances)

function openPageContextMenu(event: MouseEvent) {
	if (
		!(event.target instanceof HTMLElement) ||
		!event.target.hasAttribute('data-library-page-background')
	) {
		return
	}

	event.preventDefault()
	event.stopPropagation()
	pageOptions.value?.showMenu(event, {}, [{ name: 'new_instance' }])
}

function handlePageOption({ option }: { option: string }) {
	if (option === 'new_instance') {
		showCreationModal?.()
	}
}
</script>

<template>
	<WelcomeScreen v-if="isReady && !hasCreatedInstance" />
	<div
		v-else-if="isReady"
		data-library-page-background
		class="flex flex-col gap-6 p-6 select-none relative"
		@contextmenu="openPageContextMenu"
	>
		<!-- Cyber-Obsidian Hero Header Banner -->
		<div class="relative overflow-hidden rounded-2xl bg-gradient-to-r from-zinc-950 via-indigo-950/40 to-zinc-950 border border-white/10 p-6 shadow-2xl backdrop-blur-md flex flex-col md:flex-row items-center justify-between gap-4">
			<div class="absolute inset-0 pointer-events-none -z-10 bg-[radial-gradient(circle_at_30%_30%,rgba(99,102,241,0.15),transparent_60%)]"></div>
			<div class="flex items-center gap-4">
				<div class="relative flex items-center justify-center w-12 h-12 rounded-2xl bg-gradient-to-br from-indigo-600 via-purple-600 to-cyan-500 shadow-[0_0_20px_rgba(99,102,241,0.5)] border border-white/20 shrink-0">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" class="w-6 h-6">
						<path d="M12 2L2 9L12 22L22 9L12 2Z" fill="url(#index-diamond-grad)" fill-opacity="0.95" />
						<path d="M12 2L2 9H22L12 2Z" fill="white" fill-opacity="0.35" />
						<defs>
							<linearGradient id="index-diamond-grad" x1="2" y1="2" x2="22" y2="22" gradientUnits="userSpaceOnUse">
								<stop stop-color="#818CF8"/>
								<stop offset="0.5" stop-color="#C084FC"/>
								<stop offset="1" stop-color="#22D3EE"/>
							</linearGradient>
						</defs>
					</svg>
				</div>
				<div class="flex flex-col">
					<div class="flex items-center gap-2">
						<h1 class="text-xl font-extrabold text-white tracking-tight m-0">FreePlay Launcher</h1>
						<span class="text-[10px] font-bold px-2 py-0.5 rounded-full bg-indigo-500/20 border border-indigo-400/30 text-indigo-300 uppercase tracking-wider">Next-Gen Engine</span>
					</div>
					<div class="flex flex-wrap items-center gap-3 mt-1.5">
						<span class="flex items-center gap-1.5 text-xs font-medium text-zinc-300 bg-zinc-900/80 px-2.5 py-1 rounded-lg border border-white/5">
							<svg class="w-3.5 h-3.5 text-emerald-400 fill-current" viewBox="0 0 24 24"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
							Ultra-Fast Engine
						</span>
						<span class="flex items-center gap-1.5 text-xs font-medium text-zinc-300 bg-zinc-900/80 px-2.5 py-1 rounded-lg border border-white/5">
							<svg class="w-3.5 h-3.5 text-indigo-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
							Built-in Server Hosting
						</span>
						<span class="flex items-center gap-1.5 text-xs font-medium text-zinc-300 bg-zinc-900/80 px-2.5 py-1 rounded-lg border border-white/5">
							<svg class="w-3.5 h-3.5 text-cyan-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="13.5" cy="6.5" r=".5" fill="currentColor"/><circle cx="17.5" cy="10.5" r=".5" fill="currentColor"/><circle cx="8.5" cy="7.5" r=".5" fill="currentColor"/><circle cx="6.5" cy="12.5" r=".5" fill="currentColor"/><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.92 0 1.7-.72 1.7-1.65 0-.41-.15-.81-.44-1.12-.29-.31-.44-.72-.44-1.16 0-.93.75-1.67 1.68-1.67h2.2C19.55 16.4 22 13.95 22 10.9 22 6 17.5 2 12 2z"/></svg>
							Full Mod & Shader Support
						</span>
					</div>
				</div>
			</div>
			<!-- High-Contrast Action Buttons -->
			<div class="flex items-center gap-3 shrink-0">
				<button
					class="inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-emerald-600 hover:bg-emerald-500 active:scale-[0.98] text-white font-bold text-xs shadow-lg shadow-emerald-950/60 cursor-pointer transition-all duration-200 border border-emerald-400/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-400"
					@click="showCreationModal?.()"
				>
					<PlusIcon class="w-4 h-4" />
					+ Create an Instance
				</button>
				<router-link
					to="/hosting/manage"
					class="inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 active:scale-[0.98] text-white font-bold text-xs shadow-lg shadow-indigo-950/60 cursor-pointer transition-all duration-200 border border-indigo-400/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-400"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-indigo-200" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<rect width="20" height="8" x="2" y="2" rx="2" ry="2"/>
						<rect width="20" height="8" x="2" y="14" rx="2" ry="2"/>
						<line x1="6" x2="6.01" y1="6" y2="6"/>
						<line x1="6" x2="6.01" y1="18" y2="18"/>
					</svg>
					Server Hosting
				</router-link>
			</div>
		</div>

		<RecentWorldsList
			v-if="recentInstances?.length > 0 && themeStore.getFeatureFlag('worlds_in_home')"
			:recent-instances="recentInstances"
		/>
		<LibrarySection :instances="instances" />
		<ContextMenu ref="pageOptions" @option-clicked="handlePageOption">
			<template #new_instance> <PlusIcon /> {{ formatMessage(messages.newInstance) }} </template>
		</ContextMenu>
	</div>
</template>
