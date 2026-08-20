<script setup lang="ts">
import { BlocksIcon, ChevronRightIcon, PlayIcon, PlusIcon, StopCircleIcon } from '@freeplay/assets'
import { computed, inject, onMounted, onUnmounted, ref } from 'vue'

import { useAppEvent } from '@/composables/use-app-event'
import { create_offline_account, set_default_user } from '@/helpers/auth'
import { list as listInstances, run as runInstance } from '@/helpers/instance'
import { get_java_versions, get_max_memory } from '@/helpers/jre'
import { get_all as getRunningProcesses, kill as killProcess } from '@/helpers/process'
import { get as getSettings } from '@/helpers/settings.ts'
import type { GameInstance } from '@/helpers/types'
import { useAccountStore } from '@/store/account.ts'

const accountStore = useAccountStore()
const showCreationModal = inject<() => void>('showCreationModal', () => {})

const instances = ref<GameInstance[]>([])
const runningInstances = ref<string[]>([])
const allocatedRam = ref(4)
const totalSystemRam = ref(16)
const detectedJava = ref<string>('')
const isOnline = ref(navigator.onLine)

const heroInstance = computed(() => instances.value[0] ?? null)
const isHeroRunning = computed(() =>
	heroInstance.value ? runningInstances.value.includes(heroInstance.value.id) : false,
)

function updateOnlineStatus() {
	isOnline.value = navigator.onLine
}

async function loadData() {
	try {
		// 1. Fetch live instances and running processes
		const [instList, procs, settings, maxMemKiB, jres] = await Promise.all([
			listInstances().catch(() => []),
			getRunningProcesses().catch(() => []),
			getSettings().catch(() => null),
			get_max_memory().catch(() => null),
			get_java_versions().catch(() => []),
		])

		instances.value = instList || []
		liveProcesses.value = procs || []
		runningInstances.value = (procs || [])
			.map(
				(p: { instance_id?: string; instance?: { id?: string } }) =>
					p.instance_id || p.instance?.id,
			)
			.filter((id): id is string => Boolean(id))

		// 2. Compute true physical memory
		if (typeof maxMemKiB === 'number' && maxMemKiB > 0) {
			totalSystemRam.value = Math.round(maxMemKiB / 1024 / 1024) || 16
		}

		// 3. Compute allocated memory from settings
		if (settings) {
			const memVal =
				typeof settings.memory === 'number'
					? settings.memory
					: typeof settings.memory?.max === 'number'
						? settings.memory.max
						: 4096
			allocatedRam.value = Math.round((memVal / 1024) * 10) / 10 || 4
		}

		// 4. Detected Java version
		if (Array.isArray(jres) && jres.length > 0) {
			const topJre = jres[0]
			detectedJava.value =
				typeof topJre === 'string' ? topJre : topJre?.version ? `Java ${topJre.version}` : 'Java 21'
		} else {
			detectedJava.value = 'Java 21 (Auto)'
		}
	} catch {
		// ignore
	}
}

const liveProcesses = ref<
	Array<{ uuid: string; instance_id?: string; instance?: { id?: string } }>
>([])

async function handleStopHero() {
	if (!heroInstance.value) return
	const proc = liveProcesses.value.find(
		(p) => (p.instance_id || p.instance?.id) === heroInstance.value?.id,
	)
	if (proc?.uuid) {
		await killProcess(proc.uuid).catch(() => {})
		await loadData()
	}
}

async function handleLaunch(instanceId: string) {
	try {
		if (accountStore.activeAccount?.isOffline && accountStore.activeAccount?.name) {
			await create_offline_account(accountStore.activeAccount.name).catch(() => {})
		} else if (accountStore.activeAccountId) {
			await set_default_user(accountStore.activeAccountId).catch(() => {})
		}
		await runInstance(instanceId)
		await loadData()
	} catch {
		// ignore
	}
}

useAppEvent('process', loadData)
useAppEvent('instance', loadData)

onMounted(() => {
	accountStore.init()
	loadData()
	window.addEventListener('online', updateOnlineStatus)
	window.addEventListener('offline', updateOnlineStatus)
	window.addEventListener('focus', loadData)
	window.addEventListener('visibilitychange', loadData)
})

onUnmounted(() => {
	window.removeEventListener('online', updateOnlineStatus)
	window.removeEventListener('offline', updateOnlineStatus)
	window.removeEventListener('focus', loadData)
	window.removeEventListener('visibilitychange', loadData)
})
</script>

