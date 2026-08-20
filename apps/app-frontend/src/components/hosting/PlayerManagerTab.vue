<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref } from 'vue'

export interface PlayerEntry {
	name: string
	uuid: string
	latency?: number
	is_op?: boolean
	online?: boolean
	gamemode?: string
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
const activeTab = ref<'online' | 'whitelist' | 'banned'>('online')

// Modals
const showKickModal = ref(false)
const showBanModal = ref(false)
const showAddWhitelistModal = ref(false)
const selectedPlayer = ref<PlayerEntry | null>(null)
const actionReason = ref('')
const newWhitelistUser = ref('')

const whitelistEnabled = ref(false)
const whitelist = ref<string[]>([])
const banList = ref<{ name: string; reason: string; date: string }[]>([])

const filteredOnlinePlayers = computed(() => {
	if (!searchQuery.value.trim()) return players.value
	const q = searchQuery.value.toLowerCase()
	return players.value.filter(
		(p) => p.name.toLowerCase().includes(q) || p.uuid.toLowerCase().includes(q),
	)
})

async function fetchPlayers() {
	if (props.serverStatus !== 'online') {
		players.value = []
		return
	}
	isLoading.value = true
	try {
		const res = await invoke<PlayerEntry[]>('host_get_players')
		if (Array.isArray(res)) {
			players.value = res
		}
	} catch (e) {
		console.debug('Failed to fetch players:', e)
	} finally {
		isLoading.value = false
	}
}

async function toggleOp(player: PlayerEntry) {
	const newOpState = !player.is_op
	try {
		await invoke('host_player_action', {
			action: newOpState ? 'op' : 'deop',
			player: player.name,
		})
		player.is_op = newOpState
	} catch (e) {
		console.error('Failed to change OP status', e)
	}
}

function openKick(player: PlayerEntry) {
	selectedPlayer.value = player
	actionReason.value = 'Kicked by operator'
	showKickModal.value = true
}

function openBan(player: PlayerEntry) {
	selectedPlayer.value = player
	actionReason.value = 'Banned by operator'
	showBanModal.value = true
}

async function confirmKick() {
	if (!selectedPlayer.value) return
	try {
		await invoke('host_player_action', {
			action: 'kick',
			player: selectedPlayer.value.name,
			param: actionReason.value,
		})
		players.value = players.value.filter((p) => p.name !== selectedPlayer.value?.name)
		showKickModal.value = false
	} catch (e) {
		console.error('Kick failed', e)
	}
}

async function confirmBan() {
	if (!selectedPlayer.value) return
	try {
		await invoke('host_player_action', {
			action: 'ban',
			player: selectedPlayer.value.name,
			param: actionReason.value,
		})
		banList.value.push({
			name: selectedPlayer.value.name,
			reason: actionReason.value,
			date: new Date().toISOString().split('T')[0],
		})
		players.value = players.value.filter((p) => p.name !== selectedPlayer.value?.name)
		showBanModal.value = false
	} catch (e) {
		console.error('Ban failed', e)
	}
}

async function toggleWhitelist() {
	whitelistEnabled.value = !whitelistEnabled.value
	emit('command', whitelistEnabled.value ? 'whitelist on' : 'whitelist off')
}

function addWhitelist() {
	const name = newWhitelistUser.value.trim()
	if (name && !whitelist.value.includes(name)) {
		whitelist.value.push(name)
		emit('command', `whitelist add ${name}`)
		newWhitelistUser.value = ''
		showAddWhitelistModal.value = false
	}
}

function removeWhitelist(name: string) {
	whitelist.value = whitelist.value.filter((n) => n !== name)
	emit('command', `whitelist remove ${name}`)
}

function unbanPlayer(name: string) {
	banList.value = banList.value.filter((b) => b.name !== name)
	emit('command', `pardon ${name}`)
}

onMounted(() => {
	fetchPlayers()
})
</script>

<template>
	<div class="flex flex-col gap-4">
		<!-- Sub-Navigation & Quick Action Bar -->
		<div
			class="flex flex-wrap items-center justify-between gap-3 p-3 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm"
		>
			<div class="flex items-center gap-1 bg-surface-3 p-1 rounded-xl border border-surface-4">
				<button
					type="button"
					class="px-3 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer border-none"
					:class="
						activeTab === 'online'
							? 'bg-brand text-brand-inverted shadow-sm'
							: 'text-secondary hover:text-contrast bg-transparent'
					"
					@click="activeTab = 'online'"
				>
					Online Roster ({{ players.length }})
				</button>
				<button
					type="button"
					class="px-3 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer border-none"
					:class="
						activeTab === 'whitelist'
							? 'bg-brand text-brand-inverted shadow-sm'
							: 'text-secondary hover:text-contrast bg-transparent'
					"
					@click="activeTab = 'whitelist'"
				>
					Whitelist ({{ whitelist.length }})
				</button>
				<button
					type="button"
					class="px-3 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer border-none"
					:class="
						activeTab === 'banned'
							? 'bg-brand text-brand-inverted shadow-sm'
							: 'text-secondary hover:text-contrast bg-transparent'
					"
					@click="activeTab = 'banned'"
				>
					Ban List ({{ banList.length }})
				</button>
			</div>

