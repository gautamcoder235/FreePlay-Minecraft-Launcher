<template>
	<div class="flex flex-col gap-4 select-none">
		<!-- Hero Card: Project Install Info (when starting from a project/modpack) -->
		<div
			v-if="ctx.projectInstall.value"
			class="flex items-center gap-3.5 rounded-2xl border border-white/10 bg-[#141923] p-3.5 shadow-md"
		>
			<div class="size-12 shrink-0 overflow-hidden rounded-xl border border-white/10 bg-[#0e131d]">
				<Avatar
					v-if="ctx.projectInstall.value.iconUrl"
					:src="ctx.projectInstall.value.iconUrl"
					:alt="ctx.projectInstall.value.title"
					size="100%"
					class="!rounded-xl object-cover"
					no-shadow
				/>
			</div>
			<div class="flex flex-1 flex-col gap-1 min-w-0">
				<div class="flex items-center gap-2">
					<span class="font-bold text-white text-sm tracking-wide truncate">
						{{ ctx.projectInstall.value.title }}
					</span>
					<span
						class="text-[10px] font-mono font-bold px-2 py-0.5 rounded bg-sky-500/20 text-sky-300 border border-sky-500/30 uppercase tracking-wider"
					>
						Modpack
					</span>
				</div>
				<div
					v-if="ctx.projectInstall.value.owner"
					class="flex items-center gap-2 text-xs text-zinc-400 font-mono"
				>
					<div class="flex items-center gap-1.5 text-inherit">
						<Avatar
							:src="ctx.projectInstall.value.owner.iconUrl"
							:alt="ctx.projectInstall.value.owner.name"
							size="1rem"
							:circle="ctx.projectInstall.value.owner.circle"
							no-shadow
						/>
						<span class="font-medium">{{ ctx.projectInstall.value.owner.name }}</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Instance-specific: Profile Customization Card -->
		<div
			v-if="ctx.flowType === 'instance'"
			class="flex flex-col gap-3 rounded-2xl border border-white/10 bg-[#141923] p-3.5 shadow-md"
		>
			<div class="flex items-center justify-between">
				<label
					class="text-xs font-bold uppercase tracking-wider text-zinc-300 flex items-center gap-1.5 font-mono"
				>
					<SparklesIcon class="size-3.5 text-sky-400" />
					{{ formatMessage(messages.instanceProfileTitle) }}
				</label>
				<span
					class="text-[10px] font-mono font-medium px-2 py-0.5 rounded-md bg-white/5 text-zinc-400 border border-white/10"
				>
					Profile Setup
				</span>
			</div>

			<div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3.5">
				<!-- Custom Icon Picker Thumbnail -->
				<div
					class="group relative size-20 shrink-0 self-center sm:self-auto rounded-2xl border border-white/10 bg-[#0e131d] overflow-hidden flex items-center justify-center shadow-inner"
				>
					<Avatar
						v-if="ctx.instanceIconUrl.value"
						:src="ctx.instanceIconUrl.value"
						size="100%"
						class="!rounded-2xl object-cover"
						no-shadow
					/>
					<div v-else class="flex flex-col items-center justify-center text-zinc-500">
						<BoxesIcon class="size-8 text-zinc-600" />
					</div>

					<!-- Remove Icon overlay button -->
					<button
						v-if="ctx.instanceIconUrl.value"
						type="button"
						class="absolute right-1 top-1 size-5 rounded-md bg-red-500/80 hover:bg-red-500 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 transition-opacity cursor-pointer border-none shadow"
						:aria-label="formatMessage(commonMessages.removeImageButton)"
						@click.stop="removeIcon"
					>
						<XIcon class="size-3" />
					</button>
				</div>

				<!-- Name Field & Customization Buttons -->
				<div class="flex flex-1 flex-col gap-2 min-w-0">
					<!-- Instance Name Dark Glass Input -->
					<div
						class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-sky-500/50 focus-within:shadow-[0_0_15px_rgba(56,189,248,0.2)] transition-all duration-200"
					>
						<StyledInput
							v-model="ctx.instanceName.value"
							:placeholder="
								ctx.autoInstanceName.value || formatMessage(messages.instanceNamePlaceholder)
							"
							wrapper-class="w-full !bg-transparent"
							input-class="!bg-transparent !text-white !placeholder-zinc-500 font-medium text-sm"
						/>
					</div>

					<!-- Action Buttons -->
					<div class="flex flex-wrap items-center gap-1.5">
						<button
							type="button"
							class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg border border-white/10 bg-[#0e131d] hover:bg-white/5 hover:border-white/20 text-zinc-300 hover:text-white text-xs font-medium transition-all active:scale-95 cursor-pointer"
							@click="triggerIconInput"
						>
							<UploadIcon class="size-3.5 text-indigo-400" />
							<span>{{ formatMessage(messages.uploadIcon) }}</span>
						</button>

						<button
							type="button"
							:disabled="randomizing"
							class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg border border-white/10 bg-[#0e131d] hover:bg-white/5 hover:border-white/20 text-zinc-300 hover:text-white text-xs font-medium transition-all active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer"
							@click="randomizeIcon"
						>
							<SpinnerIcon v-if="randomizing" class="size-3.5 animate-spin text-sky-400" />
							<TagCategoryDicesIcon
								v-else
								class="size-3.5 text-sky-400 transition-transform duration-300"
								:class="{ 'rotate-180': diceSpinning }"
							/>
							<span>{{ formatMessage(messages.randomizeIcon) }}</span>
						</button>

						<button
							v-if="ctx.customizeInstanceIcon"
							type="button"
							class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg border border-white/10 bg-[#0e131d] hover:bg-white/5 hover:border-white/20 text-zinc-300 hover:text-white text-xs font-medium transition-all active:scale-95 cursor-pointer"
							@click="ctx.customizeInstanceIcon?.()"
						>
							<PaletteIcon class="size-3.5 text-purple-400" />
							<span>{{ formatMessage(messages.customizeIcon) }}</span>
						</button>
					</div>
				</div>
			</div>
		</div>

		<!-- Loader Selector: Bento Grid Pills -->
		<div v-if="!hideLoaderChips" class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<label
					class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono flex items-center gap-1.5"
				>
					<BoxesIcon class="size-3.5 text-sky-400" />
					<span>{{
						ctx.flowType === 'instance'
							? formatMessage(messages.loaderLabel)
							: formatMessage(messages.contentLoaderLabel)
					}}</span>
				</label>
				<span
					class="text-[10px] font-mono font-medium px-2 py-0.5 rounded-md bg-white/5 text-zinc-400 border border-white/10"
				>
					Select Mod Loader
				</span>
			</div>

			<div class="grid grid-cols-2 gap-2 sm:grid-cols-4">
				<button
					v-for="loader in effectiveLoaders"
					:key="loader"
					type="button"
					class="group relative flex flex-col items-center justify-center gap-2 rounded-xl p-2.5 text-center transition-all duration-200 active:scale-[0.97] border cursor-pointer select-none"
					:class="[
						selectedLoader === loader
							? 'border-sky-500/60 bg-sky-500/10 text-sky-300 shadow-[0_0_20px_rgba(56,189,248,0.18)] ring-1 ring-sky-500/40'
							: 'border-white/10 bg-[#141923] hover:bg-[#18202e] hover:border-white/20 text-zinc-400 hover:text-zinc-200',
					]"
					@click="selectedLoader = loader"
				>
					<!-- Loader Icon -->
					<div
						class="flex size-8 items-center justify-center rounded-lg border transition-transform duration-200 group-hover:scale-105"
						:class="[
							selectedLoader === loader
								? 'border-sky-500/40 bg-sky-500/20 text-sky-400 shadow-[0_0_10px_rgba(56,189,248,0.2)]'
								: 'border-white/10 bg-[#0e131d] text-zinc-400 group-hover:text-zinc-200',
						]"
					>
						<component :is="getLoaderIcon(loader)" class="size-4.5 shrink-0" />
					</div>

					<!-- Loader Name -->
					<span
						class="text-xs font-bold tracking-wide truncate w-full"
						:class="selectedLoader === loader ? 'text-white' : 'text-zinc-300'"
					>
						{{ formatLoaderLabel(loader) }}
					</span>

					<!-- Status / Compatibility Tag -->
					<span
						class="text-[9px] font-mono font-bold uppercase px-1.5 py-0.5 rounded tracking-wider"
						:class="[
							selectedLoader === loader
								? 'bg-sky-500/20 text-sky-300 border border-sky-500/30'
								: 'bg-white/5 text-zinc-500 border border-white/5 group-hover:text-zinc-400',
						]"
					>
						{{ getLoaderBadge(loader) }}
					</span>
				</button>
			</div>
		</div>

		<!-- Game Version Picker: Sleek Dropdown Capsule -->
		<div class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<label class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono">
					{{ formatMessage(commonMessages.gameVersionLabel) }}
				</label>
				<span
					v-if="selectedGameVersion"
					class="text-[10px] font-mono font-bold px-2 py-0.5 rounded-md bg-sky-500/10 text-sky-300 border border-sky-500/30"
				>
					{{ selectedGameVersion }}
				</span>
			</div>
			<div
				class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-sky-500/50 focus-within:shadow-[0_0_15px_rgba(56,189,248,0.2)] transition-all duration-200"
			>
				<Combobox
					v-model="selectedGameVersion"
					:options="gameVersionOptions"
					:no-options-message="
						gameVersionsLoading
							? formatMessage(commonMessages.loadingLabel)
							: formatMessage(messages.noVersionsAvailable)
					"
					searchable
					sync-with-selection
					show-search-icon
					:placeholder="formatMessage(messages.selectGameVersion)"
					:search-placeholder="formatMessage(messages.searchGameVersion)"
					@option-hover="handleGameVersionHover"
				>
					<template v-if="ctx.showSnapshotToggle" #dropdown-footer>
						<button
							class="flex w-full cursor-pointer items-center justify-center gap-1.5 border-0 border-t border-solid border-white/10 bg-[#141923] hover:bg-[#18202e] py-2.5 text-center text-xs font-mono font-bold text-zinc-400 transition-colors hover:text-sky-400"
							@mousedown.prevent
							@click="ctx.showSnapshots.value = !ctx.showSnapshots.value"
						>
							<EyeOffIcon v-if="ctx.showSnapshots.value" class="size-4 text-zinc-400" />
							<EyeIcon v-else class="size-4 text-sky-400" />
							{{
								ctx.showSnapshots.value
									? formatMessage(commonMessages.hideSnapshotsButton)
									: formatMessage(commonMessages.showAllVersionsButton)
							}}
						</button>
					</template>
				</Combobox>
			</div>
		</div>

		<!-- Loader Version Section -->
		<template v-if="!hideLoaderVersion">
			<Collapsible :collapsed="!selectedLoader || !selectedGameVersion" overflow-visible>
				<div class="flex flex-col gap-2 pt-1">
					<div class="flex items-center justify-between">
						<label class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono">
							{{
								isPaperLike
									? formatMessage(messages.buildNumberLabel)
									: formatMessage(messages.loaderVersionLabel)
							}}
						</label>
					</div>

					<!-- Loader Version Type Pills (Stable / Latest / Other) -->
					<div v-if="!isPaperLike" class="grid grid-cols-3 gap-2">
						<button
							v-for="item in loaderVersionTypeItems"
							:key="item"
							type="button"
							:disabled="loaderVersionTypeDisabledItems.includes(item)"
							class="flex items-center justify-center gap-1.5 py-2 px-3 rounded-xl border text-xs font-bold tracking-wide transition-all active:scale-95 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer select-none"
							:class="[
								loaderVersionType === item
									? 'border-sky-500/50 bg-sky-500/10 text-sky-300 shadow-[0_0_15px_rgba(56,189,248,0.15)] ring-1 ring-sky-500/30'
									: 'border-white/10 bg-[#141923] hover:bg-[#18202e] hover:border-white/20 text-zinc-400 hover:text-zinc-200',
							]"
							@click="loaderVersionType = item"
						>
							<span>{{ formatLoaderVersionTypeLabel(item) }}</span>
						</button>
					</div>

					<!-- Specific Version Combobox (when Other or Paper-like) -->
					<div
						v-if="isPaperLike || loaderVersionType === 'other'"
						class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-sky-500/50 focus-within:shadow-[0_0_15px_rgba(56,189,248,0.2)] transition-all duration-200"
					>
						<Combobox
							v-model="selectedLoaderVersion"
							:options="loaderVersionOptions"
							:no-options-message="
								loaderVersionsLoading
									? formatMessage(commonMessages.loadingLabel)
									: formatMessage(messages.noVersionsAvailable)
							"
							searchable
							sync-with-selection
							show-search-icon
							:placeholder="
								isPaperLike
									? formatMessage(messages.selectBuildNumber)
									: formatMessage(messages.selectLoaderVersion)
							"
							:search-placeholder="
								isPaperLike
									? formatMessage(messages.searchBuildNumber)
									: formatMessage(messages.searchLoaderVersion)
							"
						>
							<!-- When Paper, render build channel tag -->
							<template v-if="selectedLoader === 'paper'" #option="{ item, isSelected }">
								<div class="flex w-full items-center justify-between gap-2">
									<div class="flex flex-wrap items-center gap-2">
										<span
											class="font-semibold leading-tight"
											:class="isSelected ? 'text-contrast' : 'text-primary'"
										>
											{{ item.label }}
										</span>
										<PaperChannelBadge :channel="paperBuildChannelTag(String(item.value))" />
									</div>
								</div>
							</template>
							<template v-if="selectedLoader === 'paper'" #search-selection-affix="{ option }">
								<PaperChannelBadge
									affix
									:channel="option ? paperBuildChannelTag(String(option.value)) : null"
								/>
							</template>
						</Combobox>
					</div>
				</div>
			</Collapsible>
		</template>
	</div>
