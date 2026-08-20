import type { AbstractWebNotificationManager } from '@freeplay/ui'
import { provideTags } from '@freeplay/ui'
import { ref } from 'vue'

import { get_game_versions, get_loaders } from '@/helpers/tags'

export function setupTagsProvider(notificationManager: AbstractWebNotificationManager) {
	const { handleError } = notificationManager

	const gameVersions = ref([])
	const loaders = ref([])

	async function fetchTags() {
		try {
			const [versions, loaderList] = await Promise.all([
				get_game_versions().catch((err) => {
					handleError(err)
					return []
				}),
				get_loaders().catch((err) => {
					handleError(err)
					return []
				}),
			])
			if (versions) gameVersions.value = versions
			if (loaderList) loaders.value = loaderList
		} catch (error) {
			handleError(error)
		}
	}

	provideTags({ gameVersions, loaders })

	return {
		fetchTags,
	}
}
