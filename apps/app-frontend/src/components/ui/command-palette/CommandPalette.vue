<script setup lang="ts">
import {
	BlocksIcon,
	CompassIcon,
	FolderIcon,
	LayersIcon,
	PlayIcon,
	PlusIcon,
	SearchIcon,
	ServerStackIcon,
	SettingsIcon,
	ShirtIcon,
	SparklesIcon,
	UserIcon,
} from '@freeplay/assets'
import { injectNotificationManager } from '@freeplay/ui'
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'

import { list as listInstances, run as runInstance } from '@/helpers/instance'
import type { GameInstance } from '@/helpers/types'
import { showAppDbBackupsFolder, showLauncherLogsFolder } from '@/helpers/utils'

const props = defineProps<{
	modelValue: boolean
}>()

const emit = defineEmits<{
	(e: 'update:modelValue', value: boolean): void
	(e: 'create-instance' | 'open-settings' | 'open-accounts' | 'add-offline-account'): void
}>()

const router = useRouter()
const { addNotification } = injectNotificationManager()
const searchQuery = ref('')
const selectedIndex = ref(0)
const inputRef = ref<HTMLInputElement | null>(null)
const listRef = ref<HTMLDivElement | null>(null)
const instances = ref<GameInstance[]>([])

async function loadInstances() {
	try {
		instances.value = await listInstances()
	} catch {
		instances.value = []
	}
}

watch(
	() => props.modelValue,
	(open) => {
		if (open) {
			searchQuery.value = ''
			selectedIndex.value = 0
			loadInstances()
			nextTick(() => {
				inputRef.value?.focus()
			})
		}
	},
)

const navActions = [
	{
		id: 'nav-home',
		category: 'Navigation',
		title: 'Home & Instance Library',
		keywords: ['home', 'library', 'instances', 'games', 'installed', 'main', 'dashboard'],
		icon: BlocksIcon,
		action: () => {
			close()
			router.push('/')
		},
		badge: 'Home',
	},
	{
		id: 'nav-modpacks',
		category: 'Navigation',
		title: 'Discover Modpacks',
		keywords: ['modpack', 'packs', 'curseforge', 'modrinth', 'browse', 'discover', 'install'],
		icon: CompassIcon,
		action: () => {
			close()
			router.push('/browse/modpack')
		},
		badge: 'Modpacks',
	},
	{
		id: 'nav-mods',
		category: 'Navigation',
		title: 'Discover Mods',
		keywords: ['mods', 'fabric', 'forge', 'neoforge', 'quilt', 'addons', 'plugins', 'browse'],
		icon: LayersIcon,
		action: () => {
			close()
			router.push('/browse/mod')
		},
		badge: 'Mods',
	},
	{
		id: 'nav-resourcepacks',
		category: 'Navigation',
		title: 'Discover Resource Packs & Textures',
		keywords: ['resourcepack', 'resource pack', 'texture', 'textures', 'faithful', 'pvp', 'hd'],
		icon: SparklesIcon,
		action: () => {
			close()
			router.push('/browse/resourcepack')
		},
		badge: 'Textures',
	},
	{
		id: 'nav-shaders',
		category: 'Navigation',
		title: 'Discover Shaders & Visuals',
		keywords: [
			'shader',
			'shaders',
			'iris',
			'optifine',
			'bsl',
			'complementary',
			'graphics',
			'bloom',
		],
		icon: SparklesIcon,
		action: () => {
			close()
			router.push('/browse/shader')
		},
		badge: 'Shaders',
	},
	{
		id: 'nav-datapacks',
		category: 'Navigation',
		title: 'Discover Data Packs',
		keywords: ['datapack', 'data pack', 'custom', 'worldgen', 'structures', 'terralith'],
		icon: SparklesIcon,
		action: () => {
			close()
			router.push('/browse/datapack')
		},
		badge: 'Datapacks',
	},
	{
		id: 'nav-plugins',
		category: 'Navigation',
		title: 'Discover Server Plugins',
		keywords: ['plugin', 'plugins', 'paper', 'purpur', 'spigot', 'bukkit', 'server'],
		icon: SparklesIcon,
		action: () => {
			close()
			router.push('/browse/plugin')
		},
		badge: 'Plugins',
	},
	{
		id: 'nav-skins',
		category: 'Navigation',
		title: '3D Skin Studio & Cape Wardrobe',
		keywords: ['skin', 'skins', 'cape', 'capes', 'wardrobe', 'steve', 'alex', '3d', 'custom'],
		icon: ShirtIcon,
		action: () => {
			close()
			router.push('/skins')
		},
		badge: 'Skins',
	},
	{
		id: 'nav-hosting',
		category: 'Navigation',
		title: 'Server Control Room & Tunnels',
		keywords: [
			'server',
			'servers',
			'hosting',
			'host',
			'control',
			'room',
			'tunnel',
			'playit',
			'paper',
			'purpur',
			'multiplayer',
		],
		icon: ServerStackIcon,
		action: () => {
			close()
			router.push('/hosting/manage')
		},
		badge: 'Servers',
	},
]