</template>

<script setup lang="ts">
import type { Paper } from '@freeplay/api-client'
import {
	BoxesIcon,
	EyeIcon,
	EyeOffIcon,
	loaderIconMap,
	PaletteIcon,
	SparklesIcon,
	SpinnerIcon,
	TagCategoryDicesIcon,
	UploadIcon,
	XIcon,
} from '@freeplay/assets'
import { commonMessages, defineMessages, useVIntl } from '@freeplay/ui'
import { computed, onMounted, ref, watch } from 'vue'

import { useDebugLogger } from '#ui/composables/debug-logger'

import { injectFilePicker, injectFreePlayClient, injectTags } from '../../../../providers'
import Avatar from '../../../base/Avatar.vue'
import Collapsible from '../../../base/Collapsible.vue'
import Combobox, { type ComboboxOption } from '../../../base/Combobox.vue'
import PaperChannelBadge from '../../../base/PaperChannelBadge.vue'
import StyledInput from '../../../base/StyledInput.vue'
import type { LoaderVersionEntry, LoaderVersionType } from '../creation-flow-context'
import { injectCreationFlowContext } from '../creation-flow-context'
import { formatLoaderLabel } from '../shared'

const debug = useDebugLogger('CustomSetupStage')
const client = injectFreePlayClient()
const ctx = injectCreationFlowContext()
const { formatMessage } = useVIntl()
const {
	selectedLoader,
	selectedGameVersion,
	loaderVersionType,
	selectedLoaderVersion,
	hideLoaderChips,
	hideLoaderVersion,
} = ctx