			<div class="flex items-center gap-2">
				<input
					v-if="activeTab === 'online'"
					v-model="searchQuery"
					type="text"
					placeholder="Filter players..."
					class="px-3 py-1.5 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs focus:outline-none focus:border-brand w-48"
				/>
				<button
					v-if="activeTab === 'whitelist'"
					type="button"
					class="px-3 py-1.5 rounded-xl bg-brand text-brand-inverted text-xs font-bold border-none hover:bg-brand-highlight transition-all cursor-pointer shadow-sm"
					@click="showAddWhitelistModal = true"
				>
					+ Add Player
				</button>
				<button
					type="button"
					class="p-1.5 rounded-xl bg-surface-3 border border-surface-4 text-secondary hover:text-contrast transition-all cursor-pointer"
					title="Refresh Player List"
					@click="fetchPlayers"
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

		<!-- TAB 1: ONLINE PLAYERS -->
		<div v-if="activeTab === 'online'" class="flex flex-col gap-3">
			<div
				v-if="serverStatus !== 'online'"
				class="p-12 rounded-2xl bg-surface-2 border border-surface-4 text-center flex flex-col items-center justify-center gap-2"
			>
				<p class="m-0 text-base font-bold text-contrast">Server is Offline</p>
				<p class="m-0 text-xs text-secondary">
					Start your dedicated server to view connected players and live latency metrics.
				</p>
			</div>

			<div
				v-else-if="filteredOnlinePlayers.length === 0"
				class="p-12 rounded-2xl bg-surface-2 border border-surface-4 text-center flex flex-col items-center justify-center gap-2"
			>
				<p class="m-0 text-base font-bold text-contrast">No Players Online</p>
				<p class="m-0 text-xs text-secondary">
					Share your server IP or join on localhost to see players appear here in real time.
				</p>
			</div>

			<div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
				<div
					v-for="player in filteredOnlinePlayers"
					:key="player.uuid || player.name"
					class="p-3.5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm hover:border-surface-5 transition-all flex items-center justify-between gap-3 group"
				>
					<div class="flex items-center gap-3 min-w-0">
						<div
							class="relative w-10 h-10 rounded-xl bg-surface-3 border border-surface-4 flex items-center justify-center shrink-0 overflow-hidden shadow-inner"
						>
							<img
								:src="`https://mc-heads.net/avatar/${player.name}/64`"
								class="w-full h-full object-cover"
								:alt="player.name"
								loading="lazy"
							/>
							<span
								v-if="player.is_op"
								class="absolute -top-1 -right-1 w-4 h-4 rounded-full bg-amber-500 text-zinc-950 flex items-center justify-center text-[9px] font-black shadow-sm"
								title="Server Operator"
							>
								★
							</span>
						</div>

						<div class="flex flex-col min-w-0">
							<div class="flex items-center gap-1.5">
								<span class="text-sm font-bold text-contrast truncate">{{ player.name }}</span>
								<span
									v-if="player.is_op"
									class="px-1.5 py-0.2 rounded text-[10px] font-extrabold bg-amber-500/20 text-amber-300 border border-amber-500/30"
								>
									OP
								</span>
							</div>
							<div class="flex items-center gap-2 text-[11px] text-secondary">
								<span class="flex items-center gap-1 font-mono">
									<span
										class="w-1.5 h-1.5 rounded-full"
										:class="
											(player.latency ?? 20) < 50
												? 'bg-emerald-400'
												: (player.latency ?? 20) < 150
													? 'bg-amber-400'
													: 'bg-rose-400'
										"
									/>
									{{ player.latency ?? 20 }}ms
								</span>
								<span class="text-zinc-600">•</span>
								<span>{{ player.gamemode || 'Survival' }}</span>
							</div>
						</div>
					</div>

