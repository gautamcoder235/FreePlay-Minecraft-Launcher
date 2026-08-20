<script setup lang="ts">
import { defineMessages, Toggle, useVIntl } from '@freeplay/ui'
import { ref, watch } from 'vue'

import { get, set } from '@/helpers/settings.ts'
import { useTheming } from '@/store/state'
import type { FeatureFlag } from '@/store/theme.ts'

const themeStore = useTheming()
const { formatMessage } = useVIntl()

const worldsInHomeFlag: FeatureFlag = 'worlds_in_home'
const skipNonEssentialWarningsFlag: FeatureFlag = 'skip_non_essential_warnings'
const skipUnknownPackWarningFlag: FeatureFlag = 'skip_unknown_pack_warning'
const showPlayTimeFlag: FeatureFlag = 'show_instance_play_time'

const messages = defineMessages({
	startupAndNavigationTitle: {
		id: 'app.behavior-settings.startup-and-navigation.title',
		defaultMessage: 'Startup and navigation',
	},
	contentTitle: {
		id: 'app.behavior-settings.content.title',
		defaultMessage: 'Home and content',
	},
	confirmationsTitle: {
		id: 'app.behavior-settings.confirmations.title',
		defaultMessage: 'Confirmations',
	},
	minimizeLauncherTitle: {
		id: 'app.appearance-settings.minimize-launcher.title',
		defaultMessage: 'Minimize app',
	},
	minimizeLauncherDescription: {
		id: 'app.appearance-settings.minimize-launcher.description',
		defaultMessage: 'Minimize FreePlay Launcher when Minecraft starts.',
	},
	defaultLandingPageHome: {
		id: 'app.appearance-settings.default-landing-page.home',
		defaultMessage: 'Home',
	},
	defaultLandingPageLibrary: {
		id: 'app.appearance-settings.default-landing-page.library',
		defaultMessage: 'Library',
	},
	toggleSidebarTitle: {
		id: 'app.appearance-settings.toggle-sidebar.title',
		defaultMessage: 'Hide right sidebar',
	},
	toggleSidebarDescription: {
		id: 'app.appearance-settings.toggle-sidebar.description',
		defaultMessage: 'Hide the right sidebar by default and add a button to show or hide it.',
	},
	jumpBackIntoWorldsTitle: {
		id: 'app.appearance-settings.jump-back-into-worlds.title',
		defaultMessage: 'Jump into worlds or instances',
	},
	jumpBackIntoWorldsDescription: {
		id: 'app.appearance-settings.jump-back-into-worlds.description',
		defaultMessage:
			'Show recently played worlds or instances in the "Jump in" section on the Home page.',
	},
	showPlayTimeTitle: {
		id: 'app.appearance-settings.show-play-time.title',
		defaultMessage: 'Show play time',
	},
	showPlayTimeDescription: {
		id: 'app.appearance-settings.show-play-time.description',
		defaultMessage: `Show how long you've played each instance.`,
	},
	hideNametagTitle: {
		id: 'app.appearance-settings.hide-nametag.title',
		defaultMessage: 'Hide nametag',
	},
	hideNametagDescription: {
		id: 'app.appearance-settings.hide-nametag.description',
		defaultMessage: 'Hide your username above the player preview on the Skin selector page.',
	},
	unknownPackWarningTitle: {
		id: 'app.appearance-settings.unknown-pack-warning.title',
		defaultMessage: 'Warn me before installing unknown modpacks',
	},
	unknownPackWarningDescription: {
		id: 'app.appearance-settings.unknown-pack-warning.description',
		defaultMessage:
			"Show a safety warning before installing a FreePlay Pack (.mrpack) that isn't hosted on FreePlay.",
	},
	skipNonEssentialWarningsTitle: {
		id: 'app.appearance-settings.skip-non-essential-warnings.title',
		defaultMessage: 'Skip non-essential warnings',
	},
	skipNonEssentialWarningsDescription: {
		id: 'app.appearance-settings.skip-non-essential-warnings.description',
		defaultMessage:
			'Skip confirmations for low-risk actions such as duplicate installs, normal content deletion, bulk updates, unlinking, and repairs. Warnings for dangerous actions are always shown.',
	},
})