const messages = defineMessages({
	instanceProfileTitle: {
		id: 'creation-flow.modal.custom-setup.instance-profile.title',
		defaultMessage: 'Instance Profile',
	},
	uploadIcon: {
		id: 'creation-flow.modal.custom-setup.icon.select',
		defaultMessage: 'Upload',
	},
	randomizeIcon: {
		id: 'creation-flow.modal.custom-setup.icon.randomize',
		defaultMessage: 'Randomize',
	},
	customizeIcon: {
		id: 'creation-flow.modal.custom-setup.icon.customize',
		defaultMessage: 'Customize',
	},
	instanceNamePlaceholder: {
		id: 'creation-flow.modal.custom-setup.name.placeholder',
		defaultMessage: 'Enter instance name',
	},
	loaderLabel: {
		id: 'creation-flow.modal.custom-setup.loader.label',
		defaultMessage: 'Loader',
	},
	contentLoaderLabel: {
		id: 'creation-flow.modal.custom-setup.content-loader.label',
		defaultMessage: 'Content loader',
	},
	noVersionsAvailable: {
		id: 'creation-flow.modal.custom-setup.options.no-versions-available',
		defaultMessage: 'No versions available',
	},
	selectGameVersion: {
		id: 'creation-flow.modal.custom-setup.game-version.placeholder',
		defaultMessage: 'Select game version',
	},
	searchGameVersion: {
		id: 'creation-flow.modal.custom-setup.game-version.search-placeholder',
		defaultMessage: 'Search game version...',
	},
	buildNumberLabel: {
		id: 'creation-flow.modal.custom-setup.build-number.label',
		defaultMessage: 'Build number',
	},
	loaderVersionLabel: {
		id: 'creation-flow.modal.custom-setup.loader-version.label',
		defaultMessage: 'Loader version',
	},
	selectBuildNumber: {
		id: 'creation-flow.modal.custom-setup.build-number.placeholder',
		defaultMessage: 'Select build number',
	},
	selectLoaderVersion: {
		id: 'creation-flow.modal.custom-setup.loader-version.placeholder',
		defaultMessage: 'Select loader version',
	},
	searchBuildNumber: {
		id: 'creation-flow.modal.custom-setup.build-number.search-placeholder',
		defaultMessage: 'Search build number...',
	},
	searchLoaderVersion: {
		id: 'creation-flow.modal.custom-setup.loader-version.search-placeholder',
		defaultMessage: 'Search loader version...',
	},
	stableLoaderVersionType: {
		id: 'creation-flow.modal.custom-setup.loader-version-type.stable',
		defaultMessage: 'Stable',
	},
	latestLoaderVersionType: {
		id: 'creation-flow.modal.custom-setup.loader-version-type.latest',
		defaultMessage: 'Latest',
	},
	otherLoaderVersionType: {
		id: 'creation-flow.modal.custom-setup.loader-version-type.other',
		defaultMessage: 'Other',
	},
})

