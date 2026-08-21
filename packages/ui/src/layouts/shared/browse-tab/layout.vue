<script setup lang="ts">
import type { Labrinth } from '@freeplay/api-client'
import { RotateCounterClockwiseIcon, SearchIcon } from '@freeplay/assets'
import { computed, ref, toValue } from 'vue'

import Admonition from '#ui/components/base/Admonition.vue'
import { Button, IconButton } from '#ui/components/base/buttons'
import Combobox, { type ComboboxOption } from '#ui/components/base/Combobox.vue'
import NavTabs from '#ui/components/base/NavTabs.vue'
import Pagination from '#ui/components/base/Pagination.vue'
import StyledInput from '#ui/components/base/StyledInput.vue'
import Toggle from '#ui/components/base/Toggle.vue'
import ProjectCard from '#ui/components/project/card/ProjectCard.vue'
import ProjectCardSkeleton from '#ui/components/project/card/ProjectCardSkeleton.vue'
import ProjectCardList from '#ui/components/project/ProjectCardList.vue'
import SearchFilterControl from '#ui/components/search/SearchFilterControl.vue'
import { defineMessages, useVIntl } from '#ui/composables/i18n'
import { useStickyObserver } from '#ui/composables/sticky-observer'
import { commonMessages, formatProjectTypeSentence } from '#ui/utils/common-messages'
import type { SortType } from '#ui/utils/search'

import SelectedProjectsFloatingBar from './components/SelectedProjectsFloatingBar.vue'
import BrowseInstallHeader from './header.vue'
import { injectBrowseManager } from './providers/browse-manager'
import type { CardAction } from './types'

const ctx = injectBrowseManager()
const { formatMessage } = useVIntl()
const lockedMessages = computed(() => toValue(ctx.lockedFilterMessages))
const stickyInstallHeaderRef = ref<HTMLElement | null>(null)
const { isStuck: isInstallHeaderStuck } = useStickyObserver(
	stickyInstallHeaderRef,
	'BrowseInstallHeader',
)

const sortOptions = computed<ComboboxOption<SortType>[]>(() =>
	ctx.effectiveSortTypes.value.map((st) => ({
		value: st,
		label: st.display,
	})),
)

const _maxResultsOptions = computed<ComboboxOption<number>[]>(() =>
	(ctx.maxResultsOptions?.value ?? [5, 10, 15, 20, 50, 100]).map((n) => ({
		value: n,
		label: String(n),
	})),
)

const categoryFilter = computed(() =>
	ctx.isServerType.value
		? ctx.serverFilterTypes.value.find((f) => f.id.includes('category'))
		: ctx.filters.value.find((f) => f.id.startsWith('category')),
)

const categoryOptions = computed(() => categoryFilter.value?.options ?? [])

const loaderFilter = computed(() =>
	ctx.filters.value.find((f) => f.id === 'loader' || f.id === 'server_loader'),
)

const versionFilter = computed(() =>
	ctx.filters.value.find((f) => f.id === 'game_version' || f.id === 'server_game_version'),
)

const environmentFilter = computed(() => ctx.filters.value.find((f) => f.id === 'environment'))

function isOptionSelected(filterId: string, optionId: string): boolean {
	const current = ctx.isServerType.value ? ctx.serverCurrentFilters.value : ctx.currentFilters.value
	return current.some((f) => f.type === filterId && f.option === optionId)
}

function scrollToTop() {
	const viewport = document.querySelector('.app-viewport')
	if (viewport && viewport.scrollTop > 0) {
		viewport.scrollTo({ top: 0, behavior: 'smooth' })
	} else if (typeof window !== 'undefined' && window.scrollY > 0) {
		window.scrollTo({ top: 0, behavior: 'smooth' })
	}
}

function toggleOption(filterId: string, optionId: string) {
	const current = ctx.isServerType.value ? ctx.serverCurrentFilters.value : ctx.currentFilters.value
	const idx = current.findIndex((f) => f.type === filterId && f.option === optionId)
	if (idx >= 0) {
		current.splice(idx, 1)
	} else {
		current.push({ type: filterId, option: optionId })
	}
	scrollToTop()
}

