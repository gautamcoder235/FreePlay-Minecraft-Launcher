<script setup lang="ts">
import {
	BlocksIcon,
	ChevronDownIcon,
	ChevronRightIcon,
	ChevronUpIcon,
	ClockIcon,
	CompassIcon,
	CpuIcon,
	FolderIcon,
	HomeIcon,
	LayersIcon,
	PlayIcon,
	PlusIcon,
	RadioButtonCheckedIcon,
	RadioButtonIcon,
	ServerStackIcon,
	SettingsIcon,
	ShirtIcon,
	SparklesIcon,
	StopCircleIcon,
	UserIcon,
} from '@freeplay/assets'
import { defineMessages, injectNotificationManager, useVIntl } from '@freeplay/ui'
import { invoke } from '@tauri-apps/api/core'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import { computed, inject, onActivated, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import ContextMenu from '@/components/ui/ContextMenu.vue'
import LibrarySection from '@/components/ui/library/index.vue'
import OfflineAccountModal from '@/components/ui/modal/OfflineAccountModal.vue'
import WelcomeScreen from '@/components/ui/WelcomeScreen.vue'
import RecentWorldsList from '@/components/ui/world/RecentWorldsList.vue'
import { useAppEvent } from '@/composables/use-app-event'
import { create_offline_account, set_default_user } from '@/helpers/auth.js'
import { toError } from '@/helpers/errors'
import { getInstanceIconUrl, list, run } from '@/helpers/instance'
import { get_max_memory } from '@/helpers/jre'
import { get_all as getRunningProcesses, kill as killProcess } from '@/helpers/process'
import { get as getSettings } from '@/helpers/settings.ts'
import type { GameInstance } from '@/helpers/types'
import { useRootBreadcrumb } from '@/providers/breadcrumbs'
import { useAccountStore } from '@/store/account.ts'
import { useTheming } from '@/store/theme.ts'

dayjs.extend(relativeTime)

defineOptions({
	name: 'LibraryPage',
})

const router = useRouter()
const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()
const isReady = ref(true)
const hasCreatedInstance = computed(() => (instances.value?.length ?? 0) > 0)
const showCreationModal = inject<() => void>('showCreationModal')
const pageOptions = ref<InstanceType<typeof ContextMenu>>()
const themeStore = useTheming()
const accountStore = useAccountStore()
const offlineAccountModal = ref<InstanceType<typeof OfflineAccountModal> | null>(null)
const showAccountDropdown = ref(false)

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
const runningProcesses = ref<Array<{ uuid: string; instance_id: string }>>([])
const isLaunching = ref(false)
const allocatedMemoryMb = ref(4096)
const totalSystemRamGb = ref(16)
let latestInstanceFetch = 0

const recentInstances = computed(() =>
	instances.value
		.slice()
		.sort((a, b) => dayjs(b.last_played ?? b.created).diff(dayjs(a.last_played ?? a.created))),
)

const heroInstance = computed(() => recentInstances.value[0] || instances.value[0] || null)

const isHeroRunning = computed(() => {
	if (!heroInstance.value) return false
	return runningProcesses.value.some((p) => p.instance_id === heroInstance.value?.id)
})

const heroProcess = computed(() => {
	if (!heroInstance.value) return null
	return runningProcesses.value.find((p) => p.instance_id === heroInstance.value?.id) || null
})

async function fetchProcesses() {
	try {
		const procs = await getRunningProcesses()
		runningProcesses.value = procs || []
	} catch {
		runningProcesses.value = []
	}
}

async function fetchTelemetryAndUser() {
	try {
		const settings = await getSettings()
		if (settings) {
			const memVal =
				typeof settings.memory === 'number'
					? settings.memory
					: typeof settings.memory?.max === 'number'
						? settings.memory.max
						: 4096
			allocatedMemoryMb.value = memVal
		}

		const maxMemKiB = await get_max_memory().catch(() => null)
		if (typeof maxMemKiB === 'number' && maxMemKiB > 0) {
			totalSystemRamGb.value = Math.round(maxMemKiB / 1024 / 1024) || 16
		}
	} catch {
		// ignore
	}

	await accountStore.refresh()
}

async function onOfflineAccountCreated() {
	await accountStore.refresh()
	showAccountDropdown.value = false
}

async function handleSelectAccount(accountId: string) {
	await accountStore.setActiveAccount(accountId)
	showAccountDropdown.value = false
}

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
	await fetchProcesses()
}