function formatLoaderVersionTypeLabel(type: LoaderVersionType): string {
	switch (type) {
		case 'stable':
			return formatMessage(messages.stableLoaderVersionType)
		case 'latest':
			return formatMessage(messages.latestLoaderVersionType)
		case 'other':
			return formatMessage(messages.otherLoaderVersionType)
	}
}

function getLoaderIcon(loader: string) {
	return loaderIconMap[loader] || BoxesIcon
}

function getLoaderBadge(loader: string): string {
	switch (loader) {
		case 'fabric':
			return 'Popular'
		case 'neoforge':
			return 'Modern'
		case 'forge':
			return 'Classic'
		case 'quilt':
			return 'Community'
		case 'vanilla':
			return 'Official'
		case 'paper':
			return 'Fast'
		case 'purpur':
			return 'Optimized'
		default:
			return 'Loader'
	}
}

const effectiveLoaders = computed(() => {
	if (ctx.projectInstall.value) {
		return ctx.projectInstall.value.compatibleLoaders
	}
	if (ctx.flowType === 'instance') {
		return ['vanilla', ...ctx.availableLoaders.filter((l) => l !== 'vanilla')]
	}
	if (ctx.flowType === 'server-onboarding' || ctx.flowType === 'reset-server') {
		return ctx.availableLoaders.filter((l) => l !== 'vanilla')
	}
	return ctx.availableLoaders
})

