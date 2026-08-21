<script setup lang="ts">
import {
	BanIcon,
	ChevronDownIcon,
	ClockIcon,
	CompassIcon,
	EyeIcon,
	HeartIcon,
	MessageIcon,
	SignalIcon,
	SkullIcon,
	SparklesIcon,
	StarIcon,
	TagCategoryGamepad2Icon,
	TrashIcon,
	UserXIcon,
} from '@freeplay/assets'
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'

export interface PlayerEntry {
	name: string
	uuid: string
	ip?: string
	joined_at?: number
	is_op?: boolean
	online?: boolean
	ping?: number
	latency?: number
	gamemode?: string
}

export interface BannedEntry {
	name: string
	uuid?: string
	reason: string
	source?: string
	expires?: string
	date: string
}

export interface ModerationListsIpc {
	ops: string[]
	whitelist: string[]
	whitelist_enabled: boolean
	banned_players: BannedEntry[]
}

const props = defineProps<{
	serverStatus: 'offline' | 'starting' | 'online' | 'tunneling'
	canModerate?: boolean
}>()

const emit = defineEmits<{
	(e: 'command', cmd: string): void
}>()

const players = ref<PlayerEntry[]>([])
const isLoading = ref(false)
const searchQuery = ref('')
const activeTab = ref<'online' | 'whitelist' | 'banned' | 'ops'>('online')

// Moderation lists
const whitelistEnabled = ref(false)
const whitelist = ref<string[]>([])
const banList = ref<BannedEntry[]>([])
const opsList = ref<string[]>([])

// Modals
const showKickModal = ref(false)
const showBanModal = ref(false)
const showTimeoutModal = ref(false)
const showTeleportModal = ref(false)
const showMessageModal = ref(false)
const showAddWhitelistModal = ref(false)
const showAddOpModal = ref(false)

const selectedPlayer = ref<PlayerEntry | null>(null)
const actionReason = ref('')
const timeoutDuration = ref('15m')
const teleportTarget = ref('0 80 0')
const messageText = ref('')
const newWhitelistUser = ref('')
const newOpUser = ref('')
const banIp = ref(false)

// Action feedback toast
const actionToast = ref<string | null>(null)
let toastTimer: ReturnType<typeof setTimeout> | null = null

function showToast(msg: string) {
	actionToast.value = msg
	if (toastTimer) clearTimeout(toastTimer)
	toastTimer = setTimeout(() => {
		actionToast.value = null
	}, 3000)
}

const filteredOnlinePlayers = computed(() => {
	if (!searchQuery.value.trim()) return players.value
	const q = searchQuery.value.toLowerCase().trim()
	return players.value.filter(
		(p) =>
			p.name.toLowerCase().includes(q) ||
			p.uuid.toLowerCase().includes(q) ||
			(p.ip && p.ip.toLowerCase().includes(q)) ||
			(p.gamemode && p.gamemode.toLowerCase().includes(q)),
	)
})

const filteredWhitelist = computed(() => {
	if (!searchQuery.value.trim()) return whitelist.value
	const q = searchQuery.value.toLowerCase().trim()
	return whitelist.value.filter((n) => n.toLowerCase().includes(q))
})

const filteredBanList = computed(() => {
	if (!searchQuery.value.trim()) return banList.value
	const q = searchQuery.value.toLowerCase().trim()
	return banList.value.filter(
		(b) => b.name.toLowerCase().includes(q) || b.reason.toLowerCase().includes(q),
	)
})

const filteredOpsList = computed(() => {
	if (!searchQuery.value.trim()) return opsList.value
	const q = searchQuery.value.toLowerCase().trim()
	return opsList.value.filter((n) => n.toLowerCase().includes(q))
})

async function fetchPlayers() {
	if (props.serverStatus !== 'online') {
		players.value = []
		return
	}
	try {
		const res = await invoke<PlayerEntry[]>('host_get_players')
		if (Array.isArray(res)) {
			players.value = res
		}
	} catch (e) {
		console.debug('Failed to fetch players:', e)
	}
}

async function fetchModerationLists() {
	try {
		const res = await invoke<ModerationListsIpc>('host_get_moderation_lists')
		if (res) {
			opsList.value = res.ops || []
			whitelist.value = res.whitelist || []
			whitelistEnabled.value = res.whitelist_enabled || false
			banList.value = res.banned_players || []
		}
	} catch (e) {
		console.debug('Failed to fetch moderation lists:', e)
	}
}

async function refreshAll() {
	isLoading.value = true
	await Promise.all([fetchPlayers(), fetchModerationLists()])
	isLoading.value = false
}

// Player Action Handlers
async function toggleOp(player: PlayerEntry) {
	const newOpState = !player.is_op
	try {
		await invoke('host_player_action', {
			action: newOpState ? 'op' : 'deop',
			player: player.name,
		})
		player.is_op = newOpState
		if (newOpState && !opsList.value.includes(player.name)) {
			opsList.value.push(player.name)
		} else if (!newOpState) {
			opsList.value = opsList.value.filter((n) => n !== player.name)
		}
		showToast(`${player.name} is ${newOpState ? 'now an Operator' : 'no longer an Operator'}`)
	} catch (e) {
		console.error('Failed to change OP status', e)
	}
}

async function setGamemode(player: PlayerEntry, mode: string) {
	try {
		await invoke('host_player_action', {
			action: 'gamemode',
			player: player.name,
			param: mode.toLowerCase(),
		})
		player.gamemode = mode
		showToast(`Changed ${player.name}'s gamemode to ${mode}`)
	} catch (e) {
		console.error('Failed to set gamemode', e)
	}
}

async function healPlayer(player: PlayerEntry) {
	try {
		await invoke('host_player_action', {
			action: 'heal',
			player: player.name,
		})
		showToast(`Healed and fed ${player.name}`)
	} catch (e) {
		console.error('Failed to heal player', e)
	}
}

async function killPlayer(player: PlayerEntry) {
	try {
		await invoke('host_player_action', {
			action: 'kill',
			player: player.name,
		})
		showToast(`Slain ${player.name}`)
	} catch (e) {
		console.error('Failed to kill player', e)
	}
}

async function clearInventory(player: PlayerEntry) {
	try {
		await invoke('host_player_action', {
			action: 'clear',
			player: player.name,
		})
		showToast(`Cleared ${player.name}'s inventory`)
	} catch (e) {
		console.error('Failed to clear inventory', e)
	}
}

