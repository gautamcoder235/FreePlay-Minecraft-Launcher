<script setup lang="ts">
import { BanIcon, SpinnerIcon, TagCategoryWandSparklesIcon, XIcon } from '@freeplay/assets'
import {
	defineMessages,
	IconButton,
	injectNotificationManager,
	NewModal,
	useVIntl,
} from '@freeplay/ui'
import { nextTick, ref, useTemplateRef } from 'vue'

import IconEditorModal from '@/components/ui/instance_settings/icon-editor-modal/index.vue'
import { toError } from '@/helpers/errors'
import { list as listInstances } from '@/helpers/instance'

import icon01 from './assets/01.png'
import icon02 from './assets/02.png'
import icon03 from './assets/03.png'
import icon04 from './assets/04.png'
import icon05 from './assets/05.png'
import icon06 from './assets/06.png'
import icon07 from './assets/07.png'
import icon08 from './assets/08.png'
import icon09 from './assets/09.png'

const emit = defineEmits<{
	close: []
}>()

const modal = useTemplateRef<InstanceType<typeof NewModal>>('modal')
const iconEditorModal = useTemplateRef<InstanceType<typeof IconEditorModal>>('iconEditorModal')
const { formatMessage } = useVIntl()
const { addNotification, handleError } = injectNotificationManager()
const iconlessInstanceIds = ref<string[]>([])
const loading = ref(false)
const applying = ref(false)
const icons = [icon01, icon02, icon03, icon04, icon05, icon06, icon07, icon08, icon09]

const messages = defineMessages({
	badge: {
		id: 'app.icon-editor.apply-icons-modal.badge',
		defaultMessage: 'New this update',
	},
	title: {
		id: 'app.icon-editor.apply-icons-modal.title',
		defaultMessage: 'Custom icon editor',
	},
	description: {
		id: 'app.icon-editor.apply-icons-modal.description',
		defaultMessage:
			'Create custom icons for your instances right in the FreePlay Launcher. Mix and match backgrounds with symbols from Minecraft and popular mods!',
	},
	instancesWithoutIcons: {
		id: 'app.icon-editor.apply-icons-modal.instances-without-icons',
		defaultMessage:
			'{count, plural, one {# instance doesn’t} other {# instances don’t}} have icons yet. Want us to give {count, plural, one {it} other {them}} random ones to get started?',
	},
	skip: {
		id: 'app.icon-editor.apply-icons-modal.skip',
		defaultMessage: 'Skip',
	},
	randomIcons: {
		id: 'app.icon-editor.apply-icons-modal.random-icons',
		defaultMessage: 'Random icons',
	},
	close: {
		id: 'app.icon-editor.apply-icons-modal.close',
		defaultMessage: 'Close',
	},
	applyError: {
		id: 'app.icon-editor.apply-icons-modal.apply-error',
		defaultMessage: 'Failed to apply random icons to instances.',
	},
	applySuccess: {
		id: 'app.icon-editor.apply-icons-modal.apply-success',
		defaultMessage: 'Randomized icon for {num, plural, one {# instance} other {# instances}}.',
	},
})

async function show() {
	modal.value?.show()
	loading.value = true
	try {
		iconlessInstanceIds.value = (await listInstances())
			.filter((instance) => !instance.icon_path)
			.map((instance) => instance.id)
	} catch (error) {
		handleError(toError(error))
		iconlessInstanceIds.value = []
	} finally {
		loading.value = false
	}
}

function hide() {
	modal.value?.hide()
}

function handleHide() {
	emit('close')
}

async function applyIcons() {
	if (applying.value || loading.value) return

	applying.value = true
	const instanceCount = iconlessInstanceIds.value.length
	let applied = false
	try {
		const results = await Promise.all(
			iconlessInstanceIds.value.map(async (instanceId) => {
				const generated = await iconEditorModal.value?.randomize()
				if (!generated) return false

				return (
					(await iconEditorModal.value?.applyGeneratedIcon(instanceId, generated.config)) ?? false
				)
			}),
		)
		applied = results.every(Boolean)
	} finally {
		applying.value = false
	}

	if (applied) {
		await nextTick()
		hide()
		addNotification({
			type: 'success',
			title: formatMessage(messages.applySuccess, { num: instanceCount }),
		})
	} else {
		addNotification({
			type: 'error',
			title: formatMessage(messages.applyError),
		})
	}
}

