<template>
	<div class="flex flex-col gap-4 select-none">
		<!-- Section 1: Dropzone Archive & Folder Importer -->
		<div
			class="relative group rounded-2xl border-2 border-dashed transition-all duration-300 p-4 sm:p-5 flex flex-col sm:flex-row items-center justify-between gap-4 backdrop-blur-xl"
			:class="[
				isDragging
					? 'border-indigo-400 bg-indigo-950/40 shadow-[0_0_30px_rgba(99,102,241,0.25)] scale-[1.01]'
					: 'border-white/15 hover:border-indigo-500/40 bg-[#0e131d]/70 hover:bg-[#121826]/80 shadow-md',
			]"
			@dragover.prevent="isDragging = true"
			@dragleave.prevent="isDragging = false"
			@drop.prevent="handleFileDrop"
		>
			<div class="flex items-center gap-3.5 min-w-0">
				<!-- Animated Icon with Glow -->
				<div
					class="size-12 shrink-0 rounded-xl border flex items-center justify-center transition-all duration-300 shadow-inner"
					:class="[
						isDragging
							? 'bg-indigo-500/20 border-indigo-400 text-indigo-300 shadow-[0_0_20px_rgba(99,102,241,0.3)] scale-110'
							: 'bg-white/5 border-white/10 text-indigo-400 group-hover:border-indigo-500/30 group-hover:bg-indigo-500/10',
					]"
				>
					<FileArchiveIcon
						class="size-6 shrink-0 transition-transform duration-300 group-hover:scale-105"
					/>
				</div>

				<div class="flex flex-col gap-0.5 min-w-0">
					<div class="flex items-center gap-2">
						<span class="text-sm font-bold text-white tracking-wide">
							{{ formatMessage(messages.dropzoneTitle) }}
						</span>
						<span
							class="text-[10px] font-mono font-bold px-1.5 py-0.2 rounded bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 uppercase tracking-wider"
						>
							.mrpack / .zip
						</span>
					</div>
					<span class="text-xs text-zinc-400 line-clamp-1">
						{{ formatMessage(messages.dropzoneDescription) }}
					</span>
				</div>
			</div>

			<!-- Action Buttons in Dropzone -->
			<div class="flex items-center gap-2 w-full sm:w-auto shrink-0 justify-end">
				<Button
					type="outlined"
					size="sm"
					class="!rounded-xl border-white/10 hover:border-white/20 bg-white/5 hover:bg-white/10 text-zinc-300 hover:text-white !h-9 text-xs"
					@click="triggerArchivePick"
				>
					<UploadIcon class="size-3.5" />
					{{ formatMessage(messages.pickArchiveButton) }}
				</Button>
				<Button
					type="outlined"
					size="sm"
					class="!rounded-xl border-white/10 hover:border-white/20 bg-white/5 hover:bg-white/10 text-zinc-300 hover:text-white !h-9 text-xs"
					@click="browseForLauncherPath"
				>
					<FolderSearchIcon class="size-3.5" />
					{{ formatMessage(messages.scanFolderButton) }}
				</Button>
			</div>

			<!-- Folder Scan Status Notification Indicator -->
			<div
				v-if="folderScanStatus"
				class="absolute -bottom-3 left-6 right-6 sm:left-auto sm:right-6 flex items-center gap-1.5 px-3 py-1 rounded-full text-[11px] font-mono font-bold border backdrop-blur-md shadow-lg transition-all animate-fadeIn"
				:class="[
					folderScanStatus.type === 'success'
						? 'bg-emerald-950/90 text-emerald-300 border-emerald-500/40 shadow-[0_0_15px_rgba(16,185,129,0.2)]'
						: folderScanStatus.type === 'error'
							? 'bg-rose-950/90 text-rose-300 border-rose-500/40 shadow-[0_0_15px_rgba(244,63,94,0.2)]'
							: 'bg-indigo-950/90 text-indigo-300 border-indigo-500/40 shadow-[0_0_15px_rgba(99,102,241,0.2)]',
				]"
			>
				<SpinnerIcon v-if="folderScanStatus.type === 'scanning'" class="size-3.5 animate-spin" />
				<CheckCircleIcon
					v-else-if="folderScanStatus.type === 'success'"
					class="size-3.5 text-emerald-400"
				/>
				<CircleAlertIcon v-else class="size-3.5 text-rose-400" />
				<span>{{ folderScanStatus.text }}</span>
			</div>
		</div>

		<!-- Section 2: Launcher Source Cards Grid -->
		<div class="flex flex-col gap-2.5 mt-1">
			<div class="flex items-center justify-between">
				<label
					class="text-xs font-bold uppercase tracking-wider text-zinc-300 flex items-center gap-1.5 font-mono"
				>
					<BoxImportIcon class="size-3.5 text-amber-400" />
					{{ formatMessage(messages.launcherSourcesTitle) }}
				</label>
				<span
					v-if="totalDetectedInstances > 0"
					class="text-[11px] font-mono font-bold px-2 py-0.5 rounded-full bg-sky-500/15 text-sky-300 border border-sky-500/30 shadow-[0_0_12px_rgba(56,189,248,0.15)]"
				>
					{{ totalDetectedInstances }} {{ formatMessage(messages.instancesFoundPill) }}
				</span>
			</div>

			<!-- Loading Skeleton -->
			<div
				v-if="loading"
				class="flex items-center justify-center py-10 rounded-2xl bg-[#0e131d]/60 border border-white/10 text-zinc-400 text-sm font-medium gap-3"
			>
				<SpinnerIcon class="size-5 animate-spin text-indigo-400" />
				<span>{{ formatMessage(messages.detectingLauncherInstances) }}</span>
			</div>

			<!-- Selectable Tiles Grid -->
			<div v-else class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-2.5">
				<div
					v-for="launcher in allDisplayLaunchers"
					:key="launcher.id"
					class="group relative flex flex-col justify-between rounded-2xl p-3.5 border transition-all duration-200 cursor-pointer select-none"
					:class="[
						isLauncherActive(launcher.name)
							? 'border-amber-500/60 bg-amber-500/10 shadow-[0_0_20px_rgba(245,158,11,0.15)] ring-1 ring-amber-500/30'
							: launcher.detected
								? 'border-white/10 bg-[#141923] hover:border-white/20 hover:bg-[#18202e] shadow-sm'
								: 'border-white/5 bg-[#0e131d]/50 opacity-60 hover:opacity-100 hover:border-white/10',
					]"
					@click="handleLauncherCardClick(launcher)"
				>
					<!-- Top Row: Logo & Checkbox -->
					<div class="flex items-start justify-between gap-2">
						<div
							class="size-10 rounded-xl border flex items-center justify-center transition-transform group-hover:scale-105 shadow-inner"
							:class="[
								isLauncherActive(launcher.name)
									? 'bg-amber-500/20 border-amber-500/40 text-amber-300'
									: 'bg-white/5 border-white/10 text-zinc-300',
							]"
						>
							<!-- Launcher SVG Logo -->
							<component :is="getLauncherIcon(launcher.id)" class="size-5" />
						</div>

						<Checkbox
							v-if="launcher.detected && launcher.instances.length > 0"
							:model-value="getLauncherCheckState(launcher)"
							:indeterminate="getLauncherIndeterminate(launcher)"
							class="!p-0"
							@update:model-value="toggleLauncherAll(launcher, $event)"
							@click.stop
						/>
					</div>

					<!-- Bottom Row: Title & Instance Count Pill -->
					<div class="flex flex-col gap-1 mt-3">
						<div class="flex items-center justify-between gap-1">
							<span class="text-sm font-bold text-white tracking-wide truncate">
								{{ launcher.displayName }}
							</span>
						</div>

						<div class="flex items-center justify-between text-xs">
							<span
								v-if="launcher.detected && launcher.instances.length > 0"
								class="text-[10px] font-mono font-bold px-2 py-0.5 rounded-md bg-sky-500/15 text-sky-300 border border-sky-500/30"
							>
								{{ launcher.instances.length }} {{ formatMessage(messages.foundCount) }}
							</span>
							<span
								v-else-if="launcher.detected"
								class="text-[10px] font-mono text-zinc-500 px-1.5 py-0.5 rounded bg-white/5 border border-white/5"
							>
								{{ formatMessage(messages.emptyCount) }}
							</span>
							<span
								v-else
								class="text-[10px] font-mono text-zinc-500 px-1.5 py-0.5 rounded bg-white/5 border border-white/5 group-hover:text-indigo-400 group-hover:border-indigo-500/20 transition-colors"
							>
								{{ formatMessage(messages.clickToBrowse) }}
							</span>

							<span
								v-if="launcher.detected && isLauncherSelected(launcher.name)"
								class="text-[10px] font-mono font-bold text-amber-400"
							>
								{{ getSelectedCountForLauncher(launcher.name) }} selected
							</span>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Section 3: Instance Selection Grid / Table -->
		<div v-if="hasDetectedInstances" class="flex flex-col gap-3 mt-1">
			<!-- Header & Search Controls -->
			<div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-2.5">
				<!-- Search Input -->
				<div
					class="relative flex-1 group rounded-xl bg-[#0e131d] border border-white/10 p-0.5 focus-within:border-indigo-500/50 focus-within:shadow-[0_0_15px_rgba(99,102,241,0.2)] transition-all"
				>
					<StyledInput
						v-model="ctx.importSearchQuery.value"
						:icon="SearchIcon"
						:placeholder="formatMessage(messages.searchInstanceNamePlaceholder)"
						class="w-full"
					/>
				</div>

				<!-- Quick Filter Actions -->
				<div class="flex items-center gap-2 justify-between sm:justify-end shrink-0">
					<!-- Active Filter Pill -->
					<div
						class="flex items-center gap-1 bg-[#141923] border border-white/10 rounded-xl px-2.5 py-1 text-xs text-zinc-400 font-mono"
					>
						<span>{{
							activeLauncherFilter === 'all' ? 'All Launchers' : activeLauncherFilter
						}}</span>
						<button
							v-if="activeLauncherFilter !== 'all'"
							class="text-zinc-500 hover:text-white ml-1"
							@click="activeLauncherFilter = 'all'"
						>
							✕
						</button>
					</div>

					<Button
						type="quiet"
						size="xs"
						class="!rounded-xl border border-white/10 bg-white/5 hover:bg-white/10 text-zinc-300 hover:text-white !h-7 text-xs font-mono"
						:class="{ invisible: totalSelectedCount === 0 }"
						@click="clearAll"
					>
						{{ formatMessage(messages.clearAll) }} ({{ totalSelectedCount }})
					</Button>
				</div>
			</div>

			<!-- Instance Table Rows -->
			<div class="flex flex-col gap-1.5 max-h-[320px] overflow-y-auto pr-1">
				<template v-if="filteredDisplayInstances.length > 0">
					<div
						v-for="item in filteredDisplayInstances"
						:key="`${item.launcherName}-${item.instanceName}`"
						class="group flex items-center justify-between gap-3 p-3 rounded-xl border transition-all duration-150 cursor-pointer select-none"
						:class="[
							isInstanceSelected(item.launcherName, item.instanceName)
								? 'bg-indigo-950/30 border-indigo-500/40 shadow-[0_0_15px_rgba(99,102,241,0.1)]'
								: 'bg-[#141923]/90 hover:bg-[#18202e] border-white/5 hover:border-white/15',
						]"
						role="checkbox"
						:aria-checked="isInstanceSelected(item.launcherName, item.instanceName)"
						tabindex="0"
						@click="toggleInstanceRow(item.launcherName, item.instanceName)"
						@keydown.space.prevent="toggleInstanceRow(item.launcherName, item.instanceName)"
						@keydown.enter.prevent="toggleInstanceRow(item.launcherName, item.instanceName)"
					>
						<!-- Left: Checkbox & Name -->
						<div class="flex items-center gap-3 min-w-0">
							<Checkbox
								:model-value="isInstanceSelected(item.launcherName, item.instanceName)"
								class="!p-0 shrink-0"
								@update:model-value="toggleInstance(item.launcherName, item.instanceName, $event)"
								@click.stop
							/>

							<div class="flex flex-col gap-0.5 min-w-0">
								<span
									class="text-sm font-semibold text-white tracking-wide truncate group-hover:text-indigo-300 transition-colors"
								>
									{{ item.instanceName }}
								</span>
								<span class="text-[11px] text-zinc-500 line-clamp-1 font-mono">
									{{ item.launcherName }}
								</span>
							</div>
						</div>

						<!-- Right: Meta Tags -->
						<div class="flex items-center gap-1.5 shrink-0">
							<!-- Memory Tag -->
							<span
								class="hidden sm:flex items-center gap-1 text-[10px] font-mono px-2 py-0.5 rounded-md bg-zinc-800/90 text-zinc-300 border border-white/10"
							>
								<MemoryStickIcon class="size-3 text-zinc-400" />
								{{ item.memoryTag }}
							</span>

							<!-- Version Tag -->
							<span
								class="text-[10px] font-mono font-medium px-2 py-0.5 rounded-md bg-sky-500/10 text-sky-300 border border-sky-500/20"
							>
								{{ item.versionTag }}
							</span>

							<!-- Loader Tag -->
							<span
								v-if="item.loaderTag"
								class="text-[10px] font-mono px-2 py-0.5 rounded-md bg-indigo-500/10 text-indigo-300 border border-indigo-500/20"
							>
								{{ item.loaderTag }}
							</span>
						</div>
					</div>
				</template>

				<!-- Empty State for Search Filter -->
				<div
					v-else
					class="flex flex-col items-center justify-center py-8 rounded-2xl bg-[#0e131d]/60 border border-white/10 text-center gap-2"
				>
					<span class="text-sm font-medium text-zinc-400">
						{{
							formatMessage(messages.noInstancesMatchSearch, { query: ctx.importSearchQuery.value })
						}}
					</span>
					<Button
						type="quiet"
						size="xs"
						class="text-indigo-400 hover:text-indigo-300 text-xs font-mono"
						@click="ctx.importSearchQuery.value = ''"
					>
						{{ formatMessage(messages.resetSearch) }}
					</Button>
				</div>
			</div>
		</div>

		<!-- Section 4: Add Custom Launcher Path Drawer -->
		<div class="flex flex-col gap-2 pt-2 border-t border-white/10">
			<div v-if="!showAddPath" class="flex justify-between items-center">
				<Button
					type="outlined"
					size="sm"
					class="!rounded-xl border-white/10 hover:border-white/20 bg-white/5 hover:bg-white/10 text-zinc-300 hover:text-white !h-8 text-xs font-mono"
					@click="showAddPath = true"
				>
					<FolderSearchIcon class="size-3.5" />
					{{ formatMessage(messages.addLauncherPath) }}
				</Button>
			</div>
			<div
				v-else
				class="flex flex-col sm:flex-row items-stretch sm:items-center gap-2 animate-fadeIn"
			>
				<div class="flex items-center gap-2 flex-1">
					<IconButton
						label="Browse for launcher path"
						class="!rounded-xl border-white/10 hover:border-white/20 bg-white/5"
						@click="browseForLauncherPath"
					>
						<FolderSearchIcon class="size-4" />
					</IconButton>
					<StyledInput
						v-model="newLauncherPath"
						:placeholder="formatMessage(messages.launcherPathPlaceholder)"
						class="flex-1"
					/>
				</div>
				<div class="flex items-center gap-2 justify-end">
					<Button
						size="sm"
						:disabled="!newLauncherPath.trim()"
						class="!rounded-xl !h-9 text-xs"
						@click="addLauncherPath"
					>
						{{ formatMessage(messages.add) }}
					</Button>
					<Button
						type="quiet"
						size="sm"
						class="!rounded-xl !h-9 text-xs text-zinc-400"
						@click="showAddPath = false"
					>
						✕
					</Button>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import {
	BoxImportIcon,
	CheckCircleIcon,
	CircleAlertIcon,
	CurseForgeIcon,
	FileArchiveIcon,
	FolderSearchIcon,
	MemoryStickIcon,
	SearchIcon,
	SpinnerIcon,
	UploadIcon,
} from '@freeplay/assets'
import { defineMessages, useVIntl } from '@freeplay/ui'
import { computed, defineComponent, h, onMounted, ref } from 'vue'

