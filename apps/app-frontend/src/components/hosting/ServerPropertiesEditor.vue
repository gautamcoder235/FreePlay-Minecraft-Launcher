<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, reactive, ref, watch } from 'vue'

const props = defineProps<{
	serverStatus: 'offline' | 'starting' | 'online' | 'tunneling'
	serverId?: string
}>()

const serverIconUrl = ref<string | null>(null)
const isUploadingIcon = ref(false)
const fileInputRef = ref<HTMLInputElement | null>(null)

const isSaving = ref(false)
const saveSuccess = ref(false)
const isRawMode = ref(false)
const rawContent = ref('')
const searchQuery = ref('')
const selectedCategory = ref<string>('all')

const isDifficultyOpen = ref(false)
const isGamemodeOpen = ref(false)
const isLevelTypeOpen = ref(false)
const isOpLevelOpen = ref(false)

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

const levelTypeOptions = [
	{
		id: 'minecraft:normal',
		label: 'Default / Normal',
		desc: 'Standard terrain generation with hills, oceans & biomes',
	},
	{
		id: 'minecraft:flat',
		label: 'Superflat',
		desc: 'Completely flat world suitable for building & testing',
	},
	{
		id: 'minecraft:large_biomes',
		label: 'Large Biomes',
		desc: 'Biomes are 4x larger than default',
	},
	{
		id: 'minecraft:amplified',
		label: 'Amplified',
		desc: 'Extreme mountainous terrain reaching world height',
	},
	{
		id: 'minecraft:single_biome_surface',
		label: 'Single Biome',
		desc: 'World composed of a single uniform biome',
	},
]

const opLevelOptions = [
	{ id: 1, label: 'Level 1 - Bypass Spawn Protection', desc: 'Can bypass spawn protection blocks' },
	{
		id: 2,
		label: 'Level 2 - Basic Commands',
		desc: 'Access to /clear, /difficulty, /effect, /gamemode, /give',
	},
	{ id: 3, label: 'Level 3 - Moderation', desc: 'Access to /ban, /kick, /op, /deop' },
	{
		id: 4,
		label: 'Level 4 - Server Admin',
		desc: 'Full access to /stop, /save-all, and server management',
	},
]

const categories = [
	{ id: 'all', label: 'All Settings' },
	{ id: 'gameplay', label: 'Gameplay & Rules' },
	{ id: 'world', label: 'World & Generation' },
	{ id: 'security', label: 'Access & Security' },
	{ id: 'performance', label: 'Performance & Limits' },
	{ id: 'spawning', label: 'Spawning & Entities' },
	{ id: 'network', label: 'Network & Advanced' },
]

const form = reactive({
	// MOTD
	motd: 'A FreePlay Minecraft Server',

	// Gameplay
	difficulty: 'normal',
	gamemode: 'survival',
	hardcore: false,
	pvp: true,
	allow_flight: true,
	force_gamemode: false,
	enable_command_block: true,
	announce_player_achievements: true,

	// World
	level_name: 'world',
	level_seed: '',
	level_type: 'minecraft:normal',
	generate_structures: true,
	allow_nether: true,
	spawn_protection: 16,
	max_world_size: 29999984,

	// Security & Access
	online_mode: false,
	white_list: false,
	enforce_whitelist: false,
	max_players: 20,
	op_permission_level: 4,
	player_idle_timeout: 0,
	hide_online_players: false,

	// Performance
	view_distance: 10,
	simulation_distance: 10,
	entity_broadcast_range_percentage: 100,
	network_compression_threshold: 256,
	max_tick_time: 60000,
	sync_chunk_writes: true,

	// Spawning
	spawn_monsters: true,
	spawn_animals: true,
	spawn_npcs: true,

	// Network
	server_port: 25565,
	enable_rcon: false,
	rcon_port: 25575,
	rcon_password: '',
	enable_query: false,
	query_port: 25565,
	resource_pack: '',
	resource_pack_sha1: '',
	require_resource_pack: false,
})

function parsePropertiesFile(content: string) {
	const lines = content.split(/\r?\n/)
	for (const line of lines) {
		const trimmed = line.trim()
		if (!trimmed || trimmed.startsWith('#')) continue
		const eqIdx = trimmed.indexOf('=')
		if (eqIdx === -1) continue
		const key = trimmed.slice(0, eqIdx).trim()
		const val = trimmed.slice(eqIdx + 1).trim()

		switch (key) {
			case 'motd':
				form.motd = val
				break
			case 'difficulty':
				form.difficulty = val.toLowerCase()
				break
			case 'gamemode':
				form.gamemode = val.toLowerCase()
				break
			case 'hardcore':
				form.hardcore = val.toLowerCase() === 'true'
				break
			case 'pvp':
				form.pvp = val.toLowerCase() === 'true'
				break
			case 'allow-flight':
				form.allow_flight = val.toLowerCase() === 'true'
				break
			case 'force-gamemode':
				form.force_gamemode = val.toLowerCase() === 'true'
				break
			case 'enable-command-block':
				form.enable_command_block = val.toLowerCase() === 'true'
				break
			case 'announce-player-achievements':
				form.announce_player_achievements = val.toLowerCase() === 'true'
				break
			case 'level-name':
				form.level_name = val
				break
			case 'level-seed':
				form.level_seed = val
				break
			case 'level-type':
				form.level_type = val
				break
			case 'generate-structures':
				form.generate_structures = val.toLowerCase() === 'true'
				break
			case 'allow-nether':
				form.allow_nether = val.toLowerCase() === 'true'
				break
			case 'spawn-protection':
				form.spawn_protection = parseInt(val, 10) || 0
				break
			case 'max-world-size':
				form.max_world_size = parseInt(val, 10) || 29999984
				break
			case 'online-mode':
				form.online_mode = val.toLowerCase() === 'true'
				break
			case 'white-list':
				form.white_list = val.toLowerCase() === 'true'
				break
			case 'enforce-whitelist':
				form.enforce_whitelist = val.toLowerCase() === 'true'
				break
			case 'max-players':
				form.max_players = parseInt(val, 10) || 20
				break
			case 'op-permission-level':
				form.op_permission_level = parseInt(val, 10) || 4
				break
			case 'player-idle-timeout':
				form.player_idle_timeout = parseInt(val, 10) || 0
				break
			case 'hide-online-players':
				form.hide_online_players = val.toLowerCase() === 'true'
				break
			case 'view-distance':
				form.view_distance = parseInt(val, 10) || 10
				break
			case 'simulation-distance':
				form.simulation_distance = parseInt(val, 10) || 10
				break
			case 'entity-broadcast-range-percentage':
				form.entity_broadcast_range_percentage = parseInt(val, 10) || 100
				break
			case 'network-compression-threshold':
				form.network_compression_threshold = parseInt(val, 10) || 256
				break
			case 'max-tick-time':
				form.max_tick_time = parseInt(val, 10) || 60000
				break
			case 'sync-chunk-writes':
				form.sync_chunk_writes = val.toLowerCase() === 'true'
				break
			case 'spawn-monsters':
				form.spawn_monsters = val.toLowerCase() === 'true'
				break
			case 'spawn-animals':
				form.spawn_animals = val.toLowerCase() === 'true'
				break
			case 'spawn-npcs':
				form.spawn_npcs = val.toLowerCase() === 'true'
				break
			case 'server-port':
				form.server_port = parseInt(val, 10) || 25565
				break
			case 'enable-rcon':
				form.enable_rcon = val.toLowerCase() === 'true'
				break
			case 'rcon.port':
				form.rcon_port = parseInt(val, 10) || 25575
				break
			case 'rcon.password':
				form.rcon_password = val
				break
			case 'enable-query':
				form.enable_query = val.toLowerCase() === 'true'
				break
			case 'query.port':
				form.query_port = parseInt(val, 10) || 25565
				break
			case 'resource-pack':
				form.resource_pack = val
				break
			case 'resource-pack-sha1':
				form.resource_pack_sha1 = val
				break
			case 'require-resource-pack':
				form.require_resource_pack = val.toLowerCase() === 'true'
				break
		}
	}
}

