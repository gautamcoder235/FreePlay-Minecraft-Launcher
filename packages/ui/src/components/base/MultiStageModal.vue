<template>
	<NewModal
		ref="modal"
		:scrollable="true"
		max-content-height="72vh"
		:on-hide="onModalHide"
		:closable="true"
		:close-on-click-outside="closeOnClickOutside"
		:width="resolvedMaxWidth"
		:fade="fade"
		:disable-close="resolveCtxFn(currentStage?.disableClose, context)"
		class="!rounded-3xl !bg-[#090B0F]/95 backdrop-blur-2xl !border !border-white/10 !shadow-[0_0_50px_rgba(0,0,0,0.8)] overflow-hidden"
	>
		<template #title>
			<div
				v-if="breadcrumbs && !resolveCtxFn(currentStage?.nonProgressStage, context)"
				class="relative w-full flex items-center gap-3 min-w-0"
			>
				<!-- Glowing Step Indicator Badge -->
				<span
					v-if="stepProgressText"
					class="shrink-0 font-mono text-[11px] font-bold px-2.5 py-0.5 rounded-full bg-indigo-500/15 text-indigo-300 border border-indigo-500/30 shadow-[0_0_12px_rgba(99,102,241,0.25)] uppercase tracking-wider select-none"
				>
					{{ stepProgressText }}
				</span>

				<div class="relative flex-1 min-w-0">
					<div
						class="pointer-events-none absolute left-0 top-0 bottom-0 w-8 bg-gradient-to-r from-[#090B0F] to-transparent z-10 transition-opacity duration-200"
						:class="showLeftShadow ? 'opacity-100' : 'opacity-0'"
					/>
					<div
						ref="breadcrumbScroller"
						class="flex w-full items-center overflow-x-auto overflow-y-hidden scrollbar-hide pr-6 py-1 gap-1.5"
						@wheel.prevent="onBreadcrumbWheel"
						@scroll="updateScrollShadows"
					>
						<template v-for="(stage, index) in breadcrumbStages" :key="stage.id">
							<div
								:ref="(el) => setBreadcrumbRef(stage.id, el as HTMLElement | null)"
								class="flex shrink-0 items-center gap-1.5"
							>
								<button
									class="bg-transparent active:scale-95 text-xs font-semibold py-1 px-2 rounded-lg transition-all duration-150"
									:class="{
										'!text-white !font-bold bg-white/10 border border-white/15 shadow-[0_0_12px_rgba(255,255,255,0.08)]':
											resolveCtxFn(currentStage?.id, context) === stage.id,
										'text-sky-400 hover:text-sky-300 hover:bg-sky-500/10': isStageCompleted(
											stage.id,
										),
										'text-zinc-400 hover:text-zinc-200 hover:bg-white/5':
											resolveCtxFn(currentStage?.id, context) !== stage.id &&
											!isStageCompleted(stage.id),
										'opacity-40 cursor-not-allowed': cannotNavigateToStage(index),
									}"
									:disabled="cannotNavigateToStage(index)"
									@click="setStage(stage.id)"
								>
									<span v-if="isStageCompleted(stage.id)" class="mr-1 text-sky-400">✓</span>
									{{ resolveCtxFn(stage.title, context) }}
								</button>
								<ChevronRightIcon
									v-if="index < breadcrumbStages.length - 1"
									class="h-3.5 w-3.5 text-zinc-600 shrink-0"
									stroke-width="2.5"
								/>
							</div>
						</template>
					</div>
					<div
						class="pointer-events-none absolute right-0 top-0 bottom-0 w-8 bg-gradient-to-l from-[#090B0F] to-transparent z-10 transition-opacity duration-200"
						:class="showRightShadow ? 'opacity-100' : 'opacity-0'"
					/>
				</div>
			</div>
			<div v-else class="flex items-center gap-3 min-w-0">
				<!-- Glowing Step Indicator Badge -->
				<span
					v-if="stepProgressText && !resolveCtxFn(currentStage?.nonProgressStage, context)"
					class="shrink-0 font-mono text-[11px] font-bold px-2.5 py-0.5 rounded-full bg-indigo-500/15 text-indigo-300 border border-indigo-500/30 shadow-[0_0_12px_rgba(99,102,241,0.25)] uppercase tracking-wider select-none"
				>
					{{ stepProgressText }}
				</span>
				<span class="text-base sm:text-lg font-bold text-white tracking-wide truncate">
					{{ resolvedTitle }}
				</span>
			</div>
		</template>

		<!-- Glowing Top Progress Bar -->
		<div
			v-if="nonProgressStage !== true && !disableProgress"
			class="absolute top-0 left-0 right-0 h-1 bg-white/5 overflow-hidden z-20"
		>
			<div
				class="h-full bg-gradient-to-r from-indigo-500 via-sky-400 to-cyan-300 shadow-[0_0_12px_rgba(56,189,248,0.6)] transition-all duration-300 ease-out"
				:style="{ width: `${progressValue}%` }"
			/>
		</div>

		<component :is="currentStage?.stageContent" />

		<template #actions>
			<div
				class="flex flex-col justify-end gap-2.5 sm:flex-row"
				:class="leftButtonConfig || rightButtonConfig ? 'mt-3 pt-3 border-t border-white/10' : ''"
			>
				<Button
					v-if="leftButtonConfig"
					v-tooltip="leftButtonConfig.tooltip"
					type="outlined"
					class="!rounded-xl border-white/10 hover:border-white/20 bg-white/5 hover:bg-white/10 text-zinc-300 hover:text-white"
					:class="leftButtonConfig.buttonClass"
					:disabled="leftButtonConfig.disabled"
					@click="leftButtonConfig.onClick"
				>
					<component :is="leftButtonConfig.icon" />
					{{ leftButtonConfig.label }}
				</Button>
				<Button
					v-if="rightButtonConfig"
					v-tooltip="rightButtonConfig.tooltip"
					:type="
						rightButtonConfig.color && rightButtonConfig.color !== 'standard' ? 'colored' : 'base'
					"
					:color="rightButtonConfig.color === 'standard' ? undefined : rightButtonConfig.color"
					class="!rounded-xl shadow-lg transition-all duration-200"
					:class="[
						rightButtonConfig.buttonClass,
						rightButtonConfig.color === 'brand' ||
						(!rightButtonConfig.color && !rightButtonConfig.disabled)
							? '!bg-gradient-to-r !from-sky-600 !to-cyan-500 hover:!from-sky-500 hover:!to-cyan-400 !text-white !border-0 shadow-[0_0_20px_rgba(56,189,248,0.3)]'
							: '',
					]"
					:disabled="rightButtonConfig.disabled || rightButtonConfig.loading"
					@click="rightButtonConfig.onClick"
				>
					<SpinnerIcon
						v-if="rightButtonConfig.loading && rightButtonConfig.iconPosition === 'before'"
						class="animate-spin"
					/>
					<component
						:is="rightButtonConfig.icon"
						v-else-if="rightButtonConfig.iconPosition === 'before'"
						:class="rightButtonConfig.iconClass"
					/>
					{{ rightButtonConfig.label }}
					<SpinnerIcon
						v-if="rightButtonConfig.loading && rightButtonConfig.iconPosition === 'after'"
						class="animate-spin"
					/>
					<component
						:is="rightButtonConfig.icon"
						v-else-if="rightButtonConfig.iconPosition === 'after'"
						:class="rightButtonConfig.iconClass"
					/>
				</Button>
			</div>
		</template>
	</NewModal>