import { Button, IconButton } from '#ui/components/base/buttons'

import {
	injectFilePicker,
	injectInstanceImport,
	injectNotificationManager,
} from '../../../../providers'
import type { ImportableLauncher } from '../../../../providers/instance-import'
import Checkbox from '../../../base/Checkbox.vue'
import StyledInput from '../../../base/StyledInput.vue'
import { injectCreationFlowContext } from '../creation-flow-context'

// Launcher Brand SVG Icon Components
const PrismIcon = defineComponent({
	name: 'PrismIcon',
	render() {
		return h('svg', { viewBox: '0 0 24 24', fill: 'none', xmlns: 'http://www.w3.org/2000/svg' }, [
			h('path', {
				d: 'M12 1L7.2 9.3L12 12.1L16.8 9.3L17 3.5C14.8 2.2 12.5 1 12 1Z',
				fill: '#DF6277',
			}),
			h('path', {
				d: 'M17 3.5L12 12.1L16.8 14.8L21.6 6.5C21.3 6 19.2 4.6 17 3.5Z',
				fill: '#FB9168',
			}),
			h('path', {
				d: 'M21.6 6.5L12 12.1L16.8 14.8L22 12C22 9.5 21.9 7 21.6 6.5Z',
				fill: '#F3DB6C',
			}),
			h('path', { d: 'M12 12.1V17.6H21.6C21.9 17.1 22 14.5 22 12L12 12.1Z', fill: '#7AB392' }),
			h('path', {
				d: 'M12 12.1V17.6L17 20.7C19.2 19.4 21.3 18 21.6 17.6L12 12.1Z',
				fill: '#4B7CBC',
			}),
			h('path', {
				d: 'M12 12.1L7.2 14.8L12 23.1C12.5 23.1 14.8 22 17 20.7L12 12.1Z',
				fill: '#6F488C',
			}),
			h('path', { d: 'M7.2 9.3L2.4 17.6C3 18.5 10.9 23.1 12 23.1V12.1L7.2 9.3Z', fill: '#4D3F33' }),
			h('path', { d: 'M2.4 6.5C1.8 7.5 1.8 16.6 2.4 17.6L12 12.1V6.5L2.4 6.5Z', fill: '#7A573B' }),
			h('path', { d: 'M12 1C10.9 1 3 5.5 2.4 6.5L12 12.1V1Z', fill: '#99CD61' }),
			h('path', {
				d: 'M12 5.6L7.3 15.3L12 20.3L16.7 15.3L12 5.6Z',
				fill: '#FFFFFF',
				'fill-opacity': '0.85',
			}),
		])
	},
})