					<!-- Operator Controls -->
					<div class="flex items-center gap-1 shrink-0">
						<button
							type="button"
							class="p-1.5 rounded-lg border text-xs font-semibold transition-all cursor-pointer"
							:class="
								player.is_op
									? 'bg-amber-500/20 border-amber-500/40 text-amber-300 hover:bg-amber-500/30'
									: 'bg-surface-3 border-surface-4 text-secondary hover:text-contrast'
							"
							:title="player.is_op ? 'Demote from Operator' : 'Promote to Operator'"
							@click="toggleOp(player)"
						>
							{{ player.is_op ? 'De-OP' : 'OP' }}
						</button>
						<button
							type="button"
							class="p-1.5 rounded-lg bg-surface-3 border border-surface-4 text-secondary hover:text-amber-400 hover:border-amber-400/40 transition-all cursor-pointer text-xs"
							title="Kick Player"
							@click="openKick(player)"
						>
							Kick
						</button>
						<button
							type="button"
							class="p-1.5 rounded-lg bg-surface-3 border border-surface-4 text-secondary hover:text-rose-400 hover:border-rose-400/40 transition-all cursor-pointer text-xs"
							title="Ban Player"
							@click="openBan(player)"
						>
							Ban
						</button>
					</div>
				</div>
			</div>
		</div>

		<!-- TAB 2: WHITELIST -->
		<div v-if="activeTab === 'whitelist'" class="flex flex-col gap-3">
			<div
				class="p-4 rounded-2xl bg-surface-2 border border-surface-4 flex items-center justify-between gap-4"
			>
				<div class="flex flex-col gap-0.5">
					<span class="text-sm font-bold text-contrast">Enforce Server Whitelist</span>
					<span class="text-xs text-secondary"
						>When enabled, only players explicitly listed below can connect to this server.</span
					>
				</div>
				<button
					type="button"
					class="px-3 py-1.5 rounded-xl text-xs font-bold transition-all cursor-pointer border-none"
					:class="
						whitelistEnabled
							? 'bg-brand text-brand-inverted shadow-sm'
							: 'bg-surface-3 text-secondary border border-surface-4'
					"
					@click="toggleWhitelist"
				>
					{{ whitelistEnabled ? 'Whitelist Active' : 'Whitelist Disabled' }}
				</button>
			</div>

			<div
				v-if="whitelist.length === 0"
				class="p-12 rounded-2xl bg-surface-2 border border-surface-4 text-center"
			>
				<p class="m-0 text-sm font-bold text-contrast">No Whitelisted Players</p>
				<p class="m-0 text-xs text-secondary">
					Add players to restrict who can connect when whitelist enforcement is enabled.
				</p>
			</div>

			<div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
				<div
					v-for="name in whitelist"
					:key="name"
					class="p-3 rounded-2xl bg-surface-2 border border-surface-4 flex items-center justify-between gap-2 shadow-sm"
				>
					<div class="flex items-center gap-2.5 min-w-0">
						<img
							:src="`https://mc-heads.net/avatar/${name}/32`"
							class="w-7 h-7 rounded-lg object-cover bg-surface-3 border border-surface-4"
							alt=""
						/>
						<span class="text-xs font-bold text-contrast truncate">{{ name }}</span>
					</div>
					<button
						type="button"
						class="p-1 rounded-lg text-secondary hover:text-rose-400 hover:bg-rose-500/10 border-none transition-all cursor-pointer text-xs"
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
				v-if="banList.length === 0"
				class="p-12 rounded-2xl bg-surface-2 border border-surface-4 text-center"
			>
				<p class="m-0 text-sm font-bold text-contrast">No Banned Players</p>
				<p class="m-0 text-xs text-secondary">Your ban list is clean.</p>
			</div>