</template>

<script lang="ts">
import { ChevronRightIcon, SpinnerIcon } from '@freeplay/assets'
import { NewModal } from '@freeplay/ui'
import type { Component } from 'vue'
import { computed, nextTick, ref, useTemplateRef, watch } from 'vue'

import type { ButtonColor } from '#ui/components/base/buttons'
import { Button } from '#ui/components/base/buttons'

export interface StageButtonConfig {
	label?: string
	icon?: Component | null
	iconPosition?: 'before' | 'after'
	color?: ButtonColor | 'standard'
	disabled?: boolean
	loading?: boolean
	tooltip?: string
	iconClass?: string | null
	buttonClass?: string | null
	onClick?: () => void
}

export type MaybeCtxFn<T, R> = R | ((ctx: T) => R)

export interface StageConfigInput<T> {
	id: string
	stageContent: Component
	title: MaybeCtxFn<T, string>
	skip?: MaybeCtxFn<T, boolean>
	hideStageInBreadcrumb?: MaybeCtxFn<T, boolean>
	nonProgressStage?: MaybeCtxFn<T, boolean>
	cannotNavigateForward?: MaybeCtxFn<T, boolean>
	disableClose?: MaybeCtxFn<T, boolean>
	leftButtonConfig: MaybeCtxFn<T, StageButtonConfig | null>
	rightButtonConfig: MaybeCtxFn<T, StageButtonConfig | null>
	maxWidth?: MaybeCtxFn<T, string>
}

