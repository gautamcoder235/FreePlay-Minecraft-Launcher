<template>
	<div
		class="project-description markdown-body"
		v-html="renderHighlightedString(description ?? '')"
	/>
</template>

<script setup lang="ts">
import { renderHighlightedString } from '@freeplay/utils'

withDefaults(
	defineProps<{
		description: string
	}>(),
	{},
)
</script>

<style scoped lang="scss">
.project-description {
	:deep(img) {
		border-radius: 0.75rem;
		max-width: 100%;
		height: auto;
		border: 1px solid rgba(255, 255, 255, 0.08);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
		margin: 1.25rem 0;
	}

	:deep(a) {
		color: var(--color-brand);
		text-decoration: underline;
		text-underline-offset: 2px;
		transition: color 0.15s ease;

		&:hover {
			color: var(--color-brand-highlight, #1bd96a);
		}
	}

	:deep(blockquote) {
		border-left: 3px solid var(--color-brand);
		background-color: var(--color-raised-bg);
		padding: 0.75rem 1.25rem;
		border-radius: 0 0.5rem 0.5rem 0;
		margin: 1.25rem 0;
	}

	:deep(table) {
		width: 100%;
		border-collapse: collapse;
		border-radius: 0.75rem;
		overflow: hidden;
		margin: 1.25rem 0;
		border: 1px solid var(--color-super-raised-bg);

		th {
			background-color: var(--color-raised-bg);
			padding: 0.75rem 1rem;
			font-weight: 600;
			text-align: left;
			border-bottom: 1px solid var(--color-super-raised-bg);
		}

		td {
			padding: 0.75rem 1rem;
			border-bottom: 1px solid rgba(255, 255, 255, 0.05);
		}
	}

	:deep(hr) {
		border: 0;
		border-top: 1px solid var(--color-super-raised-bg);
		margin: 2rem 0;
	}
}
</style>
