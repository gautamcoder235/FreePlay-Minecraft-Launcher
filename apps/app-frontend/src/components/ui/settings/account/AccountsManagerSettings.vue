<script setup lang="ts">
import { UserIcon } from '@freeplay/assets'
import { defineMessages, useVIntl } from '@freeplay/ui'
import { ref } from 'vue'

import AccountsCard from '@/components/ui/AccountsCard.vue'

const { formatMessage } = useVIntl()
const accountsCardRef = ref<InstanceType<typeof AccountsCard> | null>(null)

const messages = defineMessages({
	minecraftAccountsTitle: {
		id: 'app.settings.accounts.minecraft-title',
		defaultMessage: 'Minecraft & Offline Player Profiles',
	},
	minecraftAccountsDesc: {
		id: 'app.settings.accounts.minecraft-desc',
		defaultMessage: 'Manage your Microsoft authentication and custom 1-click offline nicknames.',
	},
})
</script>

<template>
	<div class="flex flex-col gap-6 max-w-2xl">
		<!-- Section: Minecraft & Offline Player Profiles -->
		<div class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h3 class="text-base font-bold text-contrast m-0 flex items-center gap-2">
					<UserIcon class="w-4 h-4 text-brand" />
					{{ formatMessage(messages.minecraftAccountsTitle) }}
				</h3>
				<p class="text-xs text-secondary m-0 mt-1">
					{{ formatMessage(messages.minecraftAccountsDesc) }}
				</p>
			</div>

			<div class="rounded-2xl bg-surface-2 border border-surface-4 p-4 shadow-sm">
				<AccountsCard ref="accountsCardRef" />
			</div>
		</div>
	</div>
</template>