export function resolveCtxFn<T, R>(value: MaybeCtxFn<T, R> | undefined, ctx: T): R | undefined {
	return typeof value === 'function' ? (value as (ctx: T) => R)(ctx) : value
}
</script>

<script setup lang="ts" generic="T">
const props = withDefaults(
	defineProps<{
		stages: StageConfigInput<T>[]
		context: T
		breadcrumbs?: boolean
		fitContent?: boolean
		fade?: 'standard' | 'warning' | 'danger'
		disableProgress?: boolean
		closeOnClickOutside?: boolean
	}>(),
	{
		closeOnClickOutside: true,
	},
)

const modal = useTemplateRef<InstanceType<typeof NewModal>>('modal')
const currentStageIndex = ref<number>(0)

function show() {
	modal.value?.show()
}

function hide() {
	modal.value?.hide()
}

const setStage = (indexOrId: number | string) => {
	let index: number = 0
	if (typeof indexOrId === 'number') {
		index = indexOrId
		if (index < 0 || index >= props.stages.length) return
	} else {
		index = props.stages.findIndex((stage) => stage.id === indexOrId)
		if (index === -1) return
	}
	while (index < props.stages.length) {
		const skip = props.stages[index]?.skip
		if (!skip || !resolveCtxFn(skip, props.context)) break
		index++
	}
	if (index < props.stages.length) {
		currentStageIndex.value = index
	}
}

const nextStage = () => {
	if (currentStageIndex.value === -1) return
	if (currentStageIndex.value >= props.stages.length - 1) return
	let nextIndex = currentStageIndex.value + 1
	while (nextIndex < props.stages.length) {
		const skip = props.stages[nextIndex]?.skip
		if (!skip || !resolveCtxFn(skip, props.context)) break
		nextIndex++
	}
	if (nextIndex < props.stages.length) {
		currentStageIndex.value = nextIndex
	}
}

const prevStage = () => {
	if (currentStageIndex.value <= 0) return
	let prevIndex = currentStageIndex.value - 1
	while (prevIndex >= 0) {
		const skip = props.stages[prevIndex]?.skip
		if (!skip || !resolveCtxFn(skip, props.context)) break
		prevIndex--
	}
	if (prevIndex >= 0) {
		currentStageIndex.value = prevIndex
	}
}

const currentStage = computed(() => props.stages[currentStageIndex.value])

const resolvedTitle = computed(() => {
	const stage = currentStage.value
	if (!stage) return ''
	return resolveCtxFn(stage.title, props.context) ?? ''
})

const leftButtonConfig = computed(() => {
	const stage = currentStage.value
	if (!stage) return null
	return resolveCtxFn(stage.leftButtonConfig, props.context) ?? null
})

const rightButtonConfig = computed(() => {
	const stage = currentStage.value
	if (!stage) return null
	return resolveCtxFn(stage.rightButtonConfig, props.context) ?? null
})

const nonProgressStage = computed(() => {
	const stage = currentStage.value
	if (!stage) return false
	return resolveCtxFn(stage.nonProgressStage, props.context) ?? false
})

const resolvedMaxWidth = computed(() => {
	const stage = currentStage.value
	if (!stage?.maxWidth) return '560px'
	return resolveCtxFn(stage.maxWidth, props.context) ?? '560px'
})

const breadcrumbStages = computed(() => {
	return props.stages.filter((stage) => {
		const visibleStep =
			!resolveCtxFn(stage.skip, props.context) &&
			!resolveCtxFn(stage.nonProgressStage, props.context) &&
			!resolveCtxFn(stage.hideStageInBreadcrumb, props.context)
		return visibleStep
	})
})

const currentVisibleIndex = computed(() => {
	const current = currentStage.value
	if (!current) return 0
	const idx = breadcrumbStages.value.findIndex((s) => s.id === current.id)
	return idx >= 0 ? idx : currentStageIndex.value
})

const totalVisibleStages = computed(() => breadcrumbStages.value.length)

const stepProgressText = computed(() => {
	if (totalVisibleStages.value <= 1) return ''
	return `Step ${currentVisibleIndex.value + 1} of ${totalVisibleStages.value}`
})

