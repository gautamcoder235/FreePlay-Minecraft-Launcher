<script setup lang="ts">
import { defineMessages, Toggle, useVIntl } from '@freeplay/ui'
import { ref, watch } from 'vue'

import { get, set } from '@/helpers/settings.ts'

const { formatMessage } = useVIntl()
const settings = ref(await get())

const messages = defineMessages({
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
				<h2 class="m-0 text-base font-bold text-contrast">Integrations</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Manage third-party app presence and local integration settings.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
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