onMounted(() => {
	debug('mounted, initialLoader:', ctx.initialLoader, 'initialGameVersion:', ctx.initialGameVersion)
	if (ctx.flowType === 'instance') {
		void randomizeIcon()
	}
	if (!selectedLoader.value) {
		if (ctx.initialLoader) {
			selectedLoader.value = ctx.initialLoader
		} else {
			selectedLoader.value = 'fabric'
		}
	}
	if (ctx.initialGameVersion && !selectedGameVersion.value) {
		selectedGameVersion.value = ctx.initialGameVersion
	}
	debug('after init:', { loader: selectedLoader.value, gameVersion: selectedGameVersion.value })
})

const tags = injectTags()

const loaderVersionTypeItems: LoaderVersionType[] = ['stable', 'latest', 'other']

const loaderVersionTypeDisabledItems = computed<LoaderVersionType[]>(() => {
	const noStableVersions = !loaderVersionsData.value.some((v: LoaderVersionEntry) => v.stable)
	return noStableVersions ? ['stable'] : []
})

const isPaperLike = computed(
	() => selectedLoader.value === 'paper' || selectedLoader.value === 'purpur',
)

const filePicker = injectFilePicker()

async function triggerIconInput() {
	const picked = await filePicker.pickImage()
	if (picked) {
		ctx.instanceIcon.value = picked.file
		ctx.instanceIconUrl.value = picked.previewUrl
		ctx.instanceIconPath.value = picked.path ?? null
	}
}

