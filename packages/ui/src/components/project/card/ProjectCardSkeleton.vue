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
		v-if="layout === 'grid'"
		class="flex flex-col rounded-2xl bg-surface-2 border border-surface-4 shadow-sm overflow-hidden relative group"
	>
		<!-- Grid banner skeleton -->
		<div class="aspect-[2/1] w-full bg-surface-3/60 animate-pulse border-b border-surface-4" />

		<div class="p-4 flex flex-col gap-3 grow">
			<div class="flex gap-3">
				<div
					class="w-16 h-16 rounded-2xl bg-surface-3/80 animate-pulse shrink-0 border border-surface-4/40"
				/>
				<div class="flex flex-col gap-2 w-full min-w-0">
					<div class="h-5 w-36 bg-surface-3 rounded-lg animate-pulse" />
					<div class="h-3.5 w-[90%] bg-surface-3/60 rounded-md animate-pulse" />
					<div class="h-3.5 w-[65%] bg-surface-3/40 rounded-md animate-pulse" />
				</div>
			</div>

			<div
				class="mt-auto pt-2 flex items-center justify-between gap-2 border-t border-surface-4/40"
			>
				<div class="flex items-center gap-1.5">
					<div class="h-5 w-16 bg-surface-3/80 rounded-lg animate-pulse" />
					<div class="h-5 w-12 bg-surface-3/60 rounded-lg animate-pulse" />
				</div>
				<div class="h-4 w-16 bg-surface-3/50 rounded-md animate-pulse shrink-0" />
			</div>
		</div>
	</div>

	<div
		v-else
		class="p-4 flex flex-col justify-between gap-3 min-h-[152px] rounded-2xl bg-surface-2 border border-surface-4 shadow-sm overflow-hidden relative"
	>
		<div class="flex items-start gap-3 w-full min-w-0">
			<!-- Icon skeleton -->
			<div
				class="w-16 h-16 rounded-2xl bg-surface-3/80 animate-pulse shrink-0 mt-0.5 border border-surface-4/40"
			/>

			<div class="flex flex-col gap-2 min-w-0 flex-1">
				<!-- Title & Author line -->
				<div class="flex items-center gap-2 flex-wrap">
					<div class="h-5 w-36 bg-surface-3 rounded-lg animate-pulse" />
					<div class="h-4 w-20 bg-surface-3/60 rounded-md animate-pulse" />
				</div>
				<!-- Summary lines -->
				<div class="flex flex-col gap-1.5 w-full">
					<div class="h-3.5 w-[85%] bg-surface-3/60 rounded-md animate-pulse" />
					<div class="h-3.5 w-[60%] bg-surface-3/40 rounded-md animate-pulse" />
				</div>
			</div>

			<!-- Action button skeleton (e.g. + Install) -->
			<div
				class="h-8 w-20 rounded-xl bg-surface-3/90 animate-pulse shrink-0 ml-auto border border-surface-4/40"
			/>
		</div>

		<!-- Bottom row skeleton (tags, stats, date) -->
		<div
			class="mt-auto pt-2 flex items-center justify-between gap-2 border-t border-surface-4/40 overflow-hidden"
		>
			<!-- Tag chips -->
			<div class="flex items-center gap-1.5 flex-wrap">
				<div class="h-5 w-16 bg-surface-3/80 rounded-lg animate-pulse" />
				<div class="h-5 w-14 bg-surface-3/60 rounded-lg animate-pulse" />
				<div class="h-5 w-12 bg-surface-3/50 rounded-lg animate-pulse" />
			</div>
			<!-- Stats -->
			<div class="flex items-center gap-3 shrink-0 ml-auto">
				<div class="h-4 w-16 bg-surface-3/50 rounded-md animate-pulse" />
				<div class="h-4 w-12 bg-surface-3/40 rounded-md animate-pulse" />
				<div class="h-4 w-14 bg-surface-3/40 rounded-md animate-pulse" />
			</div>
		</div>
	</div>
</template>
