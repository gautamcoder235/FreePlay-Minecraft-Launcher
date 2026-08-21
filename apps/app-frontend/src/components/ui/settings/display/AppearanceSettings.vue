<script setup lang="ts">
import { CheckIcon, SparklesIcon } from '@freeplay/assets'
import { defineMessages, ThemeSelector, Toggle, useVIntl } from '@freeplay/ui'
import { computed, ref, watch } from 'vue'

import { get, set } from '@/helpers/settings.ts'
import { getOS } from '@/helpers/utils'
import { useTheming } from '@/store/state'
import type { AccentColor, ColorTheme } from '@/store/theme.ts'
import { ACCENT_OPTIONS, ACCENT_PRESETS } from '@/store/theme.ts'

const themeStore = useTheming()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	colorThemeTitle: {
		id: 'app.appearance-settings.color-theme.title',
		defaultMessage: 'Color theme',
	},
	colorThemeDescription: {
		id: 'app.appearance-settings.color-theme.description',
		defaultMessage: 'Choose the base color theme used by FreePlay Launcher.',
	},
	accentThemeTitle: {
		id: 'app.appearance-settings.accent-theme.title',
		defaultMessage: 'Accent color palette',
	},
	accentThemeDescription: {
		id: 'app.appearance-settings.accent-theme.description',
		defaultMessage: 'Personalize highlights, launch buttons, glowing auras, and active indicators.',
	},
	highContrastTitle: {
		id: 'app.appearance-settings.high-contrast.title',
		defaultMessage: 'High-contrast borders',
	},
	highContrastDescription: {
		id: 'app.appearance-settings.high-contrast.description',
		defaultMessage:
			'Sharpen 1px border visibility across cards, modal backdrops, and navigation panels for maximum clarity.',
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

const accentOptions = ACCENT_OPTIONS

function handleSelectAccent(accent: AccentColor) {
	themeStore.setAccentState(accent)
}

function handleToggleHighContrast(val: boolean) {
	themeStore.setHighContrastBorders(val)
	themeStore.setFeatureFlag('high_contrast_borders', val)
	settings.value.feature_flags = {
		...settings.value.feature_flags,
		high_contrast_borders: val,
	}
}

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)
</script>

