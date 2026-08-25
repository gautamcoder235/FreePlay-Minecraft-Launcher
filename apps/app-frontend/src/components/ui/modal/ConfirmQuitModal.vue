<template>
	<NewModal
		ref="modal"
		:header="'Quit FreePlay Launcher'"
		fade="danger"
		max-width="480px"
		@keydown.esc="hide"
	>
		<div class="flex flex-col gap-4 select-none p-1">
			<div
				class="flex items-center gap-4 bg-[var(--surface-2,#141923)] p-4 rounded-2xl border border-white/10 shadow-lg"
			>
				<div
					class="w-12 h-12 rounded-xl bg-red-500/10 border border-red-500/30 flex items-center justify-center shrink-0 text-red-400 shadow-inner"
				>
					<LogOutIcon class="w-6 h-6" />
				</div>
				<div class="flex flex-col gap-1 min-w-0">
					<span class="font-bold text-white text-base">Are you sure you want to quit?</span>
					<span class="text-xs text-zinc-400 leading-relaxed">
						Any active game sessions, downloads, or background tasks will be closed.
					</span>
				</div>
			</div>
		</div>

		<template #actions>
			<div class="flex gap-3 justify-end items-center mt-2">
				<Button
					type="quiet"
					class="!px-4 cursor-pointer hover:!bg-white/10 transition-colors text-zinc-300 hover:text-white"
					@click="hide"
				>
					<XIcon class="w-4 h-4 mr-1.5" />
					Cancel
				</Button>
				<Button
					type="colored"
					color="red"
					class="!bg-red-600 hover:!bg-red-500 !font-bold cursor-pointer transition-all duration-200 shadow-lg shadow-red-950/60 !px-5"
					:loading="isQuitting"
					@click="confirmQuit"
				>
					<LogOutIcon class="w-4 h-4 mr-1.5" />
					Quit FreePlay
				</Button>
			</div>
		</template>
	</NewModal>
</template>

<script setup lang="ts">
import { LogOutIcon, XIcon } from '@freeplay/assets'
import { Button, NewModal } from '@freeplay/ui'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state'
import { ref } from 'vue'

const modal = ref<InstanceType<typeof NewModal> | null>(null)
const isQuitting = ref(false)

function show() {
	modal.value?.show()
}

function hide() {
	if (isQuitting.value) return
	modal.value?.hide()
}

async function confirmQuit() {
	isQuitting.value = true
	try {
		await saveWindowState(StateFlags.ALL).catch(() => {})
	} catch {
		// ignore
	}
	try {
		await invoke('exit_app')
	} catch {
		try {
			await getCurrentWindow().destroy()
		} catch {
			await getCurrentWindow().close().catch(() => {})
		}
	}
}

defineExpose({
	show,
	hide,
})
</script>
