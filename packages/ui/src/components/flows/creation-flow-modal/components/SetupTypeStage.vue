<template>
	<div class="flex flex-col gap-4 select-none">
		<!-- Section 1: Search Spotlight Header -->
		<div class="flex flex-col gap-2.5">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2.5">
					<div
						class="p-1.5 rounded-xl bg-gradient-to-br from-indigo-500/20 to-purple-500/20 text-indigo-400 border border-indigo-500/30 shadow-sm flex items-center justify-center"
					>
						<SparklesIcon class="w-3.5 h-3.5" />
					</div>
					<label class="text-sm font-bold text-white tracking-tight">
						{{ formatMessage(messages.knownProjectPrompt) }}
					</label>
				</div>
				<div
					class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-gradient-to-r from-indigo-500/15 via-purple-500/15 to-pink-500/10 text-indigo-300 border border-indigo-500/30 text-[11px] font-semibold tracking-wide shadow-sm"
				>
					<span class="w-1.5 h-1.5 rounded-full bg-indigo-400 animate-pulse"></span>
					<span>Quick Search</span>
				</div>
			</div>
			<div
				class="relative group rounded-2xl bg-[#0e131d]/90 border border-white/10 p-0.5 hover:border-white/20 focus-within:!border-indigo-500/60 focus-within:shadow-[0_0_25px_rgba(99,102,241,0.25)] transition-all duration-300"
			>
				<Combobox
					ref="projectSearchCombobox"
					v-model="ctx.projectSearchProjectId.value"
					v-tooltip="ctx.finishDisabled.value ? ctx.finishDisabledTooltip.value : undefined"
					:options="ctx.projectSearchOptions.value"
					searchable
					show-search-icon
					:show-chevron="false"
					:disabled="ctx.finishDisabled.value"
					:search-placeholder="formatMessage(messages.searchProjectPlaceholder)"
					:no-options-message="
						searchLoading
							? formatMessage(commonMessages.loadingLabel)
							: formatMessage(messages.noResultsFound)
					"
					:disable-search-filter="true"
					@search-input="handleSearch"
				>
					<template #option-suffix="{ item }">
						<div
							class="flex shrink-0 items-center gap-1.5 text-xs font-bold text-indigo-400 opacity-0 transition-opacity group-hover/option:opacity-100 group-data-[focused=true]/option:opacity-100"
						>
							<span>
								{{
									formatMessage(
										isModpackOption(item.value) ? messages.installModpack : messages.createInstance,
									)
								}}
							</span>
							<DownloadIcon v-if="isModpackOption(item.value)" class="size-4 shrink-0" />
							<RightArrowIcon v-else class="size-4 shrink-0" />
						</div>
					</template>
				</Combobox>
			</div>
		</div>

		<!-- Styled Divider -->
		<div class="relative flex items-center justify-center my-0.5">
			<div class="absolute inset-0 flex items-center">
				<div class="w-full border-t border-white/10"></div>
			</div>
			<div
				class="relative px-3.5 py-0.5 rounded-full bg-[#0e131d] border border-white/10 text-[10px] font-extrabold text-zinc-400 uppercase tracking-widest"
			>
				{{ formatMessage(commonMessages.orLabel) }} Choose Mode
			</div>
		</div>

		<div class="flex items-center justify-between">
			<span class="text-xs font-bold uppercase tracking-wider text-zinc-400">
				{{ setupTypeTitle }}
			</span>
		</div>

		<template v-if="ctx.flowType === 'instance'">
			<div class="flex flex-col gap-2.5">
				<BigOptionButton
					:icon="BoxesIcon"
					color-theme="sky"
					badge="CUSTOM"
					:title="formatMessage(messages.customSetupTitle)"
					:description="formatMessage(messages.customSetupDescription)"
					@click="setSetupType('custom')"
				/>
				<BigOptionButton
					:icon="CompassIcon"
					color-theme="cyan"
					badge="EXPLORE"
					:title="formatMessage(messages.modpackBaseTitle)"
					:description="formatMessage(messages.modpackBaseDescription)"
					@click="browseModpacks"
				/>
				<BigOptionButton
					:icon="UploadIcon"
					color-theme="indigo"
					badge="ARCHIVE"
					:title="formatMessage(messages.uploadModpackTitle)"
					:description="formatMessage(messages.uploadModpackDescription)"
					@click="triggerFileInput"
				/>
				<BigOptionButton
					:icon="BoxImportIcon"
					color-theme="amber"
					badge="MIGRATE"
					:title="formatMessage(messages.importInstanceTitle)"
					:description="formatMessage(messages.importInstanceDescription)"
					@click="ctx.setImportMode()"
				/>
			</div>
		</template>

		<template v-else>
			<div class="flex flex-col gap-2.5">
				<BigOptionButton
					:icon="CompassIcon"
					color-theme="cyan"
					badge="EXPLORE"
					:title="formatMessage(messages.modpackBaseTitle)"
					:description="formatMessage(messages.modpackBaseDescription)"
					@click="browseModpacks"
				/>
				<BigOptionButton
					:icon="UploadIcon"
					color-theme="indigo"
					badge="ARCHIVE"
					:title="formatMessage(messages.uploadModpackTitle)"
					:description="formatMessage(messages.uploadModpackDescription)"
					@click="triggerFileInput"
				/>
				<BigOptionButton
					:icon="BoxesIcon"
					color-theme="sky"
					badge="CUSTOM"
					:title="formatMessage(messages.customSetupTitle)"
					:description="formatMessage(messages.customSetupDescription)"
					@click="setSetupType('custom')"
				/>
				<BigOptionButton
					:icon="BoxIcon"
					color-theme="purple"
					badge="VANILLA"
					:title="formatMessage(messages.vanillaMinecraftTitle)"
					:description="formatMessage(messages.vanillaMinecraftDescription)"
					@click="setSetupType('vanilla')"
				/>
			</div>
		</template>
	</div>