const MultiMCIcon = defineComponent({
	name: 'MultiMCIcon',
	render() {
		return h('svg', { viewBox: '0 0 24 24', fill: 'none', xmlns: 'http://www.w3.org/2000/svg' }, [
			h('path', { d: 'M12 2L2 7.5L12 13L22 7.5L12 2Z', fill: '#3B82F6' }),
			h('path', { d: 'M2 7.5V16.5L12 22V13L2 7.5Z', fill: '#1D4ED8' }),
			h('path', { d: 'M22 7.5V16.5L12 22V13L22 7.5Z', fill: '#EAB308' }),
			h('path', { d: 'M7 6L12 8.75L17 6L12 3.25L7 6Z', fill: '#60A5FA' }),
			h('path', { d: 'M12 13V18L18 14.7V9.7L12 13Z', fill: '#CA8A04' }),
		])
	},
})

const GDLauncherIcon = defineComponent({
	name: 'GDLauncherIcon',
	render() {
		return h('svg', { viewBox: '0 0 24 24', fill: 'none', xmlns: 'http://www.w3.org/2000/svg' }, [
			h('path', {
				d: 'M12 2L3 7V17L12 22L21 17V7L12 2Z',
				fill: '#0E7490',
				'fill-opacity': '0.3',
				stroke: '#06B6D4',
				'stroke-width': '1.5',
			}),
			h('path', {
				d: 'M12 6L6 9.5V14.5L12 18L18 14.5V11H12V13.5H15.5L12 15.5L8.5 13.5V10.5L12 8.5L15.5 10.5L17.5 9.5L12 6Z',
				fill: '#22D3EE',
			}),
		])
	},
})