await fetchInstances()
await fetchTelemetryAndUser()

useAppEvent('instance', fetchInstances)
useAppEvent('instance_groups_changed', fetchInstances)
useAppEvent('process', fetchProcesses)

onMounted(() => {
	window.addEventListener('focus', fetchProcesses)
	window.addEventListener('visibilitychange', fetchProcesses)
})

onUnmounted(() => {
	window.removeEventListener('focus', fetchProcesses)
	window.removeEventListener('visibilitychange', fetchProcesses)
})

async function handleLaunchHero() {
	if (!heroInstance.value) return

	if (isHeroRunning.value && heroProcess.value) {
		try {
			await killProcess(heroProcess.value.uuid)
			await fetchProcesses()
		} catch (err) {
			handleError(toError(err))
		}
		return
	}

	isLaunching.value = true
	try {
		if (accountStore.activeAccount?.isOffline && accountStore.activeAccount?.name) {
			await create_offline_account(accountStore.activeAccount.name).catch(() => {})
		} else if (accountStore.activeAccountId) {
			await set_default_user(accountStore.activeAccountId).catch(() => {})
		}
		await run(heroInstance.value.id)
		await fetchProcesses()
	} catch (err) {
		handleError(toError(err))
	} finally {
		isLaunching.value = false
	}
}

