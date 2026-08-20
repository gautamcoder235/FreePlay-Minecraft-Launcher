<script setup lang="ts">
import {
	CheckIcon,
	ChevronDownIcon,
	ChevronRightIcon,
	CompassIcon,
	PlusIcon,
	SparklesIcon,
	UserIcon,
} from '@freeplay/assets'
import { defineMessages, useVIntl } from '@freeplay/ui'
import { computed, ref } from 'vue'

import { injectOnboardingChecklist } from '@/providers/onboarding-checklist'

const emit = defineEmits<{
	'create-instance': []
	'login-minecraft': []
	'login-freeplay': []
}>()

const { formatMessage } = useVIntl()
const {
	hasCreatedInstance,
	hasLoggedIntoMinecraft,
	hasLoggedIntoFreePlay,
	isReady,
	showChecklist,
} = injectOnboardingChecklist()

const isOpen = ref(true)

const messages = defineMessages({
	title: {
		id: 'onboarding-checklist.title',
		defaultMessage: 'Launch Missions',
	},
	createInstance: {
		id: 'onboarding-checklist.create-instance',
		defaultMessage: 'Create First Instance',
	},
	loginMinecraft: {
		id: 'onboarding-checklist.login-minecraft',
		defaultMessage: 'Set Player Profile',
	},
	loginFreePlay: {
		id: 'onboarding-checklist.login-freeplay',
		defaultMessage: 'Connect FreePlay Cloud',
	},
})

const steps = computed(() => [
	{
		id: 'create-instance',
		label: formatMessage(messages.createInstance),
		desc: 'Setup your first instance or modpack',
		complete: hasCreatedInstance.value,
		action: () => emit('create-instance'),
		icon: PlusIcon,
	},
	{
		id: 'login-minecraft',
		label: formatMessage(messages.loginMinecraft),
		desc: 'Choose 1-Click Offline or Microsoft profile',
		complete: hasLoggedIntoMinecraft.value,
		action: () => emit('login-minecraft'),
		icon: UserIcon,
	},
	{
		id: 'login-freeplay',
		label: formatMessage(messages.loginFreePlay),
		desc: 'Sync friends, cloud saves & multiplayer tunnels',
		complete: hasLoggedIntoFreePlay.value,
		action: () => emit('login-freeplay'),
		icon: SparklesIcon,
	},
])

const completedCount = computed(() => steps.value.filter((s) => s.complete).length)
const progressPercent = computed(() =>
	Math.round((completedCount.value / steps.value.length) * 100),
)
</script>

<template>
	<div v-if="isReady && showChecklist" class="p-3 border-b border-white/10 select-none">
		<div
			class="relative overflow-hidden rounded-2xl bg-gradient-to-b from-[#141923] to-[#0d1117] border border-white/10 shadow-xl transition-all duration-300"
		>
			<!-- Header Toggle Bar -->
			<button
				type="button"
				class="w-full p-3.5 flex items-center justify-between bg-transparent border-none cursor-pointer text-left hover:bg-white/[0.03] transition-colors"
				@click="isOpen = !isOpen"
			>
				<div class="flex items-center gap-2.5">
					<div
						class="p-1.5 rounded-xl bg-sky-500/10 border border-sky-500/30 text-sky-400 shadow-[0_0_12px_rgba(56,189,248,0.2)]"
					>
						<CompassIcon class="w-4 h-4" />
					</div>
					<div class="flex flex-col">
						<div class="flex items-center gap-2">
							<span class="text-xs font-black tracking-wider uppercase text-white font-mono"
								>Launch Missions</span
							>
							<span
								class="text-[9px] font-extrabold px-1.5 py-0.5 rounded-full bg-sky-500/20 text-sky-300 border border-sky-500/30 font-mono"
							>
								{{ completedCount }}/{{ steps.length }}
							</span>
						</div>
						<span class="text-[11px] text-zinc-400 font-medium">Getting started checklist</span>
					</div>
				</div>

				<div class="flex items-center gap-2">
					<span class="text-xs font-extrabold font-mono text-sky-400">{{ progressPercent }}%</span>
					<ChevronDownIcon
						class="w-4 h-4 text-zinc-400 transition-transform duration-200"
						:class="{ '-rotate-180': isOpen }"
					/>
				</div>
			</button>

			<!-- Progress Bar Track -->
			<div class="w-full h-1 bg-zinc-800/80 overflow-hidden">
				<div
					class="h-full bg-gradient-to-r from-sky-500 via-cyan-400 to-indigo-500 transition-all duration-500"
					:style="{ width: `${progressPercent}%` }"
				></div>
			</div>

			<!-- Mission Step Items -->
			<div v-if="isOpen" class="p-3 flex flex-col gap-2 bg-black/20">
				<div
					v-for="step in steps"
					:key="step.id"
					class="group relative flex items-center justify-between p-2.5 rounded-xl border transition-all duration-200"
					:class="
						step.complete
							? 'bg-sky-500/[0.04] border-sky-500/20 text-zinc-300'
							: 'bg-zinc-900/80 border-white/5 hover:border-indigo-500/40 hover:bg-zinc-800/60 text-white cursor-pointer shadow-sm active:scale-[0.99]'
					"
					@click="!step.complete && step.action()"
				>
					<div class="flex items-center gap-2.5 min-w-0 pr-2">
						<div
							class="w-6 h-6 rounded-lg flex items-center justify-center shrink-0 transition-colors"
							:class="
								step.complete
									? 'bg-sky-500 text-zinc-950 shadow-[0_0_10px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 text-zinc-400 group-hover:text-indigo-400 group-hover:bg-indigo-500/10'
							"
						>
							<CheckIcon v-if="step.complete" class="w-3.5 h-3.5 stroke-[3]" />
							<component :is="step.icon" v-else class="w-3.5 h-3.5" />
						</div>

						<div class="flex flex-col min-w-0">
							<span
								class="text-xs font-bold truncate transition-colors"
								:class="
									step.complete
										? 'text-zinc-400 line-through'
										: 'text-zinc-200 group-hover:text-white'
								"
							>
								{{ step.label }}
							</span>
							<span class="text-[10px] text-zinc-500 truncate leading-tight">
								{{ step.desc }}
							</span>
						</div>
					</div>

					<div class="shrink-0">
						<span
							v-if="step.complete"
							class="text-[9px] font-extrabold px-1.5 py-0.5 rounded bg-sky-500/20 text-sky-300 font-mono"
						>
							DONE
						</span>
						<button
							v-else
							type="button"
							class="px-2.5 py-1 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-bold text-[10px] border-none cursor-pointer flex items-center gap-1 shadow-md shadow-indigo-950/60 transition-all"
							@click.stop="step.action"
						>
							<span>Start</span>
							<ChevronRightIcon class="w-2.5 h-2.5" />
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>
