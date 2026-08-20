<script setup lang="ts">
import { defineMessages, ThemeSelector, Toggle, useVIntl } from '@freeplay/ui'
import { computed, ref, watch } from 'vue'

import { get, set } from '@/helpers/settings.ts'
import { getOS } from '@/helpers/utils'
import { useTheming } from '@/store/state'
import type { ColorTheme } from '@/store/theme.ts'

const themeStore = useTheming()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	colorThemeTitle: {
		id: 'app.appearance-settings.color-theme.title',
		defaultMessage: 'Color theme',
	},
	colorThemeDescription: {
		id: 'app.appearance-settings.color-theme.description',
		defaultMessage: 'Choose the color theme used by FreePlay Launcher.',
	},
	advancedRenderingTitle: {
		id: 'app.appearance-settings.advanced-rendering.title',
		defaultMessage: 'Advanced rendering',
	},
	advancedRenderingDescription: {
		id: 'app.appearance-settings.advanced-rendering.description',
		defaultMessage:
			'Enable visual effects such as background blur. This may reduce performance without hardware acceleration.',
	},
	nativeDecorationsTitle: {
		id: 'app.appearance-settings.native-decorations.title',
		defaultMessage: 'System window frame',
	},
	nativeDecorationsDescription: {
		id: 'app.appearance-settings.native-decorations.description',
		defaultMessage:
			"Use your operating system's title bar and window controls. Requires an app restart.",
	},
})

const os = ref(await getOS())
const settings = ref(await get())
const themeOptions = computed(() =>
	themeStore
		.getThemeOptions()
		.filter((theme) => theme !== 'retro' || themeStore.devMode || settings.value.theme === 'retro'),
)

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)
</script>
<template>
	<div class="flex flex-col gap-6">
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(messages.colorThemeTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					{{ formatMessage(messages.colorThemeDescription) }}
				</p>
			</div>

			<div class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm">
				<ThemeSelector
					:update-color-theme="
						(theme: ColorTheme) => {
							themeStore.setThemeState(theme)
							settings.theme = theme
						}
					"
					:current-theme="settings.theme"
					:theme-options="themeOptions"
					system-theme-color="system"
				/>
			</div>
		</section>

		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Visual & Window Effects</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Customize rendering performance and window styling.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="advanced-rendering"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.advancedRenderingTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.advancedRenderingDescription) }}
						</p>
					</div>

					<Toggle
						id="advanced-rendering"
						:model-value="themeStore.advancedRendering"
						@update:model-value="
							(e) => {
								themeStore.advancedRendering = !!e
								settings.advanced_rendering = themeStore.advancedRendering
							}
						"
					/>
				</div>

				<template v-if="os !== 'MacOS'">
					<div class="h-px bg-surface-4/60 -mx-5" />

					<div class="grid grid-cols-[1fr_auto] items-center gap-6">
						<div class="flex flex-col gap-0.5">
							<label
								for="native-decorations"
								class="text-sm font-semibold text-contrast cursor-pointer"
							>
								{{ formatMessage(messages.nativeDecorationsTitle) }}
							</label>
							<p class="m-0 text-xs text-secondary leading-relaxed">
								{{ formatMessage(messages.nativeDecorationsDescription) }}
							</p>
						</div>
						<Toggle id="native-decorations" v-model="settings.native_decorations" />
					</div>
				</template>
			</div>
		</section>
	</div>
</template>
