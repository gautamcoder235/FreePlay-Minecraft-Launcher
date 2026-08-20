<template>
	<SmartClickable class="w-full project-card-container">
		<template v-if="link" #clickable>
			<AutoLink
				:to="link"
				class="rounded-xl no-outline no-click-animation custom-focus-indicator"
				@mouseenter="$emit('mouseenter')"
				@mouseleave="$emit('mouseleave')"
			></AutoLink>
		</template>
		<div v-if="layout === 'grid'" :class="[baseCardStyle, 'flex flex-col']">
			<div
				:style="{ '--_project-color': cssColor }"
				class="relative bg-project-gradient overflow-clip aspect-[2/1] w-full border-0 border-b-[1px] border-solid border-surface-4"
			>
				<img
					v-if="banner"
					:src="banner"
					alt=""
					class="absolute w-full h-full inset-0 object-cover object-center"
				/>
				<img
					v-else
					src="https://cdn-raw.freeplay.app/landing-new/landing.webp"
					alt=""
					class="absolute w-full h-full inset-0 object-cover object-center placeholder-banner scale-[200%]"
				/>
			</div>
			<div class="p-4 flex flex-col gap-3 grow">
				<div class="flex gap-3">
					<Avatar :src="iconUrl" size="96px" class="project-card__icon ease-brightness" no-shadow />
					<div class="flex flex-col gap-2 w-full">
						<div class="grid grid-cols-[1fr_auto] gap-4">
							<div class="flex flex-col gap-1">
								<div class="flex gap-2 items-center">
									<ProjectCardTitle :title="title" compact />
									<ProjectCardAuthor v-if="author" :author="author" />
									<ProjectStatusBadge v-if="status" :status="status" class="text-sm" />
								</div>
								<div class="m-0 font-normal line-clamp-2">
									{{ summary }}
								</div>
							</div>
						</div>
					</div>
				</div>
				<div class="flex gap-2 shrink-0 empty:hidden smart-clickable:allow-pointer-events">
					<slot name="actions" />
				</div>
				<div class="mt-auto flex flex-col gap-3 flex-wrap overflow-hidden justify-between grow">
					<div class="flex items-center gap-1 flex-wrap overflow-hidden">
						<template v-if="isServerProject">
							<ServerOnlinePlayers
								v-if="serverOnlinePlayers !== undefined"
								:online="serverOnlinePlayers"
								:status-online="serverStatusOnline"
								:hide-label="true"
							/>
							<ServerRecentPlays
								v-if="serverRecentPlays !== undefined"
								:recent-plays="serverRecentPlays"
								:hide-label="true"
							/>
							<ServerPing v-if="serverPing && serverStatusOnline" :ping="serverPing" />
							<ServerRegion
								v-if="serverRegion"
								:region="serverRegion"
								class="smart-clickable:allow-pointer-events"
							/>
						</template>
						<ProjectCardEnvironment v-if="environment" :environment="environment" />
						<ProjectCardTags
							v-if="tags"
							:tags="tags"
							:exclude-loaders="excludeLoaders"
							:deprioritized-tags="deprioritizedTags"
							:max-tags="(maxTags || 6) + (!!environment ? 0 : 1)"
						/>
						<ServerModpackContent
							v-if="serverModpackContent"
							:name="serverModpackContent.name"
							:icon="serverModpackContent.icon"
							:onclick="serverModpackContent.onclick"
							:show-custom-modpack-tooltip="serverModpackContent.showCustomModpackTooltip"
							class="text-primary"
						/>
					</div>
					<div
						v-if="downloads !== undefined || followers !== undefined"
						class="flex items-center gap-3 justify-between flex-wrap"
					>
						<div class="flex items-center gap-3 no-wrap flex-wrap">
							<ProjectCardStats :downloads="downloads" :followers="followers" />
						</div>
						<ProjectCardDate v-if="date && autoDisplayDate" :type="autoDisplayDate" :date="date" />
					</div>
				</div>
			</div>
		</div>
		<div
			v-else
			:class="[
				baseCardStyle,
				'p-4 flex flex-col justify-between gap-3 min-h-[152px] overflow-hidden',
				{ 'has-actions': !!$slots.actions },
			]"
		>
			<div class="flex items-start gap-3 w-full min-w-0">
				<Avatar
					:src="iconUrl"
					size="64px"
					class="project-card__icon ease-brightness shrink-0 mt-0.5"
					no-shadow
				/>
				<div class="flex flex-col gap-1.5 min-w-0 flex-1">
					<div class="flex items-center gap-2 min-w-0 flex-wrap">
						<ProjectCardTitle :title="title" compact />
						<ProjectCardAuthor v-if="author" :author="author" />
						<ProjectStatusBadge v-if="status" :status="status" />
					</div>
					<div class="project-card-summary m-0 font-normal line-clamp-2 text-secondary text-sm">
						{{ summary }}
					</div>
				</div>

				<div
					v-if="!!$slots.actions"
					class="flex gap-1 shrink-0 ml-auto empty:hidden smart-clickable:allow-pointer-events"
				>
					<slot name="actions" />
				</div>
			</div>

			<div
				class="mt-auto pt-2 flex items-center gap-x-3 gap-y-1.5 flex-wrap border-t border-surface-4/40 overflow-hidden"
			>
				<div class="flex items-center gap-1.5 min-w-0 overflow-hidden flex-wrap">
					<template v-if="isServerProject">
						<ServerOnlinePlayers
							v-if="serverOnlinePlayers !== undefined"
							:online="serverOnlinePlayers"
							:status-online="serverStatusOnline"
							:hide-label="true"
						/>
						<ServerRecentPlays
							v-if="serverRecentPlays !== undefined"
							:recent-plays="serverRecentPlays"
							:hide-label="true"
						/>
						<ServerPing v-if="serverPing && serverStatusOnline" :ping="serverPing" />
						<ServerRegion
							v-if="serverRegion"
							:region="serverRegion"
							class="smart-clickable:allow-pointer-events"
						/>
					</template>
					<ProjectCardEnvironment v-if="environment" :environment="environment" class="shrink-0" />
					<ProjectCardTags
						v-if="tags"
						:tags="tags"
						:extra-tags="extraTags"
						:exclude-loaders="excludeLoaders"
						:deprioritized-tags="deprioritizedTags"
						:max-tags="maxTags || (environment ? 2 : 3)"
					/>
					<ServerModpackContent
						v-if="serverModpackContent"
						:name="serverModpackContent.name"
						:icon="serverModpackContent.icon"
						:onclick="serverModpackContent.onclick"
						:show-custom-modpack-tooltip="serverModpackContent.showCustomModpackTooltip"
						class="text-primary"
					/>
				</div>

				<div
					class="flex items-center gap-3 shrink-0 ml-auto text-xs text-secondary whitespace-nowrap"
				>
					<div
						v-if="downloads !== undefined || followers !== undefined"
						class="flex items-center gap-3"
					>
						<ProjectCardStats :downloads="downloads" :followers="followers" />
					</div>
					<ProjectCardDate v-if="date && autoDisplayDate" :type="autoDisplayDate" :date="date" />
				</div>
			</div>
		</div>
	</SmartClickable>
