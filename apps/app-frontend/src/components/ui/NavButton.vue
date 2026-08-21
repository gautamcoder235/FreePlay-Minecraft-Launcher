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
		class="nav-button relative w-11 h-11 text-zinc-400 rounded-2xl flex items-center justify-center text-xl cursor-pointer transition-all duration-200 bg-white/[0.03] hover:bg-white/[0.08] hover:text-white hover:scale-105 active:scale-95 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--color-brand)] border border-transparent hover:border-white/10 select-none group"
	>
		<!-- Active Left Neon Indicator Bar -->
		<span
			class="active-indicator absolute -left-1.5 top-2.5 bottom-2.5 w-1 rounded-r-full bg-[var(--color-brand-highlight,var(--color-brand))] opacity-0 group-[.router-link-active]:opacity-100 group-[.subpage-active]:opacity-75 transition-all duration-300 shadow-[0_0_8px_var(--color-brand-shadow)]"
		></span>
		<slot />
	</RouterLink>
	<button
		v-else
		v-bind="$attrs"
		class="nav-button relative border border-transparent hover:border-white/10 text-zinc-400 cursor-pointer w-11 h-11 rounded-2xl flex items-center justify-center text-xl transition-all duration-200 bg-white/[0.03] hover:bg-white/[0.08] hover:text-white hover:scale-105 active:scale-95 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--color-brand)] select-none group"
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
		filter: drop-shadow(0 0 8px var(--color-brand-shadow));
	}
}

.router-link-active {
	background: var(--color-brand-bg, rgba(14, 165, 233, 0.2));
	border-color: var(--color-brand-highlight, var(--color-brand)) !important;
	color: var(--color-brand-highlight, #ffffff) !important;
	box-shadow: 0 0 16px var(--color-brand-shadow);
}

.subpage-active {
	background: var(--color-brand-bg, rgba(14, 165, 233, 0.15));
	border-color: var(--color-brand-shadow, rgba(14, 165, 233, 0.4)) !important;
	color: var(--color-brand-highlight, #ffffff) !important;
	box-shadow: 0 0 10px var(--color-brand-shadow);
}
</style>