function openKick(player: PlayerEntry) {
	selectedPlayer.value = player
	actionReason.value = 'Kicked by server operator'
	showKickModal.value = true
}

function openBan(player: PlayerEntry) {
	selectedPlayer.value = player
	actionReason.value = 'Banned by server operator'
	banIp.value = false
	showBanModal.value = true
}

function openTimeout(player: PlayerEntry) {
	selectedPlayer.value = player
	actionReason.value = 'Temporary timeout'
	timeoutDuration.value = '15m'
	showTimeoutModal.value = true
}

function openTeleport(player: PlayerEntry) {
	selectedPlayer.value = player
	teleportTarget.value = '0 80 0'
	showTeleportModal.value = true
}

function openMessage(player: PlayerEntry) {
	selectedPlayer.value = player
	messageText.value = ''
	showMessageModal.value = true
}

async function confirmKick() {
	if (!selectedPlayer.value) return
	const pName = selectedPlayer.value.name
	try {
		await invoke('host_player_action', {
			action: 'kick',
			player: pName,
			param: actionReason.value || 'Kicked by operator',
		})
		players.value = players.value.filter((p) => p.name !== pName)
		showKickModal.value = false
		showToast(`Kicked ${pName}`)
	} catch (e) {
		console.error('Kick failed', e)
	}
}

async function confirmBan() {
	if (!selectedPlayer.value) return
	const pName = selectedPlayer.value.name
	const reason = actionReason.value || 'Banned by operator'
	try {
		await invoke('host_player_action', {
			action: banIp.value ? 'ban_ip' : 'ban',
			player: pName,
			param: reason,
		})
		banList.value.push({
			name: pName,
			reason,
			date: new Date().toISOString().split('T')[0],
		})
		players.value = players.value.filter((p) => p.name !== pName)
		showBanModal.value = false
		showToast(`Banned ${pName}`)
	} catch (e) {
		console.error('Ban failed', e)
	}
}

async function confirmTimeout() {
	if (!selectedPlayer.value) return
	const pName = selectedPlayer.value.name
	const reason = actionReason.value || 'Temporary timeout'
	try {
		await invoke('host_player_action', {
			action: 'timeout',
			player: pName,
			param: `${timeoutDuration.value} ${reason}`,
		})
		players.value = players.value.filter((p) => p.name !== pName)
		showTimeoutModal.value = false
		showToast(`Timed out ${pName} for ${timeoutDuration.value}`)
	} catch (e) {
		console.error('Timeout failed', e)
	}
}

async function confirmTeleport() {
	if (!selectedPlayer.value) return
	const pName = selectedPlayer.value.name
	try {
		await invoke('host_player_action', {
			action: 'tp',
			player: pName,
			param: teleportTarget.value || '0 80 0',
		})
		showTeleportModal.value = false
		showToast(`Teleported ${pName} to ${teleportTarget.value}`)
	} catch (e) {
		console.error('Teleport failed', e)
	}
}

async function confirmMessage() {
	if (!selectedPlayer.value || !messageText.value.trim()) return
	const pName = selectedPlayer.value.name
	try {
		await invoke('host_player_action', {
			action: 'msg',
			player: pName,
			param: messageText.value.trim(),
		})
		showMessageModal.value = false
		showToast(`Sent message to ${pName}`)
	} catch (e) {
		console.error('Message failed', e)
	}
}

async function toggleWhitelist() {
	whitelistEnabled.value = !whitelistEnabled.value
	try {
		await invoke('host_player_action', {
			action: whitelistEnabled.value ? 'whitelist_on' : 'whitelist_off',
			player: 'server',
		})
		showToast(`Whitelist is now ${whitelistEnabled.value ? 'Active' : 'Disabled'}`)
	} catch {
		emit('command', whitelistEnabled.value ? 'whitelist on' : 'whitelist off')
	}
}

async function addWhitelist() {
	const name = newWhitelistUser.value.trim()
	if (name && !whitelist.value.includes(name)) {
		try {
			await invoke('host_player_action', {
				action: 'whitelist_add',
				player: name,
			})
			whitelist.value.push(name)
			newWhitelistUser.value = ''
			showAddWhitelistModal.value = false
			showToast(`Added ${name} to whitelist`)
		} catch {
			emit('command', `whitelist add ${name}`)
		}
	}
}

async function removeWhitelist(name: string) {
	try {
		await invoke('host_player_action', {
			action: 'whitelist_remove',
			player: name,
		})
		whitelist.value.filter((n) => n !== name)
		showToast(`Removed ${name} from whitelist`)
	} catch {
		emit('command', `whitelist remove ${name}`)
	}
}

async function addOp() {
	const name = newOpUser.value.trim()
	if (name && !opsList.value.includes(name)) {
		try {
			await invoke('host_player_action', {
				action: 'op',
				player: name,
			})
			opsList.value.push(name)
			newOpUser.value = ''
			showAddOpModal.value = false
			showToast(`Made ${name} an Operator`)
		} catch {
			emit('command', `op ${name}`)
		}
	}
}

async function removeOp(name: string) {
	try {
		await invoke('host_player_action', {
			action: 'deop',
			player: name,
		})
		opsList.value = opsList.value.filter((n) => n !== name)
		showToast(`Removed Operator status from ${name}`)
	} catch {
		emit('command', `deop ${name}`)
	}
}

async function unbanPlayer(name: string) {
	try {
		await invoke('host_player_action', {
			action: 'pardon',
			player: name,
		})
		banList.value = banList.value.filter((b) => b.name !== name)
		showToast(`Unbanned ${name}`)
	} catch {
		emit('command', `pardon ${name}`)
	}
}

let pollInterval: ReturnType<typeof setInterval> | null = null

onMounted(() => {
	void refreshAll()
	pollInterval = setInterval(() => {
		if (props.serverStatus === 'online') {
			void fetchPlayers()
		}
	}, 1500)
})

onUnmounted(() => {
	if (pollInterval) clearInterval(pollInterval)
	if (toastTimer) clearTimeout(toastTimer)
})

watch(
	() => props.serverStatus,
	(newStatus) => {
		if (newStatus === 'online') {
			void refreshAll()
		} else {
			players.value = []
		}
	},
)
</script>

