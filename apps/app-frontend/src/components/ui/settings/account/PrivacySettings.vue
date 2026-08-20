<script setup lang="ts">
import { Settings2Icon } from '@freeplay/assets'
import {
	Button,
	defineMessages,
	injectNotificationManager,
	injectPageContext,
	Toggle,
	useVIntl,
} from '@freeplay/ui'
import { ref, watch } from 'vue'

import { open_ads_consent_preferences } from '@/helpers/ads.js'
import { optInAnalytics, optOutAnalytics } from '@/helpers/analytics'
import { get, set } from '@/helpers/settings.ts'

const { formatMessage } = useVIntl()
const { handleError } = injectNotificationManager()
const { adConsentAvailable } = injectPageContext()
const settings = ref(await get())

const messages = defineMessages({
	adsConsentTitle: {
		id: 'app.ads-consent.title',
		defaultMessage: 'Your privacy and how ads support FreePlay',
	},
	adsConsentIntro: {
		id: 'app.settings.privacy.ads-consent.intro',
		defaultMessage:
			'Ads make FreePlay possible and fund creator payouts. Our partners may store or access cookies in the app to personalize ads and measure performance. You can opt out or manage your preferences below.',
	},
	adsConsentManage: {
		id: 'app.ads-consent.manage',
		defaultMessage: 'Manage preferences',
	},
	telemetryTitle: {
		id: 'app.settings.privacy.telemetry.title',
		defaultMessage: 'Telemetry',
	},
	telemetryDescription: {
		id: 'app.settings.privacy.telemetry.description',
		defaultMessage:
			'FreePlay collects anonymized analytics and usage data to improve our user experience and customize your experience. By disabling this option, you opt out and your data will no longer be collected.',
	},
	discordRichPresenceTitle: {
		id: 'app.settings.privacy.discord-rich-presence.title',
		defaultMessage: 'Discord Rich Presence',
	},
	discordRichPresenceDescription: {
		id: 'app.settings.privacy.discord-rich-presence.description',
		defaultMessage:
			'Show FreePlay Launcher as your current activity on Discord. This does not affect Rich Presence added to instances by mods. Requires an app restart.',
	},
})

async function manageAdsPreferences() {
	await open_ads_consent_preferences().catch(handleError)
}

watch(
	settings,
	async () => {
		if (settings.value.telemetry) {
			optInAnalytics()
		} else {
			optOutAnalytics()
		}

		await set(settings.value)
	},
	{ deep: true },
)
</script>

<template>
	<div class="flex flex-col gap-6">
		<section v-if="adConsentAvailable" class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(messages.adsConsentTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					{{ formatMessage(messages.adsConsentIntro) }}
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex items-center justify-between gap-4"
			>
				<div class="flex flex-col gap-0.5">
					<span class="text-sm font-semibold text-contrast">Ad Personalization & Consent</span>
					<p class="m-0 text-xs text-secondary">
						Review your advertising partners and cookie tracking consent options.
					</p>
				</div>
				<Button type="colored" color="brand" @click="manageAdsPreferences">
					<Settings2Icon aria-hidden="true" />
					{{ formatMessage(messages.adsConsentManage) }}
				</Button>
			</div>
		</section>

		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Privacy & Integrations</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Manage data telemetry and third-party app presence.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="opt-out-analytics"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.telemetryTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.telemetryDescription) }}
						</p>
					</div>
					<Toggle id="opt-out-analytics" v-model="settings.telemetry" />
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="disable-discord-rpc"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.discordRichPresenceTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.discordRichPresenceDescription) }}
						</p>
					</div>
					<Toggle id="disable-discord-rpc" v-model="settings.discord_rpc" />
				</div>
			</div>
		</section>
	</div>
</template>