			<div v-else class="flex flex-col gap-2">
				<div
					v-for="item in banList"
					:key="item.name"
					class="p-3.5 rounded-2xl bg-surface-2 border border-surface-4 flex items-center justify-between gap-3 shadow-sm"
				>
					<div class="flex items-center gap-3 min-w-0">
						<img
							:src="`https://mc-heads.net/avatar/${item.name}/32`"
							class="w-8 h-8 rounded-lg object-cover bg-surface-3 border border-surface-4"
							alt=""
						/>
						<div class="flex flex-col min-w-0">
							<span class="text-xs font-bold text-contrast truncate">{{ item.name }}</span>
							<span class="text-[11px] text-secondary truncate"
								>Reason: {{ item.reason }} ({{ item.date }})</span
							>
						</div>
					</div>
					<button
						type="button"
						class="px-3 py-1 rounded-xl bg-surface-3 border border-surface-4 text-xs font-semibold text-secondary hover:text-contrast hover:border-surface-5 transition-all cursor-pointer"
						@click="unbanPlayer(item.name)"
					>
						Pardon
					</button>
				</div>
			</div>
		</div>

		<!-- KICK MODAL -->
		<div
			v-if="showKickModal"
			class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-5 rounded-2xl bg-surface-1 border border-surface-4 shadow-2xl flex flex-col gap-4"
			>
				<h3 class="text-base font-bold text-contrast m-0">
					Kick Player: {{ selectedPlayer?.name }}
				</h3>
				<div class="flex flex-col gap-1.5">
					<label class="text-xs font-semibold text-secondary">Reason for Kick</label>
					<input
						v-model="actionReason"
						type="text"
						class="px-3 py-2 rounded-xl bg-surface-2 border border-surface-4 text-contrast text-xs focus:outline-none focus:border-brand"
					/>
				</div>
				<div class="flex items-center justify-end gap-2 pt-2">
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-surface-3 border border-surface-4 text-xs font-bold text-secondary hover:text-contrast cursor-pointer"
						@click="showKickModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-amber-500 text-zinc-950 text-xs font-bold border-none hover:bg-amber-400 cursor-pointer shadow-sm"
						@click="confirmKick"
					>
						Confirm Kick
					</button>
				</div>
			</div>
		</div>

		<!-- BAN MODAL -->
		<div
			v-if="showBanModal"
			class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-5 rounded-2xl bg-surface-1 border border-surface-4 shadow-2xl flex flex-col gap-4"
			>
				<h3 class="text-base font-bold text-contrast m-0">
					Ban Player: {{ selectedPlayer?.name }}
				</h3>
				<div class="flex flex-col gap-1.5">
					<label class="text-xs font-semibold text-secondary">Reason for Ban</label>
					<input
						v-model="actionReason"
						type="text"
						class="px-3 py-2 rounded-xl bg-surface-2 border border-surface-4 text-contrast text-xs focus:outline-none focus:border-brand"
					/>
				</div>
				<div class="flex items-center justify-end gap-2 pt-2">
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-surface-3 border border-surface-4 text-xs font-bold text-secondary hover:text-contrast cursor-pointer"
						@click="showBanModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-rose-500 text-white text-xs font-bold border-none hover:bg-rose-600 cursor-pointer shadow-sm"
						@click="confirmBan"
					>
						Confirm Ban
					</button>
				</div>
			</div>
		</div>

		<!-- ADD WHITELIST MODAL -->
		<div
			v-if="showAddWhitelistModal"
			class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
		>
			<div
				class="w-full max-w-md p-5 rounded-2xl bg-surface-1 border border-surface-4 shadow-2xl flex flex-col gap-4"
			>
				<h3 class="text-base font-bold text-contrast m-0">Add Player to Whitelist</h3>
				<div class="flex flex-col gap-1.5">
					<label class="text-xs font-semibold text-secondary">Minecraft Username</label>
					<input
						v-model="newWhitelistUser"
						type="text"
						placeholder="e.g. Notch"
						class="px-3 py-2 rounded-xl bg-surface-2 border border-surface-4 text-contrast text-xs focus:outline-none focus:border-brand"
					/>
				</div>
				<div class="flex items-center justify-end gap-2 pt-2">
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-surface-3 border border-surface-4 text-xs font-bold text-secondary hover:text-contrast cursor-pointer"
						@click="showAddWhitelistModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-brand text-brand-inverted text-xs font-bold border-none hover:bg-brand-highlight cursor-pointer shadow-sm"
						@click="addWhitelist"
					>
						Add to Whitelist
					</button>
				</div>
			</div>
		</div>
	</div>
</template>