function serializePropertiesFile(): string {
	const map: Record<string, string | number | boolean> = {
		motd: form.motd,
		'server-port': form.server_port,
		difficulty: form.difficulty,
		gamemode: form.gamemode,
		hardcore: form.hardcore,
		pvp: form.pvp,
		'allow-flight': form.allow_flight,
		'force-gamemode': form.force_gamemode,
		'enable-command-block': form.enable_command_block,
		'announce-player-achievements': form.announce_player_achievements,
		'level-name': form.level_name,
		'level-seed': form.level_seed,
		'level-type': form.level_type,
		'generate-structures': form.generate_structures,
		'allow-nether': form.allow_nether,
		'spawn-protection': form.spawn_protection,
		'max-world-size': form.max_world_size,
		'online-mode': form.online_mode,
		'white-list': form.white_list,
		'enforce-whitelist': form.enforce_whitelist,
		'max-players': form.max_players,
		'op-permission-level': form.op_permission_level,
		'player-idle-timeout': form.player_idle_timeout,
		'hide-online-players': form.hide_online_players,
		'view-distance': form.view_distance,
		'simulation-distance': form.simulation_distance,
		'entity-broadcast-range-percentage': form.entity_broadcast_range_percentage,
		'network-compression-threshold': form.network_compression_threshold,
		'max-tick-time': form.max_tick_time,
		'sync-chunk-writes': form.sync_chunk_writes,
		'spawn-monsters': form.spawn_monsters,
		'spawn-animals': form.spawn_animals,
		'spawn-npcs': form.spawn_npcs,
		'enable-rcon': form.enable_rcon,
		'rcon.port': form.rcon_port,
		'rcon.password': form.rcon_password,
		'enable-query': form.enable_query,
		'query.port': form.query_port,
		'resource-pack': form.resource_pack,
		'resource-pack-sha1': form.resource_pack_sha1,
		'require-resource-pack': form.require_resource_pack,
	}

	const lines = [
		'# Minecraft server properties',
		`# Generated by FreePlay Launcher - ${new Date().toISOString()}`,
	]
	for (const [k, v] of Object.entries(map)) {
		lines.push(`${k}=${v}`)
	}
	return lines.join('\n')
}

// Live formatted MOTD renderer (replaces § and & formatting codes with HTML spans)
const renderedMotd = computed(() => {
	const raw = form.motd || 'A FreePlay Minecraft Server'
	const colorCodes: Record<string, string> = {
		'0': '#000000',
		'1': '#0000AA',
		'2': '#00AA00',
		'3': '#00AAAA',
		'4': '#AA0000',
		'5': '#AA00AA',
		'6': '#FFAA00',
		'7': '#AAAAAA',
		'8': '#555555',
		'9': '#5555FF',
		a: '#55FF55',
		b: '#55FFFF',
		c: '#FF5555',
		d: '#FF55FF',
		e: '#FFFF55',
		f: '#FFFFFF',
	}

	let html = ''
	let currentColor = '#FFFFFF'
	let isBold = false
	let isItalic = false

	const tokens = raw.split(/([§&][0-9a-fk-or])/gi)
	for (const token of tokens) {
		if (token.startsWith('§') || token.startsWith('&')) {
			const code = token.charAt(1).toLowerCase()
			if (colorCodes[code]) {
				currentColor = colorCodes[code]
				isBold = false
				isItalic = false
			} else if (code === 'l') {
				isBold = true
			} else if (code === 'o') {
				isItalic = true
			} else if (code === 'r') {
				currentColor = '#FFFFFF'
				isBold = false
				isItalic = false
			}
		} else if (token) {
			const style = `color: ${currentColor}; font-weight: ${isBold ? 'bold' : 'normal'}; font-style: ${isItalic ? 'italic' : 'normal'};`
			html += `<span style="${style}">${token.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</span>`
		}
	}
	return html || '<span style="color:#ffffff;">A FreePlay Minecraft Server</span>'
})

function matchesSearch(terms: string[]): boolean {
	if (!searchQuery.value.trim()) return true
	const q = searchQuery.value.toLowerCase().trim()
	return terms.some((t) => t.toLowerCase().includes(q))
}

function selectDifficulty(id: string) {
	form.difficulty = id
	isDifficultyOpen.value = false
}

function selectGamemode(id: string) {
	form.gamemode = id
	isGamemodeOpen.value = false
}

function selectLevelType(id: string) {
	form.level_type = id
	isLevelTypeOpen.value = false
}

function selectOpLevel(id: number) {
	form.op_permission_level = id
	isOpLevelOpen.value = false
}

