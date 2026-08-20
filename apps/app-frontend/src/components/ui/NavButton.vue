<template>
	<RouterLink
		v-if="typeof to === 'string'"
		:to="to"
		v-bind="$attrs"
		:active-class="isSubpage ? '' : undefined"
		:class="{
			'router-link-active': isPrimary && isPrimary(route),
			'subpage-active': isSubpage && isSubpage(route),
			disabled: disabled,
		}"
		class="nav-button relative w-11 h-11 text-zinc-400 rounded-2xl flex items-center justify-center text-xl cursor-pointer transition-all duration-200 bg-white/[0.03] hover:bg-white/[0.08] hover:text-white hover:scale-105 active:scale-95 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-500 border border-transparent hover:border-white/10 select-none group"
	>
		<!-- Active Left Neon Indicator Bar -->
		<span
			class="active-indicator absolute -left-1.5 top-2.5 bottom-2.5 w-1 rounded-r-full bg-sky-400 opacity-0 group-[.router-link-active]:opacity-100 group-[.subpage-active]:opacity-75 transition-all duration-300 shadow-[0_0_8px_rgba(56,189,248,0.8)]"
		></span>
		<slot />
	</RouterLink>
	<button
		v-else
		v-bind="$attrs"
		class="nav-button relative border border-transparent hover:border-white/10 text-zinc-400 cursor-pointer w-11 h-11 rounded-2xl flex items-center justify-center text-xl transition-all duration-200 bg-white/[0.03] hover:bg-white/[0.08] hover:text-white hover:scale-105 active:scale-95 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-500 select-none group"
		:disabled="disabled"
		@click="to"
	>
		<slot />
	</button>
</template>

<script setup lang="ts">
import type { RouteLocationNormalizedLoaded } from 'vue-router'
import { RouterLink, useRoute } from 'vue-router'

const route = useRoute()

type NavRouteFunction = (route: RouteLocationNormalizedLoaded) => boolean

withDefaults(
	defineProps<{
		to: (() => void) | string
		isPrimary?: NavRouteFunction
		isSubpage?: NavRouteFunction
		highlightOverride?: boolean
		disabled?: boolean
	}>(),
	{
		isPrimary: undefined,
		isSubpage: undefined,
		highlightOverride: false,
		disabled: false,
	},
)

defineOptions({
	inheritAttrs: false,
})
</script>

<style lang="scss" scoped>
.router-link-active,
.subpage-active {
	svg {
		filter: drop-shadow(0 0 10px rgba(56, 189, 248, 0.7));
	}
}

.router-link-active {
	background: linear-gradient(135deg, rgba(56, 189, 248, 0.3), rgba(14, 165, 233, 0.2));
	border-color: rgba(56, 189, 248, 0.6) !important;
	color: #7dd3fc !important;
	box-shadow: 0 0 18px rgba(56, 189, 248, 0.4);
}

.subpage-active {
	background: rgba(56, 189, 248, 0.18);
	border-color: rgba(56, 189, 248, 0.4) !important;
	color: #7dd3fc !important;
	box-shadow: 0 0 12px rgba(56, 189, 248, 0.25);
}
</style>