const ATLauncherIcon = defineComponent({
	name: 'ATLauncherIcon',
	render() {
		return h('svg', { viewBox: '0 0 24 24', fill: 'none', xmlns: 'http://www.w3.org/2000/svg' }, [
			h('path', {
				d: 'M12 2L2.5 7.5V16.5L12 22L21.5 16.5V7.5L12 2Z',
				fill: '#1E293B',
				stroke: '#64748B',
				'stroke-width': '1.5',
			}),
			h('path', {
				d: 'M12 5.5L6 9V15L12 18.5L18 15V9L12 5.5Z',
				fill: '#84CC16',
				'fill-opacity': '0.2',
			}),
			h('path', {
				d: 'M12 7L7.5 14.5H9.5L10.5 12.5H13.5L14.5 14.5H16.5L12 7ZM11.2 11L12 9.2L12.8 11H11.2Z',
				fill: '#84CC16',
			}),
		])
	},
})

const VanillaMinecraftIcon = defineComponent({
	name: 'VanillaMinecraftIcon',
	render() {
		return h('svg', { viewBox: '0 0 24 24', fill: 'none', xmlns: 'http://www.w3.org/2000/svg' }, [
			h('path', { d: 'M12 2L2 7.5L12 13L22 7.5L12 2Z', fill: '#22C55E' }),
			h('path', { d: 'M2 7.5V16.5L12 22V13L2 7.5Z', fill: '#78350F' }),
			h('path', { d: 'M22 7.5V16.5L12 22V13L22 7.5Z', fill: '#92400E' }),
			h('path', { d: 'M2 7.5L12 13V15.5L2 10V7.5Z', fill: '#16A34A' }),
			h('path', { d: 'M22 7.5L12 13V15.5L22 10V7.5Z', fill: '#15803D' }),
			h('path', { d: 'M4 11L6 12V13.5L4 12.5V11Z', fill: '#16A34A' }),
			h('path', { d: 'M8 13.2L10 14.3V15.8L8 14.7V13.2Z', fill: '#16A34A' }),
			h('path', { d: 'M14 14.3L16 13.2V14.7L14 15.8V14.3Z', fill: '#15803D' }),
			h('path', { d: 'M18 12L20 11V12.5L18 13.5V12Z', fill: '#15803D' }),
		])
	},
})