defineExpose({ show, hide })
</script>

<template>
	<NewModal
		ref="modal"
		hide-header
		no-padding
		width="770px"
		max-width="calc(100vw - 2rem)"
		:aria-label="formatMessage(messages.title)"
		:on-after-hide="handleHide"
		:disable-close="applying"
		class="!overflow-hidden !rounded-3xl !border !border-[var(--border-default)] !bg-[var(--surface-1)]/95 !backdrop-blur-3xl shadow-[0_25px_60px_rgba(0,0,0,0.8),0_0_40px_rgba(6,182,212,0.15)]"
	>
		<div class="grid h-[400px] w-[768px] max-w-full grid-cols-2">
			<section class="flex min-w-0 flex-col gap-6 bg-[var(--surface-2)]/95 p-8 select-none">
				<div
					class="flex h-7 w-fit items-center rounded-full border border-cyan-500/30 bg-cyan-500/10 px-3 text-xs font-black uppercase tracking-wider text-cyan-300 font-mono shadow-inner"
				>
					{{ formatMessage(messages.badge) }}
				</div>

				<div class="flex min-w-0 flex-col gap-3">
					<h2 class="m-0 text-2xl font-black tracking-tight text-white">
						{{ formatMessage(messages.title) }}
					</h2>
					<p class="m-0 text-xs text-zinc-400 leading-relaxed">
						{{ formatMessage(messages.description) }}
					</p>
					<p
						v-if="iconlessInstanceIds.length"
						class="m-0 text-xs font-semibold text-sky-400 leading-relaxed"
					>
						{{
							formatMessage(messages.instancesWithoutIcons, {
								count: iconlessInstanceIds.length,
							})
						}}
					</p>
				</div>

				<div class="flex mt-auto items-center gap-3">
					<button
						type="button"
						class="px-4 py-2.5 rounded-xl bg-white/5 hover:bg-white/10 text-zinc-300 hover:text-white border border-[var(--border-subtle)] text-xs font-bold transition-all cursor-pointer active:scale-95 flex items-center gap-1.5"
						:disabled="applying"
						@click="hide"
					>
						<template v-if="iconlessInstanceIds.length === 0">
							{{ formatMessage(messages.close) }}
						</template>
						<template v-else>
							<BanIcon class="w-3.5 h-3.5" />
							{{ formatMessage(messages.skip) }}
						</template>
					</button>
					<button
						v-if="iconlessInstanceIds.length !== 0"
						type="button"
						class="px-5 py-2.5 rounded-xl btn-accent-primary font-black text-xs transition-all duration-200 cursor-pointer active:scale-95 border-none flex items-center gap-2"
						:disabled="loading || applying"
						@click="applyIcons"
					>
						<SpinnerIcon v-if="applying" class="animate-spin w-4 h-4" />
						<TagCategoryWandSparklesIcon v-else class="w-4 h-4" />
						{{ formatMessage(messages.randomIcons) }}
					</button>
				</div>
			</section>

			<section
				class="relative flex min-w-0 items-center justify-center border-0 border-l border-[var(--border-subtle)] bg-[var(--surface-1-5)]/90 p-8 select-none"
			>
				<IconButton
					type="quiet"
					size="sm"
					:label="formatMessage(messages.close)"
					class="!absolute right-4 top-4 z-10 !bg-white/5 hover:!bg-white/10 !text-zinc-400 hover:!text-white rounded-xl"
					:disabled="applying"
					@click="hide"
				>
					<XIcon />
				</IconButton>

				<div class="grid size-60 grid-cols-3 grid-rows-3 gap-3" aria-hidden="true">
					<div
						v-for="(icon, index) in icons"
						:key="index"
						class="min-h-0 min-w-0 rounded-2xl border border-white/10 overflow-hidden shadow-lg hover:scale-105 transition-transform duration-200"
					>
						<img :src="icon" class="w-full h-full object-cover" />
					</div>
				</div>
			</section>
		</div>
	</NewModal>
	<IconEditorModal ref="iconEditorModal" />
</template>
