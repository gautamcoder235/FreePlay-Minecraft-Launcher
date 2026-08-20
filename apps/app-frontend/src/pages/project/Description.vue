<template>
	<div class="rounded-2xl bg-surface-2 border border-surface-4 p-6 sm:p-8 shadow-sm transition-all">
		<ProjectPageDescription :description="project.body" />
	</div>
</template>

<script setup>
import { ProjectPageDescription } from '@freeplay/ui'

defineProps({
	project: {
		type: Object,
		default: () => {},
	},
})
</script>

<script>
export default {
	name: 'Description',
}
</script>