const ctx = injectCreationFlowContext()
const importProvider = injectInstanceImport()
const filePicker = injectFilePicker()
const { addNotification } = injectNotificationManager()
const { formatMessage } = useVIntl()

const loading = ref(false)
const isDragging = ref(false)
const showAddPath = ref(false)
const newLauncherPath = ref('')
const activeLauncherFilter = ref<string>('all')
const folderScanStatus = ref<{
	text: string
	type: 'scanning' | 'success' | 'error'
} | null>(null)

const messages = defineMessages({
	launcherInstancesTitle: {
		id: 'creation-flow.modal.import-instance.launcher-instances.title',
		defaultMessage: 'Launcher instances',
	},
	launcherSourcesTitle: {
		id: 'creation-flow.modal.import-instance.launcher-sources.title',
		defaultMessage: 'Detected launcher sources',
	},
	dropzoneTitle: {
		id: 'creation-flow.modal.import-instance.dropzone.title',
		defaultMessage: 'Import archive or scan folder',
	},
	dropzoneDescription: {
		id: 'creation-flow.modal.import-instance.dropzone.description',
		defaultMessage: 'Drag and drop .mrpack, .zip or scan custom launcher directory',
	},
	pickArchiveButton: {
		id: 'creation-flow.modal.import-instance.dropzone.pick-archive',
		defaultMessage: 'Import .mrpack / .zip',
	},
	scanFolderButton: {
		id: 'creation-flow.modal.import-instance.dropzone.scan-folder',
		defaultMessage: 'Scan folder...',
	},
	clearAll: {
		id: 'creation-flow.modal.import-instance.selection.clear-all',
		defaultMessage: 'Clear all',
	},
	detectingLauncherInstances: {
		id: 'creation-flow.modal.import-instance.detecting-launcher-instances',
		defaultMessage: 'Detecting launcher instances...',
	},
	searchInstanceNamePlaceholder: {
		id: 'creation-flow.modal.import-instance.search.placeholder',
		defaultMessage: 'Search instance name...',
	},
	noInstancesMatchSearch: {
		id: 'creation-flow.modal.import-instance.search.no-match',
		defaultMessage: 'No instances match "{query}"',
	},
	resetSearch: {
		id: 'creation-flow.modal.import-instance.search.reset',
		defaultMessage: 'Reset search',
	},
	addLauncherPath: {
		id: 'creation-flow.modal.import-instance.launcher-path.add',
		defaultMessage: 'Add custom launcher path',
	},
	launcherPathPlaceholder: {
		id: 'creation-flow.modal.import-instance.launcher-path.placeholder',
		defaultMessage: 'Path to launcher directory...',
	},
	add: {
		id: 'creation-flow.modal.import-instance.action.add',
		defaultMessage: 'Scan & add',
	},
	noInstancesFoundTitle: {
		id: 'creation-flow.modal.import-instance.notification.no-instances-found.title',
		defaultMessage: 'No instances found',
	},
	noInstancesFoundText: {
		id: 'creation-flow.modal.import-instance.notification.no-instances-found.text',
		defaultMessage: 'No importable instances were found at the specified path.',
	},
	customLauncherName: {
		id: 'creation-flow.modal.import-instance.custom-launcher.name',
		defaultMessage: 'Custom ({pathName})',
	},
	instancesFoundPill: {
		id: 'creation-flow.modal.import-instance.instances-found-pill',
		defaultMessage: 'instances detected',
	},
	foundCount: {
		id: 'creation-flow.modal.import-instance.found-count',
		defaultMessage: 'instances',
	},
	emptyCount: {
		id: 'creation-flow.modal.import-instance.empty-count',
		defaultMessage: '0 found',
	},
	clickToBrowse: {
		id: 'creation-flow.modal.import-instance.click-to-browse',
		defaultMessage: 'Click to scan',
	},
})

