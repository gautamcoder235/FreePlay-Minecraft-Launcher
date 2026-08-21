<script setup>
import { BoxIcon, FolderOpenIcon, FolderSearchIcon, TrashIcon } from '@freeplay/assets'
import {
	Button,
	defineMessages,
	IconButton,
	injectNotificationManager,
	Slider,
	StyledInput,
	Toggle,
	useVIntl,
} from '@freeplay/ui'
import { open } from '@tauri-apps/plugin-dialog'
import { ref, watch } from 'vue'

import ConfirmModalWrapper from '@/components/ui/modal/ConfirmModalWrapper.vue'
import { purge_cache_types } from '@/helpers/cache.js'
import { get, set } from '@/helpers/settings.ts'
import { showAppDbBackupsFolder } from '@/helpers/utils.js'
import { useTheming } from '@/store/state'

const { addNotification, handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()
const themeStore = useTheming()
const settings = ref(await get())
const purgeCacheConfirmModal = ref(null)

const messages = defineMessages({
	appDirectoryTitle: {
		id: 'app.settings.resource-management.app-directory.title',
		defaultMessage: 'App directory',
	},
	appDirectoryDescription: {
		id: 'app.settings.resource-management.app-directory.description',
		defaultMessage:
			'Where FreePlay Launcher stores instances and other files. Changes take effect after restarting the app.',
	},
	selectAppDirectory: {
		id: 'app.settings.resource-management.app-directory.select',
		defaultMessage: 'Select a new app directory',
	},
	browseAppDirectory: {
		id: 'app.settings.resource-management.app-directory.browse',
		defaultMessage: 'Browse for an app directory',
	},
	appCacheTitle: {
		id: 'app.settings.resource-management.app-cache.title',
		defaultMessage: 'App cache',
	},
	purgeCache: {
		id: 'app.settings.resource-management.app-cache.purge',
		defaultMessage: 'Purge cache',
	},
	purgeCacheConfirmTitle: {
		id: 'app.settings.resource-management.app-cache.confirm.title',
		defaultMessage: 'Purge the app cache?',
	},
	purgeCacheConfirmDescription: {
		id: 'app.settings.resource-management.app-cache.confirm.description',
		defaultMessage: 'The app may load more slowly until the cache is rebuilt.',
	},
	appCacheDescription: {
		id: 'app.settings.resource-management.app-cache.description',
		defaultMessage:
			'Clear cached data and download it again from FreePlay. The app may load more slowly until the cache is rebuilt.',
	},
	maximumConcurrentDownloadsTitle: {
		id: 'app.settings.resource-management.maximum-concurrent-downloads.title',
		defaultMessage: 'Maximum concurrent downloads',
	},
	maximumConcurrentDownloadsDescription: {
		id: 'app.settings.resource-management.maximum-concurrent-downloads.description',
		defaultMessage:
			'Number of files the app can download at once. Lower this if downloads are unreliable on your connection. Requires an app restart.',
	},
	maximumConcurrentWritesTitle: {
		id: 'app.settings.resource-management.maximum-concurrent-writes.title',
		defaultMessage: 'Maximum concurrent writes',
	},
	maximumConcurrentWritesDescription: {
		id: 'app.settings.resource-management.maximum-concurrent-writes.description',
		defaultMessage:
			'Number of files the app can write to disk at once. Lower this if you frequently encounter I/O errors. Requires an app restart.',
	},
	alwaysShowCopyDetailsTitle: {
		id: 'app.settings.resource-management.always-show-copy-details.title',
		defaultMessage: 'Always show copy details',
	},
	alwaysShowCopyDetailsDescription: {
		id: 'app.settings.resource-management.always-show-copy-details.description',
		defaultMessage:
			'Show the Copy details action while an install is queued or running. It is always available for failed or interrupted installs.',
	},
	appDatabaseBackupsTitle: {
		id: 'app.settings.resource-management.app-database-backups.title',
		defaultMessage: 'App database backups',
	},
	openBackupsFolder: {
		id: 'app.settings.resource-management.app-database-backups.open-folder',
		defaultMessage: 'Open backups folder',
	},
	appDatabaseBackupsDescription: {
		id: 'app.settings.resource-management.app-database-backups.description',
		defaultMessage:
			'Backups of important app data are stored here in case you need to recover them later.',
	},
})

watch(
	settings,
	async () => {
		const setSettings = JSON.parse(JSON.stringify(settings.value))

		if (!setSettings.custom_dir) {
			setSettings.custom_dir = null
		}

		await set(setSettings)
	},
	{ deep: true },
)

async function purgeCache() {
	try {
		await purge_cache_types([
			'project',
			'project_v3',
			'version',
			'user',
			'team',
			'organization',
			'file',
			'loader_manifest',
			'minecraft_manifest',
			'categories',
			'report_types',
			'loaders',
			'game_versions',
			'donation_platforms',
			'file_hash',
			'file_update',
			'search_results',
			'search_results_v3',
		])
		addNotification?.({
			title: 'Cache Purged',
			text: 'Application cache has been cleared successfully.',
			color: 'green',
		})
	} catch (e) {
		handleError(e)
	}
}

function handlePurgeCacheClick() {
	if (themeStore.getFeatureFlag('skip_non_essential_warnings')) {
		void purgeCache()
		return
	}

	purgeCacheConfirmModal.value?.show()
}

async function openDbBackupsFolder() {
	await showAppDbBackupsFolder().catch(handleError)
}

async function findLauncherDir() {
	const newDir = await open({
		multiple: false,
		directory: true,
		title: formatMessage(messages.selectAppDirectory),
	})

	if (newDir) {
		settings.value.custom_dir = newDir
	}
}
</script>

<template>
	<div class="flex flex-col gap-6">
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(messages.appDirectoryTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Manage data storage location and installation verbosity.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="flex flex-col gap-2">
					<label for="appDir" class="text-sm font-semibold text-contrast">App data location</label>
					<StyledInput
						id="appDir"
						v-model="settings.custom_dir"
						:icon="BoxIcon"
						type="text"
						wrapper-class="w-full"
					>
						<template #right>
							<IconButton
								v-tooltip="formatMessage(messages.browseAppDirectory)"
								:label="formatMessage(messages.browseAppDirectory)"
								class="ml-1.5"
								@click="findLauncherDir"
							>
								<FolderSearchIcon aria-hidden="true" />
							</IconButton>
						</template>
					</StyledInput>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.appDirectoryDescription) }}
					</p>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label
							for="always-show-copy-details"
							class="text-sm font-semibold text-contrast cursor-pointer"
						>
							{{ formatMessage(messages.alwaysShowCopyDetailsTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.alwaysShowCopyDetailsDescription) }}
						</p>
					</div>
					<Toggle
						id="always-show-copy-details"
						:model-value="themeStore.getFeatureFlag('always_show_copy_details')"
						@update:model-value="
							(val) => {
								themeStore.setFeatureFlag('always_show_copy_details', !!val)
								if (settings.feature_flags) settings.feature_flags.always_show_copy_details = !!val
							}
						"
					/>
				</div>
			</div>
		</section>

		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Network & Disk I/O</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Tune download concurrency and file write threads.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="flex flex-col gap-2">
					<label for="max-downloads" class="text-sm font-semibold text-contrast">
						{{ formatMessage(messages.maximumConcurrentDownloadsTitle) }}
					</label>
					<Slider
						id="max-downloads"
						v-model="settings.max_concurrent_downloads"
						:min="1"
						:max="10"
						:step="1"
					/>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.maximumConcurrentDownloadsDescription) }}
					</p>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="flex flex-col gap-2">
					<label for="max-writes" class="text-sm font-semibold text-contrast">
						{{ formatMessage(messages.maximumConcurrentWritesTitle) }}
					</label>
					<Slider
						id="max-writes"
						v-model="settings.max_concurrent_writes"
						:min="1"
						:max="50"
						:step="1"
					/>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.maximumConcurrentWritesDescription) }}
					</p>
				</div>
			</div>
		</section>

		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Maintenance & Backups</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Perform database backups and clean cached project assets.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="flex items-center justify-between gap-4">
					<div class="flex flex-col gap-0.5">
						<span class="text-sm font-semibold text-contrast">
							{{ formatMessage(messages.appCacheTitle) }}
						</span>
						<p class="m-0 text-xs text-secondary leading-relaxed max-w-lg">
							{{ formatMessage(messages.appCacheDescription) }}
						</p>
					</div>
					<Button
						id="purge-cache"
						type="colored"
						color="red"
						class="shrink-0"
						@click="handlePurgeCacheClick"
					>
						<TrashIcon aria-hidden="true" />
						{{ formatMessage(messages.purgeCache) }}
					</Button>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="flex items-center justify-between gap-4">
					<div class="flex flex-col gap-0.5">
						<span class="text-sm font-semibold text-contrast">
							{{ formatMessage(messages.appDatabaseBackupsTitle) }}
						</span>
						<p class="m-0 text-xs text-secondary leading-relaxed max-w-lg">
							{{ formatMessage(messages.appDatabaseBackupsDescription) }}
						</p>
					</div>
					<Button id="open-db-backups-folder" class="shrink-0" @click="openDbBackupsFolder">
						<FolderOpenIcon aria-hidden="true" />
						{{ formatMessage(messages.openBackupsFolder) }}
					</Button>
				</div>
			</div>

			<ConfirmModalWrapper
				ref="purgeCacheConfirmModal"
				:title="formatMessage(messages.purgeCacheConfirmTitle)"
				:description="formatMessage(messages.purgeCacheConfirmDescription)"
				:has-to-type="false"
				:proceed-label="formatMessage(messages.purgeCache)"
				:show-ad-on-close="false"
				@proceed="purgeCache"
			/>
		</section>
	</div>
</template>
