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
		class="w-11 h-11 text-zinc-400 rounded-xl flex items-center justify-center text-xl cursor-pointer transition-all duration-200 bg-white/[0.02] hover:bg-white/10 hover:text-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 relative group"
	>
		<slot />
	</RouterLink>
	<button
		v-else
		v-bind="$attrs"
		class="button-animation border-none text-zinc-400 cursor-pointer w-11 h-11 rounded-xl flex items-center justify-center text-xl transition-all duration-200 bg-white/[0.02] hover:bg-white/10 hover:text-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 relative group"
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

type RouteFunction = (route: RouteLocationNormalizedLoaded) => boolean

withDefaults(
	defineProps<{
		to: (() => void) | string
		isPrimary?: RouteFunction
		isSubpage?: RouteFunction
		highlightOverride?: boolean
		disabled?: boolean
	}>(),
	{
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
		filter: drop-shadow(0 0 8px rgba(99, 102, 241, 0.6));
	}
}

.router-link-active {
	@apply text-indigo-300 bg-indigo-600/30 border border-indigo-500/50 shadow-[0_0_16px_rgba(99,102,241,0.4)];
}

.subpage-active {
	@apply text-indigo-300 bg-indigo-600/20 border border-indigo-500/30 shadow-[0_0_12px_rgba(99,102,241,0.3)];
}
</style>