const quickActions = [
	{
		id: 'action-create',
		category: 'Quick Actions',
		title: 'Create New Instance',
		keywords: ['create', 'new', 'instance', 'install', 'version', 'minecraft', 'add'],
		icon: PlusIcon,
		action: () => {
			close()
			emit('create-instance')
		},
		badge: 'New',
	},
	{
		id: 'action-offline-account',
		category: 'Quick Actions',
		title: 'Add Offline / Cracked Minecraft Account',
		keywords: [
			'offline',
			'cracked',
			'account',
			'auth',
			'username',
			'login',
			'freeplay',
			'non-premium',
		],
		icon: UserIcon,
		action: () => {
			close()
			emit('add-offline-account')
		},
		badge: 'Free',
	},
	{
		id: 'action-accounts',
		category: 'Quick Actions',
		title: 'Manage Accounts & Switch Profile',
		keywords: [
			'account',
			'accounts',
			'profile',
			'switch',
			'microsoft',
			'msa',
			'login',
			'user',
			'skin',
		],
		icon: UserIcon,
		action: () => {
			close()
			emit('open-accounts')
		},
		badge: 'Accounts',
	},
	{
		id: 'action-settings',
		category: 'Quick Actions',
		title: 'App Settings & JVM Configuration',
		keywords: [
			'settings',
			'config',
			'jvm',
			'java',
			'ram',
			'memory',
			'theme',
			'dark mode',
			'options',
		],
		icon: SettingsIcon,
		action: () => {
			close()
			emit('open-settings')
		},
		badge: 'Config',
	},
	{
		id: 'action-logs',
		category: 'System Tools',
		title: 'Open Launcher Logs Directory',
		keywords: ['logs', 'log', 'crash', 'directory', 'folder', 'explorer', 'debug', 'files'],
		icon: FolderIcon,
		action: async () => {
			close()
			try {
				await showLauncherLogsFolder()
			} catch (err) {
				console.error('Failed to open logs folder:', err)
			}
		},
		badge: 'Logs',
	},
	{
		id: 'action-backups',
		category: 'System Tools',
		title: 'Open App Database Backups Folder',
		keywords: ['backup', 'backups', 'database', 'db', 'recovery', 'folder', 'directory'],
		icon: FolderIcon,
		action: async () => {
			close()
			try {
				await showAppDbBackupsFolder()
			} catch (err) {
				console.error('Failed to open backups folder:', err)
			}
		},
		badge: 'Backups',
	},
]

const filteredResults = computed(() => {
	const q = searchQuery.value.toLowerCase().trim()

	const instanceResults = instances.value
		.filter(
			(inst) =>
				!q ||
				inst.name.toLowerCase().includes(q) ||
				(inst.loader && inst.loader.toLowerCase().includes(q)) ||
				(inst.game_version && inst.game_version.toLowerCase().includes(q)),
		)
		.map((inst) => ({
			id: `instance-${inst.id}`,
			category: 'Game Instances',
			title: inst.name,
			subtitle: `${inst.loader || 'Vanilla'} ${inst.game_version || ''}`,
			icon: BlocksIcon,
			action: () => {
				close()
				router.push(`/instance/${inst.id}`)
			},
			playAction: async (e: MouseEvent) => {
				e.stopPropagation()
				close()
				try {
					await runInstance(inst.id)
				} catch (err) {
					console.error('Failed to launch instance:', err)
					addNotification({
						type: 'error',
						title: `Failed to launch ${inst.name}`,
					})
				}
			},
			badge: 'Playable',
			isInstance: true,
		}))

	const filteredNav = navActions.filter(
		(item) =>
			!q ||
			item.title.toLowerCase().includes(q) ||
			item.category.toLowerCase().includes(q) ||
			item.badge.toLowerCase().includes(q) ||
			item.keywords.some((k) => k.includes(q)),
	)
	const filteredQuick = quickActions.filter(
		(item) =>
			!q ||
			item.title.toLowerCase().includes(q) ||
			item.category.toLowerCase().includes(q) ||
			item.badge.toLowerCase().includes(q) ||
			item.keywords.some((k) => k.includes(q)),
	)

	return [...instanceResults, ...filteredNav, ...filteredQuick]
})