</template>

<script setup lang="ts">
import {
	BoxesIcon,
	BoxIcon,
	BoxImportIcon,
	CompassIcon,
	DownloadIcon,
	RightArrowIcon,
	SparklesIcon,
	UploadIcon,
} from '@freeplay/assets'
import { commonMessages, defineMessages, useVIntl } from '@freeplay/ui'
import { computed, defineAsyncComponent, h, onMounted, ref, watch } from 'vue'

import { useDebugLogger } from '#ui/composables/debug-logger'

import { injectFilePicker } from '../../../../providers'
import BigOptionButton from '../../../base/BigOptionButton.vue'
import Combobox from '../../../base/Combobox.vue'
import { injectCreationFlowContext } from '../creation-flow-context'

const debug = useDebugLogger('SetupTypeStage')
const ctx = injectCreationFlowContext()
const filePicker = injectFilePicker()
const { setSetupType: _setSetupType } = ctx
const { formatMessage } = useVIntl()

const searchLoading = ref(false)
const projectSearchCombobox = ref<{ $el: HTMLElement }>()

const messages = defineMessages({
	knownProjectPrompt: {
		id: 'creation-flow.modal.project.known-project.prompt',
		defaultMessage: 'Already know what you want to play?',
	},
	searchProjectPlaceholder: {
		id: 'creation-flow.modal.project.search.placeholder',
		defaultMessage: 'Search mods, modpacks, and more...',
	},
	noResultsFound: {
		id: 'creation-flow.modal.project.search.no-results',
		defaultMessage: 'No results found',
	},
	installModpack: {
		id: 'creation-flow.modal.project.search.install-modpack',
		defaultMessage: 'Install modpack',
	},
	createInstance: {
		id: 'creation-flow.modal.project.search.create-instance',
		defaultMessage: 'Create instance',
	},
	instanceTypeTitle: {
		id: 'creation-flow.modal.setup-type.title.instance',
		defaultMessage: 'Choose instance type',
	},
	installationTypeTitle: {
		id: 'creation-flow.modal.setup-type.title.installation',
		defaultMessage: 'Select installation type',
	},
	worldTypeTitle: {
		id: 'creation-flow.modal.setup-type.title.world',
		defaultMessage: 'Select world type',
	},
	customSetupTitle: {
		id: 'creation-flow.modal.setup-type.option.custom-setup.title',
		defaultMessage: 'Custom setup',
	},
	customSetupDescription: {
		id: 'creation-flow.modal.setup-type.option.custom-setup.description',
		defaultMessage: 'Start from scratch by picking a loader and game version.',
	},
	modpackBaseTitle: {
		id: 'creation-flow.modal.setup-type.option.modpack-base.title',
		defaultMessage: 'Start from a mod or modpack',
	},
	modpackBaseDescription: {
		id: 'creation-flow.modal.setup-type.option.modpack-base.description',
		defaultMessage: 'Choose a project and we’ll use its latest version.',
	},
	uploadModpackTitle: {
		id: 'creation-flow.modal.setup-type.option.upload-modpack.title',
		defaultMessage: 'Upload a modpack',
	},
	uploadModpackDescription: {
		id: 'creation-flow.modal.setup-type.option.upload-modpack.description',
		defaultMessage: 'Install a modpack from an .mrpack file on your device.',
	},
	importInstanceTitle: {
		id: 'creation-flow.modal.setup-type.option.import-instance.title',
		defaultMessage: 'Import instance',
	},
	importInstanceDescription: {
		id: 'creation-flow.modal.setup-type.option.import-instance.description',
		defaultMessage: 'Import an instance from Prism, CurseForge, or similar.',
	},
	vanillaMinecraftTitle: {
		id: 'creation-flow.modal.setup-type.option.vanilla-minecraft.title',
		defaultMessage: 'Vanilla Minecraft',
	},
	vanillaMinecraftDescription: {
		id: 'creation-flow.modal.setup-type.option.vanilla-minecraft.description',
		defaultMessage: 'Classic Minecraft with no mods or plugins.',
	},
})