function selectSingleOption(filterId: string, optionId: string | null) {
	const current = ctx.isServerType.value ? ctx.serverCurrentFilters.value : ctx.currentFilters.value
	for (let i = current.length - 1; i >= 0; i--) {
		if (current[i].type === filterId) {
			current.splice(i, 1)
		}
	}
	if (optionId) {
		current.push({ type: filterId, option: optionId })
	}
	scrollToTop()
}

const selectedLoader = computed(() => {
	if (!loaderFilter.value) return ''
	const current = ctx.isServerType.value ? ctx.serverCurrentFilters.value : ctx.currentFilters.value
	const found = current.find((f) => f.type === loaderFilter.value!.id)
	return found?.option ?? ''
})

const loaderOptions = computed<ComboboxOption<string>[]>(() => [
	{ value: '', label: 'All Loaders' },
	...(loaderFilter.value?.options ?? []).map((o) => ({
		value: o.id,
		label: o.formatted_name ?? o.name ?? o.id,
	})),
])

const selectedLoaderOption = computed(
	() => loaderOptions.value.find((o) => o.value === selectedLoader.value) ?? loaderOptions.value[0],
)

const selectedVersion = computed(() => {
	if (!versionFilter.value) return ''
	const current = ctx.isServerType.value ? ctx.serverCurrentFilters.value : ctx.currentFilters.value
	const found = current.find((f) => f.type === versionFilter.value!.id)
	return found?.option ?? ''
})

const versionOptions = computed<ComboboxOption<string>[]>(() => [
	{ value: '', label: 'All Versions' },
	...(versionFilter.value?.options ?? []).map((o) => ({
		value: o.id,
		label: o.formatted_name ?? o.name ?? o.id,
	})),
])

const selectedVersionOption = computed(
	() =>
		versionOptions.value.find((o) => o.value === selectedVersion.value) ?? versionOptions.value[0],
)

const selectedEnvironment = computed(() => {
	if (!environmentFilter.value) return ''
	const current = ctx.isServerType.value ? ctx.serverCurrentFilters.value : ctx.currentFilters.value
	const found = current.find((f) => f.type === environmentFilter.value!.id)
	return found?.option ?? ''
})

const instanceOptions = computed<ComboboxOption<string>[]>(() => toValue(ctx.instanceOptions) ?? [])

const selectedInstanceId = computed(() => {
	const raw = toValue(ctx.selectedInstanceId)
	return raw !== undefined && raw !== null ? String(raw) : ''
})

const messages = defineMessages({
	searchPlaceholder: {
		id: 'browse.search.placeholder',
		defaultMessage: 'Search {projectType}...',
	},
	viewPrefix: {
		id: 'browse.view-prefix',
		defaultMessage: 'View:',
	},
	filterResults: {
		id: 'browse.filter-results',
		defaultMessage: 'Filter results...',
	},
	offline: {
		id: 'browse.offline',
		defaultMessage: 'You are currently offline. Connect to the internet to browse FreePlay!',
	},
	noResults: {
		id: 'browse.no-results',
		defaultMessage: 'No results found for your query!',
	},
	linkOverridingPreferences: {
		id: 'browse.advanced-filters.link-overriding-preferences',
		defaultMessage: "This link's filters differ from your saved advanced exclusions",
	},
	applySavedPreferences: {
		id: 'browse.advanced-filters.apply-saved-preferences',
		defaultMessage: 'Apply saved preferences',
	},
})

function cardActionType(action: CardAction) {
	if (action.type === 'transparent') return 'quiet'
	if (action.type === 'outlined') return 'outlined'
	return action.color && action.color !== 'standard' ? 'colored' : 'base'
}

function cardActionColor(action: CardAction) {
	const type = cardActionType(action)
	return type === 'colored' || type === 'quiet' ? action.color : undefined
}

function cardActionClass(action: CardAction) {
	if (action.type !== 'outlined' || !action.color || action.color === 'standard') return undefined

	return {
		brand: '!text-brand [&>svg]:!text-brand !shadow-[inset_0_0_0_1px_var(--color-brand)]',
		red: '!text-red [&>svg]:!text-red !shadow-[inset_0_0_0_1px_var(--color-red)]',
		green: '!text-green [&>svg]:!text-green !shadow-[inset_0_0_0_1px_var(--color-green)]',
	}[action.color]
}

