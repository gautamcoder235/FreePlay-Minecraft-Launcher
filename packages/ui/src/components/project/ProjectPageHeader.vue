<template>
	<div
		class="relative flex flex-col gap-4 rounded-2xl bg-surface-2 border border-surface-4 p-5 sm:p-6 shadow-sm overflow-hidden transition-all duration-300"
		@contextmenu="emit('contextmenu', $event)"
	>
		<div
			class="pointer-events-none absolute -right-16 -top-16 size-72 rounded-full opacity-15 blur-3xl"
			:style="{ backgroundColor: ambientColor }"
		/>
		<div
			class="pointer-events-none absolute -left-16 -bottom-16 size-56 rounded-full opacity-10 blur-2xl"
			:style="{ backgroundColor: ambientColor }"
		/>

		<div class="relative z-10 flex flex-col md:flex-row items-start justify-between gap-4 sm:gap-6">
			<div class="flex items-start gap-4 sm:gap-5 min-w-0 flex-1">
				<div class="shrink-0 relative group">
					<Avatar
						:src="project.icon_url"
						:alt="project.title"
						:tint-by="project.id"
						size="88px"
						class="rounded-2xl ring-1 ring-surface-4/80 bg-surface-3 shadow-md overflow-hidden transition-transform duration-300 group-hover:scale-[1.03]"
					/>
				</div>

				<div class="flex flex-col gap-1.5 min-w-0 flex-1">
					<div class="flex items-center gap-2.5 flex-wrap">
						<h1
							class="m-0 text-2xl sm:text-3xl font-extrabold text-contrast tracking-tight leading-tight"
						>
							{{ project.title }}
						</h1>
						<ProjectStatusBadge
							v-if="showStatusBadge && projectV3?.status"
							:status="projectV3.status"
						/>
						<span
							v-if="project.project_type"
							class="px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wider bg-surface-3 text-secondary border border-surface-4"
						>
							{{ project.project_type }}
						</span>
					</div>

					<p
						v-if="project.description"
						class="m-0 text-secondary text-sm sm:text-base leading-relaxed max-w-3xl line-clamp-3 font-normal"
					>
						{{ project.description }}
					</p>
				</div>
			</div>

			<div
				class="relative z-10 shrink-0 flex items-center gap-2 self-start md:self-center max-md:w-full max-md:justify-end"
			>
				<slot name="actions" />
			</div>
		</div>

		<div
			class="relative z-10 pt-3 border-t border-surface-4/40 flex flex-wrap items-center justify-between gap-3 text-xs"
		>
			<div class="flex items-center gap-2.5 flex-wrap">
				<template v-if="projectV3?.minecraft_server != null">
					<ServerDetails
						v-if="projectV3?.status !== 'draft'"
						:online-players="projectV3?.minecraft_java_server?.ping?.data?.players_online ?? 0"
						:status-online="!!projectV3?.minecraft_java_server?.ping?.data"
						:recent-plays="projectV3?.minecraft_java_server?.verified_plays_2w ?? 0"
					/>
				</template>
				<template v-else>
					<div
						v-if="project.downloads !== undefined"
						v-tooltip="
							formatNumber(project.downloads) +
							' ' +
							formatMessage(messages.downloadsStat, { count: project.downloads })
						"
						class="flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-surface-3 border border-surface-4 text-xs font-semibold text-primary shadow-xs transition-colors hover:bg-surface-4/60"
					>
						<DownloadIcon class="size-3.5 text-brand shrink-0" />
						<span>{{ formatNumber(project.downloads) }}</span>
						<span class="text-secondary font-normal">{{
							formatMessage(messages.downloadsStat, { count: project.downloads })
						}}</span>
					</div>

					<div
						v-if="project.followers !== undefined"
						v-tooltip="
							formatNumber(project.followers) +
							' ' +
							formatMessage(messages.followersStat, { count: project.followers })
						"
						class="flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-surface-3 border border-surface-4 text-xs font-semibold text-primary shadow-xs transition-colors hover:bg-surface-4/60"
					>
						<HeartIcon class="size-3.5 text-rose-400 shrink-0" />
						<span>{{ formatNumber(project.followers) }}</span>
						<span class="text-secondary font-normal">{{
							formatMessage(messages.followersStat, { count: project.followers })
						}}</span>
					</div>
				</template>
			</div>

			<div
				v-if="project.categories && project.categories.length > 0"
				class="flex items-center gap-1.5 flex-wrap ml-auto"
			>
				<TagItem
					v-for="category in project.categories"
					:key="category"
					class="transition-all hover:scale-105"
					:action="() => emit('category', category)"
				>
					<FormattedTag :tag="category" />
				</TagItem>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import type { Labrinth } from '@freeplay/api-client'
import { DownloadIcon, HeartIcon } from '@freeplay/assets'
import { computed } from 'vue'

import { defineMessages, useFormatNumber, useVIntl } from '../../composables'
import Avatar from '../base/Avatar.vue'
import FormattedTag from '../base/FormattedTag.vue'
import TagItem from '../base/TagItem.vue'
import ProjectStatusBadge from './ProjectStatusBadge.vue'
import ServerDetails from './server/ServerDetails.vue'

type HeaderProject = Pick<
	Labrinth.Projects.v2.Project,
	'id' | 'title' | 'description' | 'status' | 'downloads' | 'followers' | 'categories'
> & {
	icon_url?: string | null
	color?: number | null
	project_type?: string | null
}

type HeaderProjectV3 = Pick<
	Labrinth.Projects.v3.Project,
	'status' | 'minecraft_server' | 'minecraft_java_server'
>

const props = withDefaults(
	defineProps<{
		project: HeaderProject
		projectV3?: HeaderProjectV3 | null
		showStatusBadge?: boolean
	}>(),
	{
		projectV3: null,
		showStatusBadge: false,
	},
)

const emit = defineEmits<{
	category: [category: string]
	contextmenu: [event: MouseEvent]
}>()

const messages = defineMessages({
	downloadsStat: {
		id: 'project.stats.downloads-label',
		defaultMessage: '{count, plural, one {download} other {downloads}}',
	},
	followersStat: {
		id: 'project.stats.followers-label',
		defaultMessage: '{count, plural, one {follower} other {followers}}',
	},
})

const { formatMessage } = useVIntl()
const formatNumber = useFormatNumber()

function clamp(value: number) {
	return Math.max(0, Math.min(255, value))
}

function toHex(value: number) {
	return clamp(value).toString(16).padStart(2, '0')
}

function decimalToHexColor(decimal: number) {
	const r = (decimal >> 16) & 255
	const g = (decimal >> 8) & 255
	const b = decimal & 255
	return `#${toHex(r)}${toHex(g)}${toHex(b)}`
}

const ambientColor = computed(() => {
	if (props.project?.color != null) {
		return decimalToHexColor(props.project.color)
	}
	return 'var(--color-brand)'
})
</script>
