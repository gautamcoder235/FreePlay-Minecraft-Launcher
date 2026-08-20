<template>
	<div class="flex flex-col gap-4 select-none">
		<!-- When a Modpack is Selected: Summary Hero Card with Version Selection -->
		<div
			v-if="selectedModpack"
			class="flex flex-col gap-3 rounded-2xl border border-white/10 bg-[#141923] p-4 shadow-md"
		>
			<div class="flex items-center justify-between">
				<label
					class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono flex items-center gap-1.5"
				>
					<SparklesIcon class="size-3.5 text-sky-400" />
					<span>{{ formatMessage(messages.selectedModpackTitle) }}</span>
				</label>
				<button
					type="button"
					class="flex items-center gap-1 text-xs text-zinc-400 hover:text-white font-mono px-2 py-0.5 rounded border border-white/10 bg-[#0e131d] hover:bg-white/5 transition-all cursor-pointer"
					@click="clearSelection"
				>
					<XIcon class="size-3 text-zinc-400" />
					<span>{{ formatMessage(messages.clearSelection) }}</span>
				</button>
			</div>

			<!-- Modpack Hero Summary -->
			<div class="flex items-center gap-3.5 p-3 rounded-xl bg-[#0e131d] border border-white/10">
				<div
					class="size-14 shrink-0 overflow-hidden rounded-xl border border-white/10 bg-[#141923] flex items-center justify-center shadow-inner"
				>
					<Avatar
						v-if="selectedModpack.iconUrl"
						:src="selectedModpack.iconUrl"
						:alt="selectedModpack.name"
						size="100%"
						class="!rounded-xl object-cover"
						no-shadow
					/>
					<BoxesIcon v-else class="size-7 text-sky-400" />
				</div>

				<div class="flex flex-1 flex-col gap-1 min-w-0">
					<div class="flex items-center gap-2">
						<span class="font-bold text-white text-sm tracking-wide truncate">
							{{ selectedModpack.name }}
						</span>
						<span
							class="text-[10px] font-mono font-bold px-2 py-0.5 rounded bg-sky-500/20 text-sky-300 border border-sky-500/30 uppercase tracking-wider"
						>
							Modpack
						</span>
					</div>
					<div class="flex flex-wrap items-center gap-2 text-xs text-zinc-400 font-mono">
						<span v-if="selectedVersionName" class="text-zinc-300">
							{{ selectedVersionName }}
						</span>
						<span class="text-zinc-600">•</span>
						<span class="text-sky-400 font-semibold">Ready to Install</span>
					</div>
				</div>
			</div>

			<!-- Version & Release Selector Pills -->
			<div v-if="versionOptions.length > 0" class="flex flex-col gap-2 pt-1">
				<div class="flex items-center justify-between">
					<label class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono">
						{{ formatMessage(messages.versionLabel) }}
					</label>
					<span
						v-if="selectedVersionId"
						class="text-[10px] font-mono font-bold px-2 py-0.5 rounded bg-sky-500/10 text-sky-300 border border-sky-500/30"
					>
						{{
							versionOptions.find((v) => v.value === selectedVersionId)?.label ?? selectedVersionId
						}}
					</span>
				</div>

				<!-- Quick Pills for Top Versions -->
				<div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
					<button
						v-for="ver in versionOptions.slice(0, 3)"
						:key="ver.value"
						type="button"
						class="flex items-center justify-between py-2 px-2.5 rounded-xl border text-xs font-bold tracking-wide transition-all active:scale-95 cursor-pointer select-none"
						:class="[
							selectedVersionId === ver.value
								? 'border-sky-500/60 bg-sky-500/10 text-sky-300 shadow-[0_0_15px_rgba(56,189,248,0.2)] ring-1 ring-sky-500/40'
								: 'border-white/10 bg-[#0e131d] hover:bg-[#18202e] hover:border-white/20 text-zinc-400 hover:text-zinc-200',
						]"
						@click="selectVersion(ver.value)"
					>
						<span class="truncate">{{ ver.label }}</span>
						<span
							v-if="ver.value === versionOptions[0]?.value"
							class="text-[9px] font-mono font-bold uppercase px-1 py-0.2 rounded bg-sky-500/20 text-sky-300 border border-sky-500/30 ml-1 shrink-0"
						>
							Latest
						</span>
					</button>
				</div>

				<!-- Dropdown if more than 3 versions -->
				<div
					v-if="versionOptions.length > 3"
					class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-sky-500/50 transition-all duration-200 mt-1"
				>
					<Combobox
						v-model="selectedVersionId"
						:options="versionOptions"
						searchable
						sync-with-selection
						show-search-icon
						:placeholder="formatMessage(messages.selectVersionPlaceholder)"
					/>
				</div>
			</div>

			<!-- Action Proceed Button -->
			<div class="pt-2">
				<Button
					type="colored"
					color="brand"
					class="w-full !bg-sky-600 hover:!bg-sky-500 !text-white shadow-lg !shadow-sky-950/60 font-bold !border-none rounded-xl py-3 transition-all active:scale-95"
					:disabled="ctx.finishDisabled.value"
					@click="proceedWithModpack"
				>
					<DownloadIcon class="size-4" />
					<span>{{ formatMessage(messages.installSelectedModpack) }}</span>
				</Button>
			</div>
		</div>

		<!-- Search Section (when no modpack is active) -->
		<template v-else>
			<div class="flex flex-col gap-2">
				<div class="flex items-center justify-between">
					<label
						class="text-xs font-bold uppercase tracking-wider text-zinc-300 flex items-center gap-1.5 font-mono"
					>
						<SparklesIcon class="size-3.5 text-indigo-400" />
						{{ formatMessage(messages.knownModpackPrompt) }}
					</label>
					<span
						class="text-[10px] font-mono font-medium px-2 py-0.5 rounded-md bg-white/5 text-zinc-400 border border-white/10"
					>
						⚡ Quick Search
					</span>
				</div>

				<!-- Search Spotlight Capsule -->
				<div
					class="relative group rounded-2xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-indigo-500/50 focus-within:shadow-[0_0_20px_rgba(99,102,241,0.2)] transition-all duration-200"
				>
					<Combobox
						v-model="selectedSearchProjectId"
						v-tooltip="ctx.finishDisabled.value ? ctx.finishDisabledTooltip.value : undefined"
						:options="searchOptions"
						searchable
						show-search-icon
						:show-chevron="false"
						:disabled="ctx.finishDisabled.value"
						:search-placeholder="formatMessage(messages.searchModpackPlaceholder)"
						:no-options-message="
							searchLoading
								? formatMessage(commonMessages.loadingLabel)
								: formatMessage(messages.noResultsFound)
						"
						:disable-search-filter="true"
						@search-input="handleSearch"
					>
						<template #option-suffix>
							<div
								class="flex shrink-0 items-center gap-1.5 text-xs font-bold text-indigo-400 opacity-0 transition-opacity group-hover/option:opacity-100 group-data-[focused=true]/option:opacity-100"
							>
								<span>Select</span>
								<RightArrowIcon class="size-4 shrink-0" />
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
					class="relative px-3 py-0.5 rounded-full bg-[#141923] border border-white/10 text-[10px] font-bold text-zinc-400 font-mono tracking-wider uppercase"
				>
					{{ formatMessage(commonMessages.orLabel) }} Choose Option
				</div>
			</div>

			<!-- Alternative Option Cards -->
			<div class="flex flex-col gap-2.5">
				<BigOptionButton
					:icon="CompassIcon"
					color-theme="cyan"
					badge="EXPLORE"
					:title="formatMessage(messages.browseModpacks)"
					:description="formatMessage(messages.browseModpacksDescription)"
					@click="ctx.browseModpacks()"
				/>
				<BigOptionButton
					:icon="ImportIcon"
					color-theme="indigo"
					badge="ARCHIVE"
					:title="formatMessage(messages.importModpack)"
					:description="formatMessage(messages.importModpackDescription)"
					@click="triggerFileInput"
				/>
			</div>
		</template>
	</div>