async function loadProperties() {
	try {
		const raw = await invoke<string>('host_read_file', { path: 'server.properties' })
		if (typeof raw === 'string' && raw.trim().length > 0) {
			rawContent.value = raw
			parsePropertiesFile(raw)
		}
	} catch (e) {
		console.debug('Failed to read server.properties file:', e)
	}
}

async function saveProperties() {
	isSaving.value = true
	saveSuccess.value = false
	try {
		const toSave = isRawMode.value ? rawContent.value : serializePropertiesFile()
		await invoke('host_save_file', { path: 'server.properties', content: toSave })
		rawContent.value = toSave
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

async function fetchServerIcon() {
	try {
		const icon = await invoke<string | null>('host_get_server_icon', {
			serverId: props.serverId,
		})
		serverIconUrl.value = icon || null
	} catch (e) {
		console.debug('Failed to fetch server icon:', e)
	}
}

function triggerIconUpload() {
	fileInputRef.value?.click()
}

async function handleIconFile(e: Event) {
	const file = (e.target as HTMLInputElement).files?.[0]
	if (!file) return

	isUploadingIcon.value = true
	try {
		const base64Data = await new Promise<string>((resolve, reject) => {
			const reader = new FileReader()
			reader.onload = (readerEvent) => {
				const img = new Image()
				img.onload = () => {
					const canvas = document.createElement('canvas')
					canvas.width = 64
					canvas.height = 64
					const ctx = canvas.getContext('2d')
					if (!ctx) {
						reject(new Error('Canvas context not available'))
						return
					}
					ctx.imageSmoothingEnabled = true
					ctx.imageSmoothingQuality = 'high'
					ctx.drawImage(img, 0, 0, 64, 64)
					resolve(canvas.toDataURL('image/png'))
				}
				img.onerror = () => reject(new Error('Failed to load image'))
				img.src = readerEvent.target?.result as string
			}
			reader.onerror = () => reject(new Error('Failed to read file'))
			reader.readAsDataURL(file)
		})

		const savedUrl = await invoke<string>('host_set_server_icon', {
			serverId: props.serverId,
			iconBase64: base64Data,
		})
		serverIconUrl.value = savedUrl
	} catch (err) {
		console.error('Failed to update server icon:', err)
		alert(`Failed to set server icon: ${err instanceof Error ? err.message : String(err)}`)
	} finally {
		isUploadingIcon.value = false
		if (fileInputRef.value) fileInputRef.value.value = ''
	}
}

async function resetServerIcon() {
	try {
		await invoke('host_delete_server_icon', {
			serverId: props.serverId,
		})
		serverIconUrl.value = null
	} catch (err) {
		console.error('Failed to delete server icon:', err)
	}
}

watch(
	() => props.serverId,
	() => {
		fetchServerIcon()
	},
)

onMounted(() => {
	loadProperties()
	fetchServerIcon()
})
</script>

<template>
	<div class="flex flex-col gap-5">
		<!-- Top Control Ribbon -->
		<div
			class="flex flex-wrap items-center justify-between gap-4 p-5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm"
		>
			<div class="flex flex-col gap-1">
				<div class="flex items-center gap-2.5">
					<h3 class="text-base font-black text-contrast m-0 tracking-tight">
						Server Properties & Engine Configuration
					</h3>
					<span
						v-if="serverStatus === 'online'"
						class="px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-amber-500/20 text-amber-300 border border-amber-500/30 uppercase tracking-wide"
					>
						Restart required to apply
					</span>
				</div>
			</div>

			<div class="flex items-center gap-3">
				<!-- Search Bar -->
				<div v-if="!isRawMode" class="relative flex items-center">
					<input
						v-model="searchQuery"
						type="text"
						placeholder="Search properties (e.g. pvp, port, seed)..."
						class="w-64 pl-8 pr-7 py-1.5 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs placeholder:text-zinc-500 focus:outline-none focus:border-brand transition-all"
					/>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5 text-zinc-500 absolute left-2.5 pointer-events-none"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<circle cx="11" cy="11" r="8" />
						<line x1="21" y1="21" x2="16.65" y2="16.65" />
					</svg>
					<button
						v-if="searchQuery"
						type="button"
						class="absolute right-2 text-zinc-400 hover:text-white text-xs cursor-pointer border-none bg-transparent flex items-center justify-center p-0.5"
						@click="searchQuery = ''"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<line x1="18" y1="6" x2="6" y2="18" />
							<line x1="6" y1="6" x2="18" y2="18" />
						</svg>
					</button>
				</div>

				<!-- Mode Switcher -->
				<button
					type="button"
					class="px-3.5 py-1.5 rounded-xl border text-xs font-bold transition-all cursor-pointer select-none"
					:class="
						isRawMode
							? 'bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border-[var(--color-brand-shadow)] shadow-[var(--accent-glow)]'
							: 'bg-surface-3 text-secondary border-surface-4 hover:text-contrast'
					"
					@click="isRawMode = !isRawMode"
				>
					{{ isRawMode ? 'Switch to Visual GUI' : 'Switch to Raw File' }}
				</button>

				<!-- Save Button -->
				<button
					type="button"
					class="px-4 py-1.5 rounded-xl btn-accent-primary text-xs font-bold border-none transition-all cursor-pointer flex items-center gap-1.5 shadow-sm active:scale-95 disabled:opacity-50 select-none"
					:disabled="isSaving"
					@click="saveProperties"
				>
					<span v-if="saveSuccess" class="font-bold">✓ Saved!</span>
					<span v-else-if="isSaving">Saving...</span>
					<span v-else>Save Configuration</span>
				</button>
			</div>
		</div>

		<!-- RAW TEXT FILE MODE -->
		<div v-if="isRawMode" class="flex flex-col gap-2">
			<div class="flex items-center justify-between text-xs text-zinc-400 px-1">
				<span>Direct editing of <code>server.properties</code></span>
				<span>Lines will be saved verbatim to disk</span>
			</div>
			<textarea
				v-model="rawContent"
				rows="24"
				class="w-full font-mono text-xs p-4 rounded-2xl bg-surface-3 border border-surface-4 text-contrast focus:outline-none focus:border-brand transition-all resize-y leading-relaxed"
				placeholder="# Minecraft server properties..."
			/>
		</div>

		<!-- VISUAL GUI FORM MODE -->
		<div v-else class="flex flex-col gap-5">
			<!-- Hero Card: Live MOTD & Server Card Preview -->
			<div
				v-if="!searchQuery && (selectedCategory === 'all' || selectedCategory === 'network')"
				class="p-5 rounded-2xl bg-gradient-to-br from-[#121622] to-[#0d1017] border border-surface-4 shadow-md flex flex-col gap-4"
			>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<div
							class="w-6 h-6 rounded-lg bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] flex items-center justify-center text-[var(--color-brand-highlight,var(--color-brand))]"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-3.5 h-3.5"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path
									d="M12 2H2v10l9.29 9.29c.94.94 2.48.94 3.42 0l6.58-6.58c.94-.94.94-2.48 0-3.42L12 2Z"
								/>
								<path d="M7 7h.01" />
							</svg>
						</div>
						<span class="text-xs font-bold text-contrast uppercase tracking-wider"
							>Server Brand &amp; MOTD Message</span
						>
					</div>
					<span class="text-[11px] text-zinc-400 font-mono">motd={{ form.motd }}</span>
				</div>

				<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
					<!-- MOTD Input -->
					<div class="flex flex-col gap-2">
						<label class="text-xs font-semibold text-secondary">
							Message of the Day (Supports Color Codes e.g. &amp;a, &amp;b, &amp;l)
						</label>
						<input
							v-model="form.motd"
							type="text"
							class="w-full px-3.5 py-2.5 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand transition-all"
							placeholder="&amp;bA FreePlay &amp;aMinecraft Server"
						/>
						<div class="flex items-center gap-2 text-[10px] text-zinc-400">
							<span>Codes:</span>
							<span class="text-emerald-400 font-mono">&amp;a Green</span>
							<span class="text-sky-400 font-mono">&amp;b Aqua</span>
							<span class="text-rose-400 font-mono">&amp;c Red</span>
							<span class="text-amber-400 font-mono">&amp;e Yellow</span>
							<span class="text-purple-400 font-mono">&amp;d Pink</span>
							<span class="text-white font-bold font-mono">&amp;l Bold</span>
						</div>
					</div>

					<!-- Simulated Minecraft Server List Preview -->
					<div class="flex flex-col gap-1.5">
						<div class="flex items-center justify-between">
							<span class="text-xs font-semibold text-secondary">Multiplayer List Preview</span>
							<div class="flex items-center gap-2">
								<button
									v-if="serverIconUrl"
									type="button"
									class="text-[10px] text-zinc-400 hover:text-rose-400 font-semibold cursor-pointer border-none bg-transparent transition-colors"
									@click="resetServerIcon"
								>
									Reset Icon
								</button>
								<button
									type="button"
									class="text-[10px] text-[var(--color-brand-highlight,var(--color-brand))] hover:underline font-semibold cursor-pointer border-none bg-transparent flex items-center gap-1"
									@click="triggerIconUpload"
								>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										class="w-3 h-3"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										stroke-linecap="round"
										stroke-linejoin="round"
									>
										<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
										<polyline points="17 8 12 3 7 8" />
										<line x1="12" y1="3" x2="12" y2="15" />
									</svg>
									<span>{{ serverIconUrl ? 'Change Icon' : 'Upload Icon (64x64)' }}</span>
								</button>
							</div>
						</div>

						<!-- Hidden file input for custom server icon -->
						<input
							ref="fileInputRef"
							type="file"
							accept="image/png,image/jpeg,image/webp,image/gif"
							class="hidden"
							@change="handleIconFile"
						/>

						<div
							class="p-3 rounded-xl bg-[#080b11] border border-white/10 flex items-center justify-between gap-3 shadow-inner group/server"
						>
							<div class="flex items-center gap-3 min-w-0">
								<!-- Interactive Server Icon Box -->
								<div
									class="relative w-12 h-12 rounded-xl bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] flex items-center justify-center shrink-0 overflow-hidden shadow-sm cursor-pointer group/icon transition-transform active:scale-95"
									:title="
										serverIconUrl
											? 'Click to change server icon (64x64 PNG)'
											: 'Click to upload custom server icon'
									"
									@click="triggerIconUpload"
								>
									<img
										v-if="serverIconUrl"
										:src="serverIconUrl"
										alt="Server Icon"
										class="w-full h-full object-cover rounded-xl [image-rendering:pixelated]"
									/>
									<svg
										v-else
										xmlns="http://www.w3.org/2000/svg"
										class="w-6 h-6 text-[var(--color-brand-highlight,var(--color-brand))]"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="1.8"
										stroke-linecap="round"
										stroke-linejoin="round"
									>
										<path
											d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
										/>
										<polyline points="3.27 6.96 12 12.01 20.73 6.96" />
										<line x1="12" y1="22.08" x2="12" y2="12" />
									</svg>

									<!-- Upload Overlay on Hover -->
									<div
										class="absolute inset-0 bg-black/60 backdrop-blur-[2px] opacity-0 group-hover/icon:opacity-100 flex flex-col items-center justify-center transition-opacity text-white"
									>
										<svg
											xmlns="http://www.w3.org/2000/svg"
											class="w-4 h-4"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2.2"
											stroke-linecap="round"
											stroke-linejoin="round"
										>
											<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
											<polyline points="17 8 12 3 7 8" />
											<line x1="12" y1="3" x2="12" y2="15" />
										</svg>
										<span class="text-[8px] font-bold mt-0.5">Edit</span>
									</div>
								</div>

								<div class="flex flex-col font-mono text-xs leading-snug truncate">
									<span class="font-bold text-white tracking-wide truncate">FreePlay Server</span>
									<!-- eslint-disable-next-line vue/no-v-html -->
									<span class="text-xs truncate" v-html="renderedMotd" />
								</div>
							</div>
							<div
								class="flex flex-col items-end gap-1 shrink-0 font-mono text-[11px] text-zinc-400"
							>
								<div class="flex items-center gap-1 text-emerald-400">
									<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
									<span>0/{{ form.max_players }}</span>
								</div>
								<span class="text-[10px] text-zinc-500">:{{ form.server_port }}</span>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Category Filter Pills -->
			<div v-if="!searchQuery" class="flex items-center gap-2 overflow-x-auto pb-1 scrollbar-none">
				<button
					v-for="cat in categories"
					:key="cat.id"
					type="button"
					class="px-3.5 py-1.5 rounded-xl text-xs font-bold whitespace-nowrap transition-all cursor-pointer border select-none flex items-center gap-1.5"
					:class="
						selectedCategory === cat.id
							? 'bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border-[var(--color-brand-shadow)] shadow-[var(--accent-glow)] scale-100'
							: 'bg-surface-2 text-secondary border-surface-4 hover:border-surface-5 hover:text-contrast'
					"
					@click="selectedCategory = cat.id"
				>
					<svg
						v-if="cat.id === 'all'"
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
					</svg>
					<svg
						v-else-if="cat.id === 'gameplay'"
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<line x1="6" y1="12" x2="10" y2="12" />
						<line x1="8" y1="10" x2="8" y2="14" />
						<line x1="15" y1="13" x2="15.01" y2="13" />
						<line x1="18" y1="11" x2="18.01" y2="11" />
						<rect x="2" y="6" width="20" height="12" rx="6" />
					</svg>
					<svg
						v-else-if="cat.id === 'world'"
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<circle cx="12" cy="12" r="10" />
						<line x1="2" y1="12" x2="22" y2="12" />
						<path
							d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"
						/>
					</svg>
					<svg
						v-else-if="cat.id === 'security'"
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
						<path d="M7 11V7a5 5 0 0 1 10 0v4" />
					</svg>
					<svg
						v-else-if="cat.id === 'performance'"
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="m12 14 4-4" />
						<path d="M3.34 19a10 10 0 1 1 17.32 0" />
					</svg>
					<svg
						v-else-if="cat.id === 'spawning'"
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M9 10h.01" />
						<path d="M15 10h.01" />
						<path d="M10 2v2" />
						<path d="M14 2v2" />
						<path d="M12 17a4 4 0 0 1-4-4V7a4 4 0 0 1 8 0v6a4 4 0 0 1-4 4Z" />
						<path d="M18 13a6 6 0 0 0-12 0" />
						<path d="M6 13v4" />
						<path d="M18 13v4" />
					</svg>
					<svg
						v-else-if="cat.id === 'network'"
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9" />
						<path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5" />
						<circle cx="12" cy="12" r="2" />
						<path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5" />
						<path d="M19.1 4.9C23 8.8 23 15.2 19.1 19.1" />
					</svg>
					<span>{{ cat.label }}</span>
				</button>
			</div>

			<!-- Grid of Organized Setting Cards -->
			<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
				<!-- SECTION 1: GAMEPLAY & CORE RULES -->
				<div
					v-if="
						(selectedCategory === 'all' || selectedCategory === 'gameplay') &&
						matchesSearch([
							'difficulty',
							'gamemode',
							'pvp',
							'hardcore',
							'flight',
							'command',
							'achievements',
							'rules',
						])
					"
					class="p-5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
				>
					<div class="flex items-center justify-between pb-3 border-b border-surface-4">
						<div class="flex items-center gap-2">
							<div
								class="w-6 h-6 rounded-lg bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] flex items-center justify-center text-[var(--color-brand-highlight,var(--color-brand))]"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<line x1="6" y1="12" x2="10" y2="12" />
									<line x1="8" y1="10" x2="8" y2="14" />
									<line x1="15" y1="13" x2="15.01" y2="13" />
									<line x1="18" y1="11" x2="18.01" y2="11" />
									<rect x="2" y="6" width="20" height="12" rx="6" />
								</svg>
							</div>
							<span class="text-xs font-bold text-contrast uppercase tracking-wider"
								>Gameplay &amp; Core Rules</span
							>
						</div>
					</div>

					<div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
						<!-- Difficulty Dropdown -->
						<div class="relative flex flex-col gap-1.5">
							<div class="flex items-center justify-between">
								<label class="text-xs font-semibold text-secondary">Difficulty</label>
								<span class="text-[10px] text-zinc-500 font-mono">difficulty</span>
							</div>
							<button
								type="button"
								class="flex items-center justify-between px-3.5 py-2.5 rounded-xl bg-surface-3 border border-surface-4 hover:border-brand/50 text-contrast text-xs font-medium cursor-pointer transition-all focus:outline-none"
								:class="{
									'border-[var(--color-brand-shadow)] shadow-[var(--accent-glow)]':
										isDifficultyOpen,
								}"
								@click="isDifficultyOpen = !isDifficultyOpen"
							>
								<span class="font-bold capitalize">{{ form.difficulty }}</span>
								<span class="text-zinc-400 text-xs">▼</span>
							</button>

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
												? 'bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border border-[var(--color-brand-shadow)] font-bold'
												: 'hover:bg-zinc-800/80 text-zinc-300 hover:text-white border border-transparent'
										"
										@click="selectDifficulty(opt.id)"
									>
										<span class="text-xs">{{ opt.label }}</span>
										<span
											v-if="form.difficulty === opt.id"
											class="text-[var(--color-brand-highlight,var(--color-brand))] text-xs font-bold"
											>✓</span
										>
									</div>
								</div>
							</transition>
						</div>

						<!-- Gamemode Dropdown -->
						<div class="relative flex flex-col gap-1.5">
							<div class="flex items-center justify-between">
								<label class="text-xs font-semibold text-secondary">Default Gamemode</label>
								<span class="text-[10px] text-zinc-500 font-mono">gamemode</span>
							</div>
							<button
								type="button"
								class="flex items-center justify-between px-3.5 py-2.5 rounded-xl bg-surface-3 border border-surface-4 hover:border-brand/50 text-contrast text-xs font-medium cursor-pointer transition-all focus:outline-none"
								:class="{
									'border-[var(--color-brand-shadow)] shadow-[var(--accent-glow)]': isGamemodeOpen,
								}"
								@click="isGamemodeOpen = !isGamemodeOpen"
							>
								<span class="font-bold capitalize">{{ form.gamemode }}</span>
								<span class="text-zinc-400 text-xs">▼</span>
							</button>

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
												? 'bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border border-[var(--color-brand-shadow)] font-bold'
												: 'hover:bg-zinc-800/80 text-zinc-300 hover:text-white border border-transparent'
										"
										@click="selectGamemode(opt.id)"
									>
										<span class="text-xs">{{ opt.label }}</span>
										<span
											v-if="form.gamemode === opt.id"
											class="text-[var(--color-brand-highlight,var(--color-brand))] text-xs font-bold"
											>✓</span
										>
									</div>
								</div>
							</transition>
						</div>
					</div>

					<!-- Gameplay Toggles -->
					<div class="flex flex-col gap-2 pt-1">
						<!-- PvP Toggle -->
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.pvp = !form.pvp"
						>
							<span class="text-xs font-semibold text-contrast">Player vs Player (PvP)</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.pvp ? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]' : 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.pvp }"
								/>
							</div>
						</div>

						<!-- Hardcore Toggle -->
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.hardcore = !form.hardcore"
						>
							<span class="text-xs font-semibold text-contrast">Hardcore Mode</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="form.hardcore ? 'bg-rose-500' : 'bg-zinc-700'"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.hardcore }"
								/>
							</div>
						</div>

						<!-- Flight Toggle -->
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.allow_flight = !form.allow_flight"
						>
							<span class="text-xs font-semibold text-contrast">Allow Survival Flight</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.allow_flight
										? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
										: 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.allow_flight }"
								/>
							</div>
						</div>

						<!-- Command Blocks Toggle -->
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.enable_command_block = !form.enable_command_block"
						>
							<span class="text-xs font-semibold text-contrast">Enable Command Blocks</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.enable_command_block
										? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
										: 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.enable_command_block }"
								/>
							</div>
						</div>
					</div>
				</div>

				<!-- SECTION 2: WORLD & GENERATION -->
				<div
					v-if="
						(selectedCategory === 'all' || selectedCategory === 'world') &&
						matchesSearch(['world', 'seed', 'nether', 'spawn', 'level', 'structures', 'protection'])
					"
					class="p-5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
				>
					<div class="flex items-center justify-between pb-3 border-b border-surface-4">
						<div class="flex items-center gap-2">
							<div
								class="w-6 h-6 rounded-lg bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] flex items-center justify-center text-[var(--color-brand-highlight,var(--color-brand))]"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<circle cx="12" cy="12" r="10" />
									<line x1="2" y1="12" x2="22" y2="12" />
									<path
										d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"
									/>
								</svg>
							</div>
							<span class="text-xs font-bold text-contrast uppercase tracking-wider"
								>World &amp; Generation</span
							>
						</div>
					</div>

					<div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
						<!-- World Name -->
						<div class="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-secondary">World Folder Name</label>
							<input
								v-model="form.level_name"
								type="text"
								class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand"
								placeholder="world"
							/>
						</div>

						<!-- World Seed -->
						<div class="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-secondary">World Seed (Optional)</label>
							<input
								v-model="form.level_seed"
								type="text"
								class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand"
								placeholder="Leave blank for random"
							/>
						</div>
					</div>

					<!-- World Type Dropdown -->
					<div class="relative flex flex-col gap-1.5">
						<div class="flex items-center justify-between">
							<label class="text-xs font-semibold text-secondary">World Generator Type</label>
							<span class="text-[10px] text-zinc-500 font-mono">level-type</span>
						</div>
						<button
							type="button"
							class="flex items-center justify-between px-3.5 py-2.5 rounded-xl bg-surface-3 border border-surface-4 hover:border-brand/50 text-contrast text-xs font-medium cursor-pointer transition-all focus:outline-none"
							:class="{ 'border-brand shadow-[0_0_12px_rgba(56,189,248,0.2)]': isLevelTypeOpen }"
							@click="isLevelTypeOpen = !isLevelTypeOpen"
						>
							<span class="font-bold">{{
								levelTypeOptions.find((l) => l.id === form.level_type)?.label || form.level_type
							}}</span>
							<span class="text-zinc-400 text-xs">▼</span>
						</button>

						<transition name="fade">
							<div
								v-if="isLevelTypeOpen"
								class="absolute top-[calc(100%+4px)] left-0 right-0 z-50 p-1.5 rounded-2xl bg-[#0e121a] border border-white/15 shadow-2xl backdrop-blur-xl flex flex-col gap-1"
							>
								<div
									v-for="opt in levelTypeOptions"
									:key="opt.id"
									class="p-2 rounded-xl flex items-center justify-between gap-2 transition-all cursor-pointer select-none"
									:class="
										form.level_type === opt.id
											? 'bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border border-[var(--color-brand-shadow)] font-bold'
											: 'hover:bg-zinc-800/80 text-zinc-300 hover:text-white border border-transparent'
									"
									@click="selectLevelType(opt.id)"
								>
									<span class="text-xs">{{ opt.label }}</span>
									<span
										v-if="form.level_type === opt.id"
										class="text-[var(--color-brand-highlight,var(--color-brand))] text-xs font-bold"
										>✓</span
									>
								</div>
							</div>
						</transition>
					</div>

					<div class="flex flex-col gap-2 pt-1">
						<!-- Nether Dimension Toggle -->
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.allow_nether = !form.allow_nether"
						>
							<span class="text-xs font-semibold text-contrast">Allow Nether Dimension</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.allow_nether
										? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
										: 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.allow_nether }"
								/>
							</div>
						</div>

						<!-- Spawn Protection Radius Slider -->
						<div class="p-3 rounded-xl bg-surface-3 border border-surface-4 flex flex-col gap-2">
							<div class="flex items-center justify-between">
								<span class="text-xs font-semibold text-contrast">Spawn Protection Radius</span>
								<span
									class="text-xs font-bold text-[var(--color-brand-highlight,var(--color-brand))] font-mono"
									>{{ form.spawn_protection }} blocks</span
								>
							</div>
							<input
								v-model.number="form.spawn_protection"
								type="range"
								min="0"
								max="64"
								step="1"
								class="w-full accent-[var(--color-brand)] cursor-pointer h-1.5 bg-surface-2 rounded-lg appearance-none"
							/>
						</div>
					</div>
				</div>

				<!-- SECTION 3: ACCESS & SECURITY -->
				<div
					v-if="
						(selectedCategory === 'all' || selectedCategory === 'security') &&
						matchesSearch([
							'online',
							'mojang',
							'auth',
							'whitelist',
							'players',
							'slots',
							'op',
							'timeout',
							'idle',
						])
					"
					class="p-5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
				>
					<div class="flex items-center justify-between pb-3 border-b border-surface-4">
						<div class="flex items-center gap-2">
							<div
								class="w-6 h-6 rounded-lg bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] flex items-center justify-center text-[var(--color-brand-highlight,var(--color-brand))]"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
									<path d="M7 11V7a5 5 0 0 1 10 0v4" />
								</svg>
							</div>
							<span class="text-xs font-bold text-contrast uppercase tracking-wider"
								>Access, Security &amp; Slots</span
							>
						</div>
					</div>

					<!-- Max Players Slider -->
					<div class="p-3 rounded-xl bg-surface-3 border border-surface-4 flex flex-col gap-2">
						<div class="flex items-center justify-between">
							<span class="text-xs font-semibold text-contrast">Max Player Slots</span>
							<span
								class="text-sm font-black text-[var(--color-brand-highlight,var(--color-brand))] font-mono"
								>{{ form.max_players }} players</span
							>
						</div>
						<input
							v-model.number="form.max_players"
							type="range"
							min="1"
							max="100"
							step="1"
							class="w-full accent-[var(--color-brand)] cursor-pointer h-1.5 bg-surface-2 rounded-lg appearance-none"
						/>
					</div>

					<!-- Online Mode Toggle -->
					<div
						class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.online_mode = !form.online_mode"
					>
						<div class="flex items-center gap-2">
							<span class="text-xs font-semibold text-contrast">Online Mode (Mojang Auth)</span>
							<span
								class="px-1.5 py-0.2 rounded text-[9px] font-bold"
								:class="
									form.online_mode
										? 'bg-emerald-500/20 text-emerald-300'
										: 'bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))]'
								"
							>
								{{ form.online_mode ? 'Official Only' : 'Offline / Cracked Enabled' }}
							</span>
						</div>
						<div
							class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
							:class="
								form.online_mode
									? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
									: 'bg-zinc-700'
							"
						>
							<div
								class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
								:class="{ 'translate-x-4.5': form.online_mode }"
							/>
						</div>
					</div>

					<!-- Whitelist Toggles -->
					<div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.white_list = !form.white_list"
						>
							<span class="text-xs font-semibold text-contrast">Server Whitelist</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.white_list
										? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
										: 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.white_list }"
								/>
							</div>
						</div>

						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.enforce_whitelist = !form.enforce_whitelist"
						>
							<span class="text-xs font-semibold text-contrast">Enforce Whitelist</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.enforce_whitelist
										? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
										: 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.enforce_whitelist }"
								/>
							</div>
						</div>
					</div>

					<!-- OP Permission Level Dropdown -->
					<div class="relative flex flex-col gap-1.5">
						<label class="text-xs font-semibold text-secondary">Default OP Permission Level</label>
						<button
							type="button"
							class="flex items-center justify-between px-3.5 py-2.5 rounded-xl bg-surface-3 border border-surface-4 hover:border-brand/50 text-contrast text-xs font-medium cursor-pointer transition-all focus:outline-none"
							:class="{
								'border-[var(--color-brand-shadow)] shadow-[var(--accent-glow)]': isOpLevelOpen,
							}"
							@click="isOpLevelOpen = !isOpLevelOpen"
						>
							<span class="font-bold">{{
								opLevelOptions.find((o) => o.id === form.op_permission_level)?.label ||
								`Level ${form.op_permission_level}`
							}}</span>
							<span class="text-zinc-400 text-xs">▼</span>
						</button>

						<transition name="fade">
							<div
								v-if="isOpLevelOpen"
								class="absolute top-[calc(100%+4px)] left-0 right-0 z-50 p-1.5 rounded-2xl bg-[#0e121a] border border-white/15 shadow-2xl backdrop-blur-xl flex flex-col gap-1"
							>
								<div
									v-for="opt in opLevelOptions"
									:key="opt.id"
									class="p-2 rounded-xl flex items-center justify-between gap-2 transition-all cursor-pointer select-none"
									:class="
										form.op_permission_level === opt.id
											? 'bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border border-[var(--color-brand-shadow)] font-bold'
											: 'hover:bg-zinc-800/80 text-zinc-300 hover:text-white border border-transparent'
									"
									@click="selectOpLevel(opt.id)"
								>
									<span class="text-xs">{{ opt.label }}</span>
									<span
										v-if="form.op_permission_level === opt.id"
										class="text-[var(--color-brand-highlight,var(--color-brand))] text-xs font-bold"
										>✓</span
									>
								</div>
							</div>
						</transition>
					</div>
				</div>

				<!-- SECTION 4: PERFORMANCE & LIMITS -->
				<div
					v-if="
						(selectedCategory === 'all' || selectedCategory === 'performance') &&
						matchesSearch([
							'distance',
							'view',
							'simulation',
							'chunks',
							'performance',
							'tick',
							'compression',
						])
					"
					class="p-5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
				>
					<div class="flex items-center justify-between pb-3 border-b border-surface-4">
						<div class="flex items-center gap-2">
							<div
								class="w-6 h-6 rounded-lg bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] flex items-center justify-center text-[var(--color-brand-highlight,var(--color-brand))]"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<path d="m12 14 4-4" />
									<path d="M3.34 19a10 10 0 1 1 17.32 0" />
								</svg>
							</div>
							<span class="text-xs font-bold text-contrast uppercase tracking-wider"
								>Performance &amp; Chunk Limits</span
							>
						</div>
					</div>

					<!-- View Distance -->
					<div class="p-3 rounded-xl bg-surface-3 border border-surface-4 flex flex-col gap-2">
						<div class="flex items-center justify-between">
							<span class="text-xs font-semibold text-contrast">View Distance</span>
							<span class="text-xs font-bold text-emerald-400 font-mono"
								>{{ form.view_distance }} chunks</span
							>
						</div>
						<input
							v-model.number="form.view_distance"
							type="range"
							min="4"
							max="32"
							step="1"
							class="w-full accent-emerald-500 cursor-pointer h-1.5 bg-surface-2 rounded-lg appearance-none"
						/>
					</div>

					<!-- Simulation Distance -->
					<div class="p-3 rounded-xl bg-surface-3 border border-surface-4 flex flex-col gap-2">
						<div class="flex items-center justify-between">
							<span class="text-xs font-semibold text-contrast">Simulation Distance</span>
							<span class="text-xs font-bold text-emerald-400 font-mono"
								>{{ form.simulation_distance }} chunks</span
							>
						</div>
						<input
							v-model.number="form.simulation_distance"
							type="range"
							min="3"
							max="16"
							step="1"
							class="w-full accent-emerald-500 cursor-pointer h-1.5 bg-surface-2 rounded-lg appearance-none"
						/>
					</div>

					<!-- Network Compression Threshold -->
					<div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
						<div class="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-secondary">Compression Threshold</label>
							<input
								v-model.number="form.network_compression_threshold"
								type="number"
								min="64"
								max="1024"
								class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand"
							/>
						</div>

						<div class="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-secondary">Max Tick Time (ms)</label>
							<input
								v-model.number="form.max_tick_time"
								type="number"
								min="-1"
								max="120000"
								class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand"
							/>
						</div>
					</div>
				</div>

				<!-- SECTION 5: ENTITY & MOB SPAWNING -->
				<div
					v-if="
						(selectedCategory === 'all' || selectedCategory === 'spawning') &&
						matchesSearch(['spawn', 'monsters', 'animals', 'villagers', 'mobs', 'entities', 'npcs'])
					"
					class="p-5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
				>
					<div class="flex items-center justify-between pb-3 border-b border-surface-4">
						<div class="flex items-center gap-2">
							<div
								class="w-6 h-6 rounded-lg bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] flex items-center justify-center text-[var(--color-brand-highlight,var(--color-brand))]"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<path d="M9 10h.01" />
									<path d="M15 10h.01" />
									<path d="M10 2v2" />
									<path d="M14 2v2" />
									<path d="M12 17a4 4 0 0 1-4-4V7a4 4 0 0 1 8 0v6a4 4 0 0 1-4 4Z" />
									<path d="M18 13a6 6 0 0 0-12 0" />
									<path d="M6 13v4" />
									<path d="M18 13v4" />
								</svg>
							</div>
							<span class="text-xs font-bold text-contrast uppercase tracking-wider"
								>Entity &amp; Mob Spawning</span
							>
						</div>
					</div>

					<div class="flex flex-col gap-2.5">
						<!-- Spawn Monsters Toggle -->
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.spawn_monsters = !form.spawn_monsters"
						>
							<span class="text-xs font-semibold text-contrast">Spawn Hostile Monsters</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.spawn_monsters
										? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
										: 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.spawn_monsters }"
								/>
							</div>
						</div>

						<!-- Spawn Animals Toggle -->
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.spawn_animals = !form.spawn_animals"
						>
							<span class="text-xs font-semibold text-contrast">Spawn Passive Animals</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.spawn_animals
										? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
										: 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.spawn_animals }"
								/>
							</div>
						</div>

						<!-- Spawn NPCs Toggle -->
						<div
							class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
							@click="form.spawn_npcs = !form.spawn_npcs"
						>
							<span class="text-xs font-semibold text-contrast">Spawn Villagers & NPCs</span>
							<div
								class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
								:class="
									form.spawn_npcs
										? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
										: 'bg-zinc-700'
								"
							>
								<div
									class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
									:class="{ 'translate-x-4.5': form.spawn_npcs }"
								/>
							</div>
						</div>
					</div>
				</div>

				<!-- SECTION 6: NETWORK & PORTS -->
				<div
					v-if="
						(selectedCategory === 'all' || selectedCategory === 'network') &&
						matchesSearch(['port', 'rcon', 'query', 'resource', 'network', 'pack', 'sha1'])
					"
					class="p-5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-4"
				>
					<div class="flex items-center justify-between pb-3 border-b border-surface-4">
						<div class="flex items-center gap-2">
							<div
								class="w-6 h-6 rounded-lg bg-[var(--color-brand-bg)] border border-[var(--color-brand-shadow)] flex items-center justify-center text-[var(--color-brand-highlight,var(--color-brand))]"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9" />
									<path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5" />
									<circle cx="12" cy="12" r="2" />
									<path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5" />
									<path d="M19.1 4.9C23 8.8 23 15.2 19.1 19.1" />
								</svg>
							</div>
							<span class="text-xs font-bold text-contrast uppercase tracking-wider"
								>Network, Ports &amp; Resources</span
							>
						</div>
					</div>

					<div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
						<!-- Server Port -->
						<div class="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-secondary">Local Server Port</label>
							<input
								v-model.number="form.server_port"
								type="number"
								min="1024"
								max="65535"
								class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand font-bold"
							/>
						</div>

						<!-- Query Port -->
						<div class="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-secondary">GS4 Query Port</label>
							<input
								v-model.number="form.query_port"
								type="number"
								min="1024"
								max="65535"
								class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand"
							/>
						</div>
					</div>

					<!-- Server Resource Pack URL -->
					<div class="flex flex-col gap-1.5">
						<label class="text-xs font-semibold text-secondary"
							>Direct Resource Pack URL (Optional)</label
						>
						<input
							v-model="form.resource_pack"
							type="text"
							class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand"
							placeholder="https://example.com/texturepack.zip"
						/>
					</div>

					<!-- RCON Remote Console Toggle -->
					<div
						class="flex items-center justify-between p-3 rounded-xl bg-surface-3 border border-surface-4 cursor-pointer hover:border-surface-5 transition-all select-none"
						@click="form.enable_rcon = !form.enable_rcon"
					>
						<span class="text-xs font-semibold text-contrast">Enable Remote RCON Console</span>
						<div
							class="w-10 h-5.5 rounded-full p-0.5 transition-colors duration-200"
							:class="
								form.enable_rcon
									? 'bg-[var(--color-brand)] shadow-[var(--accent-glow)]'
									: 'bg-zinc-700'
							"
						>
							<div
								class="w-4.5 h-4.5 rounded-full bg-white transition-transform duration-200 shadow-sm"
								:class="{ 'translate-x-4.5': form.enable_rcon }"
							/>
						</div>
					</div>

					<div v-if="form.enable_rcon" class="grid grid-cols-1 sm:grid-cols-2 gap-3 pt-1">
						<div class="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-secondary">RCON Port</label>
							<input
								v-model.number="form.rcon_port"
								type="number"
								class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand"
							/>
						</div>

						<div class="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-secondary">RCON Password</label>
							<input
								v-model="form.rcon_password"
								type="password"
								class="w-full px-3 py-2 rounded-xl bg-surface-3 border border-surface-4 text-contrast text-xs font-mono focus:outline-none focus:border-brand"
								placeholder="Enter secure password"
							/>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>
