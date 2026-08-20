<script setup lang="ts">
withDefaults(
	defineProps<{
		layout?: 'list' | 'grid' | 'gallery'
	}>(),
	{
		layout: 'list',
	},
)
</script>
<template>
	<div
		class="gap-3"
		:class="{
			'grid grid-cols-1 md:grid-cols-2': layout === 'list',
			'grid grid-project-list': layout === 'grid' || layout === 'gallery',
		}"
		role="list"
	>
		<slot />
	</div>
</template>
<style scoped lang="scss">
.grid-project-list {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
}
</style>