function removeIcon() {
	ctx.instanceIcon.value = null
	ctx.instanceIconUrl.value = null
	ctx.instanceIconPath.value = null
}

const randomizing = ref(false)
const diceSpinning = ref(false)

async function randomizeIcon() {
	if (!ctx.randomizeInstanceIcon || randomizing.value) return

	randomizing.value = true
	diceSpinning.value = !diceSpinning.value
	try {
		const generated = await ctx.randomizeInstanceIcon()
		if (!generated) return
		ctx.instanceIcon.value = null
		ctx.instanceIconUrl.value = generated.previewUrl
		ctx.instanceIconPath.value = generated.path
	} finally {
		randomizing.value = false
	}
}

const loaderVersionsLoading = ref(false)
const loaderVersionsData = ref<LoaderVersionEntry[]>([])

const paperVersions = ref<Record<string, Paper.Versions.v3.Build[]>>({})
const purpurVersions = ref<Record<string, string[]>>({})

function toApiLoaderName(loader: string): string {
	return loader === 'neoforge' ? 'neo' : loader
}

const gameVersionsLoading = computed(() => {
	if (ctx.projectInstall.value) return false
	const loader = selectedLoader.value
	if (!loader || loader === 'vanilla') return false
	if (loader === 'paper') return ctx.paperSupportedVersions.value === null
	if (loader === 'purpur') return ctx.purpurSupportedVersions.value === null
	return ctx.loaderVersionsCache.value[toApiLoaderName(loader)] === undefined
})

const gameVersionOptions = computed<ComboboxOption<string>[]>(() => {
	if (ctx.projectInstall.value) {
		const versions =
			ctx.showSnapshots.value || ctx.projectInstall.value.releaseGameVersions.size === 0
				? ctx.projectInstall.value.gameVersions
				: ctx.projectInstall.value.gameVersions.filter((version) =>
						ctx.projectInstall.value!.releaseGameVersions.has(version),
					)
		return versions.map((version) => ({ value: version, label: version }))
	}

	const versions = ctx.showSnapshots.value
		? tags.gameVersions.value
		: tags.gameVersions.value.filter((v) => v.version_type === 'release')

	if (selectedLoader.value && selectedLoader.value !== 'vanilla') {
		if (selectedLoader.value === 'paper') {
			if (!ctx.paperSupportedVersions.value) return []
			return versions
				.filter((v) => ctx.paperSupportedVersions.value!.has(v.version))
				.map((v) => ({ value: v.version, label: v.version }))
		}

		if (selectedLoader.value === 'purpur') {
			if (!ctx.purpurSupportedVersions.value) return []
			return versions
				.filter((v) => ctx.purpurSupportedVersions.value!.has(v.version))
				.map((v) => ({ value: v.version, label: v.version }))
		}

		const apiLoader = toApiLoaderName(selectedLoader.value)
		const manifest = ctx.loaderVersionsCache.value[apiLoader]
		if (!manifest) return []

		const isPlaceholder = (id: string) => id.includes('gameVersion}')
		const hasPlaceholder = manifest.gameVersions.some((x) => isPlaceholder(x.id))
		const supportedVersions = new Set(
			manifest.gameVersions
				.filter(
					(x) =>
						!isPlaceholder(x.id) && (hasPlaceholder || x.loaders.length > 0 || !!x.versionGroup),
				)
				.map((x) => x.id),
		)
		return versions
			.filter((v) => supportedVersions.has(v.version))
			.map((v) => ({ value: v.version, label: v.version }))
	}

	return versions.map((v) => ({ value: v.version, label: v.version }))
})