<template>
	<div class="p-3 flex flex-col gap-3 select-none text-white">
		<!-- RECENT INSTANCE (Emerald / Teal) -->
		<div
			v-if="heroInstance"
			class="relative overflow-hidden rounded-2xl bg-[#141923] border border-white/10 p-3.5 flex flex-col gap-3 shadow-xl group hover:border-emerald-500/30 transition-all duration-200"
		>
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<div
						class="p-1.5 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-400"
					>
						<BlocksIcon class="w-3.5 h-3.5" />
					</div>
					<span class="text-xs font-black uppercase tracking-wider text-white font-mono">
						Recent Instance
					</span>
				</div>
				<router-link
					:to="`/instance/${heroInstance.id}`"
					class="text-[10px] font-bold text-zinc-400 hover:text-emerald-400 transition-colors flex items-center gap-0.5"
				>
					Manage <ChevronRightIcon class="w-2.5 h-2.5" />
				</router-link>
			</div>

			<div class="flex items-center gap-3 p-2 rounded-xl bg-black/30 border border-white/5">
				<div
					class="w-10 h-10 rounded-xl bg-gradient-to-br from-emerald-500/20 to-teal-500/20 border border-emerald-500/30 flex items-center justify-center text-emerald-300 font-bold text-sm shrink-0 shadow-md"
				>
					{{ heroInstance.name.substring(0, 2).toUpperCase() }}
				</div>
				<div class="flex flex-col min-w-0 flex-1">
					<span class="text-xs font-bold text-white truncate">{{ heroInstance.name }}</span>
					<span class="text-[10px] text-zinc-400 font-mono truncate">
						{{ heroInstance.game_version || '1.21.1' }} • {{ heroInstance.loader || 'Fabric' }}
					</span>
				</div>
			</div>

			<button
				type="button"
				class="w-full py-2.5 rounded-xl text-xs font-extrabold flex items-center justify-center gap-2 cursor-pointer border-none shadow-lg active:scale-95 transition-all"
				:class="
					isHeroRunning
						? 'bg-rose-600 hover:bg-rose-500 text-white shadow-rose-950/60'
						: 'bg-gradient-to-r from-emerald-400 to-teal-500 hover:from-emerald-300 hover:to-teal-400 text-zinc-950 shadow-emerald-950/60 font-black'
				"
				@click="isHeroRunning ? handleStopHero() : handleLaunch(heroInstance.id)"
			>
				<StopCircleIcon v-if="isHeroRunning" class="w-3.5 h-3.5" />
				<PlayIcon v-else class="w-3.5 h-3.5 fill-current" />
				<span>{{ isHeroRunning ? 'Stop Game Session' : 'PLAY NOW' }}</span>
			</button>
		</div>

		<!-- WIDGET 2 (FALLBACK): NO INSTANCES - CREATE CTA (Soft Blueish / Sky) -->
		<div
			v-else
			class="relative overflow-hidden rounded-2xl bg-[#141923] border border-white/10 p-3.5 flex flex-col gap-2.5 shadow-xl group hover:border-sky-500/30 transition-all duration-200"
		>
			<div class="flex items-center gap-2">
				<div
					class="p-1.5 rounded-xl bg-sky-500/15 border border-sky-500/30 text-sky-400 shadow-inner"
				>
					<PlusIcon class="w-3.5 h-3.5" />
				</div>
				<span class="text-xs font-black uppercase tracking-wider text-white font-mono">
					Library Ready
				</span>
			</div>

			<p class="text-xs text-zinc-400 m-0 leading-relaxed">
				No instances installed yet. Create an instance or discover modpacks.
			</p>

			<button
				type="button"
				class="w-full py-2.5 rounded-xl bg-gradient-to-r from-sky-400 to-blue-500 hover:from-sky-300 hover:to-blue-400 text-zinc-950 font-black text-xs flex items-center justify-center gap-1.5 shadow-md shadow-sky-950/50 hover:shadow-[0_0_15px_rgba(56,189,248,0.4)] transition-all cursor-pointer border-none active:scale-95"
				@click="showCreationModal?.()"
			>
				<PlusIcon class="w-3.5 h-3.5" />
				<span>Create First Instance</span>
			</button>
		</div>
	</div>
</template>