</template>

<script setup lang="ts">
import {
	BoxesIcon,
	CompassIcon,
	DownloadIcon,
	ImportIcon,
	RightArrowIcon,
	SparklesIcon,
	XIcon,
} from '@freeplay/assets'
import { commonMessages, defineMessages, useVIntl } from '@freeplay/ui'
import { computed, defineAsyncComponent, h, onMounted, ref, watch } from 'vue'

import { Button } from '#ui/components/base/buttons'
import { useDebugLogger } from '#ui/composables/debug-logger'

import { injectFilePicker } from '../../../../providers'
import Avatar from '../../../base/Avatar.vue'
import BigOptionButton from '../../../base/BigOptionButton.vue'
import Combobox, { type ComboboxOption } from '../../../base/Combobox.vue'
import type { ModpackSelection } from '../creation-flow-context'
import { injectCreationFlowContext } from '../creation-flow-context'

const debug = useDebugLogger('ModpackStage')
const ctx = injectCreationFlowContext()
const filePicker = injectFilePicker()
const { formatMessage } = useVIntl()

const searchLoading = ref(false)
const selectedSearchProjectId = ref<string | undefined>()
const searchOptions = ref<ComboboxOption<string>[]>([])
const searchHits = ref<Record<string, { title: string; iconUrl?: string }>>({})

const selectedModpack = ref<ModpackSelection | null>(null)
const selectedVersionId = ref<string | undefined>()
const versionOptions = ref<ComboboxOption<string>[]>([])