watch(
	gameVersionOptions,
	(options) => {
		if (options.length === 0) {
			selectedGameVersion.value = null
			return
		}
		if (!selectedGameVersion.value || !options.some((o) => o.value === selectedGameVersion.value)) {
			selectedGameVersion.value = options[0].value
		}
	},
	{ immediate: true },
)

async function fetchLoaderManifest(loader: string) {
	const apiLoader = toApiLoaderName(loader)
	debug(
		'fetchLoaderManifest:',
		loader,
		'apiLoader:',
		apiLoader,
		'cached:',
		!!ctx.loaderVersionsCache.value[apiLoader],
	)
	await ctx.fetchLoaderMetadata(loader)
}

async function fetchLoaderMetadata(loader?: string | null) {
	await ctx.fetchLoaderMetadata(loader)
}

function paperBuildChannelTag(buildId: string): 'ALPHA' | 'BETA' | null {
	const gv = selectedGameVersion.value
	if (!gv || selectedLoader.value !== 'paper') return null
	const b = paperVersions.value[gv]?.find((x) => String(x.id) === buildId)
	if (!b) return null
	const u = String(b.channel).toUpperCase()
	if (u === 'ALPHA' || u === 'BETA') return u
	return null
}

async function fetchPaperVersions(mcVersion: string) {
	if (paperVersions.value[mcVersion]) return
	try {
		const data = await client.paper.versions_v3.getBuilds(mcVersion)
		paperVersions.value[mcVersion] = data.builds.toSorted((a, b) => b.id - a.id)
	} catch {
		paperVersions.value[mcVersion] = []
	}
}

function handleGameVersionHover(option: ComboboxOption<string | null>) {
	const v = option.value
	if (v == null || v === '') return
	if (selectedLoader.value === 'paper') void fetchPaperVersions(v)
	else if (selectedLoader.value === 'purpur') void fetchPurpurVersions(v)
}

async function fetchPurpurVersions(mcVersion: string) {
	if (purpurVersions.value[mcVersion]) return
	try {
		const data = await client.purpur.versions_v2.getBuilds(mcVersion)
		purpurVersions.value[mcVersion] = data.builds.all.sort((a, b) => parseInt(b) - parseInt(a))
	} catch {
		purpurVersions.value[mcVersion] = []
	}
}

function getLoaderVersionsForGameVersion(
	loader: string,
	gameVersion: string,
): LoaderVersionEntry[] {
	const apiLoader = toApiLoaderName(loader)
	const manifest = ctx.loaderVersionsCache.value[apiLoader]
	debug('getLoaderVersionsForGameVersion:', {
		loader,
		apiLoader,
		gameVersion,
		hasManifest: !!manifest,
		manifestLength: manifest?.gameVersions.length,
	})
	if (!manifest) return []

	const placeholder = manifest.gameVersions.find((x) => x.id.includes('gameVersion}'))
	if (placeholder) {
		if (!manifest.gameVersions.some((x) => x.id === gameVersion)) return []
		debug(
			'getLoaderVersionsForGameVersion: using placeholder, loaders:',
			placeholder.loaders.length,
		)
		return placeholder.loaders
	}

	const entry = manifest.gameVersions.find((x) => x.id === gameVersion)
	if (entry?.versionGroup) {
		const loaders =
			manifest.versionGroups?.find((group) => group.id === entry.versionGroup)?.loaders ?? []
		debug(
			'getLoaderVersionsForGameVersion: version group for',
			gameVersion,
			':',
			entry.versionGroup,
			loaders.length + ' loaders',
		)
		return loaders
	}

	debug(
		'getLoaderVersionsForGameVersion: entry for',
		gameVersion,
		':',
		entry ? entry.loaders.length + ' loaders' : 'NOT FOUND',
	)
	return entry?.loaders ?? []
}

