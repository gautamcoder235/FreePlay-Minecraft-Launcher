<script setup lang="ts">
import { LogInIcon, SparklesIcon, UserIcon } from '@freeplay/assets'
import { AccountProfileSettings, Button, defineMessages, injectAuth, useVIntl } from '@freeplay/ui'
import { inject, onBeforeUnmount, onMounted, ref } from 'vue'

import AccountsCard from '@/components/ui/AccountsCard.vue'
import {
	change_user_avatar,
	delete_user_avatar,
	get_user_profile,
	patch_user,
} from '@/helpers/users'
import { appSettingsModalContextKey } from '@/providers/app-settings-modal'

const { formatMessage } = useVIntl()
const settingsModal = inject(appSettingsModalContextKey, null)
const auth = injectAuth()
const profileSettings = ref<InstanceType<typeof AccountProfileSettings> | null>(null)
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
	cloudAccountTitle: {
		id: 'app.settings.accounts.cloud-title',
		defaultMessage: 'FreePlay Cloud Account',
	},
	cloudAccountDesc: {
		id: 'app.settings.accounts.cloud-desc',
		defaultMessage: 'Sync your cloud settings, modpacks, server bookmarks, and friends list.',
	},
})

onMounted(() => {
	settingsModal?.registerUnsavedChangesController({
		hasChanges: () => profileSettings.value?.hasChanges ?? false,
		getOriginal: () => profileSettings.value?.originalState ?? {},
		getModified: () => profileSettings.value?.modifiedState ?? {},
		isSaving: () => profileSettings.value?.saving ?? false,
		reset: () => profileSettings.value?.reset(),
		save: () => profileSettings.value?.save(),
	})
})

onBeforeUnmount(() => {
	settingsModal?.registerUnsavedChangesController(null)
})

function handleProfileLinkClick(event: MouseEvent): void {
	if (settingsModal && !settingsModal.close()) {
		event.preventDefault()
	}
}

function patchUser(userId: string, patch: Partial<Record<string, unknown>>): Promise<void> {
	return patch_user(userId, patch)
}

async function changeAvatar(userId: string, file: Blob, extension: string): Promise<void> {
	await change_user_avatar(userId, new Uint8Array(await file.arrayBuffer()), extension)
}

function deleteAvatar(userId: string): Promise<void> {
	return delete_user_avatar(userId)
}

function getAuthenticatedUser(): Promise<Record<string, unknown>> {
	const userId = auth.user.value?.id
	if (!userId) throw new Error('Cannot refresh a signed-out user.')
	return get_user_profile(userId)
}
</script>

<template>
	<div class="flex flex-col gap-6 max-w-2xl">
		<!-- Section 1: Minecraft & Offline Player Profiles -->
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

		<!-- Section 2: FreePlay Cloud Account -->
		<div class="flex flex-col gap-3 pt-4 border-t border-surface-4/60">
			<div class="flex flex-col">
				<h3 class="text-base font-bold text-contrast m-0 flex items-center gap-2">
					<SparklesIcon class="w-4 h-4 text-brand" />
					{{ formatMessage(messages.cloudAccountTitle) }}
				</h3>
				<p class="text-xs text-secondary m-0 mt-1">
					{{ formatMessage(messages.cloudAccountDesc) }}
				</p>
			</div>

			<div
				v-if="auth.user.value"
				class="rounded-2xl bg-surface-2 border border-surface-4 p-4 shadow-sm"
			>
				<AccountProfileSettings
					ref="profileSettings"
					:patch-user="patchUser"
					:change-avatar="changeAvatar"
					:delete-avatar="deleteAvatar"
					:get-authenticated-user="getAuthenticatedUser"
					@profile-link-click="handleProfileLinkClick"
				/>
			</div>

			<div
				v-else
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex items-center justify-between gap-4"
			>
				<div class="flex flex-col gap-1">
					<span class="text-sm font-bold text-contrast">Optional Cloud Synchronization</span>
					<span class="text-xs text-secondary"
						>Sign in with FreePlay to publish modpacks and join public community lobbies.</span
					>
				</div>
				<Button
					type="colored"
					color="brand"
					class="!font-bold cursor-pointer transition-all shadow-md shrink-0"
					@click="auth.signIn()"
				>
					<LogInIcon class="w-4 h-4" />
					Sign In
				</Button>
			</div>
		</div>
	</div>
</template>
