<template>
	<div class="h-full w-full">
		<FreePlayHostingView />
	</div>
</template>

<script setup lang="ts">
import { ServerStackIcon } from '@modrinth/assets'

import FreePlayHostingView from '@/pages/hosting/FreePlayHostingView.vue'
import { provideBreadcrumbParent, useBreadcrumb } from '@/providers/breadcrumbs'

const serverBreadcrumb = useBreadcrumb({
	slot: 'server',
	id: () => 'server:freeplay-server',
	label: 'FreePlay Server Control Room',
	visual: { type: 'icon', component: ServerStackIcon },
	to: () => '/hosting/manage',
})
provideBreadcrumbParent(serverBreadcrumb)
</script>