const setupTypeTitle = computed(() => {
	if (ctx.flowType === 'instance') {
		return formatMessage(messages.instanceTypeTitle)
	}
	if (ctx.flowType === 'server-onboarding' || ctx.flowType === 'reset-server') {
		return formatMessage(messages.installationTypeTitle)
	}
	return formatMessage(messages.worldTypeTitle)
})

function isModpackOption(projectId: string) {
	return ctx.projectSearchHits.value[projectId]?.projectType === 'modpack'
}

function setSetupType(type: 'custom' | 'vanilla') {
	debug('selected:', type)
	_setSetupType(type)
}

function selectModpack() {
	debug('selected: modpack')
	_setSetupType('modpack')
}

function proceedWithModpack() {
	if (ctx.finishDisabled.value) return

	if (ctx.flowType === 'instance') {
		ctx.finish()
	} else {
		ctx.modal.value?.setStage('final-config')
	}
}

function browseModpacks() {
	if (ctx.finishDisabled.value) return

	selectModpack()
	ctx.browseModpacks()
}

async function triggerFileInput() {
	if (ctx.finishDisabled.value) return

	const picked = await filePicker.pickModpackFile({
		readFile: ctx.flowType !== 'instance',
	})
	if (!picked) return

	selectModpack()
	ctx.modpackFile.value = picked.file ?? null
	ctx.modpackFilePath.value = picked.path ?? null
	proceedWithModpack()
}

async function search(query: string) {
	try {
		if (!query.trim()) {
			ctx.projectSearchOptions.value = []
			return
		}
		const results = await ctx.searchProjects(query.trim(), 10)

		ctx.projectSearchHits.value = {}
		for (const hit of results.hits) {
			ctx.projectSearchHits.value[hit.project_id] = {
				title: hit.title,
				iconUrl: hit.icon_url,
				latestVersion: hit.latest_version,
				projectType: hit.project_type ?? 'modpack',
			}
		}

		ctx.projectSearchOptions.value = results.hits.map((hit) => ({
			label: hit.title,
			value: hit.project_id,
			icon: defineAsyncComponent(() =>
				Promise.resolve({
					setup: () => () =>
						h('img', {
							src: hit.icon_url,
							alt: hit.title,
							class: 'h-6 w-6 rounded-lg border border-white/10 object-cover shadow-sm bg-black/40',
						}),
				}),
			),
		}))
	} catch (error) {
		debug('project search failed:', error)
		ctx.projectSearchOptions.value = []
	} finally {
		searchLoading.value = false
	}
}

async function handleSearch(query: string) {
	searchLoading.value = true
	await search(query)
}

onMounted(() => {
	ctx.projectSearchProjectId.value = undefined
	search('')
	setTimeout(() => {
		projectSearchCombobox.value?.$el.querySelector('input')?.focus()
	}, 150)
})

watch(
	() => ctx.projectSearchProjectId.value,
	async (projectId, oldProjectId) => {
		if (projectId === oldProjectId) return

		if (!projectId) return
		const hit = ctx.projectSearchHits.value[projectId]

		if (ctx.flowType === 'instance') {
			void ctx.selectProject(projectId, hit?.projectType ?? 'mod')
			return
		}

		try {
			const versions = await ctx.getProjectVersions(projectId)
			if (ctx.projectSearchProjectId.value !== projectId || versions.length === 0) return

			selectModpack()
			ctx.modpackSelection.value = {
				projectId,
				versionId: versions[0].id,
				name: hit?.title ?? '',
				iconUrl: hit?.iconUrl,
			}
			proceedWithModpack()
		} catch (error) {
			debug('failed to load project versions:', error)
		}
	},
)
</script>
