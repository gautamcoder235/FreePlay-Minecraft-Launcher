<template>
	<div data-tauri-drag-region class="app-header bg-zinc-950/80 backdrop-blur-md border-b border-white/10 h-[--top-bar-height] flex select-none">
		<div data-tauri-drag-region class="flex min-w-0 flex-1 items-center overflow-hidden p-2">
			<!-- FreePlay Diamond Brand Logo -->
			<div class="flex items-center gap-2.5 mr-3 pointer-events-none select-none pl-1">
				<div class="relative flex items-center justify-center w-8 h-8 rounded-xl bg-gradient-to-br from-indigo-600 via-purple-600 to-cyan-500 shadow-[0_0_20px_rgba(99,102,241,0.5)] border border-white/20">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" class="w-4 h-4">
						<path d="M12 2L2 9L12 22L22 9L12 2Z" fill="url(#diamond-grad)" fill-opacity="0.9" />
						<path d="M12 2L2 9H22L12 2Z" fill="white" fill-opacity="0.3" />
						<path d="M12 2L7 9L12 22L17 9L12 2Z" fill="white" fill-opacity="0.2" />
						<defs>
							<linearGradient id="diamond-grad" x1="2" y1="2" x2="22" y2="22" gradientUnits="userSpaceOnUse">
								<stop stop-color="#818CF8"/>
								<stop offset="0.5" stop-color="#C084FC"/>
								<stop offset="1" stop-color="#22D3EE"/>
							</linearGradient>
						</defs>
					</svg>
				</div>
				<div class="flex flex-col leading-none">
					<span class="font-black tracking-wider text-sm text-contrast font-sans flex items-center gap-1.5">
						FREEPLAY
						<span class="text-[9px] font-extrabold px-1.5 py-0.5 rounded bg-gradient-to-r from-indigo-500/30 to-cyan-500/30 border border-indigo-400/40 text-cyan-300 uppercase tracking-widest shadow-[0_0_10px_rgba(34,211,238,0.2)]">Launcher</span>
					</span>
				</div>
			</div>

			<!-- History Navigation -->
			<div data-tauri-drag-region class="flex shrink-0 items-center gap-2">
				<IconButton
					type="outlined"
					:label="formatMessage(messages.goBack)"
					class="!h-7 !min-w-7 !w-7 !border !border-white/10 !bg-white/[0.03] hover:!bg-white/10 hover:!border-indigo-500/50 !p-0 !opacity-100 cursor-pointer transition-all duration-200"
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
					class="!h-7 !min-w-7 !w-7 !border !border-white/10 !bg-white/[0.03] hover:!bg-white/10 hover:!border-indigo-500/50 !p-0 !opacity-100 cursor-pointer transition-all duration-200"
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
				class="mr-3 transition-transform cursor-pointer"
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
