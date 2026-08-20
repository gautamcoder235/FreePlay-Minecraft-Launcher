<template>
	<NewModal ref="modal" header="Play as Offline Player" max-width="480px">
		<div
			class="flex flex-col gap-4 p-2 bg-[#141923] rounded-2xl border border-white/10 select-none"
		>
			<div
				class="flex items-center gap-4 bg-[#0e131d] p-3.5 rounded-2xl border border-white/10 shadow-lg"
			>
				<div
					class="w-14 h-14 rounded-xl bg-zinc-800 flex items-center justify-center overflow-hidden shrink-0 border border-white/20 shadow-inner"
				>
					<img
						:src="`https://mc-heads.net/avatar/${cleanUsername || 'Steve'}/64`"
						alt="Player Skin"
						class="w-12 h-12 rounded-lg object-cover shadow"
					/>
				</div>
				<div class="flex flex-col">
					<span class="font-bold text-white text-base">{{ cleanUsername || 'Steve' }}</span>
					<span class="text-xs text-sky-300 font-medium"
						>Free 1-Click Offline Profile (No Purchase Required)</span
					>
				</div>
			</div>

			<div class="flex flex-col gap-2">
				<label class="text-sm font-bold text-zinc-200">Player Nickname</label>
				<input
					v-model="username"
					type="text"
					placeholder="Enter nickname (e.g. ShadowCrafter)"
					maxlength="16"
					class="w-full px-4 py-2.5 bg-zinc-900 border border-white/10 focus:border-sky-500 focus:ring-2 focus:ring-sky-500/50 rounded-xl text-white placeholder-zinc-500 outline-none transition-all duration-200 text-sm font-medium"
					@keydown.enter="submit"
				/>
				<span class="text-xs text-zinc-400">
					Letters, numbers, and underscores only. Max 16 characters.
				</span>
			</div>

			<div
				v-if="errorMessage"
				class="p-3 rounded-xl bg-red-950/60 border border-red-500/30 text-red-300 text-xs font-semibold"
			>
				{{ errorMessage }}
			</div>

			<div class="flex justify-end gap-3 mt-2">
				<Button
					type="quiet"
					class="!px-4 cursor-pointer hover:!bg-white/10 transition-colors"
					@click="modal?.hide()"
				>
					Cancel
				</Button>
				<Button
					type="colored"
					color="brand"
					class="!bg-sky-600 hover:!bg-sky-500 !font-bold cursor-pointer transition-all duration-200 shadow-lg shadow-sky-950/60 !px-5"
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
import { CheckIcon, SpinnerIcon } from '@freeplay/assets'
import { Button, NewModal } from '@freeplay/ui'
import { computed, ref, useTemplateRef } from 'vue'

import { useAccountStore } from '@/store/account.ts'

const emit = defineEmits<{
	created: [account: Record<string, unknown>]
}>()

const accountStore = useAccountStore()
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

	const cleanName = cleanUsername.value

	try {
		const account = await accountStore.createOfflineAccount(cleanName)
		emit('created', account as unknown as Record<string, unknown>)
		modal.value?.hide()
		username.value = ''
	} catch (err: unknown) {
		const message =
			typeof err === 'string'
				? err
				: err instanceof Error
					? err.message
					: (err as { message?: string })?.message
		errorMessage.value = message || 'Failed to create offline profile'
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
