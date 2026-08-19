<template>
	<NewModal
		ref="modal"
		header="Play as Offline Player"
		max-width="480px"
	>
		<div class="flex flex-col gap-4 p-2">
			<div class="flex items-center gap-4 bg-surface-2 p-3 rounded-xl border border-surface-5">
				<div class="w-14 h-14 rounded-lg bg-surface-4 flex items-center justify-center overflow-hidden shrink-0 border border-white/10">
					<img
						:src="`https://mc-heads.net/avatar/${cleanUsername || 'Steve'}/64`"
						alt="Player Skin"
						class="w-12 h-12 rounded"
					/>
				</div>
				<div class="flex flex-col">
					<span class="font-semibold text-contrast text-base">{{ cleanUsername || 'Steve' }}</span>
					<span class="text-xs text-secondary">Free Offline Profile (No Purchase Required)</span>
				</div>
			</div>

			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-contrast">Player Nickname</label>
				<input
					v-model="username"
					type="text"
					placeholder="Enter nickname (e.g. ShadowCrafter)"
					maxlength="16"
					class="w-full px-3.5 py-2.5 bg-surface-2 border border-surface-5 focus:border-brand rounded-xl text-contrast placeholder-secondary outline-none transition-colors"
					@keydown.enter="submit"
				/>
				<span class="text-xs text-secondary">
					Letters, numbers, and underscores only. Max 16 characters.
				</span>
			</div>

			<div v-if="errorMessage" class="p-2.5 rounded-lg bg-red-950/40 border border-red-800/40 text-red-300 text-xs">
				{{ errorMessage }}
			</div>

			<div class="flex justify-end gap-2 mt-2">
				<Button type="quiet" @click="modal?.hide()">
					Cancel
				</Button>
				<Button
					type="colored"
					color="brand"
					:disabled="!isValid || isSubmitting"
					@click="submit"
				>
					<SpinnerIcon v-if="isSubmitting" class="animate-spin" />
					<CheckIcon v-else />
					Play as {{ cleanUsername || 'Player' }}
				</Button>
			</div>
		</div>
	</NewModal>
</template>

<script setup lang="ts">
import { CheckIcon, SpinnerIcon } from '@modrinth/assets'
import { Button, NewModal } from '@modrinth/ui'
import { computed, ref, useTemplateRef } from 'vue'
import { create_offline_account, set_default_user } from '@/helpers/auth'

const emit = defineEmits<{
	created: [account: any]
}>()

const modal = useTemplateRef('modal')
const username = ref('')
const isSubmitting = ref(false)
const errorMessage = ref('')

const cleanUsername = computed(() => username.value.trim().replace(/[^a-zA-Z0-9_]/g, ''))
const isValid = computed(() => cleanUsername.value.length >= 1 && cleanUsername.value.length <= 16)

async function submit() {
	if (!isValid.value || isSubmitting.value) return

	isSubmitting.value = true
	errorMessage.value = ''

	try {
		const account = await create_offline_account(cleanUsername.value)
		if (account?.profile?.id) {
			await set_default_user(account.profile.id)
		}
		emit('created', account)
		modal.value?.hide()
		username.value = ''
	} catch (err: any) {
		errorMessage.value = err?.message || 'Failed to create offline profile'
	} finally {
		isSubmitting.value = false
	}
}

defineExpose({
	show: () => {
		username.value = ''
		errorMessage.value = ''
		modal.value?.show()
	},
	hide: () => modal.value?.hide(),
})
</script>
