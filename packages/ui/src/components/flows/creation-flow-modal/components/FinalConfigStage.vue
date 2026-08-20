<template>
	<div class="flex flex-col gap-4 select-none">
		<!-- Summary Hero Card: Instance/Server Preview Card -->
		<div
			class="flex items-center gap-3.5 rounded-2xl border border-white/10 bg-[#141923] p-3.5 shadow-md"
		>
			<!-- Preview Thumbnail -->
			<div
				class="size-14 shrink-0 overflow-hidden rounded-xl border border-white/10 bg-[#0e131d] flex items-center justify-center shadow-inner"
			>
				<Avatar
					v-if="heroIconUrl"
					:src="heroIconUrl"
					:alt="heroTitle"
					size="100%"
					class="!rounded-xl object-cover"
					no-shadow
				/>
				<component :is="heroLoaderIcon" v-else class="size-7 text-sky-400" />
			</div>

			<!-- Summary Info Body -->
			<div class="flex flex-1 flex-col gap-1 min-w-0">
				<div class="flex items-center gap-2">
					<span class="font-bold text-white text-sm tracking-wide truncate">
						{{ heroTitle }}
					</span>
					<span
						class="text-[10px] font-mono font-bold px-2 py-0.5 rounded bg-sky-500/20 text-sky-300 border border-sky-500/30 uppercase tracking-wider shrink-0"
					>
						{{ heroTypeBadge }}
					</span>
				</div>

				<div class="flex flex-wrap items-center gap-2 text-xs text-zinc-400 font-mono">
					<!-- Loader Badge -->
					<div
						class="flex items-center gap-1.5 px-2 py-0.5 rounded bg-white/5 border border-white/10 text-zinc-300"
					>
						<component :is="heroLoaderIcon" class="size-3 text-sky-400" />
						<span>{{ heroLoaderName }}</span>
					</div>

					<!-- Game Version Badge -->
					<div
						v-if="ctx.selectedGameVersion.value"
						class="flex items-center gap-1 px-2 py-0.5 rounded bg-white/5 border border-white/10 text-zinc-300"
					>
						<span>MC {{ ctx.selectedGameVersion.value }}</span>
					</div>

					<!-- Status Badge -->
					<div
						class="flex items-center gap-1 px-2 py-0.5 rounded bg-sky-500/10 border border-sky-500/20 text-sky-400 font-semibold text-[10px]"
					>
						<span class="size-1.5 rounded-full bg-sky-400 animate-pulse"></span>
						<span>Config Ready</span>
					</div>
				</div>
			</div>
		</div>

		<!-- World / Server Name Field (When not server-onboarding / reset-server) -->
		<div
			v-if="ctx.flowType !== 'server-onboarding' && ctx.flowType !== 'reset-server'"
			class="flex flex-col gap-2 rounded-2xl border border-white/10 bg-[#141923] p-3.5 shadow-md"
		>
			<div class="flex items-center justify-between">
				<label
					class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono flex items-center gap-1.5"
				>
					<SparklesIcon class="size-3.5 text-sky-400" />
					<span>{{ formatMessage(messages.worldNameLabel) }}</span>
				</label>
				<span
					class="text-[10px] font-mono font-medium px-2 py-0.5 rounded-md bg-white/5 text-zinc-400 border border-white/10"
				>
					Required
				</span>
			</div>
			<div
				class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-sky-500/50 focus-within:shadow-[0_0_15px_rgba(56,189,248,0.2)] transition-all duration-200"
			>
				<StyledInput
					v-model="worldName"
					:placeholder="formatMessage(messages.worldNamePlaceholder)"
					wrapper-class="w-full !bg-transparent"
					input-class="!bg-transparent !text-white !placeholder-zinc-500 font-medium text-sm"
				/>
			</div>
		</div>

		<!-- Vanilla Flow: Game Version Picker -->
		<div v-if="ctx.setupType.value === 'vanilla'" class="flex flex-col gap-2">
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
					searchable
					sync-with-selection
					show-search-icon
					:placeholder="formatMessage(messages.gameVersionPlaceholder)"
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

		<!-- Gamemode Selector: Bento Pills -->
		<div class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<label class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono">
					{{ formatMessage(messages.gamemodeLabel) }}
				</label>
				<span
					class="text-[10px] font-mono font-medium px-2 py-0.5 rounded-md bg-white/5 text-zinc-400 border border-white/10"
				>
					Mode
				</span>
			</div>
			<div class="grid grid-cols-3 gap-2">
				<button
					v-for="item in gamemodeItems"
					:key="item"
					type="button"
					class="flex flex-col items-center justify-center gap-1.5 py-2.5 px-3 rounded-xl border text-xs font-bold tracking-wide transition-all active:scale-95 cursor-pointer select-none"
					:class="[
						gamemode === item
							? item === 'hardcore'
								? 'border-red-500/60 bg-red-500/10 text-red-300 shadow-[0_0_15px_rgba(239,68,68,0.2)] ring-1 ring-red-500/40'
								: 'border-sky-500/60 bg-sky-500/10 text-sky-300 shadow-[0_0_15px_rgba(56,189,248,0.2)] ring-1 ring-sky-500/40'
							: 'border-white/10 bg-[#141923] hover:bg-[#18202e] hover:border-white/20 text-zinc-400 hover:text-zinc-200',
					]"
					@click="gamemode = item"
				>
					<span :class="gamemode === item ? 'text-white' : 'text-zinc-300'">
						{{ formatGamemodeLabel(item) }}
					</span>
				</button>
			</div>
		</div>

		<!-- Difficulty Selector: Bento Pills (when not hardcore) -->
		<div v-if="gamemode !== 'hardcore'" class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<label class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono">
					{{ formatMessage(messages.difficultyLabel) }}
				</label>
				<span
					class="text-[10px] font-mono font-medium px-2 py-0.5 rounded-md bg-white/5 text-zinc-400 border border-white/10"
				>
					Difficulty
				</span>
			</div>
			<div class="grid grid-cols-2 gap-2 sm:grid-cols-4">
				<button
					v-for="item in difficultyItems"
					:key="item"
					type="button"
					class="flex items-center justify-center py-2 px-2.5 rounded-xl border text-xs font-bold tracking-wide transition-all active:scale-95 cursor-pointer select-none"
					:class="[
						difficulty === item
							? 'border-sky-500/60 bg-sky-500/10 text-sky-300 shadow-[0_0_15px_rgba(56,189,248,0.2)] ring-1 ring-sky-500/40'
							: 'border-white/10 bg-[#141923] hover:bg-[#18202e] hover:border-white/20 text-zinc-400 hover:text-zinc-200',
					]"
					@click="difficulty = item"
				>
					<span>{{ formatDifficultyLabel(item) }}</span>
				</button>
			</div>
		</div>

		<!-- World Type Dropdown -->
		<div class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<label class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono">
					{{ formatMessage(messages.worldTypeLabel) }}
				</label>
			</div>
			<div
				class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-sky-500/50 focus-within:shadow-[0_0_15px_rgba(56,189,248,0.2)] transition-all duration-200"
			>
				<Combobox
					v-model="worldTypeOption"
					:options="worldTypeOptions"
					:placeholder="formatMessage(messages.worldTypePlaceholder)"
				/>
			</div>
		</div>

		<!-- World Seed Field -->
		<div class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<label
					class="text-xs font-bold uppercase tracking-wider text-zinc-300 font-mono flex items-center gap-1.5"
				>
					<IntlFormatted :message-id="messages.worldSeedLabelWithOptional">
						<template #optional="{ children }">
							<span class="text-zinc-500 font-normal lowercase">
								<component :is="() => children" />
							</span>
						</template>
					</IntlFormatted>
				</label>
				<span class="text-[10px] text-zinc-500 font-mono">
					{{ formatMessage(messages.worldSeedDescription) }}
				</span>
			</div>
			<div
				class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-sky-500/50 focus-within:shadow-[0_0_15px_rgba(56,189,248,0.2)] transition-all duration-200"
			>
				<StyledInput
					v-model="worldSeed"
					:placeholder="formatMessage(messages.worldSeedPlaceholder)"
					wrapper-class="w-full !bg-transparent"
					input-class="!bg-transparent !text-white !placeholder-zinc-500 font-mono text-sm"
				/>
			</div>
		</div>

		<!-- Styled Divider -->
		<div class="relative flex items-center justify-center my-1">
			<div class="absolute inset-0 flex items-center">
				<div class="w-full border-t border-white/10"></div>
			</div>
		</div>

		<!-- Additional Settings Accordion -->
		<div class="rounded-2xl border border-white/10 bg-[#141923] p-3.5 shadow-md">
			<Accordion overflow-visible button-class="w-full bg-transparent m-0 p-0 border-none">
				<template #title>
					<div class="flex items-center gap-2">
						<SettingsIcon class="size-4 shrink-0 text-sky-400" />
						<span class="font-bold text-white text-sm font-mono uppercase tracking-wider">
							{{ formatMessage(messages.additionalSettingsTitle) }}
						</span>
					</div>
				</template>
				<div class="flex flex-col gap-4 pt-4 border-t border-white/10 mt-3">
					<!-- Generate Structures Toggle -->
					<div
						class="flex w-full flex-row items-center justify-between gap-4 p-3 rounded-xl bg-[#0e131d] border border-white/10"
					>
						<div class="flex flex-col gap-0.5">
							<span class="font-bold text-white text-xs">
								{{ formatMessage(messages.generateStructuresLabel) }}
							</span>
							<span class="text-xs text-zinc-400">
								{{ formatMessage(messages.generateStructuresDescription) }}
							</span>
						</div>
						<Toggle v-model="generateStructures" small class="shrink-0" />
					</div>

					<!-- Generator Settings -->
					<div class="flex flex-col gap-2">
						<span class="font-bold text-white text-xs font-mono uppercase tracking-wider">
							{{ formatMessage(messages.generatorSettingsLabel) }}
						</span>
						<div
							class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-sky-500/50 transition-all duration-200"
						>
							<Combobox
								v-model="generatorSettingsMode"
								:options="generatorSettingsOptions"
								:placeholder="formatMessage(messages.generatorSettingsPlaceholder)"
							/>
						</div>
						<div
							v-if="generatorSettingsMode === 'custom'"
							class="relative group rounded-xl bg-[#0e131d] border border-white/10 p-2 focus-within:border-sky-500/50 transition-all duration-200 mt-1"
						>
							<StyledInput
								v-model="generatorSettingsCustom"
								multiline
								:rows="4"
								:placeholder="formatMessage(messages.generatorSettingsJsonPlaceholder)"
								wrapper-class="w-full !bg-transparent"
								input-class="!bg-transparent !text-white !placeholder-zinc-500 font-mono text-xs"
							/>
						</div>
						<span class="text-xs text-zinc-500">
							{{ formatMessage(messages.generatorSettingsDescription) }}
						</span>
					</div>
				</div>
			</Accordion>
		</div>

		<!-- Reset Server Flow Inline Backup -->
		<InlineBackupCreator
			v-if="ctx.flowType === 'reset-server'"
			ref="backupCreator"
			:backup-name="formatMessage(messages.beforeResetServerBackupName)"
			hide-shift-click-hint
			@update:buttons-disabled="ctx.isBackingUp.value = $event"
		/>
	</div>