watch(
	() => selectedLoader.value,
	async (loader) => {
		if (ctx.projectInstall.value) return
		await fetchLoaderMetadata(loader)
	},
	{ immediate: true },
)

let loaderVersionWatchId = 0
watch(
	[() => selectedLoader.value, () => selectedGameVersion.value],
	async ([loader, gameVersion]) => {
		const watchId = ++loaderVersionWatchId
		debug('watch [loader, gameVersion] fired:', { loader, gameVersion, watchId })
		loaderVersionsData.value = []
		selectedLoaderVersion.value = null

		if (ctx.projectInstall.value) return
		if (!loader || !gameVersion || loader === 'vanilla') return

		loaderVersionsLoading.value = true

		if (loader === 'paper') {
			await fetchPaperVersions(gameVersion)
			if (watchId !== loaderVersionWatchId) return
			loaderVersionsLoading.value = false
			const builds = paperVersions.value[gameVersion]
			if (builds?.length) {
				selectedLoaderVersion.value = `${builds[0].id}`
			}
			return
		}

		if (loader === 'purpur') {
			await fetchPurpurVersions(gameVersion)
			if (watchId !== loaderVersionWatchId) return
			loaderVersionsLoading.value = false
			const builds = purpurVersions.value[gameVersion]
			if (builds?.length) {
				selectedLoaderVersion.value = builds[0]
			}
			return
		}

		await fetchLoaderManifest(loader)
		if (watchId !== loaderVersionWatchId) {
			debug('watch [loader, gameVersion]: stale execution, skipping', {
				watchId,
				current: loaderVersionWatchId,
			})
			return
		}
		loaderVersionsData.value = getLoaderVersionsForGameVersion(loader, gameVersion)
		debug(
			'watch [loader, gameVersion]: loaderVersionsData set, count:',
			loaderVersionsData.value.length,
		)
		loaderVersionsLoading.value = false

		autoSelectLoaderVersion()
	},
)

watch(
	() => loaderVersionType.value,
	() => autoSelectLoaderVersion(),
)

function autoSelectLoaderVersion() {
	debug(
		'autoSelectLoaderVersion: type:',
		loaderVersionType.value,
		'dataCount:',
		loaderVersionsData.value.length,
		'stableCount:',
		loaderVersionsData.value.filter((v) => v.stable).length,
		'first:',
		loaderVersionsData.value[0]?.id,
	)
	if (
		loaderVersionType.value === 'stable' &&
		loaderVersionTypeDisabledItems.value.includes('stable')
	) {
		debug("'stable' loader version type is disabled, switching to 'latest'...")
		loaderVersionType.value = 'latest'
	}
	if (loaderVersionType.value === 'stable') {
		const stable = loaderVersionsData.value.find((v) => v.stable)
		selectedLoaderVersion.value = stable?.id ?? loaderVersionsData.value[0]?.id ?? null
	} else if (loaderVersionType.value === 'latest') {
		selectedLoaderVersion.value = loaderVersionsData.value[0]?.id ?? null
	} else if (loaderVersionType.value === 'other' && !selectedLoaderVersion.value) {
		selectedLoaderVersion.value = loaderVersionsData.value[0]?.id ?? null
	}
	debug('autoSelectLoaderVersion: result:', selectedLoaderVersion.value)
}

const loaderVersionOptions = computed<ComboboxOption<string>[]>(() => {
	if (selectedLoader.value === 'paper' && selectedGameVersion.value) {
		const builds = paperVersions.value[selectedGameVersion.value] ?? []
		return builds.map((b) => ({
			value: `${b.id}`,
			label: `Build ${b.id}`,
		}))
	}

	if (selectedLoader.value === 'purpur' && selectedGameVersion.value) {
		const builds = purpurVersions.value[selectedGameVersion.value] ?? []
		return builds.map((b) => ({ value: b, label: `Build ${b}` }))
	}

	return loaderVersionsData.value.map((v) => ({
		value: v.id,
		label: v.stable ? `${v.id} (stable)` : v.id,
	}))
})
</script>