// Known launcher definitions
interface LauncherCardItem {
	id: string
	name: string
	displayName: string
	detected: boolean
	path?: string
	instances: string[]
}

const KNOWN_LAUNCHERS_CONFIG = [
	{
		id: 'prism',
		name: 'PrismLauncher',
		displayName: 'Prism Launcher',
		keywords: ['prism', 'prismlauncher'],
	},
	{
		id: 'curseforge',
		name: 'Curseforge',
		displayName: 'CurseForge App',
		keywords: ['curseforge', 'curse', 'overwolf'],
	},
	{ id: 'multimc', name: 'MultiMC', displayName: 'MultiMC', keywords: ['multimc', 'mmc'] },
	{
		id: 'gdlauncher',
		name: 'GDLauncher',
		displayName: 'GDLauncher',
		keywords: ['gdlauncher', 'gdl'],
	},
	{
		id: 'atlauncher',
		name: 'ATLauncher',
		displayName: 'ATLauncher',
		keywords: ['atlauncher', 'atl'],
	},
	{
		id: 'vanilla',
		name: 'Vanilla Minecraft',
		displayName: 'Vanilla Minecraft',
		keywords: ['vanilla', 'minecraft', 'official'],
	},
]

function getLauncherIcon(id: string) {
	switch (id) {
		case 'prism':
			return PrismIcon
		case 'curseforge':
			return CurseForgeIcon
		case 'multimc':
			return MultiMCIcon
		case 'gdlauncher':
			return GDLauncherIcon
		case 'atlauncher':
			return ATLauncherIcon
		case 'vanilla':
			return VanillaMinecraftIcon
		default:
			return FolderSearchIcon
	}
}