</template>

<script setup lang="ts">
import type { ProjectStatus } from '@freeplay/utils'
import dayjs from 'dayjs'
import { computed } from 'vue'
import type { RouteLocationRaw } from 'vue-router'

import { AutoLink, Avatar } from '../../base'
import { SmartClickable } from '../../base/index.ts'
import ProjectStatusBadge from '../ProjectStatusBadge.vue'
import ServerModpackContent from '../server/ServerModpackContent.vue'
import ServerOnlinePlayers from '../server/ServerOnlinePlayers.vue'
import ServerPing from '../server/ServerPing.vue'
import ServerRecentPlays from '../server/ServerRecentPlays.vue'
import ServerRegion from '../server/ServerRegion.vue'
import ProjectCardAuthor from './ProjectCardAuthor.vue'
import ProjectCardDate from './ProjectCardDate.vue'
import ProjectCardEnvironment, {
	type ProjectCardEnvironmentValue,
} from './ProjectCardEnvironment.vue'
import ProjectCardStats from './ProjectCardStats.vue'
import ProjectCardTags from './ProjectCardTags.vue'
import ProjectCardTitle from './ProjectCardTitle.vue'

defineEmits<{
	mouseenter: []
	mouseleave: []
}>()

const props = defineProps<{
	layout: 'list' | 'grid'
	link?: string | RouteLocationRaw | (() => void)
	iconUrl?: string
	title: string
	author?: {
		name: string
		link?: string
	}
	summary?: string
	tags?: string[]
	allTags?: string[]
	deprioritizedTags?: string[]
	excludeLoaders?: boolean
	downloads?: number
	followers?: number
	dateUpdated?: string
	datePublished?: string
	displayedDate?: 'updated' | 'published'
	serverRegion?: string
	serverOnlinePlayers?: number
	serverStatusOnline?: boolean
	serverRecentPlays?: number
	serverPing?: number
	serverModpackContent?: {
		name: string
		icon?: string
		onclick?: () => void
		showCustomModpackTooltip?: boolean
	}
	isServerProject?: boolean
	banner?: string
	color?: string | number
	environment?: ProjectCardEnvironmentValue
	status?: ProjectStatus
	maxTags?: number
}>()

