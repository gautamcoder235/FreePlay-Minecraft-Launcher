<script setup lang="ts">
import { ServerStackIcon } from '@freeplay/assets'

import FreePlayHostingView from '@/pages/hosting/FreePlayHostingView.vue'
import { useRootBreadcrumb } from '@/providers/breadcrumbs'

useRootBreadcrumb({
	slot: 'root',
	id: 'servers',
	label: 'Server Hosting',
	to: '/hosting/manage',
	visual: { type: 'icon', component: ServerStackIcon },
})
</script>

<template>
	<div class="h-full w-full">
		<FreePlayHostingView />
	</div>
</template>