// Map detected launchers + placeholder known launchers
const allDisplayLaunchers = computed<LauncherCardItem[]>(() => {
	const detected = ctx.importLaunchers.value
	const result: LauncherCardItem[] = []

	for (const config of KNOWN_LAUNCHERS_CONFIG) {
		const found = detected.find((l) =>
			config.keywords.some((k) => l.name.toLowerCase().includes(k)),
		)
		if (found) {
			result.push({
				id: config.id,
				name: found.name,
				displayName: config.displayName,
				detected: true,
				path: found.path,
				instances: found.instances,
			})
		} else {
			result.push({
				id: config.id,
				name: config.name,
				displayName: config.displayName,
				detected: false,
				instances: [],
			})
		}
	}

	for (const l of detected) {
		const isMatched = KNOWN_LAUNCHERS_CONFIG.some((config) =>
			config.keywords.some((k) => l.name.toLowerCase().includes(k)),
		)
		if (!isMatched) {
			result.push({
				id: 'custom',
				name: l.name,
				displayName: l.name,
				detected: true,
				path: l.path,
				instances: l.instances,
			})
		}
	}

	return result
})

const totalDetectedInstances = computed(() => {
	return ctx.importLaunchers.value.reduce((acc, l) => acc + l.instances.length, 0)
})

const hasDetectedInstances = computed(() => totalDetectedInstances.value > 0)

onMounted(async () => {
	if (ctx.importLaunchers.value.length > 0) return

	loading.value = true
	try {
		ctx.importLaunchers.value = await importProvider.getDetectedLaunchers()
		const firstWithInstances = ctx.importLaunchers.value.find((l) => l.instances.length > 0)
		if (firstWithInstances) {
			activeLauncherFilter.value = 'all'
		}
	} catch {
		ctx.importLaunchers.value = []
	}
	loading.value = false
})

function extractVersion(name: string): string {
	const match = name.match(/\b1\.\d+(\.\d+)?\b/)
	return match ? match[0] : '1.20.x'
}

function extractLoader(name: string): string | null {
	const lower = name.toLowerCase()
	if (lower.includes('fabric')) return 'Fabric'
	if (lower.includes('neoforge') || lower.includes('neo forge')) return 'NeoForge'
	if (lower.includes('forge')) return 'Forge'
	if (lower.includes('quilt')) return 'Quilt'
	if (lower.includes('optifine')) return 'OptiFine'
	return null
}

function extractMemory(name: string): string {
	const match = name.match(/\b(\d+)\s*(g|gb|gi|gib)\b/i)
	return match ? `${match[1]} GB RAM` : '4 GB RAM'
}

interface FlattenedInstance {
	launcherName: string
	instanceName: string
	versionTag: string
	loaderTag: string | null
	memoryTag: string
}

const filteredDisplayInstances = computed<FlattenedInstance[]>(() => {
	const query = ctx.importSearchQuery.value.toLowerCase().trim()
	const activeFilter = activeLauncherFilter.value
	const list: FlattenedInstance[] = []

	for (const launcher of ctx.importLaunchers.value) {
		if (activeFilter !== 'all' && launcher.name !== activeFilter) continue

		for (const instance of launcher.instances) {
			if (query && !instance.toLowerCase().includes(query)) continue

			list.push({
				launcherName: launcher.name,
				instanceName: instance,
				versionTag: extractVersion(instance),
				loaderTag: extractLoader(instance),
				memoryTag: extractMemory(instance),
			})
		}
	}

	return list
})

function isLauncherActive(launcherName: string): boolean {
	return activeLauncherFilter.value === launcherName
}

function isLauncherSelected(launcherName: string): boolean {
	const set = ctx.importSelectedInstances.value[launcherName]
	return !!set && set.size > 0
}

function getSelectedCountForLauncher(launcherName: string): number {
	return ctx.importSelectedInstances.value[launcherName]?.size ?? 0
}

function handleLauncherCardClick(launcher: LauncherCardItem) {
	if (!launcher.detected) {
		void browseForSpecificLauncher(launcher)
		return
	}

	if (activeLauncherFilter.value === launcher.name) {
		activeLauncherFilter.value = 'all'
	} else {
		activeLauncherFilter.value = launcher.name
	}
}

async function browseForSpecificLauncher(launcher: LauncherCardItem) {
	const path = await importProvider.selectDirectory()
	if (!path) return

	await scanAndAddPath(path, launcher.displayName)
}

function isInstanceSelected(launcherName: string, instance: string): boolean {
	return ctx.importSelectedInstances.value[launcherName]?.has(instance) ?? false
}

function toggleInstance(launcherName: string, instance: string, selected: boolean) {
	if (!ctx.importSelectedInstances.value[launcherName]) {
		ctx.importSelectedInstances.value[launcherName] = new Set()
	}
	if (selected) {
		ctx.importSelectedInstances.value[launcherName].add(instance)
	} else {
		ctx.importSelectedInstances.value[launcherName].delete(instance)
	}
	ctx.importSelectedInstances.value = { ...ctx.importSelectedInstances.value }
}

function toggleInstanceRow(launcherName: string, instance: string) {
	const currentlySelected = isInstanceSelected(launcherName, instance)
	toggleInstance(launcherName, instance, !currentlySelected)
}