const baseCardStyle =
	'w-full h-full border-[1px] border-solid border-surface-4 overflow-hidden bg-surface-3 rounded-2xl transition-all smart-clickable:outline-on-focus smart-clickable:highlight-on-hover'

const updatedDate = computed(() =>
	props.dateUpdated ? dayjs(props.dateUpdated).toDate() : undefined,
)
const publishedDate = computed(() =>
	props.datePublished ? dayjs(props.datePublished).toDate() : undefined,
)

const autoDisplayDate = computed(() => {
	if (props.displayedDate) {
		return props.displayedDate
	} else if (props.dateUpdated) {
		return 'updated'
	} else if (props.datePublished) {
		return 'published'
	} else {
		return undefined
	}
})

const date = computed(() => {
	if (autoDisplayDate.value === 'updated') {
		return updatedDate.value
	} else if (autoDisplayDate.value === 'published') {
		return publishedDate.value
	}
	return undefined
})

const extraTags = computed(() => props.allTags?.filter((tag) => !props.tags?.includes(tag)))

const cssColor = computed(() => {
	if (props.color === undefined || typeof props.color === 'string') {
		return props.color
	}

	const color = props.color >>> 0
	const b = color & 0xff
	const g = (color & 0xff00) >>> 8
	const r = (color & 0xff0000) >>> 16
	return 'rgba(' + [r, g, b, 1].join(',') + ')'
})
</script>
<style scoped>
.no-outline {
	outline: none;
}

:deep(.project-card-container) {
	container-type: inline-size;
}

.project-card__icon {
	--_override-size: 64px;
}

/*noinspection CssUnresolvedCustomProperty*/
.bg-project-gradient {
	--_gradient-start: var(--_project-color, #000);
	--_gradient-end: var(--_project-color, #000);
	@supports (background-color: oklch(from var(--_project-color, #000) l c h)) {
		--_gradient-start: oklch(
			from var(--_project-color, #000) calc(l * 0.8) calc(c * 0.8) calc(h + 15)
		);
		--_gradient-end: oklch(from var(--_project-color, #000) calc(l * 0.5) calc(c * 0.9) h);
	}
	background-color: var(--_gradient-start);
	background-image: linear-gradient(to bottom right, var(--_gradient-start), var(--_gradient-end));
}

.placeholder-banner {
	opacity: 0.7;
}
</style>