</template>

<script setup lang="ts">
import {
	BoxesIcon,
	EyeIcon,
	EyeOffIcon,
	loaderIconMap,
	SettingsIcon,
	SparklesIcon,
} from '@freeplay/assets'
import { commonMessages, defineMessages, IntlFormatted, useVIntl } from '@freeplay/ui'
import { computed, ref, watch } from 'vue'

import { useDebugLogger } from '#ui/composables/debug-logger'

import InlineBackupCreator from '../../../../layouts/shared/content-tab/components/modals/InlineBackupCreator.vue'
import { injectTags } from '../../../../providers'
import Accordion from '../../../base/Accordion.vue'
import Avatar from '../../../base/Avatar.vue'
import Combobox, { type ComboboxOption } from '../../../base/Combobox.vue'
import StyledInput from '../../../base/StyledInput.vue'
import Toggle from '../../../base/Toggle.vue'
import type { Difficulty, Gamemode, GeneratorSettingsMode } from '../creation-flow-context'
import { injectCreationFlowContext } from '../creation-flow-context'
import { formatLoaderLabel } from '../shared'

const debug = useDebugLogger('FinalConfigStage')
const ctx = injectCreationFlowContext()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	worldNameLabel: {
		id: 'creation-flow.modal.final-config.world-name.label',
		defaultMessage: 'World name',
	},
	worldNamePlaceholder: {
		id: 'creation-flow.modal.final-config.world-name.placeholder',
		defaultMessage: 'Enter world name',
	},
	gameVersionPlaceholder: {
		id: 'creation-flow.modal.final-config.game-version.placeholder',
		defaultMessage: 'Select game version',
	},
	gamemodeLabel: {
		id: 'creation-flow.modal.final-config.gamemode.label',
		defaultMessage: 'Gamemode',
	},
	gamemodeSurvival: {
		id: 'creation-flow.modal.final-config.gamemode.survival',
		defaultMessage: 'Survival',
	},
	gamemodeCreative: {
		id: 'creation-flow.modal.final-config.gamemode.creative',
		defaultMessage: 'Creative',
	},
	gamemodeHardcore: {
		id: 'creation-flow.modal.final-config.gamemode.hardcore',
		defaultMessage: 'Hardcore',
	},
	difficultyLabel: {
		id: 'creation-flow.modal.final-config.difficulty.label',
		defaultMessage: 'Difficulty',
	},
	difficultyPeaceful: {
		id: 'creation-flow.modal.final-config.difficulty.peaceful',
		defaultMessage: 'Peaceful',
	},
	difficultyEasy: {
		id: 'creation-flow.modal.final-config.difficulty.easy',
		defaultMessage: 'Easy',
	},
	difficultyNormal: {
		id: 'creation-flow.modal.final-config.difficulty.normal',
		defaultMessage: 'Normal',
	},
	difficultyHard: {
		id: 'creation-flow.modal.final-config.difficulty.hard',
		defaultMessage: 'Hard',
	},
	worldTypeLabel: {
		id: 'creation-flow.modal.final-config.world-type.label',
		defaultMessage: 'World type',
	},
	worldTypePlaceholder: {
		id: 'creation-flow.modal.final-config.world-type.placeholder',
		defaultMessage: 'Select world type',
	},
	worldTypeDefault: {
		id: 'creation-flow.modal.final-config.world-type.default',
		defaultMessage: 'Default',
	},
	worldTypeSuperflat: {
		id: 'creation-flow.modal.final-config.world-type.superflat',
		defaultMessage: 'Superflat',
	},
	worldTypeLargeBiomes: {
		id: 'creation-flow.modal.final-config.world-type.large-biomes',
		defaultMessage: 'Large Biomes',
	},
	worldTypeAmplified: {
		id: 'creation-flow.modal.final-config.world-type.amplified',
		defaultMessage: 'Amplified',
	},
	worldTypeSingleBiome: {
		id: 'creation-flow.modal.final-config.world-type.single-biome',
		defaultMessage: 'Single Biome',
	},
	worldSeedLabelWithOptional: {
		id: 'creation-flow.modal.final-config.world-seed.label-with-optional',
		defaultMessage: 'World seed <optional>(Optional)</optional>',
	},
	worldSeedPlaceholder: {
		id: 'creation-flow.modal.final-config.world-seed.placeholder',
		defaultMessage: 'Enter world seed',
	},
	worldSeedDescription: {
		id: 'creation-flow.modal.final-config.world-seed.description',
		defaultMessage: 'Leave blank for a random seed.',
	},
	additionalSettingsTitle: {
		id: 'creation-flow.modal.final-config.additional-settings.title',
		defaultMessage: 'Additional settings',
	},
	generateStructuresLabel: {
		id: 'creation-flow.modal.final-config.generate-structures.label',
		defaultMessage: 'Generate structures',
	},
	generateStructuresDescription: {
		id: 'creation-flow.modal.final-config.generate-structures.description',
		defaultMessage:
			'Controls whether villages, strongholds, and other structures generate in new chunks.',
	},
	generatorSettingsLabel: {
		id: 'creation-flow.modal.final-config.generator-settings.label',
		defaultMessage: 'Generator settings',
	},
	generatorSettingsPlaceholder: {
		id: 'creation-flow.modal.final-config.generator-settings.placeholder',
		defaultMessage: 'Select generator settings',
	},
	generatorSettingsDefault: {
		id: 'creation-flow.modal.final-config.generator-settings.default',
		defaultMessage: 'Default',
	},
	generatorSettingsFlat: {
		id: 'creation-flow.modal.final-config.generator-settings.flat',
		defaultMessage: 'Flat',
	},
	generatorSettingsCustom: {
		id: 'creation-flow.modal.final-config.generator-settings.custom',
		defaultMessage: 'Custom',
	},
	generatorSettingsJsonPlaceholder: {
		id: 'creation-flow.modal.final-config.generator-settings-json.placeholder',
		defaultMessage: 'Enter generator settings JSON',
	},
	generatorSettingsDescription: {
		id: 'creation-flow.modal.final-config.generator-settings.description',
		defaultMessage: 'Used for advanced world customization such as custom Superflat layers.',
	},
	beforeResetServerBackupName: {
		id: 'creation-flow.modal.final-config.backup.before-reset-server.name',
		defaultMessage: 'Before reset server',
	},
})