function close() {
	emit('update:modelValue', false)
}

function scrollToActiveItem() {
	nextTick(() => {
		if (!listRef.value) return
		const activeEl = listRef.value.children[selectedIndex.value] as HTMLElement | undefined
		if (activeEl) {
			activeEl.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
		}
	})
}

function handleKeyDown(e: KeyboardEvent) {
	if (e.key === 'ArrowDown') {
		e.preventDefault()
		if (filteredResults.value.length > 0) {
			selectedIndex.value = (selectedIndex.value + 1) % filteredResults.value.length
			scrollToActiveItem()
		}
	} else if (e.key === 'ArrowUp') {
		e.preventDefault()
		if (filteredResults.value.length > 0) {
			selectedIndex.value =
				(selectedIndex.value - 1 + filteredResults.value.length) % filteredResults.value.length
			scrollToActiveItem()
		}
	} else if (e.key === 'Enter') {
		e.preventDefault()
		const item = filteredResults.value[selectedIndex.value]
		if (item) {
			item.action()
		}
	} else if (e.key === 'Escape') {
		e.preventDefault()
		close()
	}
}

function handleGlobalShortcut(e: KeyboardEvent) {
	if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
		e.preventDefault()
		emit('update:modelValue', !props.modelValue)
	}
}

onMounted(() => {
	window.addEventListener('keydown', handleGlobalShortcut)
})

onUnmounted(() => {
	window.removeEventListener('keydown', handleGlobalShortcut)
})
</script>