function getLoaderFieldValues(
	result: Labrinth.Search.v3.ResultSearchProject,
	field: string,
): string[] {
	return (result.project_loader_fields?.[field] ?? []).filter(
		(value): value is string => typeof value === 'string',
	)
}

function getProjectCardTags(result: Labrinth.Search.v3.ResultSearchProject, displayOnly: boolean) {
	const tags = new Set(displayOnly ? result.display_categories : result.categories)

	for (const loader of result.loaders) {
		if (loader !== 'mrpack') {
			tags.add(loader)
		}
	}

	if (result.loaders.includes('mrpack')) {
		for (const loader of getLoaderFieldValues(result, 'mrpack_loaders')) {
			tags.add(loader)
		}
	}

	return Array.from(tags)
}
</script>

<template>
	<template v-if="ctx.installContext?.value && ctx.variant !== 'web'">
		<div
			ref="stickyInstallHeaderRef"
			class="sticky top-0 z-20 -mx-6 -mt-6 rounded-tl-[--radius-xl] border-0 border-b border-solid bg-surface-1 px-6 py-4 border-surface-5"
			:class="[isInstallHeaderStuck ? 'border-t' : '']"
		>
			<BrowseInstallHeader />
		</div>
	</template>
	<SelectedProjectsFloatingBar v-if="ctx.installContext?.value && ctx.variant !== 'web'" />

	<!-- Sticky top bar: NavTabs + Search bar + Category tags -->
	<div
		class="sticky z-10 -mx-6 px-6 pb-2.5 bg-bg/95 backdrop-blur-md flex flex-col gap-3 transition-all border-b border-surface-4/20"
		:class="[
			!(ctx.installContext?.value && ctx.variant !== 'web')
				? 'top-0 -mt-6 pt-6'
				: 'top-[73px] pt-2',
		]"
	>
		<div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
			<NavTabs
				v-if="ctx.showProjectTypeTabs.value"
				:links="ctx.selectableProjectTypes.value"
				:replace="ctx.variant === 'app'"
				class="!m-0 shrink-0"
			/>

			<div class="flex-1 max-w-xl md:ml-auto">
				<StyledInput
					v-model="ctx.query.value"
					:icon="SearchIcon"
					type="text"
					autocomplete="off"
					:placeholder="
						formatMessage(messages.searchPlaceholder, {
							projectType: formatProjectTypeSentence(formatMessage, ctx.projectType.value, 2),
						})
					"
					clearable
					wrapper-class="w-full !bg-surface-2 border border-surface-4 hover:border-surface-5 focus-within:!border-brand focus-within:!shadow-[0_0_16px_var(--color-brand-shadow)] rounded-2xl transition-all shadow-sm"
					input-class="!h-11 text-sm font-medium"
					@clear="ctx.clearSearch()"
				/>
			</div>
		</div>

		<!-- Category tags bar with stable minimum height to prevent vertical jumping between sections with different tag counts -->
		<div class="min-h-[4.25rem] flex flex-wrap content-start items-center gap-1.5 py-1 select-none">
			<template v-if="categoryFilter && categoryOptions.length > 0">
				<button
					v-for="opt in categoryOptions"
					:key="opt.id"
					type="button"
					class="h-7 px-3 rounded-xl border text-xs font-semibold flex items-center gap-1.5 cursor-pointer whitespace-nowrap transition-all duration-200 active:scale-95 shrink-0"
					:class="
						isOptionSelected(categoryFilter.id, opt.id)
							? 'bg-brand border-brand text-brand-inverted shadow-[0_0_12px_var(--color-brand-shadow)] font-bold'
							: 'bg-surface-2 border-surface-4 text-secondary hover:text-contrast hover:border-surface-5 hover:bg-surface-3'
					"
					@click="toggleOption(categoryFilter.id, opt.id)"
				>
					<component :is="opt.icon" v-if="opt.icon" class="w-3.5 h-3.5" />
					<span>{{ opt.formatted_name ?? opt.name ?? opt.id }}</span>
				</button>
			</template>
		</div>
	</div>

	<Admonition
		v-if="ctx.linkOverridesAdvancedPrefs.value"
		type="info"
		:header="formatMessage(messages.linkOverridingPreferences)"
		inline-actions
		center-content
	>
		<template #actions>
			<Button type="colored" color="blue" @click="ctx.applySavedAdvancedPrefs()">
				<RotateCounterClockwiseIcon />
				{{ formatMessage(messages.applySavedPreferences) }}
			</Button>
		</template>
	</Admonition>

	<div
		class="flex flex-wrap items-center justify-between gap-2.5 p-2.5 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm"
	>
		<div class="flex flex-wrap items-center gap-2 min-w-0 flex-1">
			<!-- Target Instance Selector Option -->
			<Combobox
				v-if="instanceOptions.length > 0"
				:model-value="selectedInstanceId"
				:options="instanceOptions"
				trigger-type="base"
				class="!w-[13.5rem] min-w-0"
				@update:model-value="(val: string) => ctx.onSelectInstance?.(val || null)"
			>
				<template #prefix>
					<span class="font-semibold text-secondary text-xs">Instance:</span>
				</template>
			</Combobox>

			<!-- 1st Option: Version Dropdown -->
			<Combobox
				v-if="versionFilter"
				:model-value="selectedVersionOption"
				:options="versionOptions"
				trigger-type="base"
				class="!w-[10.5rem] min-w-0"
				@update:model-value="
					(val: ComboboxOption<string>) =>
						versionFilter ? selectSingleOption(versionFilter.id, val.value || null) : null
				"
			>
				<template #prefix>
					<span class="font-semibold text-secondary text-xs">Version:</span>
				</template>
			</Combobox>

			<!-- 3rd Option: Sort By Dropdown (Together with Version) -->
			<Combobox
				:model-value="ctx.effectiveCurrentSortType.value"
				:options="sortOptions"
				trigger-type="base"
				class="!w-[12.5rem] min-w-0"
				@update:model-value="(val: SortType) => (ctx.effectiveCurrentSortType.value = val)"
			>
				<template #prefix>
					<span class="font-semibold text-secondary text-xs">{{
						formatMessage(commonMessages.sortByLabel)
					}}</span>
				</template>
			</Combobox>

			<!-- Loader Dropdown -->
			<Combobox
				v-if="loaderFilter"
				:model-value="selectedLoaderOption"
				:options="loaderOptions"
				trigger-type="base"
				class="!w-[10rem] min-w-0"
				@update:model-value="
					(val: ComboboxOption<string>) =>
						loaderFilter ? selectSingleOption(loaderFilter.id, val.value || null) : null
				"
			>
				<template #prefix>
					<span class="font-semibold text-secondary text-xs">Loader:</span>
				</template>
			</Combobox>

			<!-- Environment Switch -->
			<div
				v-if="environmentFilter"
				class="flex items-center p-0.5 rounded-xl bg-surface-3 border border-surface-4 select-none h-8 shrink-0"
			>
				<button
					type="button"
					class="px-2.5 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer border-none"
					:class="
						selectedEnvironment === ''
							? 'bg-brand text-brand-inverted shadow-sm'
							: 'text-secondary hover:text-contrast bg-transparent'
					"
					@click="selectSingleOption(environmentFilter.id, null)"
				>
					All
				</button>
				<button
					type="button"
					class="px-2.5 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer border-none"
					:class="
						selectedEnvironment === 'client'
							? 'bg-brand text-brand-inverted shadow-sm'
							: 'text-secondary hover:text-contrast bg-transparent'
					"
					@click="selectSingleOption(environmentFilter.id, 'client')"
				>
					Client
				</button>
				<button
					type="button"
					class="px-2.5 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer border-none"
					:class="
						selectedEnvironment === 'server'
							? 'bg-brand text-brand-inverted shadow-sm'
							: 'text-secondary hover:text-contrast bg-transparent'
					"
					@click="selectSingleOption(environmentFilter.id, 'server')"
				>
					Server
				</button>
			</div>

			<!-- Hide Installed Toggle -->
			<label
				v-if="ctx.showHideInstalled?.value"
				class="flex items-center gap-2 px-3 py-1 rounded-xl bg-surface-3 border border-surface-4 text-xs font-semibold text-secondary cursor-pointer select-none hover:border-surface-5 hover:text-contrast transition-all h-8 shrink-0"
			>
				<Toggle v-model="ctx.hideInstalled.value" size="sm" />
				<span class="text-xs">{{ ctx.hideInstalledLabel?.value || 'Hide Installed' }}</span>
			</label>
		</div>

		<div class="flex items-center gap-2 shrink-0 ml-auto pl-1">
			<IconButton
				v-if="ctx.cycleDisplayMode"
				label="Change display mode"
				class="!h-8 !w-8"
				@click="ctx.cycleDisplayMode!()"
			>
				<slot name="display-mode-icon" />
			</IconButton>

			<Pagination
				:page="ctx.currentPage.value"
				:count="ctx.pageCount.value"
				class="shrink-0"
				@switch-page="ctx.setPage"
			/>
		</div>
	</div>

	<SearchFilterControl
		v-if="ctx.isServerType.value"
		v-model:selected-filters="ctx.serverCurrentFilters.value"
		:filters="ctx.serverFilterTypes.value"
		:provided-filters="[]"
		:overridden-provided-filter-types="[]"
	/>
	<SearchFilterControl
		v-else
		v-model:selected-filters="ctx.currentFilters.value"
		:filters="
			ctx.filters.value.filter(
				(f) => f.display !== 'none' && !(ctx.hiddenFilterTypes?.value ?? []).includes(f.id),
			)
		"
		:provided-filters="ctx.providedFilters?.value ?? []"
		:overridden-provided-filter-types="ctx.overriddenProvidedFilterTypes.value"
		:provided-message="lockedMessages?.providedBy"
	/>

	<div class="search flex flex-col gap-4">
		<!-- Ghost Placeholder Skeleton Grid during section switching and loading -->
		<ProjectCardList v-if="ctx.loading.value" :layout="ctx.effectiveLayout.value">
			<ProjectCardSkeleton v-for="i in 8" :key="`ghost-${i}`" :layout="ctx.effectiveLayout.value" />
		</ProjectCardList>
		<section
			v-else-if="ctx.offline?.value && ctx.totalHits.value === 0"
			class="flex flex-col items-center justify-center p-12 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm text-center gap-2"
		>
			<p class="m-0 text-base font-bold text-contrast">{{ formatMessage(messages.offline) }}</p>
			<p class="m-0 text-xs text-secondary">
				You are currently offline. Check your network connection.
			</p>
		</section>
		<section
			v-else-if="
				ctx.isServerType.value
					? ctx.serverHits.value.length === 0
					: ctx.projectHits.value.length === 0
			"
			class="flex flex-col items-center justify-center p-12 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm text-center gap-2"
		>
			<p class="m-0 text-base font-bold text-contrast">{{ formatMessage(messages.noResults) }}</p>
			<p class="m-0 text-xs text-secondary">
				Try adjusting your filters or searching for something else.
			</p>
		</section>

		<ProjectCardList v-else :layout="ctx.effectiveLayout.value">
			<template v-if="ctx.isServerType.value">
				<ProjectCard
					v-for="result in ctx.serverHits.value"
					:key="`server-card-${result.project_id}`"
					:title="result.name"
					:icon-url="result.icon_url || undefined"
					:summary="result.summary"
					:tags="result.categories"
					:link="ctx.getServerProjectLink(result)"
					:server-online-players="result.minecraft_java_server?.ping?.data?.players_online ?? 0"
					:server-region="result.minecraft_server?.region"
					:server-recent-plays="result.minecraft_java_server?.verified_plays_2w ?? 0"
					:server-modpack-content="ctx.getServerModpackContent?.(result)"
					:server-ping="ctx.serverPings?.value?.[result.project_id]"
					:server-status-online="!!result.minecraft_java_server?.ping?.data"
					:hide-online-players-label="ctx.variant === 'app'"
					:hide-recent-plays-label="ctx.variant === 'app'"
					:layout="ctx.effectiveLayout.value"
					:max-tags="2"
					is-server-project
					exclude-loaders
					:color="result.color ?? undefined"
					:banner="result.featured_gallery ?? undefined"
					@contextmenu.prevent.stop="(event: MouseEvent) => ctx.onContextMenu?.(event, result)"
					@mouseenter="ctx.onServerProjectHover?.(result)"
					@mouseleave="ctx.onProjectHoverEnd?.()"
				>
					<template v-if="ctx.getCardActions?.(result, ctx.projectType.value)?.length" #actions>
						<div class="flex gap-2">
							<template
								v-for="action in ctx.getCardActions(result, ctx.projectType.value)"
								:key="action.key"
							>
								<IconButton
									v-if="action.circular"
									v-tooltip="action.tooltip"
									:type="cardActionType(action)"
									:color="cardActionColor(action)"
									:class="cardActionClass(action)"
									:label="action.label || action.tooltip || action.key"
									:disabled="action.disabled"
									@click.stop="action.onClick"
								>
									<component :is="action.icon" :class="action.iconClass" />
								</IconButton>
								<Button
									v-else
									v-tooltip="action.tooltip"
									:type="cardActionType(action)"
									:color="cardActionColor(action)"
									:class="cardActionClass(action)"
									:disabled="action.disabled"
									@click.stop="action.onClick"
								>
									<component :is="action.icon" :class="action.iconClass" />
									{{ action.label }}
								</Button>
							</template>
						</div>
					</template>
				</ProjectCard>
			</template>
			<template v-else>
				<ProjectCard
					v-for="result in ctx.projectHits.value"
					:key="result.project_id"
					:link="ctx.getProjectLink(result)"
					:title="result.name"
					:icon-url="result.icon_url ?? undefined"
					:author="{
						name: result.organization == null ? result.author : result.organization,
						link:
							result.organization_id == null
								? `/user/${encodeURIComponent(result.author_id ?? result.author)}`
								: ctx.variant === 'web'
									? `/organization/${result.organization_id}`
									: `https://freeplay.app/organization/${result.organization_id}`,
					}"
					:date-updated="result.date_modified"
					:date-published="result.date_created"
					:displayed-date="
						ctx.effectiveCurrentSortType.value.name === 'newest' ? 'published' : 'updated'
					"
					:downloads="result.downloads"
					:summary="result.summary"
					:tags="getProjectCardTags(result, true)"
					:all-tags="getProjectCardTags(result, false)"
					:deprioritized-tags="ctx.deprioritizedTags.value"
					:exclude-loaders="ctx.excludeLoaders.value"
					:followers="result.follows"
					:banner="result.featured_gallery ?? undefined"
					:color="result.color ?? undefined"
					:environment="
						['mod', 'modpack'].includes(ctx.projectType.value)
							? result.project_loader_fields?.environment?.[0]
							: undefined
					"
					:layout="ctx.effectiveLayout.value"
					@contextmenu.prevent.stop="(event: MouseEvent) => ctx.onContextMenu?.(event, result)"
					@mouseenter="ctx.onProjectHover?.(result)"
					@mouseleave="ctx.onProjectHoverEnd?.()"
				>
					<template v-if="ctx.getCardActions?.(result, ctx.projectType.value)?.length" #actions>
						<div class="flex gap-2">
							<template
								v-for="action in ctx.getCardActions(result, ctx.projectType.value)"
								:key="action.key"
							>
								<IconButton
									v-if="action.circular"
									v-tooltip="action.tooltip"
									:type="cardActionType(action)"
									:color="cardActionColor(action)"
									:class="cardActionClass(action)"
									:label="action.label || action.tooltip || action.key"
									:disabled="action.disabled"
									@click.stop="action.onClick"
								>
									<component :is="action.icon" :class="action.iconClass" />
								</IconButton>
								<Button
									v-else
									v-tooltip="action.tooltip"
									:type="cardActionType(action)"
									:color="cardActionColor(action)"
									:class="cardActionClass(action)"
									:disabled="action.disabled"
									@click.stop="action.onClick"
								>
									<component :is="action.icon" :class="action.iconClass" />
									{{ action.label }}
								</Button>
							</template>
						</div>
					</template>
				</ProjectCard>
			</template>
		</ProjectCardList>

		<div :class="ctx.variant === 'web' ? 'pagination-after mt-3' : 'flex justify-end mt-3'">
			<Pagination
				:page="ctx.currentPage.value"
				:count="ctx.pageCount.value"
				:class="ctx.variant === 'web' ? 'justify-end' : 'pagination-after'"
				@switch-page="ctx.setPage"
			/>
		</div>
	</div>

	<slot name="after" />
</template>