<template>
	<div class="flex flex-col gap-8 pb-10">
		<!-- ========================================== -->
		<!-- LIVE THEME & ACCENT PREVIEW STAGE          -->
		<!-- ========================================== -->
		<section class="flex flex-col gap-3">
			<div class="flex items-center justify-between">
				<div class="flex flex-col">
					<h2 class="m-0 text-base font-extrabold text-contrast flex items-center gap-2">
						<SparklesIcon class="w-4 h-4 text-[var(--color-brand)]" />
						Live Theme Preview
					</h2>
					<p class="m-0 text-xs text-secondary mt-0.5">
						Real-time preview of your selected theme, accent palette, and border styling.
					</p>
				</div>
				<span
					class="text-[10px] font-extrabold uppercase tracking-wider px-2.5 py-1 rounded-full border border-white/10 bg-[var(--surface-3)] text-contrast"
				>
					{{ themeStore.resolvedTheme }} • {{ themeStore.currentAccentPreset.name }}
				</span>
			</div>

			<!-- Interactive Preview Card Container -->
			<div
				class="relative overflow-hidden rounded-3xl bg-[var(--surface-2)] border border-[var(--border-default)] p-6 shadow-2xl backdrop-blur-xl transition-all duration-300"
			>
				<!-- Subtle Ambient Glow in selected accent -->
				<div
					class="absolute -right-16 -top-16 w-64 h-64 rounded-full blur-3xl pointer-events-none opacity-20 transition-all duration-500"
					:style="{ background: themeStore.currentAccentPreset.color }"
				/>
				<div
					class="absolute -left-16 -bottom-16 w-64 h-64 rounded-full blur-3xl pointer-events-none opacity-10 transition-all duration-500"
					:style="{ background: themeStore.currentAccentPreset.color }"
				/>

				<div class="relative z-10 flex flex-col gap-5">
					<!-- Top Bar Simulation -->
					<div
						class="flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-[var(--border-subtle)]"
					>
						<div class="flex items-center gap-3">
							<!-- Themed Diamond Logo -->
							<div
								class="relative flex items-center justify-center w-9 h-9 rounded-xl shadow-lg border border-white/20 transition-all duration-300"
								:style="{
									background: themeStore.currentAccentPreset.gradient,
									boxShadow: themeStore.currentAccentPreset.glow,
								}"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="none"
									class="w-4 h-4 text-white"
								>
									<path d="M12 2L2 9L12 22L22 9L12 2Z" fill="currentColor" fill-opacity="0.95" />
									<path d="M12 2L2 9H22L12 2Z" fill="white" fill-opacity="0.4" />
								</svg>
							</div>
							<div class="flex flex-col">
								<span
									class="text-sm font-extrabold text-white tracking-wide flex items-center gap-2"
								>
									FREEPLAY
									<span
										class="text-[9px] font-black px-1.5 py-0.5 rounded text-white uppercase tracking-widest"
										:style="{ background: themeStore.currentAccentPreset.gradient }"
									>
										{{ themeStore.currentAccentPreset.name }}
									</span>
								</span>
								<span class="text-[11px] text-zinc-400">Customized Game Launcher Experience</span>
							</div>
						</div>

						<!-- Action Badges -->
						<div class="flex items-center gap-2">
							<span
								class="text-xs font-bold px-3 py-1 rounded-xl border border-[var(--border-default)] bg-[var(--surface-3)] text-contrast"
							>
								OLED: {{ themeStore.isOled ? 'Active (Pitch Black)' : 'Standard' }}
							</span>
							<span
								class="text-xs font-bold px-3 py-1 rounded-xl text-white border border-white/10 shadow-sm"
								:style="{ background: themeStore.currentAccentPreset.gradient }"
							>
								Active Preset
							</span>
						</div>
					</div>

					<!-- Middle Interactive Component Preview Elements -->
					<div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
						<!-- Primary Launch Button Sample -->
						<button
							type="button"
							class="h-12 px-5 rounded-2xl text-white font-extrabold text-sm flex items-center justify-center gap-2 shadow-xl hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 cursor-pointer border border-white/20"
							:style="{
								background: themeStore.currentAccentPreset.gradient,
								boxShadow: themeStore.currentAccentPreset.glow,
							}"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4 fill-current"
								viewBox="0 0 24 24"
							>
								<polygon points="5 3 19 12 5 21 5 3" />
							</svg>
							PLAY INSTANCE
						</button>

						<!-- Secondary Pill / Tab Sample -->
						<div
							class="h-12 px-4 rounded-2xl bg-[var(--surface-3)] border border-[var(--border-default)] flex items-center justify-between text-xs font-semibold text-contrast"
						>
							<span class="flex items-center gap-2">
								<span
									class="w-2.5 h-2.5 rounded-full animate-pulse"
									:style="{ background: themeStore.currentAccentPreset.color }"
								/>
								Accent Glow Engine
							</span>
							<span
								class="text-[10px] font-bold px-2 py-0.5 rounded-lg border border-[var(--border-subtle)] bg-[var(--surface-4)] text-zinc-300"
							>
								2.5 ms
							</span>
						</div>

						<!-- Progress Bar Sample -->
						<div class="flex flex-col gap-1.5 justify-center">
							<div class="flex items-center justify-between text-[11px] font-bold text-zinc-300">
								<span>Asset Download Pipeline</span>
								<span :style="{ color: themeStore.currentAccentPreset.highlight }">100% Ready</span>
							</div>
							<div
								class="h-2 w-full rounded-full bg-[var(--surface-4)] overflow-hidden p-0.5 border border-[var(--border-subtle)]"
							>
								<div
									class="h-full rounded-full transition-all duration-500"
									:style="{
										width: '100%',
										background: themeStore.currentAccentPreset.gradient,
										boxShadow: themeStore.currentAccentPreset.glow,
									}"
								/>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<!-- ========================================== -->
		<!-- BASE THEME MODE SELECTOR                   -->
		<!-- ========================================== -->
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(messages.colorThemeTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					{{ formatMessage(messages.colorThemeDescription) }}
				</p>
			</div>

			<div
				class="rounded-2xl bg-[var(--surface-2)] border border-[var(--border-default)] p-5 shadow-sm"
			>
				<ThemeSelector
					:update-color-theme="
						(theme: ColorTheme) => {
							themeStore.setThemeState(theme)
							settings.theme = theme
						}
					"
					:current-theme="settings.theme"
					:theme-options="themeOptions"
					:system-theme-color="themeStore.resolvedTheme"
				/>
			</div>
		</section>

		<!-- ========================================== -->
		<!-- ACCENT COLOR PALETTE SELECTOR (8 PRESETS)  -->
		<!-- ========================================== -->
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(messages.accentThemeTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					{{ formatMessage(messages.accentThemeDescription) }}
				</p>
			</div>

			<div
				class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3.5 p-5 rounded-2xl bg-[var(--surface-2)] border border-[var(--border-default)] shadow-sm"
			>
				<button
					v-for="accent in accentOptions"
					:key="accent"
					type="button"
					class="relative flex items-center gap-3.5 p-3.5 rounded-2xl border transition-all duration-200 text-left cursor-pointer group hover:scale-[1.02] active:scale-[0.98]"
					:class="[
						themeStore.selectedAccent === accent
							? 'bg-[var(--surface-3)] border-white/30 shadow-lg'
							: 'bg-[var(--surface-1-5)] border-[var(--border-subtle)] hover:border-white/20 hover:bg-[var(--surface-3)]/60',
					]"
					:style="
						themeStore.selectedAccent === accent
							? {
									boxShadow: ACCENT_PRESETS[accent].glow,
									borderColor: ACCENT_PRESETS[accent].highlight,
								}
							: {}
					"
					@click="handleSelectAccent(accent)"
				>
					<!-- Color Swatch Circle with Gradient -->
					<div
						class="relative flex items-center justify-center w-10 h-10 rounded-xl shadow-md border border-white/20 shrink-0 transition-transform duration-300 group-hover:scale-105"
						:style="{
							background: ACCENT_PRESETS[accent].gradient,
						}"
					>
						<CheckIcon
							v-if="themeStore.selectedAccent === accent"
							class="w-5 h-5 text-white drop-shadow-md"
						/>
					</div>

					<!-- Swatch Name & Flavor Description -->
					<div class="flex flex-col min-w-0 flex-1">
						<span
							class="text-xs font-extrabold text-contrast truncate group-hover:text-white transition-colors"
						>
							{{ ACCENT_PRESETS[accent].name }}
						</span>
						<span class="text-[10px] text-secondary truncate mt-0.5">
							{{ ACCENT_PRESETS[accent].flavor }}
						</span>
					</div>

					<!-- Active Indicator Dot -->
					<div
						v-if="themeStore.selectedAccent === accent"
						class="w-2 h-2 rounded-full shrink-0"
						:style="{ background: ACCENT_PRESETS[accent].color }"
					/>
				</button>
			</div>
		</section>

		<!-- ========================================== -->
		<!-- VISUAL & WINDOW EFFECTS                    -->
		<!-- ========================================== -->
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Visual & Window Effects</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Customize rendering performance, border sharpness, and window styling.
				</p>
			</div>

			<div
				class="rounded-2xl bg-[var(--surface-2)] border border-[var(--border-default)] p-5 shadow-sm flex flex-col gap-5"
			>
				<!-- High Contrast Borders Toggle -->
				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="high-contrast-borders"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.highContrastTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.highContrastDescription) }}
						</p>
					</div>

					<Toggle
						id="high-contrast-borders"
						:model-value="themeStore.highContrastBorders"
						@update:model-value="handleToggleHighContrast(!!$event)"
					/>
				</div>

				<div class="h-px bg-[var(--border-subtle)] -mx-5" />

				<!-- Advanced Rendering Toggle -->
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
								themeStore.setAdvancedRendering(!!e)
								settings.advanced_rendering = !!e
							}
						"
					/>
				</div>

				<template v-if="os !== 'MacOS'">
					<div class="h-px bg-[var(--border-subtle)] -mx-5" />

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
