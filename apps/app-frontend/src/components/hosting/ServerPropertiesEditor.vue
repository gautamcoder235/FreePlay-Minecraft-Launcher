<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, reactive, ref } from 'vue'

defineProps<{
	serverStatus: 'offline' | 'starting' | 'online' | 'tunneling'
}>()

const isSaving = ref(false)
const saveSuccess = ref(false)
const isRawMode = ref(false)
const rawContent = ref('')

const isDifficultyOpen = ref(false)
const isGamemodeOpen = ref(false)

const difficultyOptions = [
	{ id: 'peaceful', label: 'Peaceful', desc: 'No hostile mobs, automatic health regeneration' },
	{ id: 'easy', label: 'Easy', desc: 'Hostile mobs spawn but deal reduced damage' },
	{ id: 'normal', label: 'Normal', desc: 'Standard Minecraft survival experience' },
	{ id: 'hard', label: 'Hard', desc: 'Hostile mobs deal maximum damage, starvation can kill' },
]

const gamemodeOptions = [
	{ id: 'survival', label: 'Survival', desc: 'Gather resources, craft items, and manage hunger' },
	{
		id: 'creative',
		label: 'Creative',
		desc: 'Unlimited items, flight, and instant block breaking',
	},
	{
		id: 'adventure',
		label: 'Adventure',
		desc: 'Custom maps mode, blocks cannot be broken without tools',
	},
	{ id: 'spectator', label: 'Spectator', desc: 'Invisible fly-through mode for observing players' },
]

function toggleDifficulty() {
	isDifficultyOpen.value = !isDifficultyOpen.value
	isGamemodeOpen.value = false
}

function toggleGamemode() {
	isGamemodeOpen.value = !isGamemodeOpen.value
	isDifficultyOpen.value = false
}

function selectDifficulty(val: string) {
	form.difficulty = val
	isDifficultyOpen.value = false
}

function selectGamemode(val: string) {
	form.gamemode = val
	isGamemodeOpen.value = false
}

const form = reactive({
	difficulty: 'normal',
	gamemode: 'survival',
	hardcore: false,
	pvp: true,
	allow_flight: true,
	view_distance: 10,
	simulation_distance: 10,
	spawn_protection: 16,
	allow_nether: true,
	online_mode: false,
	white_list: false,
	enable_command_block: true,
	max_players: 20,
	spawn_monsters: true,
	spawn_animals: true,
	spawn_npcs: true,
	motd: 'A FreePlay Minecraft Server',
	server_port: 25565,
})

async function loadProperties() {
	try {
		const res = await invoke<Record<string, unknown>>('host_get_server_properties')
		if (res && typeof res === 'object') {
			Object.assign(form, res)
		}
	} catch (e) {
		console.debug('Failed to load server properties via API:', e)
	}

	try {
		const raw = await invoke<string>('host_read_file', { path: 'server.properties' })
		if (typeof raw === 'string') {
			rawContent.value = raw
		}
	} catch (e) {
		console.debug('Raw properties file read failed:', e)
	}
}

async function saveProperties() {
	isSaving.value = true
	saveSuccess.value = false
	try {
		if (isRawMode.value) {
			await invoke('host_save_file', { path: 'server.properties', content: rawContent.value })
		} else {
			await invoke('host_save_server_properties', { properties: form })
		}
		saveSuccess.value = true
		setTimeout(() => {
			saveSuccess.value = false
		}, 3000)
	} catch (e) {
		console.error('Failed to save server properties:', e)
	} finally {
		isSaving.value = false
	}
}

onMounted(() => {
	loadProperties()
})
</script>