const backupCreator = ref<InstanceType<typeof InlineBackupCreator> | null>(null)
watch(backupCreator, (creator) => {
	ctx.cancelBackup.value = creator?.cancelBackup ?? null
})
const {
	worldName,
	gamemode,
	difficulty,
	worldTypeOption,
	worldSeed,
	generateStructures,
	generatorSettingsMode,
	generatorSettingsCustom,
	selectedGameVersion,
} = ctx

debug(
	'mounted, setupType:',
	ctx.setupType.value,
	'loader:',
	ctx.selectedLoader.value,
	'gameVersion:',
	ctx.selectedGameVersion.value,
	'loaderVersion:',
	ctx.selectedLoaderVersion.value,
)

// Hero summary computed helpers
const heroTitle = computed(() => {
	if (worldName.value) return worldName.value
	if (ctx.instanceName.value) return ctx.instanceName.value
	if (ctx.projectInstall.value?.title) return ctx.projectInstall.value.title
	if (ctx.modpackSelection.value?.name) return ctx.modpackSelection.value.name
	return ctx.flowType === 'world' ? 'New World' : 'Minecraft Profile'
})

const heroIconUrl = computed(() => {
	return (
		ctx.instanceIconUrl.value ||
		ctx.projectInstall.value?.iconUrl ||
		ctx.modpackSelection.value?.iconUrl ||
		undefined
	)
})