function isStageCompleted(stageId: string): boolean {
	const current = currentStage.value
	if (!current) return false
	const stageIdx = props.stages.findIndex((s) => s.id === stageId)
	return stageIdx !== -1 && stageIdx < currentStageIndex.value
}

const progressValue = computed(() => {
	const isProgressStage = (stage: StageConfigInput<T>) => {
		if (resolveCtxFn(stage.nonProgressStage, props.context)) return false
		const skip = stage.skip ? resolveCtxFn(stage.skip, props.context) : false
		return !skip
	}

	const completedCount = props.stages
		.slice(0, currentStageIndex.value + 1)
		.filter(isProgressStage).length
	const totalCount = props.stages.filter(isProgressStage).length

	return totalCount > 0 ? (completedCount / totalCount) * 100 : 0
})

const breadcrumbScroller = ref<HTMLElement | null>(null)
const breadcrumbRefs = ref<Map<string, HTMLElement>>(new Map())
const showLeftShadow = ref(false)
const showRightShadow = ref(false)

function setBreadcrumbRef(stageId: string, el: HTMLElement | null) {
	if (el) breadcrumbRefs.value.set(stageId, el)
	else breadcrumbRefs.value.delete(stageId)
}

function scrollToCurrentBreadcrumb() {
	const stage = currentStage.value
	if (!stage || !breadcrumbScroller.value) return

	const el = breadcrumbRefs.value.get(stage.id)
	if (!el) return

	nextTick(() => {
		breadcrumbScroller.value?.scrollTo({
			left: el.offsetLeft - 50,
			behavior: 'smooth',
		})
	})
}

function updateScrollShadows() {
	const el = breadcrumbScroller.value
	if (!el) {
		showLeftShadow.value = false
		showRightShadow.value = false
		return
	}

	showLeftShadow.value = el.scrollLeft > 0
	showRightShadow.value = el.scrollLeft < el.scrollWidth - el.clientWidth - 1
}

function onBreadcrumbWheel(e: WheelEvent) {
	if (!breadcrumbScroller.value) return

	const el = breadcrumbScroller.value
	const canScrollHorizontally = el.scrollWidth > el.clientWidth

	if (canScrollHorizontally) {
		const delta = Math.abs(e.deltaX) > Math.abs(e.deltaY) ? e.deltaX : e.deltaY
		el.scrollLeft += delta
	}
}

function cannotNavigateToStage(breadcrumbIndex: number): boolean {
	const targetStage = breadcrumbStages.value[breadcrumbIndex]
	if (!targetStage) return false

	const targetStageIndex = props.stages.findIndex((s) => s.id === targetStage.id)
	if (targetStageIndex === -1) return false

	if (targetStageIndex <= currentStageIndex.value) return false

	for (let i = currentStageIndex.value; i < targetStageIndex; i++) {
		const stage = props.stages[i]
		if (stage.skip && resolveCtxFn(stage.skip, props.context)) continue
		if (resolveCtxFn(stage.cannotNavigateForward, props.context)) {
			return true
		}
	}

	return false
}

watch([breadcrumbStages, currentStageIndex], () => nextTick(() => updateScrollShadows()), {
	immediate: true,
})

watch(currentStageIndex, () => {
	scrollToCurrentBreadcrumb()
})

const emit = defineEmits<{
	(e: 'refresh-data' | 'hide'): void
}>()

function onModalHide() {
	emit('hide')
}

defineExpose({
	show,
	hide,
	setStage,
	nextStage,
	prevStage,
	currentStageIndex,
})
</script>

<style scoped>
.scrollbar-hide {
	-ms-overflow-style: none;
	scrollbar-width: none;
}

.scrollbar-hide::-webkit-scrollbar {
	display: none;
}

:deep(.modal-body button[aria-label*='Close'] svg),
:deep(.modal-body button[aria-label*='close'] svg),
:deep(.modal-body button:has(svg.lucide-x) svg) {
	transition:
		transform 0.3s cubic-bezier(0.4, 0, 0.2, 1),
		color 0.2s ease;
}

:deep(.modal-body button[aria-label*='Close']:hover svg),
:deep(.modal-body button[aria-label*='close']:hover svg),
:deep(.modal-body button:has(svg.lucide-x):hover svg) {
	transform: rotate(90deg);
}
</style>