<template>
	<Teleport to="body">
		<Transition
			enter-active-class="transition duration-150 ease-out"
			enter-from-class="opacity-0 scale-95"
			enter-to-class="opacity-100 scale-100"
			leave-active-class="transition duration-100 ease-in"
			leave-from-class="opacity-100 scale-100"
			leave-to-class="opacity-0 scale-95"
		>
			<div
				v-if="modelValue"
				class="fixed inset-0 z-50 flex items-start justify-center pt-20 px-4 bg-black/75 backdrop-blur-2xl select-none"
				@click.self="close"
				@keydown="handleKeyDown"
			>
				<div
					class="w-full max-w-2xl overflow-hidden rounded-3xl bg-[var(--surface-1)]/95 backdrop-blur-3xl border border-[var(--border-default)] shadow-[0_30px_70px_rgba(0,0,0,0.9),0_0_50px_var(--color-brand-shadow)] flex flex-col max-h-[75vh]"
				>
					<!-- Search Header Spotlight with Curved Border -->
					<div class="p-3 border-b border-[var(--border-subtle)] bg-[var(--surface-1)]/50">
						<div
							class="relative flex items-center px-3.5 py-2.5 rounded-2xl bg-[var(--surface-1-5)] border border-[var(--border-subtle)] focus-within:border-[var(--color-brand)]/50 transition-all duration-200"
							:style="{
								boxShadow: '0 0 15px var(--color-brand-bg)',
							}"
						>
							<div
								class="w-7 h-7 rounded-xl bg-[var(--color-brand)]/10 border border-[var(--color-brand)]/30 flex items-center justify-center text-[var(--color-brand)] mr-2.5 shrink-0 shadow-inner"
							>
								<SearchIcon class="w-3.5 h-3.5" />
							</div>
							<input
								ref="inputRef"
								v-model="searchQuery"
								type="text"
								placeholder="Type a command, search instances, mods, shaders, settings..."
								class="w-full bg-transparent text-sm font-medium text-white placeholder-zinc-500 !outline-none !shadow-none !border-none"
								@keydown="handleKeyDown"
								@input="selectedIndex = 0"
							/>
							<span
								class="text-[10px] font-mono font-bold px-2 py-0.5 rounded-lg bg-white/5 text-zinc-400 border border-[var(--border-subtle)] shrink-0 ml-2"
							>
								ESC
							</span>
						</div>
					</div>

					<!-- Results List -->
					<div ref="listRef" class="overflow-y-auto p-2.5 flex flex-col gap-1.5 scrollbar-thin">
						<div
							v-if="filteredResults.length === 0"
							class="py-12 text-center text-zinc-500 text-xs font-mono"
						>
							No commands or instances found matching "<span class="text-white font-semibold">{{
								searchQuery
							}}</span
							>"
						</div>

						<div
							v-for="(item, idx) in filteredResults"
							:key="item.id"
							class="group flex items-center justify-between px-3.5 py-2.5 rounded-2xl cursor-pointer transition-all duration-150"
							:class="[
								selectedIndex === idx
									? 'bg-gradient-to-r from-[var(--color-brand)]/20 via-[var(--color-brand)]/10 to-transparent border border-[var(--color-brand)]/50 text-white'
									: 'bg-[var(--surface-2)]/60 hover:bg-[var(--surface-2)] text-zinc-300 border border-[var(--border-subtle)]',
							]"
							@mouseenter="selectedIndex = idx"
							@click="item.action"
						>
							<div class="flex items-center gap-3 min-w-0">
								<div
									class="flex items-center justify-center w-8 h-8 rounded-xl shrink-0 border transition-transform duration-150 group-hover:scale-105"
									:class="[
										selectedIndex === idx
											? 'bg-[var(--color-brand)]/20 border-[var(--color-brand)]/40 text-[var(--color-brand-highlight)] shadow-inner'
											: 'bg-white/5 border-[var(--border-subtle)] text-zinc-400',
									]"
								>
									<component :is="item.icon" class="w-4 h-4" />
								</div>
								<div class="flex flex-col min-w-0">
									<span class="text-xs font-bold truncate text-white tracking-wide">{{
										item.title
									}}</span>
									<span
										v-if="'subtitle' in item"
										class="text-[11px] text-zinc-400 font-mono truncate"
										>{{ item.subtitle }}</span
									>
								</div>
							</div>

							<div class="flex items-center gap-2 shrink-0 ml-3">
								<button
									v-if="'isInstance' in item"
									type="button"
									class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-xl btn-accent-primary font-extrabold text-xs border-none cursor-pointer active:scale-95"
									title="Launch Game"
									@click="item.playAction"
								>
									<PlayIcon class="w-3 h-3 fill-current" />
									Play
								</button>
								<span
									class="text-[9px] font-extrabold font-mono px-2 py-0.5 rounded uppercase tracking-wider shrink-0"
									:class="[
										item.badge === 'Home' ||
										item.badge === 'Playable' ||
										item.badge === 'Free' ||
										item.badge === 'New' ||
										item.badge === 'Accounts' ||
										item.badge === 'Multiplayer'
											? 'bg-sky-500/20 text-sky-300 border border-sky-500/30'
											: item.badge === 'Modpacks' ||
												  item.badge === 'Mods' ||
												  item.badge === 'Textures' ||
												  item.badge === 'Shaders' ||
												  item.badge === 'Datapacks' ||
												  item.badge === 'Plugins'
												? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30'
												: item.badge === 'Skins'
													? 'bg-purple-500/20 text-purple-300 border border-purple-500/30'
													: 'bg-amber-500/20 text-amber-300 border border-amber-500/30',
									]"
								>
									{{ item.badge || item.category }}
								</span>
							</div>
						</div>
					</div>

					<!-- Footer Helper -->
					<div
						class="flex items-center justify-between px-4 py-2.5 bg-[#090b0f] border-t border-white/5 text-[11px] text-zinc-400 font-mono"
					>
						<div class="flex items-center gap-3">
							<span class="flex items-center gap-1">
								<kbd
									class="px-1.5 py-0.5 rounded-md bg-white/5 border border-white/10 text-zinc-300 font-mono text-[10px]"
									>↑</kbd
								>
								<kbd
									class="px-1.5 py-0.5 rounded-md bg-white/5 border border-white/10 text-zinc-300 font-mono text-[10px]"
									>↓</kbd
								>
								Navigate
							</span>
							<span class="flex items-center gap-1">
								<kbd
									class="px-1.5 py-0.5 rounded-md bg-white/5 border border-white/10 text-zinc-300 font-mono text-[10px]"
									>↵</kbd
								>
								Select
							</span>
						</div>
						<span class="text-zinc-400">
							Press
							<kbd
								class="px-1.5 py-0.5 rounded-md bg-white/5 border border-white/10 text-zinc-300 font-mono text-[10px]"
								>⌘K / Ctrl+K</kbd
							>
							anywhere
						</span>
					</div>
				</div>
			</div>
		</Transition>
	</Teleport>
</template>