const messages = defineMessages({
	knownModpackPrompt: {
		id: 'creation-flow.modal.modpack.known-modpack.prompt',
		defaultMessage: 'Already know the modpack you want to install?',
	},
	searchModpackPlaceholder: {
		id: 'creation-flow.modal.modpack.search.placeholder',
		defaultMessage: 'Search for modpack...',
	},
	noResultsFound: {
		id: 'creation-flow.modal.modpack.search.no-results',
		defaultMessage: 'No results found',
	},
	importModpack: {
		id: 'creation-flow.modal.modpack.action.import',
		defaultMessage: 'Import modpack file',
	},
	importModpackDescription: {
		id: 'creation-flow.modal.modpack.action.import-description',
		defaultMessage: 'Load an .mrpack file directly from your local computer.',
	},
	browseModpacks: {
		id: 'creation-flow.modal.modpack.action.browse',
		defaultMessage: 'Browse modpacks catalog',
	},
	browseModpacksDescription: {
		id: 'creation-flow.modal.modpack.action.browse-description',
		defaultMessage: 'Explore thousands of curated modpacks with filters.',
	},
	selectedModpackTitle: {
		id: 'creation-flow.modal.modpack.selected-modpack.title',
		defaultMessage: 'Selected Modpack',
	},
	clearSelection: {
		id: 'creation-flow.modal.modpack.clear-selection',
		defaultMessage: 'Change',
	},
	versionLabel: {
		id: 'creation-flow.modal.modpack.version.label',
		defaultMessage: 'Modpack version',
	},
	selectVersionPlaceholder: {
		id: 'creation-flow.modal.modpack.version.placeholder',
		defaultMessage: 'Select specific version...',
	},
	installSelectedModpack: {
		id: 'creation-flow.modal.modpack.install-selected',
		defaultMessage: 'Install Modpack',
	},
})

const selectedVersionName = computed(() => {
	if (!selectedVersionId.value) return ''
	const option = versionOptions.value.find((o) => o.value === selectedVersionId.value)
	return option?.label || selectedVersionId.value
})

function clearSelection() {
	selectedModpack.value = null
	selectedVersionId.value = undefined
	versionOptions.value = []
	selectedSearchProjectId.value = undefined
	ctx.modpackSelection.value = null
}

function selectVersion(versionId: string) {
	selectedVersionId.value = versionId
	if (selectedModpack.value) {
		selectedModpack.value.versionId = versionId
		ctx.modpackSelection.value = {
			...selectedModpack.value,
			versionId,
		}
	}
}

function proceedWithModpack() {
	if (ctx.finishDisabled.value) return

	if (selectedModpack.value) {
		ctx.modpackSelection.value = selectedModpack.value
	}

	debug('proceedWithModpack:', {
		flowType: ctx.flowType,
		modpackSelection: ctx.modpackSelection.value,
	})
	if (ctx.flowType === 'instance') {
		ctx.finish()
	} else {
		ctx.modal.value?.setStage('final-config')
	}
}

const search = async (query: string) => {
	query = query.trim()
	debug('search() called:', { query })

	try {
		const results = await ctx.searchProjects(query, 10)
		const hits: Record<string, { title: string; iconUrl?: string }> = {}
		for (const hit of results.hits) {
			hits[hit.project_id] = {
				title: hit.title,
				iconUrl: hit.icon_url,
			}
		}
		searchHits.value = hits

		searchOptions.value = results.hits.map((hit) => ({
			label: hit.title,
			value: hit.project_id,
			icon: defineAsyncComponent(() =>
				Promise.resolve({
					setup: () => () =>
						h('img', {
							src: hit.icon_url,
							alt: hit.title,
							class: 'size-5 rounded object-cover',
						}),
				}),
			),
		}))
	} catch (err) {
		debug('search() ERROR:', err)
		searchOptions.value = []
	} finally {
		searchLoading.value = false
	}
}

const handleSearch = async (query: string) => {
	searchLoading.value = true
	await search(query)
}

onMounted(() => {
	selectedSearchProjectId.value = undefined
	search('')
})

// When a project is selected via search, fetch versions and show hero card with version selector
watch(selectedSearchProjectId, async (projectId) => {
	if (!projectId) return

	const hit = searchHits.value[projectId]
	try {
		const versions = await ctx.getProjectVersions(projectId)
		if (selectedSearchProjectId.value !== projectId) return
		if (versions.length > 0) {
			versionOptions.value = versions.map((v) => ({
				label:
					(v as { name?: string; version_number?: string; id: string }).name ||
					(v as { version_number?: string; id: string }).version_number ||
					v.id,
				value: v.id,
			}))
			selectedVersionId.value = versions[0].id
			selectedModpack.value = {
				projectId,
				versionId: versions[0].id,
				name: hit?.title ?? 'Modpack',
				iconUrl: hit?.iconUrl,
			}
			ctx.modpackSelection.value = selectedModpack.value
		}
	} catch (err) {
		debug('Failed to fetch modpack versions:', err)
	}
})

async function triggerFileInput() {
	if (ctx.finishDisabled.value) return

	const picked = await filePicker.pickModpackFile({
		readFile: ctx.flowType !== 'instance',
	})
	if (picked) {
		ctx.modpackFile.value = picked.file ?? null
		ctx.modpackFilePath.value = picked.path ?? null
		proceedWithModpack()
	}
}
</script>
