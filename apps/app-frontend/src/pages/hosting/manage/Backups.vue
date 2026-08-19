<script setup lang="ts">
import {
	injectFreePlayClient,
	injectFreePlayServerContext,
	ServersManageBackupsPage,
} from '@freeplay/ui'
import { useQueryClient } from '@tanstack/vue-query'

const client = injectFreePlayClient()
const { serverId, worldId, isServerRunning } = injectFreePlayServerContext()
const queryClient = useQueryClient()

if (worldId.value) {
	try {
		await queryClient.ensureQueryData({
			queryKey: ['backups', 'list', serverId],
			queryFn: () => client.archon.backups_v1.list(serverId, worldId.value!),
			staleTime: 30_000,
		})
	} catch {
		// Let mounted layouts' useQuery surface errors; do not fail route setup.
	}
}
</script>

<template>
	<ServersManageBackupsPage :is-server-running="isServerRunning" />
</template>