<template>
	<div class="flex flex-col gap-4">
		<!-- Header & Mode Switcher -->
		<div
			class="flex flex-wrap items-center justify-between gap-3 p-4 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm"
		>
			<div class="flex flex-col gap-0.5">
				<h3 class="text-sm font-bold text-contrast m-0 flex items-center gap-2">
					Visual server.properties Editor
					<span
						v-if="serverStatus === 'online'"
						class="px-2 py-0.5 rounded-md text-[10px] font-bold bg-amber-500/20 text-amber-300 border border-amber-500/30"
					>
						Restart required to apply changes
					</span>
				</h3>
				<p class="text-xs text-secondary m-0">
					Configure dedicated Minecraft engine parameters with automatic syntax validation.
				</p>
			</div>

			<div class="flex items-center gap-2">
				<button
					type="button"
					class="px-3 py-1.5 rounded-xl border text-xs font-bold transition-all cursor-pointer"
					:class="
						isRawMode
							? 'bg-brand text-brand-inverted border-brand'
							: 'bg-surface-3 text-secondary border-surface-4 hover:text-contrast'
					"
					@click="isRawMode = !isRawMode"
				>
					{{ isRawMode ? 'Switch to Form GUI' : 'Switch to Raw File' }}
				</button>

				<button
					type="button"
					class="px-4 py-1.5 rounded-xl bg-brand text-brand-inverted text-xs font-bold border-none hover:bg-brand-highlight transition-all cursor-pointer flex items-center gap-1.5 shadow-sm active:scale-95 disabled:opacity-50"
					:disabled="isSaving"
					@click="saveProperties"
				>
					<span v-if="saveSuccess" class="text-emerald-300">✓ Saved!</span>
					<span v-else-if="isSaving">Saving...</span>
					<span v-else>Save Configuration</span>
				</button>
			</div>
		</div>

		<!-- RAW TEXT MODE -->
		<div v-if="isRawMode" class="flex flex-col gap-2">
			<textarea
				v-model="rawContent"
				rows="22"
				class="w-full font-mono text-xs p-4 rounded-2xl bg-surface-3 border border-surface-4 text-contrast focus:outline-none focus:border-brand transition-all resize-y leading-relaxed"
				placeholder="# Minecraft server properties..."
			/>
		</div>

		<!-- VISUAL FORM MODE -->
		<div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<!-- CARD 1: GAMEPLAY & RULES -->
			<div
				class="p-4 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
			>
				<div class="flex items-center gap-2 pb-2 border-b border-surface-4">
					<span class="text-xs font-bold text-contrast uppercase tracking-wider"
						>Gameplay & Core Rules</span
					>
				</div>

				<div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
					<!-- Custom Difficulty Dropdown -->
					<div class="relative flex flex-col gap-1.5">
						<label class="text-xs font-semibold text-secondary">Difficulty</label>
						<button
							type="button"
							class="flex items-center justify-between px-3.5 py-2.5 rounded-xl bg-surface-3 border border-surface-4 hover:border-brand/50 text-contrast text-xs font-medium cursor-pointer transition-all duration-150 focus:outline-none focus:border-brand"
							:class="{ 'border-brand shadow-[0_0_12px_rgba(56,189,248,0.2)]': isDifficultyOpen }"
							@click="toggleDifficulty"
						>
							<div class="flex items-center gap-2">
								<span class="font-bold capitalize">{{ form.difficulty }}</span>
							</div>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-3.5 h-3.5 text-secondary transition-transform duration-200"
								:class="{ 'rotate-180 text-brand': isDifficultyOpen }"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<polyline points="6 9 12 15 18 9" />
							</svg>
						</button>

						<!-- Custom Difficulty Menu -->
						<transition name="fade">
							<div
								v-if="isDifficultyOpen"
								class="absolute top-[calc(100%+4px)] left-0 right-0 z-50 p-1.5 rounded-2xl bg-[#0e121a] border border-white/15 shadow-2xl backdrop-blur-xl flex flex-col gap-1"
							>
								<div
									v-for="opt in difficultyOptions"
									:key="opt.id"
									class="p-2 rounded-xl flex items-center justify-between gap-2 transition-all cursor-pointer select-none"
									:class="
										form.difficulty === opt.id
											? 'bg-sky-500/20 text-sky-200 border border-sky-500/30 font-bold'
											: 'hover:bg-zinc-800/80 text-zinc-300 hover:text-white border border-transparent'
									"
									@click="selectDifficulty(opt.id)"
								>
									<div class="flex flex-col">
										<span class="text-xs">{{ opt.label }}</span>
										<span class="text-[10px] text-zinc-400 font-normal leading-tight">{{
											opt.desc
										}}</span>
									</div>
									<svg
										v-if="form.difficulty === opt.id"
										xmlns="http://www.w3.org/2000/svg"
										class="w-4 h-4 text-sky-400 shrink-0"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2.5"
										stroke-linecap="round"
										stroke-linejoin="round"
									>
										<polyline points="20 6 9 17 4 12" />
									</svg>
								</div>
							</div>
						</transition>
					</div>

					<!-- Custom Gamemode Dropdown -->
					<div class="relative flex flex-col gap-1.5">
						<label class="text-xs font-semibold text-secondary">Default Gamemode</label>
						<button
							type="button"
							class="flex items-center justify-between px-3.5 py-2.5 rounded-xl bg-surface-3 border border-surface-4 hover:border-brand/50 text-contrast text-xs font-medium cursor-pointer transition-all duration-150 focus:outline-none focus:border-brand"
							:class="{ 'border-brand shadow-[0_0_12px_rgba(56,189,248,0.2)]': isGamemodeOpen }"
							@click="toggleGamemode"
						>
							<div class="flex items-center gap-2">
								<span class="font-bold capitalize">{{ form.gamemode }}</span>
							</div>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-3.5 h-3.5 text-secondary transition-transform duration-200"
								:class="{ 'rotate-180 text-brand': isGamemodeOpen }"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<polyline points="6 9 12 15 18 9" />
							</svg>
						</button>

						<!-- Custom Gamemode Menu -->
						<transition name="fade">
							<div
								v-if="isGamemodeOpen"
								class="absolute top-[calc(100%+4px)] left-0 right-0 z-50 p-1.5 rounded-2xl bg-[#0e121a] border border-white/15 shadow-2xl backdrop-blur-xl flex flex-col gap-1"
							>
								<div
									v-for="opt in gamemodeOptions"
									:key="opt.id"
									class="p-2 rounded-xl flex items-center justify-between gap-2 transition-all cursor-pointer select-none"
									:class="
										form.gamemode === opt.id
											? 'bg-sky-500/20 text-sky-200 border border-sky-500/30 font-bold'
											: 'hover:bg-zinc-800/80 text-zinc-300 hover:text-white border border-transparent'
									"
									@click="selectGamemode(opt.id)"
								>
									<div class="flex flex-col">
										<span class="text-xs">{{ opt.label }}</span>
										<span class="text-[10px] text-zinc-400 font-normal leading-tight">{{
											opt.desc
										}}</span>
									</div>
									<svg
										v-if="form.gamemode === opt.id"
										xmlns="http://www.w3.org/2000/svg"
										class="w-4 h-4 text-sky-400 shrink-0"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2.5"
										stroke-linecap="round"
										stroke-linejoin="round"
									>
										<polyline points="20 6 9 17 4 12" />
									</svg>
								</div>
							</div>
						</transition>
					</div>
				</div>

				<div class="flex flex-col gap-2 pt-1">
					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.pvp = !form.pvp"
					>
						<span class="text-xs font-semibold text-contrast">Player vs Player (PvP)</span>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.pvp
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.pvp ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>

					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.hardcore = !form.hardcore"
					>
						<span class="text-xs font-semibold text-contrast">Hardcore Mode</span>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.hardcore
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.hardcore ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>

					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.allow_flight = !form.allow_flight"
					>
						<span class="text-xs font-semibold text-contrast">Allow Player Flight</span>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.allow_flight
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.allow_flight ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>
				</div>
			</div>

			<!-- CARD 2: WORLD & SIMULATION -->
			<div
				class="p-4 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
			>
				<div class="flex items-center gap-2 pb-2 border-b border-surface-4">
					<span class="text-xs font-bold text-contrast uppercase tracking-wider"
						>World & Simulation</span
					>
				</div>

				<div class="flex flex-col gap-2">
					<div class="flex items-center justify-between">
						<span class="text-xs font-semibold text-secondary">View Distance</span>
						<span class="text-xs font-bold text-contrast font-mono"
							>{{ form.view_distance }} chunks</span
						>
					</div>
					<input
						v-model.number="form.view_distance"
						type="range"
						min="4"
						max="32"
						step="1"
						class="w-full accent-sky-500 cursor-pointer h-1.5 bg-surface-3 rounded-lg appearance-none"
					/>
				</div>

				<div class="flex flex-col gap-2">
					<div class="flex items-center justify-between">
						<span class="text-xs font-semibold text-secondary">Simulation Distance</span>
						<span class="text-xs font-bold text-contrast font-mono"
							>{{ form.simulation_distance }} chunks</span
						>
					</div>
					<input
						v-model.number="form.simulation_distance"
						type="range"
						min="3"
						max="16"
						step="1"
						class="w-full accent-sky-500 cursor-pointer h-1.5 bg-surface-3 rounded-lg appearance-none"
					/>
				</div>

				<div class="flex flex-col gap-2 pt-1">
					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.allow_nether = !form.allow_nether"
					>
						<span class="text-xs font-semibold text-contrast">Allow Nether Dimension</span>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.allow_nether
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.allow_nether ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>

					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4"
					>
						<span class="text-xs font-semibold text-contrast">Spawn Protection Radius</span>
						<input
							v-model.number="form.spawn_protection"
							type="number"
							min="0"
							max="64"
							class="w-16 px-2 py-1 rounded-lg bg-surface-2 border border-surface-4 text-contrast text-xs text-center font-mono focus:outline-none focus:border-brand"
						/>
					</div>
				</div>
			</div>

			<!-- CARD 3: ACCESS & SECURITY -->
			<div
				class="p-4 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
			>
				<div class="flex items-center gap-2 pb-2 border-b border-surface-4">
					<span class="text-xs font-bold text-contrast uppercase tracking-wider"
						>Access & Security</span
					>
				</div>

				<div class="flex flex-col gap-2">
					<div class="flex items-center justify-between">
						<span class="text-xs font-semibold text-secondary">Max Players</span>
						<span class="text-xs font-bold text-contrast font-mono"
							>{{ form.max_players }} slots</span
						>
					</div>
					<input
						v-model.number="form.max_players"
						type="range"
						min="1"
						max="100"
						step="1"
						class="w-full accent-sky-500 cursor-pointer h-1.5 bg-surface-3 rounded-lg appearance-none"
					/>
				</div>

				<div class="flex flex-col gap-2 pt-1">
					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.online_mode = !form.online_mode"
					>
						<div class="flex flex-col">
							<span class="text-xs font-semibold text-contrast">Online Mode (Mojang Auth)</span>
							<span class="text-[10px] text-secondary"
								>Disable to allow offline / non-Mojang accounts to join</span
							>
						</div>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.online_mode
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.online_mode ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>

					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.enable_command_block = !form.enable_command_block"
					>
						<span class="text-xs font-semibold text-contrast">Enable Command Blocks</span>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.enable_command_block
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.enable_command_block ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>
				</div>
			</div>

			<!-- CARD 4: ENTITY SPAWNING -->
			<div
				class="p-4 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
			>
				<div class="flex items-center gap-2 pb-2 border-b border-surface-4">
					<span class="text-xs font-bold text-contrast uppercase tracking-wider"
						>Entity Spawning</span
					>
				</div>

				<div class="flex flex-col gap-2">
					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.spawn_monsters = !form.spawn_monsters"
					>
						<span class="text-xs font-semibold text-contrast">Spawn Monsters</span>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.spawn_monsters
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.spawn_monsters ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>

					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.spawn_animals = !form.spawn_animals"
					>
						<span class="text-xs font-semibold text-contrast">Spawn Passive Animals</span>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.spawn_animals
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.spawn_animals ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>

					<div
						class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.spawn_npcs = !form.spawn_npcs"
					>
						<span class="text-xs font-semibold text-contrast">Spawn Villagers & NPCs</span>
						<button
							type="button"
							class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out p-0.5 items-center pointer-events-none"
							:class="
								form.spawn_npcs
									? 'bg-sky-500 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
						>
							<span
								class="pointer-events-none inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition duration-200 ease-in-out"
								:class="form.spawn_npcs ? 'translate-x-4' : 'translate-x-0'"
							/>
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>