async function handleOpenInstanceDir(instanceId: string) {
	try {
		await invoke('plugin:instance|instance_open_dir', { instanceId })
	} catch (err) {
		handleError(toError(err))
	}
}

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
		class="flex flex-col gap-6 p-6 select-none relative max-w-[1600px] mx-auto w-full"
		@contextmenu="openPageContextMenu"
	>
		<!-- ========================================== -->
		<!-- BENTO GRID DASHBOARD                       -->
		<!-- ========================================== -->
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-5 items-stretch">
			<!-- TILE 1: 2x2 HERO INSTANCE CAPSULE -->
			<div
				class="lg:col-span-2 relative overflow-hidden rounded-3xl bg-[#141923]/90 backdrop-blur-xl border border-white/10 shadow-2xl p-6 lg:p-7 flex flex-col justify-between group transition-all duration-300 hover:border-sky-500/30 min-h-[250px]"
			>
				<!-- Subtle Ambient Radial Glow Backdrop -->
				<div
					class="absolute -right-20 -top-20 w-96 h-96 rounded-full bg-sky-500/10 blur-3xl pointer-events-none -z-10 group-hover:bg-sky-500/20 transition-all duration-500"
				></div>
				<div
					class="absolute -left-20 -bottom-20 w-80 h-80 rounded-full bg-indigo-500/10 blur-3xl pointer-events-none -z-10"
				></div>

				<!-- Top Badges & Metadata -->
				<div class="flex items-start justify-between gap-4 z-10">
					<div class="flex items-center gap-3.5 min-w-0">
						<div
							class="relative flex items-center justify-center w-14 h-14 rounded-2xl bg-gradient-to-br from-[#1c2331] to-[#0d1117] border border-white/15 shadow-xl overflow-hidden shrink-0 group-hover:border-sky-400/40 transition-colors"
						>
							<img
								v-if="heroInstance?.icon_path"
								:src="getInstanceIconUrl(heroInstance.icon_path) || ''"
								alt="Instance Icon"
								class="w-full h-full object-cover"
							/>
							<BlocksIcon v-else class="w-7 h-7 text-sky-400" />
						</div>
						<div class="flex flex-col min-w-0">
							<div class="flex items-center gap-2">
								<span
									v-if="isHeroRunning"
									class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-extrabold bg-sky-500/20 text-sky-300 border border-sky-400/40 uppercase tracking-widest animate-pulse"
								>
									<span class="w-1.5 h-1.5 rounded-full bg-sky-400"></span>
									Live Game Active
								</span>
								<span
									v-else
									class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-[10px] font-extrabold bg-white/5 text-zinc-300 border border-white/10 uppercase tracking-widest font-mono"
								>
									{{ heroInstance?.loader || 'Vanilla' }}
									{{ heroInstance?.game_version || 'Latest' }}
								</span>
							</div>
							<h2
								class="text-2xl font-black text-white tracking-tight mt-1 hover:text-sky-400 transition-colors cursor-pointer truncate"
								@click="heroInstance && router.push(`/instance/${heroInstance.id}`)"
							>
								{{ heroInstance?.name || 'No Instance Created' }}
							</h2>
						</div>
					</div>

					<!-- Quick Action Configure Button -->
					<button
						v-if="heroInstance"
						class="p-2.5 rounded-xl bg-white/[0.04] hover:bg-white/[0.08] border border-white/10 text-zinc-300 hover:text-white transition-all cursor-pointer shadow-sm active:scale-95 shrink-0"
						title="Instance Configuration"
						@click="router.push(`/instance/${heroInstance.id}`)"
					>
						<SettingsIcon class="w-4 h-4" />
					</button>
				</div>

				<!-- Center Telemetry Chips -->
				<div class="flex flex-wrap items-center gap-2.5 my-4 z-10">
					<div
						class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-black/40 border border-white/5 text-xs text-zinc-300 font-medium"
					>
						<ClockIcon class="w-3.5 h-3.5 text-amber-400" />
						<span
							>Last played
							{{
								heroInstance?.last_played ? dayjs(heroInstance.last_played).fromNow() : 'Never'
							}}</span
						>
					</div>
					<div
						class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-black/40 border border-white/5 text-xs text-zinc-300 font-medium"
					>
						<LayersIcon class="w-3.5 h-3.5 text-sky-400" />
						<span>High-Speed Native Engine</span>
					</div>
					<div
						class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-black/40 border border-white/5 text-xs text-zinc-300 font-medium"
					>
						<SparklesIcon class="w-3.5 h-3.5 text-indigo-400" />
						<span>Shaders & Performance Ready</span>
					</div>
				</div>

				<!-- Bottom Action Row: Launch & Tools -->
				<div
					class="flex flex-wrap items-center justify-between gap-4 pt-3.5 border-t border-white/10 z-10"
				>
					<div class="flex items-center gap-3">
						<!-- Primary 1-Click Launch Button -->
						<button
							v-if="heroInstance"
							class="inline-flex items-center justify-center gap-2.5 px-7 py-3 rounded-2xl font-black text-xs uppercase tracking-wider text-white shadow-xl cursor-pointer transition-all duration-200 active:scale-[0.98] border"
							:class="[
								isHeroRunning
									? 'bg-rose-600 hover:bg-rose-500 shadow-rose-950/60 border-rose-400/40'
									: 'bg-gradient-to-r from-sky-500 via-blue-600 to-indigo-600 hover:from-sky-400 hover:to-blue-500 shadow-sky-950/70 border-sky-400/40 hover:shadow-[0_0_30px_rgba(56,189,248,0.4)]',
							]"
							:disabled="isLaunching"
							@click="handleLaunchHero"
						>
							<template v-if="isLaunching">
								<svg class="animate-spin w-4 h-4 text-white" viewBox="0 0 24 24" fill="none">
									<circle
										class="opacity-25"
										cx="12"
										cy="12"
										r="10"
										stroke="currentColor"
										stroke-width="4"
									></circle>
									<path
										class="opacity-75"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									></path>
								</svg>
								Syncing Assets & Launching...
							</template>
							<template v-else-if="isHeroRunning">
								<StopCircleIcon class="w-4 h-4 fill-current" />
								Stop Game Session
							</template>
							<template v-else>
								<PlayIcon class="w-4 h-4 fill-current text-white" />
								PLAY MINECRAFT
							</template>
						</button>

						<button
							v-else
							class="inline-flex items-center justify-center gap-2 px-6 py-3 rounded-2xl bg-gradient-to-r from-sky-500 to-blue-600 hover:from-sky-400 hover:to-blue-500 text-white font-bold text-xs uppercase tracking-wider shadow-lg shadow-sky-950/60 cursor-pointer border border-sky-400/30 active:scale-95 transition-all"
							@click="showCreationModal?.()"
						>
							<PlusIcon class="w-4 h-4" />
							Create First Instance
						</button>

						<!-- Secondary Mod Manager Button -->
						<button
							v-if="heroInstance"
							class="inline-flex items-center gap-2 px-4 py-3 rounded-2xl bg-white/[0.05] hover:bg-white/[0.1] text-zinc-200 font-semibold text-xs border border-white/10 transition-all cursor-pointer active:scale-95"
							@click="router.push(`/instance/${heroInstance.id}`)"
						>
							<LayersIcon class="w-3.5 h-3.5 text-zinc-400" />
							Mod Manager
						</button>
					</div>

					<button
						v-if="heroInstance"
						class="inline-flex items-center gap-1.5 text-xs text-zinc-400 hover:text-white transition-colors cursor-pointer"
						@click="handleOpenInstanceDir(heroInstance.id)"
					>
						<FolderIcon class="w-3.5 h-3.5" />
						<span>Open Folder</span>
					</button>
				</div>
			</div>

			<!-- TILE 2: CUSTOM 3D WARDROBE & PLAYER HUB -->
			<div
				class="relative overflow-visible rounded-3xl bg-[#141923]/90 backdrop-blur-xl border border-white/10 shadow-2xl p-6 flex flex-col justify-between group transition-all duration-300 hover:border-amber-500/30 min-h-[250px]"
			>
				<!-- Atmospheric Glow Backdrop -->
				<div
					class="absolute -top-24 -right-24 w-48 h-48 rounded-full bg-amber-500/10 blur-3xl pointer-events-none overflow-hidden"
				></div>
				<div
					class="absolute -bottom-24 -left-24 w-48 h-48 rounded-full bg-sky-500/10 blur-3xl pointer-events-none overflow-hidden"
				></div>

				<!-- Header -->
				<div class="flex items-center justify-between z-20 relative">
					<div class="flex items-center gap-2.5">
						<div class="p-1.5 rounded-xl bg-amber-500/15 border border-amber-500/30 text-amber-400">
							<ShirtIcon class="w-4 h-4" />
						</div>
						<div class="flex flex-col">
							<span class="text-xs font-bold uppercase tracking-widest text-zinc-200"
								>Wardrobe & Skins</span
							>
							<span class="text-[10px] text-zinc-400 font-mono">
								{{ accountStore.accountCount }}
								{{ accountStore.accountCount === 1 ? 'Profile' : 'Profiles' }}
							</span>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<!-- Floating Switcher Trigger -->
						<div class="relative">
							<button
								v-if="accountStore.accountCount > 1"
								type="button"
								class="text-xs font-bold px-2.5 py-1 rounded-lg border transition-all flex items-center gap-1 cursor-pointer"
								:class="
									showAccountDropdown
										? 'bg-sky-500/20 border-sky-500/50 text-white shadow-sm shadow-sky-950/50'
										: 'bg-white/5 hover:bg-white/10 text-zinc-300 hover:text-white border-white/10'
								"
								@click="showAccountDropdown = !showAccountDropdown"
							>
								Switch
								<ChevronDownIcon v-if="!showAccountDropdown" class="w-3 h-3" />
								<ChevronUpIcon v-else class="w-3 h-3" />
							</button>

							<!-- Click-outside backdrop -->
							<div
								v-if="showAccountDropdown"
								class="fixed inset-0 z-40"
								@click="showAccountDropdown = false"
							></div>

							<!-- Floating Profile Switcher Popover -->
							<div
								v-if="showAccountDropdown && accountStore.accountCount > 1"
								class="absolute right-0 top-full mt-2 w-72 p-2.5 rounded-2xl bg-[#0c1017]/95 border border-white/20 shadow-2xl backdrop-blur-2xl flex flex-col gap-1.5 z-50"
							>
								<div
									class="flex items-center justify-between px-2 py-1 border-b border-white/10 mb-0.5"
								>
									<span class="text-[10px] font-bold uppercase tracking-wider text-zinc-400"
										>Select Profile</span
									>
									<span class="text-[10px] text-zinc-500 font-mono"
										>{{ accountStore.accountCount }} Available</span
									>
								</div>
								<div class="flex flex-col gap-1 max-h-56 overflow-y-auto pr-0.5">
									<button
										v-for="acc in accountStore.accounts"
										:key="acc.id"
										type="button"
										class="flex items-center justify-between p-2 rounded-xl border transition-all cursor-pointer text-left group/item"
										:class="
											acc.active
												? 'bg-sky-500/20 border-sky-500/50 text-white font-bold shadow-sm'
												: 'bg-white/5 border-transparent text-zinc-300 hover:bg-white/10 hover:text-white'
										"
										@click="handleSelectAccount(acc.id)"
									>
										<div class="flex items-center gap-2.5 min-w-0">
											<img
												:src="acc.avatarUrl"
												class="w-7 h-7 rounded-lg object-cover bg-zinc-800 border border-white/10 shrink-0"
												alt=""
											/>
											<div class="flex flex-col min-w-0">
												<span
													class="text-xs truncate font-bold group-hover/item:text-sky-300 transition-colors"
													>{{ acc.name }}</span
												>
												<span class="text-[9px] text-zinc-400 font-mono">
													{{ acc.isOffline ? 'Offline Profile' : 'Microsoft Account' }}
												</span>
											</div>
										</div>
										<RadioButtonCheckedIcon
											v-if="acc.active"
											class="w-4 h-4 text-sky-400 shrink-0"
										/>
										<RadioButtonIcon
											v-else
											class="w-4 h-4 text-zinc-500 group-hover/item:text-zinc-300 shrink-0"
										/>
									</button>
								</div>
							</div>
						</div>

						<router-link
							to="/skins"
							class="text-xs font-bold text-amber-400 hover:text-amber-300 flex items-center gap-1 transition-colors group-hover:translate-x-0.5"
						>
							Studio <ChevronRightIcon class="w-3 h-3" />
						</router-link>
					</div>
				</div>

				<!-- 3D Skin Avatar Banner (Compact Horizontal Stage) -->
				<div
					class="relative my-3 flex items-center gap-3.5 p-3 rounded-2xl bg-gradient-to-b from-white/[0.04] to-black/40 border border-white/10 shadow-inner z-10"
				>
					<div class="relative group/avatar cursor-pointer shrink-0" @click="router.push('/skins')">
						<div
							class="relative w-14 h-14 rounded-2xl bg-gradient-to-br from-amber-500/20 via-indigo-500/20 to-sky-500/20 p-0.5 border border-white/15 shadow-[0_0_20px_rgba(245,158,11,0.2)] flex items-center justify-center group-hover/avatar:scale-105 group-hover/avatar:shadow-[0_0_30px_rgba(56,189,248,0.3)] transition-all duration-300 overflow-hidden"
						>
							<img
								v-if="accountStore.activePlayerAvatar"
								:src="accountStore.activePlayerAvatar"
								alt="Skin Avatar"
								class="w-full h-full rounded-xl object-contain drop-shadow-md"
							/>
							<UserIcon v-else class="w-8 h-8 text-amber-400" />
						</div>
						<!-- Live Active Status Dot -->
						<span class="absolute -bottom-0.5 -right-0.5 flex h-3.5 w-3.5">
							<span
								class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
							></span>
							<span
								class="relative inline-flex rounded-full h-3.5 w-3.5 bg-emerald-500 border-2 border-[#141923]"
							></span>
						</span>
					</div>

					<div class="flex flex-col min-w-0 flex-1">
						<div class="flex items-center gap-2">
							<span class="text-sm font-extrabold text-white truncate">
								{{ accountStore.activePlayerName }}
							</span>
							<span
								class="text-[9px] font-extrabold px-1.5 py-0.5 rounded uppercase tracking-wider font-mono shrink-0"
								:class="
									accountStore.isActiveOffline
										? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
										: 'bg-sky-500/20 text-sky-300 border border-sky-500/30'
								"
							>
								{{ accountStore.isActiveOffline ? 'Offline' : 'Microsoft' }}
							</span>
						</div>
						<span class="text-[11px] font-medium text-emerald-400 flex items-center gap-1.5 mt-0.5">
							<span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
							Active Player Profile
						</span>
					</div>
				</div>

				<!-- Action Buttons Grid -->
				<div class="grid grid-cols-2 gap-2 z-10">
					<button
						type="button"
						class="py-2.5 px-2 rounded-xl bg-gradient-to-r from-sky-500/20 to-blue-500/20 hover:from-sky-500/30 hover:to-blue-500/30 border border-sky-400/40 text-sky-200 font-bold text-xs flex items-center justify-center gap-1.5 cursor-pointer transition-all active:scale-98 shadow-sm truncate"
						@click="offlineAccountModal?.show()"
					>
						<UserIcon class="w-3.5 h-3.5 text-sky-400 shrink-0" />
						<span class="truncate">+ Add Account</span>
					</button>

					<router-link
						to="/skins"
						class="py-2.5 px-2 rounded-xl bg-gradient-to-r from-amber-500/15 to-orange-500/15 hover:from-amber-500/25 hover:to-orange-500/25 border border-amber-400/30 text-amber-200 font-bold text-xs flex items-center justify-center gap-1.5 cursor-pointer transition-all active:scale-98 truncate"
					>
						<SparklesIcon class="w-3.5 h-3.5 text-amber-400 shrink-0" />
						<span class="truncate">Skin & Capes</span>
					</router-link>
				</div>
			</div>
		</div>

		<!-- SECOND ROW BENTO WIDGETS -->
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5">
			<!-- WIDGET 1: SERVER TUNNELS & MULTIPLAYER -->
			<router-link
				to="/hosting/manage"
				class="relative overflow-hidden rounded-2xl bg-[#141923]/90 backdrop-blur-xl border border-white/10 p-5 flex flex-col justify-between group hover:border-indigo-500/40 hover:shadow-[0_0_25px_rgba(99,102,241,0.2)] transition-all duration-200 min-h-[160px]"
			>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2.5">
						<div
							class="p-2 rounded-xl bg-indigo-500/10 border border-indigo-400/20 text-indigo-400"
						>
							<ServerStackIcon class="w-4 h-4" />
						</div>
						<span class="text-xs font-bold text-white uppercase tracking-wider"
							>FreePlay Tunnel</span
						>
					</div>
					<span
						class="text-[10px] font-bold px-2 py-0.5 rounded-full bg-indigo-500/20 text-indigo-300 border border-indigo-400/30 font-mono"
						>P2P LAN</span
					>
				</div>
				<div class="my-2">
					<span class="text-sm font-bold text-white block">Zero-Config Multiplayer</span>
					<p class="text-xs text-zinc-400 mt-0.5 leading-relaxed">
						Host singleplayer worlds online with friends in 1-click.
					</p>
				</div>
				<span
					class="text-xs font-bold text-indigo-400 flex items-center gap-1 group-hover:translate-x-1 transition-transform"
				>
					Open Control Room <ChevronRightIcon class="w-3 h-3" />
				</span>
			</router-link>

			<!-- WIDGET 2: CONTENT DISCOVERY MODPACKS -->
			<router-link
				to="/browse/modpack"
				class="relative overflow-hidden rounded-2xl bg-[#141923]/90 backdrop-blur-xl border border-white/10 p-5 flex flex-col justify-between group hover:border-sky-500/40 hover:shadow-[0_0_25px_rgba(56,189,248,0.2)] transition-all duration-200 min-h-[160px]"
			>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2.5">
						<div class="p-2 rounded-xl bg-sky-500/10 border border-sky-400/20 text-sky-400">
							<CompassIcon class="w-4 h-4" />
						</div>
						<span class="text-xs font-bold text-white uppercase tracking-wider">Browse Index</span>
					</div>
					<span
						class="text-[10px] font-bold px-2 py-0.5 rounded-full bg-sky-500/20 text-sky-300 border border-sky-400/30 font-mono"
						>Modrinth</span
					>
				</div>
				<div class="my-2">
					<span class="text-sm font-bold text-white block">50,000+ Modpacks & Mods</span>
					<p class="text-xs text-zinc-400 mt-0.5 leading-relaxed">
						Fabulously Optimized, Create, Cobblemon, and shaders.
					</p>
				</div>
				<span
					class="text-xs font-bold text-sky-400 flex items-center gap-1 group-hover:translate-x-1 transition-transform"
				>
					Explore Catalog <ChevronRightIcon class="w-3 h-3" />
				</span>
			</router-link>

			<!-- WIDGET 3: TELEMETRY & JVM ALLOCATOR -->
			<div
				class="relative overflow-hidden rounded-2xl bg-[#141923]/90 backdrop-blur-xl border border-white/10 p-5 flex flex-col justify-between group hover:border-amber-500/50 hover:shadow-[0_0_25px_rgba(245,158,11,0.25)] transition-all duration-200 min-h-[160px]"
			>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2.5">
						<div class="p-2 rounded-xl bg-amber-500/10 border border-amber-400/20 text-amber-400">
							<CpuIcon class="w-4 h-4" />
						</div>
						<span class="text-xs font-bold text-white uppercase tracking-wider"
							>RAM Allocation</span
						>
					</div>
					<span
						class="text-[10px] font-bold px-2 py-0.5 rounded-full bg-amber-500/20 text-amber-300 border border-amber-400/30 font-mono"
						>JVM</span
					>
				</div>
				<div class="my-2">
					<div class="flex items-baseline justify-between">
						<span class="text-base font-extrabold text-white font-mono"
							>{{ (allocatedMemoryMb / 1024).toFixed(1) }} GB</span
						>
						<span class="text-xs text-zinc-400 font-mono"
							>of {{ totalSystemRamGb.toFixed(1) }} GB System</span
						>
					</div>
					<!-- RAM Visual Progress Bar -->
					<div class="w-full h-2 rounded-full bg-white/10 mt-2 overflow-hidden">
						<div
							class="h-full bg-gradient-to-r from-sky-500 to-amber-500 rounded-full"
							:style="{
								width: `${Math.min(100, (allocatedMemoryMb / (totalSystemRamGb * 1024)) * 100)}%`,
							}"
						></div>
					</div>
				</div>
				<span class="text-[11px] text-zinc-400 flex items-center gap-1.5">
					<span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
					Auto-tuned GC active
				</span>
			</div>

			<!-- WIDGET 4: INSTANCE CREATION SHORTCUT -->
			<div
				class="relative overflow-hidden rounded-2xl bg-gradient-to-br from-indigo-950/40 via-[#141923] to-[#141923] border border-indigo-500/30 p-5 flex flex-col justify-between cursor-pointer group hover:border-indigo-400/60 hover:shadow-[0_0_25px_rgba(99,102,241,0.3)] transition-all duration-200 min-h-[160px]"
				@click="showCreationModal?.()"
			>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2.5">
						<div
							class="p-2 rounded-xl bg-indigo-500/20 border border-indigo-400/40 text-indigo-300"
						>
							<PlusIcon class="w-4 h-4" />
						</div>
						<span class="text-xs font-bold text-indigo-200 uppercase tracking-wider"
							>New Instance</span
						>
					</div>
				</div>
				<div class="my-2">
					<span class="text-sm font-bold text-white block">Create Custom Setup</span>
					<p class="text-xs text-zinc-400 mt-0.5 leading-relaxed">
						Choose Fabric, Forge, NeoForge, or Vanilla version.
					</p>
				</div>
				<span
					class="text-xs font-bold text-indigo-300 flex items-center gap-1 group-hover:translate-x-1 transition-transform"
				>
					+ Launch Setup Wizard <ChevronRightIcon class="w-3 h-3" />
				</span>
			</div>
		</div>

		<!-- RECENT SINGLEPLAYER WORLDS -->
		<RecentWorldsList
			v-if="recentInstances?.length > 0 && themeStore.getFeatureFlag('worlds_in_home')"
			:recent-instances="recentInstances"
		/>

		<!-- COMPLETE INSTANCES LIBRARY SECTION -->
		<div class="mt-4 pt-6 border-t border-white/10">
			<div class="flex items-center justify-between mb-4">
				<div class="flex items-center gap-3">
					<h3 class="text-lg font-extrabold text-white tracking-tight m-0">Instance Library</h3>
					<span class="text-xs font-bold px-2 py-0.5 rounded-full bg-white/10 text-zinc-300">
						{{ instances.length }} {{ instances.length === 1 ? 'Instance' : 'Instances' }}
					</span>
				</div>
			</div>
			<LibrarySection :instances="instances" />
		</div>

		<OfflineAccountModal ref="offlineAccountModal" @created="onOfflineAccountCreated" />

		<ContextMenu ref="pageOptions" @option-clicked="handlePageOption">
			<template #new_instance> <PlusIcon /> {{ formatMessage(messages.newInstance) }} </template>
		</ContextMenu>
	</div>
</template>
