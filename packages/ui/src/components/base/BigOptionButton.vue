<template>
	<button
		type="button"
		class="group relative flex w-full hover:cursor-pointer items-center gap-3.5 rounded-2xl p-3 text-left transition-all duration-200 active:scale-[0.98] border border-white/10 bg-[#141923] hover:bg-[#18202e] shadow-md hover:shadow-xl select-none"
		:class="[selected ? '!border-sky-500 !bg-sky-950/30' : themeClasses.border]"
		@click="$emit('click')"
	>
		<!-- Icon Container with Themed Glow -->
		<div
			class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl border transition-transform duration-200 group-hover:scale-105 shadow-inner"
			:class="[selected ? 'bg-sky-500/20 border-sky-500/40 text-sky-400' : themeClasses.iconBox]"
		>
			<component
				:is="icon"
				class="size-6 shrink-0 transition-colors"
				:class="selected ? 'text-sky-400' : themeClasses.icon"
				stroke-width="1.75"
			/>
		</div>

		<!-- Title & Description Body -->
		<div class="flex flex-1 flex-col gap-0.5 min-w-0">
			<div class="flex items-center gap-2">
				<span class="text-sm font-bold text-white group-hover:text-white tracking-wide truncate">
					{{ title }}
				</span>
				<span
					v-if="badge"
					class="text-[9px] font-extrabold px-1.5 py-0.2 rounded font-mono uppercase tracking-wider shrink-0"
					:class="themeClasses.badge"
				>
					{{ badge }}
				</span>
			</div>
			<span class="text-xs font-medium text-zinc-400 line-clamp-1 leading-relaxed">
				{{ description }}
			</span>
		</div>

		<!-- Animated Action Indicator -->
		<div
			class="flex items-center gap-1 shrink-0 pr-1 text-zinc-500 group-hover:text-white transition-all duration-150"
		>
			<ChevronRightIcon
				class="size-4 shrink-0 transform group-hover:translate-x-1 transition-transform"
			/>
		</div>
	</button>
</template>

<script setup lang="ts">
import { ChevronRightIcon } from '@freeplay/assets'
import type { Component } from 'vue'
import { computed } from 'vue'

const props = withDefaults(
	defineProps<{
		icon: Component
		title: string
		description: string
		selected?: boolean
		badge?: string
		colorTheme?: 'emerald' | 'sky' | 'cyan' | 'indigo' | 'purple' | 'amber' | 'default'
	}>(),
	{
		selected: false,
		badge: undefined,
		colorTheme: 'default',
	},
)

defineEmits<{
	(e: 'click'): void
}>()

const themeClasses = computed(() => {
	switch (props.colorTheme) {
		case 'emerald':
		case 'sky':
			return {
				border: 'hover:border-sky-500/40 hover:shadow-[0_0_20px_rgba(56,189,248,0.15)]',
				iconBox: 'bg-sky-500/10 border-sky-500/30 text-sky-400',
				icon: 'text-sky-400',
				badge: 'bg-sky-500/20 text-sky-300 border border-sky-500/30',
			}
		case 'cyan':
			return {
				border: 'hover:border-cyan-500/40 hover:shadow-[0_0_20px_rgba(6,182,212,0.15)]',
				iconBox: 'bg-cyan-500/10 border-cyan-500/30 text-cyan-400',
				icon: 'text-cyan-400',
				badge: 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30',
			}
		case 'indigo':
			return {
				border: 'hover:border-indigo-500/40 hover:shadow-[0_0_20px_rgba(99,102,241,0.15)]',
				iconBox: 'bg-indigo-500/10 border-indigo-500/30 text-indigo-400',
				icon: 'text-indigo-400',
				badge: 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30',
			}
		case 'purple':
			return {
				border: 'hover:border-purple-500/40 hover:shadow-[0_0_20px_rgba(168,85,247,0.15)]',
				iconBox: 'bg-purple-500/10 border-purple-500/30 text-purple-400',
				icon: 'text-purple-400',
				badge: 'bg-purple-500/20 text-purple-300 border border-purple-500/30',
			}
		case 'amber':
			return {
				border: 'hover:border-amber-500/40 hover:shadow-[0_0_20px_rgba(245,158,11,0.15)]',
				iconBox: 'bg-amber-500/10 border-amber-500/30 text-amber-400',
				icon: 'text-amber-400',
				badge: 'bg-amber-500/20 text-amber-300 border border-amber-500/30',
			}
		default:
			return {
				border: 'hover:border-white/20 hover:shadow-lg',
				iconBox: 'bg-white/5 border-white/10 text-zinc-300',
				icon: 'text-zinc-300',
				badge: 'bg-white/10 text-zinc-300 border border-white/10',
			}
	}
})
</script>