const heroLoaderName = computed(() => {
	if (ctx.selectedLoader.value) return formatLoaderLabel(ctx.selectedLoader.value)
	if (ctx.setupType.value === 'vanilla') return 'Vanilla'
	return 'Vanilla'
})

const heroLoaderIcon = computed(() => {
	const loader = ctx.selectedLoader.value || 'vanilla'
	return loaderIconMap[loader] || BoxesIcon
})

const heroTypeBadge = computed(() => {
	if (ctx.flowType === 'world') return 'World'
	if (ctx.flowType === 'server-onboarding') return 'Server'
	if (ctx.flowType === 'reset-server') return 'Reset'
	return 'Instance'
})

// Game version options for vanilla flow
const tags = injectTags()
const gameVersionOptions = computed<ComboboxOption<string>[]>(() => {
	const versions = ctx.showSnapshots.value
		? tags.gameVersions.value
		: tags.gameVersions.value.filter((v) => v.version_type === 'release')
	return versions.map((v) => ({ value: v.version, label: v.version }))
})

// Auto-select latest game version for vanilla
watch(
	gameVersionOptions,
	(options) => {
		if (!selectedGameVersion.value && options.length > 0) {
			selectedGameVersion.value = options[0].value
		}
	},
	{ immediate: true },
)