<template>
	<div class="flex flex-col gap-4 relative">
		<!-- Action Toast Notification -->
		<transition
			enter-active-class="transition duration-200 ease-out"
			enter-from-class="transform -translate-y-2 opacity-0"
			enter-to-class="transform translate-y-0 opacity-100"
			leave-active-class="transition duration-150 ease-in"
			leave-from-class="transform translate-y-0 opacity-100"
			leave-to-class="transform -translate-y-2 opacity-0"
		>
			<div
				v-if="actionToast"
				class="fixed top-6 right-6 z-50 px-4 py-2.5 rounded-xl bg-gradient-to-r from-cyan-600 to-blue-600 text-white font-bold text-xs shadow-2xl flex items-center gap-2 border border-cyan-400/40 backdrop-blur-md"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="w-4 h-4 text-cyan-200"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2.5"
				>
					<polyline points="20 6 9 17 4 12" />
				</svg>
				<span>{{ actionToast }}</span>
			</div>
		</transition>

		<!-- Sub-Navigation & Searchbar Bar -->
		<div
			class="flex flex-col lg:flex-row items-stretch lg:items-center justify-between gap-3 p-4 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md"
		>
			<!-- Navigation Tabs -->
			<div
				class="flex items-center gap-1.5 bg-[#0D1117] p-1.5 rounded-xl border border-white/10 overflow-x-auto"
			>
				<button
					type="button"
					class="px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer border-none flex items-center gap-1.5 shrink-0"
					:class="
						activeTab === 'online'
							? 'bg-cyan-500 text-zinc-950 shadow-[0_0_12px_rgba(6,182,212,0.4)]'
							: 'text-zinc-400 hover:text-white bg-transparent'
					"
					@click="activeTab = 'online'"
				>
					<span
						class="w-2 h-2 rounded-full"
						:class="players.length > 0 ? 'bg-emerald-400 animate-pulse' : 'bg-zinc-500'"
					/>
					<span>Online Roster ({{ players.length }})</span>
				</button>
				<button
					type="button"
					class="px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer border-none flex items-center gap-1.5 shrink-0"
					:class="
						activeTab === 'whitelist'
							? 'bg-cyan-500 text-zinc-950 shadow-[0_0_12px_rgba(6,182,212,0.4)]'
							: 'text-zinc-400 hover:text-white bg-transparent'
					"
					@click="activeTab = 'whitelist'"
				>
					<span>Whitelist ({{ whitelist.length }})</span>
				</button>
				<button
					type="button"
					class="px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer border-none flex items-center gap-1.5 shrink-0"
					:class="
						activeTab === 'banned'
							? 'bg-cyan-500 text-zinc-950 shadow-[0_0_12px_rgba(6,182,212,0.4)]'
							: 'text-zinc-400 hover:text-white bg-transparent'
					"
					@click="activeTab = 'banned'"
				>
					<span>Ban List ({{ banList.length }})</span>
				</button>
				<button
					type="button"
					class="px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer border-none flex items-center gap-1.5 shrink-0"
					:class="
						activeTab === 'ops'
							? 'bg-cyan-500 text-zinc-950 shadow-[0_0_12px_rgba(6,182,212,0.4)]'
							: 'text-zinc-400 hover:text-white bg-transparent'
					"
					@click="activeTab = 'ops'"
				>
					<span>Operators ({{ opsList.length }})</span>
				</button>
			</div>

			<!-- Searchbar & Controls -->
			<div class="flex items-center gap-2.5">
				<!-- Search Input -->
				<div class="relative flex-1 lg:w-72">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-zinc-400 pointer-events-none"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<circle cx="11" cy="11" r="8" />
						<line x1="21" y1="21" x2="16.65" y2="16.65" />
					</svg>
					<input
						v-model="searchQuery"
						type="text"
						placeholder="Search players by name, UUID, IP..."
						class="w-full pl-9 pr-8 py-2 rounded-xl bg-[#0D1117] border border-white/10 text-white text-xs placeholder-zinc-500 focus:outline-none focus:border-cyan-400 transition-colors shadow-inner"
					/>
					<button
						v-if="searchQuery"
						type="button"
						class="absolute right-2.5 top-1/2 -translate-y-1/2 text-zinc-400 hover:text-white border-none bg-transparent cursor-pointer text-xs"
						@click="searchQuery = ''"
					>
						✕
					</button>
				</div>

				<!-- Action Tab Trigger Buttons -->
				<button
					v-if="activeTab === 'whitelist'"
					type="button"
					class="px-3.5 py-2 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-zinc-950 text-xs font-bold border-none transition-all cursor-pointer shadow-[0_0_15px_rgba(6,182,212,0.3)] shrink-0 flex items-center gap-1.5"
					@click="showAddWhitelistModal = true"
				>
					<span>+ Whitelist Player</span>
				</button>

				<button
					v-if="activeTab === 'ops'"
					type="button"
					class="px-3.5 py-2 rounded-xl bg-amber-500 hover:bg-amber-400 text-zinc-950 text-xs font-bold border-none transition-all cursor-pointer shadow-[0_0_15px_rgba(245,158,11,0.3)] shrink-0 flex items-center gap-1.5"
					@click="showAddOpModal = true"
				>
					<span>+ Add Operator</span>
				</button>

				<!-- Refresh Button -->
				<button
					type="button"
					class="p-2 rounded-xl bg-[#0D1117] border border-white/10 text-zinc-400 hover:text-white hover:border-white/20 transition-all cursor-pointer shrink-0"
					:class="{ 'animate-spin': isLoading }"
					title="Refresh Player List"
					@click="refreshAll"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-4 h-4"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" />
						<path d="M21 3v5h-5" />
						<path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16" />
						<path d="M8 16H3v5" />
					</svg>
				</button>
			</div>
		</div>

		<!-- TAB 1: ONLINE ROSTER (ONE PLAYER PER ROW) -->
		<div v-if="activeTab === 'online'" class="flex flex-col gap-3">
			<!-- Server Offline State -->
			<div
				v-if="serverStatus !== 'online'"
				class="p-12 rounded-2xl bg-[#141923]/80 border border-white/10 text-center flex flex-col items-center justify-center gap-3 backdrop-blur-md"
			>
				<div
					class="w-12 h-12 rounded-2xl bg-zinc-800/80 border border-white/10 flex items-center justify-center text-zinc-400"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-6 h-6"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<circle cx="12" cy="12" r="10" />
						<line x1="4.93" y1="4.93" x2="19.07" y2="19.07" />
					</svg>
				</div>
				<p class="m-0 text-base font-bold text-white">Server is Offline</p>
				<p class="m-0 text-xs text-zinc-400 max-w-sm">
					Start your dedicated server to view connected players, live latency, and execute instant
					moderation commands.
				</p>
			</div>

			<!-- No Players State -->
			<div
				v-else-if="filteredOnlinePlayers.length === 0"
				class="p-12 rounded-2xl bg-[#141923]/80 border border-white/10 text-center flex flex-col items-center justify-center gap-3 backdrop-blur-md"
			>
				<div
					class="w-12 h-12 rounded-2xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-[0_0_15px_rgba(6,182,212,0.2)]"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-6 h-6"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
						<circle cx="9" cy="7" r="4" />
						<path d="M23 21v-2a4 4 0 0 0-3-3.87" />
						<path d="M16 3.13a4 4 0 0 1 0 7.75" />
					</svg>
				</div>
				<p class="m-0 text-base font-bold text-white">
					{{ searchQuery ? 'No Matching Players Found' : 'No Players Online' }}
				</p>
				<p class="m-0 text-xs text-zinc-400 max-w-sm">
					{{
						searchQuery
							? 'Try adjusting your search query or clear the filter.'
							: 'Share your server IP or join on localhost to see players appear here in real time.'
					}}
				</p>
			</div>

			<!-- ONE PLAYER IN A ROW CARDS -->
			<div v-else class="flex flex-col gap-2.5">
				<div
					v-for="player in filteredOnlinePlayers"
					:key="player.uuid || player.name"
					class="p-4 rounded-2xl bg-[#141923]/90 border border-white/10 hover:border-cyan-500/40 transition-all duration-200 flex flex-col lg:flex-row lg:items-center justify-between gap-4 shadow-lg backdrop-blur-md group"
				>
					<!-- Left Section: Avatar + Identity Info -->
					<div class="flex items-center gap-3.5 min-w-0">
						<!-- Avatar Head -->
						<div
							class="relative w-12 h-12 rounded-2xl bg-[#0D1117] border border-white/15 flex items-center justify-center shrink-0 overflow-hidden shadow-md group-hover:scale-105 transition-transform"
						>
							<img
								:src="`https://mc-heads.net/avatar/${player.name}/64`"
								class="w-full h-full object-cover"
								:alt="player.name"
								loading="lazy"
							/>
							<span
								v-if="player.is_op"
								class="absolute -top-1 -right-1 w-4.5 h-4.5 rounded-full bg-amber-500 text-zinc-950 flex items-center justify-center shadow-[0_0_8px_rgba(245,158,11,0.6)]"
								title="Server Operator"
							>
								<StarIcon class="w-3 h-3 text-zinc-950 fill-current" />
							</span>
						</div>

						<!-- Name & Badges -->
						<div class="flex flex-col gap-1 min-w-0">
							<div class="flex items-center gap-2 flex-wrap">
								<span class="text-base font-black text-white truncate">{{ player.name }}</span>

								<!-- Operator Badge -->
								<span
									v-if="player.is_op"
									class="px-2 py-0.5 rounded-md text-[10px] font-black bg-amber-500/20 text-amber-300 border border-amber-500/40 shadow-[0_0_10px_rgba(245,158,11,0.2)] flex items-center gap-1"
								>
									<StarIcon class="w-3 h-3 text-amber-400 fill-current" />
									<span>OPERATOR</span>
								</span>

								<!-- Online Status Badge -->
								<span
									class="px-2 py-0.5 rounded-md text-[10px] font-bold bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 flex items-center gap-1"
								>
									<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
									<span>ONLINE</span>
								</span>
							</div>

							<!-- Metrics Row: Ping, Gamemode, IP -->
							<div class="flex items-center gap-3 text-xs text-zinc-400 flex-wrap">
								<!-- Latency / Ping -->
								<span class="flex items-center gap-1 font-mono text-[11px] text-zinc-300">
									<SignalIcon class="w-3.5 h-3.5 text-emerald-400 shrink-0" />
									<span>{{ player.ping ?? player.latency ?? 15 }}ms</span>
								</span>

								<span class="text-zinc-600">•</span>

								<!-- Gamemode Tag -->
								<span
									class="px-2 py-0.5 rounded bg-zinc-800/80 text-zinc-300 text-[11px] font-semibold border border-white/5 flex items-center gap-1.5"
								>
									<SparklesIcon
										v-if="player.gamemode?.toLowerCase() === 'creative'"
										class="w-3 h-3 text-amber-400 shrink-0"
									/>
									<CompassIcon
										v-else-if="player.gamemode?.toLowerCase() === 'adventure'"
										class="w-3 h-3 text-cyan-400 shrink-0"
									/>
									<EyeIcon
										v-else-if="player.gamemode?.toLowerCase() === 'spectator'"
										class="w-3 h-3 text-indigo-400 shrink-0"
									/>
									<TagCategoryGamepad2Icon v-else class="w-3 h-3 text-purple-400 shrink-0" />
									<span>{{ player.gamemode || 'Survival' }}</span>
								</span>

								<span v-if="player.ip" class="text-zinc-600">•</span>
								<span v-if="player.ip" class="font-mono text-[11px] text-zinc-400">
									IP: {{ player.ip }}
								</span>
							</div>
						</div>
					</div>

					<!-- Right Section: Common Action Commands Bar -->
					<div
						class="flex items-center gap-1.5 flex-wrap shrink-0 justify-end pt-2 lg:pt-0 border-t lg:border-t-0 border-white/5"
					>
						<!-- Toggle OP Status Button -->
						<button
							type="button"
							class="px-3 py-1.5 rounded-xl border text-xs font-bold transition-all cursor-pointer flex items-center gap-1.5 shadow-sm"
							:class="
								player.is_op
									? 'bg-amber-500/20 border-amber-500/40 text-amber-300 hover:bg-amber-500/30 shadow-[0_0_10px_rgba(245,158,11,0.2)]'
									: 'bg-[#0D1117] border-white/10 text-zinc-300 hover:text-white hover:border-white/20'
							"
							:title="player.is_op ? 'Demote Operator' : 'Promote to Operator'"
							@click="toggleOp(player)"
						>
							<StarIcon
								class="w-3.5 h-3.5 shrink-0"
								:class="player.is_op ? 'text-amber-400 fill-current' : 'text-zinc-400'"
							/>
							<span>{{ player.is_op ? 'De-OP' : 'Make OP' }}</span>
						</button>

						<!-- Gamemode Quick Selector -->
						<div class="relative group/gm">
							<button
								type="button"
								class="px-2.5 py-1.5 rounded-xl bg-[#0D1117] border border-white/10 text-zinc-300 hover:text-white hover:border-white/20 text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5"
								title="Change Gamemode"
							>
								<span>Mode</span>
								<ChevronDownIcon
									class="w-3.5 h-3.5 text-zinc-400 group-hover/gm:text-white transition-colors shrink-0"
								/>
							</button>
							<div
								class="absolute right-0 top-full mt-1 hidden group-hover/gm:flex flex-col bg-[#0D1117] border border-white/15 rounded-xl shadow-2xl p-1 z-30 min-w-36"
							>
								<button
									type="button"
									class="px-3 py-1.5 rounded-lg text-left text-xs font-semibold text-zinc-300 hover:bg-cyan-500 hover:text-zinc-950 border-none transition-colors cursor-pointer flex items-center gap-2"
									@click="setGamemode(player, 'Survival')"
								>
									<TagCategoryGamepad2Icon class="w-3.5 h-3.5 text-purple-400" />
									<span>Survival</span>
								</button>
								<button
									type="button"
									class="px-3 py-1.5 rounded-lg text-left text-xs font-semibold text-zinc-300 hover:bg-cyan-500 hover:text-zinc-950 border-none transition-colors cursor-pointer flex items-center gap-2"
									@click="setGamemode(player, 'Creative')"
								>
									<SparklesIcon class="w-3.5 h-3.5 text-amber-400" />
									<span>Creative</span>
								</button>
								<button
									type="button"
									class="px-3 py-1.5 rounded-lg text-left text-xs font-semibold text-zinc-300 hover:bg-cyan-500 hover:text-zinc-950 border-none transition-colors cursor-pointer flex items-center gap-2"
									@click="setGamemode(player, 'Adventure')"
								>
									<CompassIcon class="w-3.5 h-3.5 text-cyan-400" />
									<span>Adventure</span>
								</button>
								<button
									type="button"
									class="px-3 py-1.5 rounded-lg text-left text-xs font-semibold text-zinc-300 hover:bg-cyan-500 hover:text-zinc-950 border-none transition-colors cursor-pointer flex items-center gap-2"
									@click="setGamemode(player, 'Spectator')"
								>
									<EyeIcon class="w-3.5 h-3.5 text-indigo-400" />
									<span>Spectator</span>
								</button>
							</div>
						</div>

						<!-- Heal Button -->
						<button
							type="button"
							class="px-2.5 py-1.5 rounded-xl bg-[#0D1117] border border-white/10 text-emerald-400 hover:bg-emerald-500/10 hover:border-emerald-500/30 text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5"
							title="Heal and Feed Player"
							@click="healPlayer(player)"
						>
							<HeartIcon class="w-3.5 h-3.5 text-emerald-400 shrink-0" />
							<span class="hidden sm:inline">Heal</span>
						</button>

						<!-- Clear Inv Button -->
						<button
							type="button"
							class="px-2.5 py-1.5 rounded-xl bg-[#0D1117] border border-white/10 text-amber-400 hover:bg-amber-500/10 hover:border-amber-500/30 text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5"
							title="Clear Player Inventory"
							@click="clearInventory(player)"
						>
							<TrashIcon class="w-3.5 h-3.5 text-amber-400 shrink-0" />
							<span class="hidden sm:inline">Clear</span>
						</button>

						<!-- Kill Button -->
						<button
							type="button"
							class="px-2.5 py-1.5 rounded-xl bg-[#0D1117] border border-white/10 text-rose-400 hover:bg-rose-500/10 hover:border-rose-500/30 text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5"
							title="Kill Player"
							@click="killPlayer(player)"
						>
							<SkullIcon class="w-3.5 h-3.5 text-rose-400 shrink-0" />
							<span class="hidden sm:inline">Kill</span>
						</button>

						<!-- Teleport Button -->
						<button
							type="button"
							class="px-2.5 py-1.5 rounded-xl bg-[#0D1117] border border-white/10 text-cyan-400 hover:bg-cyan-500/10 hover:border-cyan-500/30 text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5"
							title="Teleport Player"
							@click="openTeleport(player)"
						>
							<CompassIcon class="w-3.5 h-3.5 text-cyan-400 shrink-0" />
							<span class="hidden sm:inline">TP</span>
						</button>

						<!-- Direct Whisper Message Button -->
						<button
							type="button"
							class="px-2.5 py-1.5 rounded-xl bg-[#0D1117] border border-white/10 text-sky-400 hover:bg-sky-500/10 hover:border-sky-500/30 text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5"
							title="Send Private Message"
							@click="openMessage(player)"
						>
							<MessageIcon class="w-3.5 h-3.5 text-sky-400 shrink-0" />
							<span class="hidden sm:inline">Msg</span>
						</button>

						<!-- Timeout Button -->
						<button
							type="button"
							class="px-2.5 py-1.5 rounded-xl bg-[#0D1117] border border-white/10 text-orange-400 hover:bg-orange-500/10 hover:border-orange-500/30 text-xs font-semibold transition-all cursor-pointer flex items-center gap-1.5"
							title="Temporary Timeout / Mute"
							@click="openTimeout(player)"
						>
							<ClockIcon class="w-3.5 h-3.5 text-orange-400 shrink-0" />
							<span class="hidden sm:inline">Timeout</span>
						</button>

						<!-- Kick Button -->
						<button
							type="button"
							class="px-2.5 py-1.5 rounded-xl bg-amber-500/10 border border-amber-500/30 text-amber-300 hover:bg-amber-500 hover:text-zinc-950 text-xs font-bold transition-all cursor-pointer flex items-center gap-1.5 shadow-sm"
							title="Kick Player from Server"
							@click="openKick(player)"
						>
							<UserXIcon class="w-3.5 h-3.5 shrink-0" />
							<span>Kick</span>
						</button>

						<!-- Ban Button -->
						<button
							type="button"
							class="px-2.5 py-1.5 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-400 hover:bg-rose-500 hover:text-white text-xs font-bold transition-all cursor-pointer flex items-center gap-1.5 shadow-sm"
							title="Ban Player from Server"
							@click="openBan(player)"
						>
							<BanIcon class="w-3.5 h-3.5 shrink-0" />
							<span>Ban</span>
						</button>
					</div>
				</div>
			</div>
		</div>

		<!-- TAB 2: WHITELIST -->
		<div v-if="activeTab === 'whitelist'" class="flex flex-col gap-3">
			<div
				class="p-4 rounded-2xl bg-[#141923]/90 border border-white/10 flex items-center justify-between gap-4 shadow-md backdrop-blur-md"
			>
				<div class="flex flex-col gap-0.5">
					<span class="text-sm font-bold text-white">Enforce Server Whitelist</span>
					<span class="text-xs text-zinc-400"
						>When enabled, only players explicitly listed below can connect to this server.</span
					>
				</div>
				<button
					type="button"
					class="px-4 py-2 rounded-xl text-xs font-bold transition-all cursor-pointer border-none shadow-md"
					:class="
						whitelistEnabled
							? 'bg-emerald-500 text-zinc-950 shadow-[0_0_15px_rgba(16,185,129,0.4)]'
							: 'bg-[#0D1117] text-zinc-400 border border-white/10 hover:text-white'
					"
					@click="toggleWhitelist"
				>
					{{ whitelistEnabled ? '✓ Whitelist Active' : 'Whitelist Disabled' }}
				</button>
			</div>

			<div
				v-if="filteredWhitelist.length === 0"
				class="p-12 rounded-2xl bg-[#141923]/80 border border-white/10 text-center"
			>
				<p class="m-0 text-sm font-bold text-white">No Whitelisted Players</p>
				<p class="m-0 text-xs text-zinc-400">
					Add players to restrict who can connect when whitelist enforcement is enabled.
				</p>
			</div>

			<div v-else class="flex flex-col gap-2">
				<div
					v-for="name in filteredWhitelist"
					:key="name"
					class="p-3.5 rounded-2xl bg-[#141923]/90 border border-white/10 flex items-center justify-between gap-3 shadow-md backdrop-blur-md"
				>
					<div class="flex items-center gap-3 min-w-0">
						<img
							:src="`https://mc-heads.net/avatar/${name}/32`"
							class="w-8 h-8 rounded-lg object-cover bg-[#0D1117] border border-white/10"
							alt=""
						/>
						<span class="text-xs font-bold text-white truncate">{{ name }}</span>
					</div>
					<button
						type="button"
						class="px-3 py-1.5 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-400 hover:bg-rose-500 hover:text-white transition-all cursor-pointer text-xs font-semibold"
						title="Remove from whitelist"
						@click="removeWhitelist(name)"
					>
						Remove
					</button>
				</div>
			</div>
		</div>

		<!-- TAB 3: BAN LIST -->
		<div v-if="activeTab === 'banned'" class="flex flex-col gap-3">
			<div
				v-if="filteredBanList.length === 0"
				class="p-12 rounded-2xl bg-[#141923]/80 border border-white/10 text-center"
			>
				<p class="m-0 text-sm font-bold text-white">No Banned Players</p>
				<p class="m-0 text-xs text-zinc-400">Your ban list is clean.</p>
			</div>

			<div v-else class="flex flex-col gap-2">
				<div
					v-for="item in filteredBanList"
					:key="item.name"
					class="p-3.5 rounded-2xl bg-[#141923]/90 border border-white/10 flex items-center justify-between gap-3 shadow-md backdrop-blur-md"
				>
					<div class="flex items-center gap-3 min-w-0">
						<img
							:src="`https://mc-heads.net/avatar/${item.name}/32`"
							class="w-9 h-9 rounded-xl object-cover bg-[#0D1117] border border-white/10"
							alt=""
						/>
						<div class="flex flex-col min-w-0">
							<span class="text-xs font-bold text-white truncate">{{ item.name }}</span>
							<span class="text-[11px] text-zinc-400 truncate"
								>Reason: {{ item.reason }} <span v-if="item.date">({{ item.date }})</span></span
							>
						</div>
					</div>
					<button
						type="button"
						class="px-3.5 py-1.5 rounded-xl bg-[#0D1117] border border-white/10 text-xs font-semibold text-emerald-400 hover:bg-emerald-500/10 hover:border-emerald-500/30 transition-all cursor-pointer"
						@click="unbanPlayer(item.name)"
					>
						Pardon / Unban
					</button>
				</div>
			</div>
		</div>

		<!-- TAB 4: OPERATORS LIST -->
		<div v-if="activeTab === 'ops'" class="flex flex-col gap-3">
			<div
				v-if="filteredOpsList.length === 0"
				class="p-12 rounded-2xl bg-[#141923]/80 border border-white/10 text-center"
			>
				<p class="m-0 text-sm font-bold text-white">No Server Operators</p>
				<p class="m-0 text-xs text-zinc-400">
					Add operators to grant server administrative privileges.
				</p>
			</div>

			<div v-else class="flex flex-col gap-2">
				<div
					v-for="name in filteredOpsList"
					:key="name"
					class="p-3.5 rounded-2xl bg-[#141923]/90 border border-white/10 flex items-center justify-between gap-3 shadow-md backdrop-blur-md"
				>
					<div class="flex items-center gap-3 min-w-0">
						<img
							:src="`https://mc-heads.net/avatar/${name}/32`"
							class="w-8 h-8 rounded-lg object-cover bg-[#0D1117] border border-white/10"
							alt=""
						/>
						<div class="flex items-center gap-2">
							<span class="text-xs font-bold text-white truncate">{{ name }}</span>
							<span
								class="px-2 py-0.5 rounded text-[10px] font-black bg-amber-500/20 text-amber-300 border border-amber-500/40"
								>OP</span
							>
						</div>
					</div>
					<button
						type="button"
						class="px-3 py-1.5 rounded-xl bg-amber-500/10 border border-amber-500/30 text-amber-300 hover:bg-amber-500 hover:text-zinc-950 transition-all cursor-pointer text-xs font-semibold"
						title="Revoke Operator Status"
						@click="removeOp(name)"
					>
						Demote from OP
					</button>
				</div>
			</div>
		</div>

		<!-- MODAL: KICK PLAYER -->
		<div
			v-if="showKickModal"
			class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-6 rounded-3xl bg-[#141923] border border-white/15 shadow-2xl flex flex-col gap-5"
			>
				<div class="flex items-center justify-between">
					<h3 class="text-base font-black text-white m-0 flex items-center gap-2">
						<UserXIcon class="w-5 h-5 text-amber-400 shrink-0" />
						<span>Kick Player: {{ selectedPlayer?.name }}</span>
					</h3>
					<button
						type="button"
						class="text-zinc-400 hover:text-white border-none bg-transparent cursor-pointer text-sm"
						@click="showKickModal = false"
					>
						✕
					</button>
				</div>

				<div class="flex flex-col gap-2">
					<label class="text-xs font-bold text-zinc-300">Reason for Kick</label>
					<input
						v-model="actionReason"
						type="text"
						placeholder="e.g. Inappropriate behavior / AFK"
						class="px-3.5 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-white text-xs focus:outline-none focus:border-amber-400"
					/>
				</div>

				<div class="flex items-center justify-end gap-2.5 pt-2">
					<button
						type="button"
						class="px-4 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-xs font-bold text-zinc-300 hover:text-white cursor-pointer"
						@click="showKickModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-5 py-2.5 rounded-xl bg-amber-500 hover:bg-amber-400 text-zinc-950 text-xs font-black border-none cursor-pointer shadow-[0_0_15px_rgba(245,158,11,0.4)]"
						@click="confirmKick"
					>
						Confirm Kick
					</button>
				</div>
			</div>
		</div>

		<!-- MODAL: BAN PLAYER -->
		<div
			v-if="showBanModal"
			class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-6 rounded-3xl bg-[#141923] border border-white/15 shadow-2xl flex flex-col gap-5"
			>
				<div class="flex items-center justify-between">
					<h3 class="text-base font-black text-rose-400 m-0 flex items-center gap-2">
						<BanIcon class="w-5 h-5 text-rose-400 shrink-0" />
						<span>Ban Player: {{ selectedPlayer?.name }}</span>
					</h3>
					<button
						type="button"
						class="text-zinc-400 hover:text-white border-none bg-transparent cursor-pointer text-sm"
						@click="showBanModal = false"
					>
						✕
					</button>
				</div>

				<div class="flex flex-col gap-2">
					<label class="text-xs font-bold text-zinc-300">Reason for Ban</label>
					<input
						v-model="actionReason"
						type="text"
						placeholder="e.g. Griefing / Cheating"
						class="px-3.5 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-white text-xs focus:outline-none focus:border-rose-400"
					/>
				</div>

				<div class="flex items-center gap-2">
					<input
						id="ban-ip-check"
						v-model="banIp"
						type="checkbox"
						class="rounded bg-[#0D1117] border-white/10 text-rose-500 focus:ring-rose-400"
					/>
					<label for="ban-ip-check" class="text-xs text-zinc-300 font-semibold cursor-pointer">
						Also Ban Player IP Address
					</label>
				</div>

				<div class="flex items-center justify-end gap-2.5 pt-2">
					<button
						type="button"
						class="px-4 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-xs font-bold text-zinc-300 hover:text-white cursor-pointer"
						@click="showBanModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-5 py-2.5 rounded-xl bg-rose-500 hover:bg-rose-600 text-white text-xs font-black border-none cursor-pointer shadow-[0_0_15px_rgba(244,63,94,0.4)]"
						@click="confirmBan"
					>
						Confirm Ban
					</button>
				</div>
			</div>
		</div>

		<!-- MODAL: TIMEOUT PLAYER -->
		<div
			v-if="showTimeoutModal"
			class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-6 rounded-3xl bg-[#141923] border border-white/15 shadow-2xl flex flex-col gap-5"
			>
				<div class="flex items-center justify-between">
					<h3 class="text-base font-black text-orange-400 m-0 flex items-center gap-2">
						<ClockIcon class="w-5 h-5 text-orange-400 shrink-0" />
						<span>Timeout Player: {{ selectedPlayer?.name }}</span>
					</h3>
					<button
						type="button"
						class="text-zinc-400 hover:text-white border-none bg-transparent cursor-pointer text-sm"
						@click="showTimeoutModal = false"
					>
						✕
					</button>
				</div>

				<div class="flex flex-col gap-2">
					<label class="text-xs font-bold text-zinc-300">Timeout Duration</label>
					<div class="grid grid-cols-4 gap-2">
						<button
							v-for="dur in ['5m', '15m', '1h', '24h']"
							:key="dur"
							type="button"
							class="py-2 rounded-xl text-xs font-bold transition-all border-none cursor-pointer"
							:class="
								timeoutDuration === dur
									? 'bg-orange-500 text-zinc-950 font-black'
									: 'bg-[#0D1117] text-zinc-400 hover:text-white'
							"
							@click="timeoutDuration = dur"
						>
							{{ dur }}
						</button>
					</div>
				</div>

				<div class="flex flex-col gap-2">
					<label class="text-xs font-bold text-zinc-300">Reason</label>
					<input
						v-model="actionReason"
						type="text"
						placeholder="e.g. Chat spam / Cooling off"
						class="px-3.5 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-white text-xs focus:outline-none focus:border-orange-400"
					/>
				</div>

				<div class="flex items-center justify-end gap-2.5 pt-2">
					<button
						type="button"
						class="px-4 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-xs font-bold text-zinc-300 hover:text-white cursor-pointer"
						@click="showTimeoutModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-5 py-2.5 rounded-xl bg-orange-500 hover:bg-orange-400 text-zinc-950 text-xs font-black border-none cursor-pointer shadow-[0_0_15px_rgba(249,115,22,0.4)]"
						@click="confirmTimeout"
					>
						Apply Timeout
					</button>
				</div>
			</div>
		</div>

		<!-- MODAL: TELEPORT PLAYER -->
		<div
			v-if="showTeleportModal"
			class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-6 rounded-3xl bg-[#141923] border border-white/15 shadow-2xl flex flex-col gap-5"
			>
				<div class="flex items-center justify-between">
					<h3 class="text-base font-black text-cyan-400 m-0 flex items-center gap-2">
						<span>🌐</span>
						<span>Teleport: {{ selectedPlayer?.name }}</span>
					</h3>
					<button
						type="button"
						class="text-zinc-400 hover:text-white border-none bg-transparent cursor-pointer text-sm"
						@click="showTeleportModal = false"
					>
						✕
					</button>
				</div>

				<div class="flex flex-col gap-2">
					<label class="text-xs font-bold text-zinc-300">Target Coordinates or Destination</label>
					<input
						v-model="teleportTarget"
						type="text"
						placeholder="e.g. 0 80 0 or player name"
						class="px-3.5 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-white text-xs focus:outline-none focus:border-cyan-400 font-mono"
					/>
					<div class="flex items-center gap-2 pt-1">
						<button
							type="button"
							class="px-2.5 py-1 rounded-lg bg-[#0D1117] border border-white/10 text-[11px] text-zinc-400 hover:text-white cursor-pointer"
							@click="teleportTarget = '0 80 0'"
						>
							Spawn (0, 80, 0)
						</button>
						<button
							type="button"
							class="px-2.5 py-1 rounded-lg bg-[#0D1117] border border-white/10 text-[11px] text-zinc-400 hover:text-white cursor-pointer"
							@click="teleportTarget = '~ ~10 ~'"
						>
							Up 10 blocks
						</button>
					</div>
				</div>

				<div class="flex items-center justify-end gap-2.5 pt-2">
					<button
						type="button"
						class="px-4 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-xs font-bold text-zinc-300 hover:text-white cursor-pointer"
						@click="showTeleportModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-5 py-2.5 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-zinc-950 text-xs font-black border-none cursor-pointer shadow-[0_0_15px_rgba(6,182,212,0.4)]"
						@click="confirmTeleport"
					>
						Teleport
					</button>
				</div>
			</div>
		</div>

		<!-- MODAL: SEND PRIVATE MESSAGE -->
		<div
			v-if="showMessageModal"
			class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-6 rounded-3xl bg-[#141923] border border-white/15 shadow-2xl flex flex-col gap-5"
			>
				<div class="flex items-center justify-between">
					<h3 class="text-base font-black text-sky-400 m-0 flex items-center gap-2">
						<span>💬</span>
						<span>Message: {{ selectedPlayer?.name }}</span>
					</h3>
					<button
						type="button"
						class="text-zinc-400 hover:text-white border-none bg-transparent cursor-pointer text-sm"
						@click="showMessageModal = false"
					>
						✕
					</button>
				</div>

				<div class="flex flex-col gap-2">
					<label class="text-xs font-bold text-zinc-300">Message Text (Whisper)</label>
					<input
						v-model="messageText"
						type="text"
						placeholder="Type a private message..."
						class="px-3.5 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-white text-xs focus:outline-none focus:border-sky-400"
						@keyup.enter="confirmMessage"
					/>
				</div>

				<div class="flex items-center justify-end gap-2.5 pt-2">
					<button
						type="button"
						class="px-4 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-xs font-bold text-zinc-300 hover:text-white cursor-pointer"
						@click="showMessageModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-5 py-2.5 rounded-xl bg-sky-500 hover:bg-sky-400 text-zinc-950 text-xs font-black border-none cursor-pointer shadow-[0_0_15px_rgba(56,189,248,0.4)]"
						@click="confirmMessage"
					>
						Send Message
					</button>
				</div>
			</div>
		</div>

		<!-- MODAL: ADD WHITELIST PLAYER -->
		<div
			v-if="showAddWhitelistModal"
			class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-6 rounded-3xl bg-[#141923] border border-white/15 shadow-2xl flex flex-col gap-5"
			>
				<div class="flex items-center justify-between">
					<h3 class="text-base font-black text-white m-0">Add Player to Whitelist</h3>
					<button
						type="button"
						class="text-zinc-400 hover:text-white border-none bg-transparent cursor-pointer text-sm"
						@click="showAddWhitelistModal = false"
					>
						✕
					</button>
				</div>
				<div class="flex flex-col gap-2">
					<label class="text-xs font-bold text-zinc-300">Minecraft Username</label>
					<input
						v-model="newWhitelistUser"
						type="text"
						placeholder="e.g. Notch"
						class="px-3.5 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-white text-xs focus:outline-none focus:border-cyan-400"
						@keyup.enter="addWhitelist"
					/>
				</div>
				<div class="flex items-center justify-end gap-2.5 pt-2">
					<button
						type="button"
						class="px-4 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-xs font-bold text-zinc-300 hover:text-white cursor-pointer"
						@click="showAddWhitelistModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-5 py-2.5 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-zinc-950 text-xs font-black border-none cursor-pointer shadow-[0_0_15px_rgba(6,182,212,0.4)]"
						@click="addWhitelist"
					>
						Add to Whitelist
					</button>
				</div>
			</div>
		</div>

		<!-- MODAL: ADD OPERATOR -->
		<div
			v-if="showAddOpModal"
			class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-6 rounded-3xl bg-[#141923] border border-white/15 shadow-2xl flex flex-col gap-5"
			>
				<div class="flex items-center justify-between">
					<h3 class="text-base font-black text-amber-400 m-0">Add Server Operator</h3>
					<button
						type="button"
						class="text-zinc-400 hover:text-white border-none bg-transparent cursor-pointer text-sm"
						@click="showAddOpModal = false"
					>
						✕
					</button>
				</div>
				<div class="flex flex-col gap-2">
					<label class="text-xs font-bold text-zinc-300">Minecraft Username</label>
					<input
						v-model="newOpUser"
						type="text"
						placeholder="e.g. Alex"
						class="px-3.5 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-white text-xs focus:outline-none focus:border-amber-400"
						@keyup.enter="addOp"
					/>
				</div>
				<div class="flex items-center justify-end gap-2.5 pt-2">
					<button
						type="button"
						class="px-4 py-2.5 rounded-xl bg-[#0D1117] border border-white/10 text-xs font-bold text-zinc-300 hover:text-white cursor-pointer"
						@click="showAddOpModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-5 py-2.5 rounded-xl bg-amber-500 hover:bg-amber-400 text-zinc-950 text-xs font-black border-none cursor-pointer shadow-[0_0_15px_rgba(245,158,11,0.4)]"
						@click="addOp"
					>
						Grant OP Privileges
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
