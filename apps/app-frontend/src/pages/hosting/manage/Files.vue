<script setup lang="ts">
import {
	injectFreePlayClient,
	injectFreePlayServerContext,
	ServersManageFilesPage,
} from '@freeplay/ui'
import { useQueryClient } from '@tanstack/vue-query'

const client = injectFreePlayClient()
const { serverId } = injectFreePlayServerContext()
const queryClient = useQueryClient()

try {
	await queryClient.ensureQueryData({
		queryKey: ['files', serverId, '/'],
		queryFn: () => client.kyros.files_v0.listDirectory('/', 1, 2000),
		staleTime: 30_000,
	})
} catch {
	// Let mounted layouts' useQuery surface errors; do not fail route setup.
}
</script>

<template>
	<ServersManageFilesPage />
</template>
