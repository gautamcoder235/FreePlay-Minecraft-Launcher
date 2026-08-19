<template>
	<div data-tauri-drag-region class="app-header bg-bg-raised h-[--top-bar-height] flex select-none">
		<div data-tauri-drag-region class="flex min-w-0 flex-1 items-center overflow-hidden p-2">
			<!-- FreePlay Brand Logo -->
			<div class="flex items-center gap-2.5 mr-3 pointer-events-none select-none pl-1">
				<div class="relative flex items-center justify-center w-7 h-7 rounded-xl bg-gradient-to-br from-[#6366f1] via-[#8b5cf6] to-[#06b6d4] shadow-[0_0_15px_rgba(99,102,241,0.5)] border border-white/20">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" class="w-3.5 h-3.5">
						<path d="M4 6.5C4 5.11929 5.11929 4 6.5 4H17.5C18.8807 4 20 5.11929 20 6.5V11C20 15.4183 16.4183 19 12 19C7.58172 19 4 15.4183 4 11V6.5Z" fill="white" fill-opacity="0.25"/>
						<path d="M8.5 8L16.5 12L8.5 16V8Z" fill="white"/>
					</svg>
				</div>
				<div class="flex flex-col leading-none">
					<span class="font-black tracking-wider text-sm text-contrast font-sans flex items-center gap-1.5">
						FREEPLAY
						<span class="text-[9px] font-extrabold px-1.5 py-0.5 rounded bg-gradient-to-r from-indigo-500/20 to-cyan-500/20 border border-indigo-500/30 text-cyan-300 uppercase tracking-widest">Launcher</span>
					</span>
				</div>
			</div>

			<!-- History Navigation -->
			<div data-tauri-drag-region class="flex shrink-0 items-center gap-2">
				<IconButton
					type="outlined"
					:label="formatMessage(messages.goBack)"
					class="!h-7 !min-w-7 !w-7 !border !border-surface-4 !p-0 !opacity-100 hover:!border-brand/40 transition-colors"
					:disabled="!canNavigateBack"
					@click="router.back()"
				>
					<ChevronLeftIcon
						class="!size-4 !text-primary"
						:class="{ 'opacity-20': !canNavigateBack }"
					/>
				</IconButton>
				<IconButton
					type="outlined"
					:label="formatMessage(messages.goForward)"
					class="!h-7 !min-w-7 !w-7 !border !border-surface-4 !p-0 !opacity-100 hover:!border-brand/40 transition-colors"
					:disabled="!canNavigateForward"
					@click="router.forward()"
				>
					<ChevronRightIcon
						class="!size-4 !text-primary"
						:class="{ 'opacity-20': !canNavigateForward }"
					/>
				</IconButton>
			</div>

			<Breadcrumbs />
		</div>

		<!-- Action bar & Window Controls -->
		<section data-tauri-drag-region class="flex shrink-0 ml-auto items-center">
			<IconButton
				v-if="!forceSidebar && themeStore.toggleSidebar"
				:type="sidebarToggled ? 'base' : 'quiet'"
				:label="formatMessage(messages.nextImage)"
				class="mr-3 transition-transform"
				:class="{ 'rotate-180': !sidebarToggled }"
				@click="emit('toggleSidebar')"
			>
				<RightArrowIcon />
			</IconButton>
			<div class="flex mr-3">
				<Suspense>
					<AppActionBar />
				</Suspense>
			</div>
			<WindowControls />
		</section>
	</div>
</template>

<script setup lang="ts">
import { ChevronLeftIcon, ChevronRightIcon, RightArrowIcon } from '@modrinth/assets'
import { defineMessages, IconButton, useVIntl } from '@modrinth/ui'
import { useRouter } from 'vue-router'

import AppActionBar from '@/components/ui/AppActionBar.vue'
import Breadcrumbs from '@/components/ui/Breadcrumbs.vue'
import WindowControls from '@/components/ui/WindowControls.vue'
import { useTheming } from '@/store/theme'

defineProps<{
	canNavigateBack: boolean
	canNavigateForward: boolean
	sidebarToggled: boolean
	forceSidebar: boolean
}>()

const emit = defineEmits<{
	(e: 'toggleSidebar'): void
}>()

const router = useRouter()
const themeStore = useTheming()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	goBack: { id: 'app.navigation.go-back', defaultMessage: 'Go back' },
	goForward: { id: 'app.navigation.go-forward', defaultMessage: 'Go forward' },
	nextImage: { id: 'app.navigation.next-image', defaultMessage: 'Next image' },
})
</script>

<style scoped>
.app-header {
	padding-right: var(--window-controls-width, 0px);
	position: relative;
	z-index: 2;
}
</style>