function getLauncherCheckState(launcher: { name: string; instances: string[] }): boolean {
	const set = ctx.importSelectedInstances.value[launcher.name]
	if (!set || set.size === 0) return false
	return launcher.instances.length > 0 && launcher.instances.every((i) => set.has(i))
}

function getLauncherIndeterminate(launcher: { name: string; instances: string[] }): boolean {
	const set = ctx.importSelectedInstances.value[launcher.name]
	if (!set || set.size === 0) return false
	const selected = launcher.instances.filter((i) => set.has(i))
	return selected.length > 0 && selected.length < launcher.instances.length
}

function toggleLauncherAll(launcher: { name: string; instances: string[] }, selected: boolean) {
	if (!ctx.importSelectedInstances.value[launcher.name]) {
		ctx.importSelectedInstances.value[launcher.name] = new Set()
	}
	for (const instance of launcher.instances) {
		if (selected) {
			ctx.importSelectedInstances.value[launcher.name].add(instance)
		} else {
			ctx.importSelectedInstances.value[launcher.name].delete(instance)
		}
	}
	ctx.importSelectedInstances.value = { ...ctx.importSelectedInstances.value }
}

const totalSelectedCount = computed(() => {
	let count = 0
	for (const set of Object.values(ctx.importSelectedInstances.value)) {
		count += set.size
	}
	return count
})

function clearAll() {
	ctx.importSelectedInstances.value = {}
}

async function triggerArchivePick() {
	if (ctx.finishDisabled.value) return

	const picked = await filePicker.pickModpackFile({
		readFile: ctx.flowType !== 'instance',
	})
	if (!picked) return

	ctx.setupType.value = 'modpack'
	ctx.modpackFile.value = picked.file ?? null
	ctx.modpackFilePath.value = picked.path ?? null
	ctx.finish()
}

async function handleFileDrop(e: DragEvent) {
	isDragging.value = false
	const files = e.dataTransfer?.files
	if (!files || files.length === 0) return

	const file = files[0]
	const name = file.name.toLowerCase()

	if (name.endsWith('.mrpack') || name.endsWith('.zip')) {
		ctx.setupType.value = 'modpack'
		ctx.modpackFile.value = file
		ctx.modpackFilePath.value = (file as File & { path?: string }).path ?? null
		ctx.finish()
	} else if ((file as File & { path?: string }).path) {
		const filePath = (file as File & { path?: string }).path
		if (filePath) {
			await scanAndAddPath(filePath)
		}
	}
}

async function browseForLauncherPath() {
	const path = await importProvider.selectDirectory()
	if (path) {
		newLauncherPath.value = path
		await scanAndAddPath(path)
	}
}

async function scanAndAddPath(path: string, customTitle?: string) {
	folderScanStatus.value = {
		text: 'Scanning folder for instances...',
		type: 'scanning',
	}

	try {
		const instances = await importProvider.getImportableInstances('Custom', path)
		if (instances.length === 0) {
			folderScanStatus.value = {
				text: 'No instances found in folder',
				type: 'error',
			}
			addNotification({
				type: 'error',
				title: formatMessage(messages.noInstancesFoundTitle),
				text: formatMessage(messages.noInstancesFoundText),
			})
			setTimeout(() => {
				folderScanStatus.value = null
			}, 4000)
			return
		}

		const displayName =
			customTitle ||
			formatMessage(messages.customLauncherName, {
				pathName: path.split(/[\\/]/).pop() || path,
			})

		const existing = ctx.importLaunchers.value.find((l) => l.path === path)
		if (!existing) {
			const launcher: ImportableLauncher = {
				name: displayName,
				path,
				instances,
			}
			ctx.importLaunchers.value = [...ctx.importLaunchers.value, launcher]
		}

		activeLauncherFilter.value = displayName
		folderScanStatus.value = {
			text: `✓ Found ${instances.length} instances`,
			type: 'success',
		}
		newLauncherPath.value = ''
		showAddPath.value = false

		setTimeout(() => {
			folderScanStatus.value = null
		}, 4000)
	} catch {
		folderScanStatus.value = {
			text: 'Failed to scan directory',
			type: 'error',
		}
		addNotification({
			type: 'error',
			title: formatMessage(messages.noInstancesFoundTitle),
			text: formatMessage(messages.noInstancesFoundText),
		})
		setTimeout(() => {
			folderScanStatus.value = null
		}, 4000)
	}
}

async function addLauncherPath() {
	const path = newLauncherPath.value.trim()
	if (!path) return

	await scanAndAddPath(path)
}
</script>

<style scoped>
@keyframes fadeIn {
	from {
		opacity: 0;
		transform: translateY(-4px);
	}
	to {
		opacity: 1;
		transform: translateY(0);
	}
}

.animate-fadeIn {
	animation: fadeIn 0.2s ease-out;
}
</style>