const settings = ref(await get())

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
					{{ formatMessage(messages.startupAndNavigationTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Control launcher behavior when games start and navigating views.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="minimize-launcher"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.minimizeLauncherTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.minimizeLauncherDescription) }}
						</p>
					</div>
					<Toggle id="minimize-launcher" v-model="settings.hide_on_process_start" />
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label for="toggle-sidebar" class="text-sm font-semibold text-contrast cursor-pointer">
							{{ formatMessage(messages.toggleSidebarTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.toggleSidebarDescription) }}
						</p>
					</div>
					<Toggle
						id="toggle-sidebar"
						:model-value="settings.toggle_sidebar"
						@update:model-value="
							(e) => {
								settings.toggle_sidebar = !!e
								themeStore.toggleSidebar = settings.toggle_sidebar
							}
						"
					/>
				</div>
			</div>
		</section>

		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(messages.contentTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Manage home dashboard feed widgets and identity display.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="jump-back-into-worlds"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.jumpBackIntoWorldsTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.jumpBackIntoWorldsDescription) }}
						</p>
					</div>
					<Toggle
						id="jump-back-into-worlds"
						:model-value="themeStore.getFeatureFlag(worldsInHomeFlag)"
						@update:model-value="
							() => {
								const newValue = !themeStore.getFeatureFlag(worldsInHomeFlag)
								themeStore.featureFlags[worldsInHomeFlag] = newValue
								settings.feature_flags[worldsInHomeFlag] = newValue
							}
						"
					/>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label for="show-play-time" class="text-sm font-semibold text-contrast cursor-pointer">
							{{ formatMessage(messages.showPlayTimeTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.showPlayTimeDescription) }}
						</p>
					</div>
					<Toggle
						id="show-play-time"
						:model-value="themeStore.getFeatureFlag(showPlayTimeFlag)"
						@update:model-value="
							() => {
								const newValue = !themeStore.getFeatureFlag(showPlayTimeFlag)
								themeStore.featureFlags[showPlayTimeFlag] = newValue
								settings.feature_flags[showPlayTimeFlag] = newValue
							}
						"
					/>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="hide-nametag-skins-page"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.hideNametagTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.hideNametagDescription) }}
						</p>
					</div>
					<Toggle
						id="hide-nametag-skins-page"
						:model-value="themeStore.hideNametagSkinsPage"
						@update:model-value="
							(e) => {
								themeStore.hideNametagSkinsPage = !!e
								settings.hide_nametag_skins_page = themeStore.hideNametagSkinsPage
							}
						"
					/>
				</div>
			</div>
		</section>

		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(messages.confirmationsTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Configure warning dialogs and action confirmations.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="warn-before-installing-unknown-modpacks"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.unknownPackWarningTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.unknownPackWarningDescription) }}
						</p>
					</div>
					<Toggle
						id="warn-before-installing-unknown-modpacks"
						:model-value="!themeStore.getFeatureFlag(skipUnknownPackWarningFlag)"
						@update:model-value="
							(e) => {
								const warnBeforeUnknownPackInstall = !!e
								const skipUnknownPackWarning = !warnBeforeUnknownPackInstall
								themeStore.featureFlags[skipUnknownPackWarningFlag] = skipUnknownPackWarning
								settings.feature_flags[skipUnknownPackWarningFlag] = skipUnknownPackWarning
							}
						"
					/>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="skip-non-essential-warnings"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.skipNonEssentialWarningsTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.skipNonEssentialWarningsDescription) }}
						</p>
					</div>
					<Toggle
						id="skip-non-essential-warnings"
						:model-value="themeStore.getFeatureFlag(skipNonEssentialWarningsFlag)"
						@update:model-value="
							() => {
								const newValue = !themeStore.getFeatureFlag(skipNonEssentialWarningsFlag)
								themeStore.featureFlags[skipNonEssentialWarningsFlag] = newValue
								settings.feature_flags[skipNonEssentialWarningsFlag] = newValue
							}
						"
					/>
				</div>
			</div>
		</section>
	</div>
</template>