// Hardcore locks difficulty to hard
let previousDifficulty: Difficulty = difficulty.value
watch(gamemode, (mode) => {
	if (mode === 'hardcore') {
		previousDifficulty = difficulty.value
		difficulty.value = 'hard'
	} else {
		difficulty.value = previousDifficulty
	}
})

const gamemodeItems: Gamemode[] = ['survival', 'creative', 'hardcore']
const difficultyItems: Difficulty[] = ['peaceful', 'easy', 'normal', 'hard']

function formatGamemodeLabel(mode: Gamemode): string {
	switch (mode) {
		case 'survival':
			return formatMessage(messages.gamemodeSurvival)
		case 'creative':
			return formatMessage(messages.gamemodeCreative)
		case 'hardcore':
			return formatMessage(messages.gamemodeHardcore)
	}
}

function formatDifficultyLabel(value: Difficulty): string {
	switch (value) {
		case 'peaceful':
			return formatMessage(messages.difficultyPeaceful)
		case 'easy':
			return formatMessage(messages.difficultyEasy)
		case 'normal':
			return formatMessage(messages.difficultyNormal)
		case 'hard':
			return formatMessage(messages.difficultyHard)
	}
}

const worldTypeOptions = computed<ComboboxOption<string>[]>(() => [
	{ value: 'minecraft:normal', label: formatMessage(messages.worldTypeDefault) },
	{ value: 'minecraft:flat', label: formatMessage(messages.worldTypeSuperflat) },
	{ value: 'minecraft:large_biomes', label: formatMessage(messages.worldTypeLargeBiomes) },
	{ value: 'minecraft:amplified', label: formatMessage(messages.worldTypeAmplified) },
	{ value: 'minecraft:single_biome_surface', label: formatMessage(messages.worldTypeSingleBiome) },
])

const generatorSettingsOptions = computed<ComboboxOption<GeneratorSettingsMode>[]>(() => [
	{ value: 'default', label: formatMessage(messages.generatorSettingsDefault) },
	{ value: 'flat', label: formatMessage(messages.generatorSettingsFlat) },
	{ value: 'custom', label: formatMessage(messages.generatorSettingsCustom) },
])
</script>
