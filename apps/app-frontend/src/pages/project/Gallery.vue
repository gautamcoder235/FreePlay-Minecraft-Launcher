<template>
	<div class="grid grid-cols-[repeat(auto-fill,minmax(18rem,1fr))] gap-4 w-full">
		<div
			v-for="(image, index) in filteredGallery"
			:key="image.url"
			class="group flex flex-col rounded-2xl bg-surface-2 border border-surface-4 shadow-sm overflow-hidden transition-all duration-300 hover:border-surface-5 hover:shadow-md cursor-pointer"
			@click="expandImage(image, index)"
		>
			<div class="relative w-full aspect-[16/9] overflow-hidden bg-surface-3">
				<img
					:src="image.url"
					:alt="image.title"
					class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
				/>
				<div
					class="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end p-3"
				>
					<span class="text-white text-xs font-semibold flex items-center gap-1">
						<ExpandIcon class="size-3.5" /> View full image
					</span>
				</div>
			</div>
			<div class="flex-1 p-4 flex flex-col justify-between gap-2">
				<div>
					<h3
						v-if="image.title"
						class="m-0 text-base font-bold text-contrast group-hover:text-brand transition-colors"
					>
						{{ image.title }}
					</h3>
					<p v-if="image.description" class="m-0 mt-1 text-xs text-secondary line-clamp-2">
						{{ image.description }}
					</p>
				</div>
				<span
					class="flex items-center gap-1.5 text-xs text-secondary pt-2 border-t border-surface-4/40 mt-auto"
				>
					<CalendarIcon class="size-3.5" />
					{{ formatDate(new Date(image.created)) }}
				</span>
			</div>
		</div>
	</div>
	<Teleport to="#teleports">
		<div v-if="expandedGalleryItem" class="expanded-image-modal" @click="hideImage">
			<div class="content">
				<img
					class="image"
					:class="{ 'zoomed-in': zoomedIn }"
					:src="
						expandedGalleryItem.raw_url
							? expandedGalleryItem.raw_url
							: 'https://cdn.freeplay.app/placeholder-banner.svg'
					"
					:alt="expandedGalleryItem.title ? expandedGalleryItem.title : 'gallery-image'"
					@click.stop="() => {}"
				/>

				<div class="floating" @click.stop="() => {}">
					<div class="text">
						<h2 v-if="expandedGalleryItem.title">
							{{ expandedGalleryItem.title }}
						</h2>
						<p v-if="expandedGalleryItem.description">
							{{ expandedGalleryItem.description }}
						</p>
					</div>
					<div class="controls">
						<div class="buttons">
							<IconButton label="Close" class="close" @click="hideImage">
								<XIcon aria-hidden="true" />
							</IconButton>
							<ButtonLink
								class="open btn icon-only !w-9 !px-0 !rounded-full"
								target="_blank"
								:href="
									expandedGalleryItem.raw_url
										? expandedGalleryItem.raw_url
										: 'https://cdn.freeplay.app/placeholder-banner.svg'
								"
							>
								<ExternalIcon aria-hidden="true" />
							</ButtonLink>
							<IconButton label="Toggle zoom" @click="zoomedIn = !zoomedIn">
								<ExpandIcon v-if="!zoomedIn" aria-hidden="true" />
								<ContractIcon v-else aria-hidden="true" />
							</IconButton>
							<IconButton
								v-if="filteredGallery.length > 1"
								label="Previous image"
								class="previous"
								@click="previousImage()"
							>
								<LeftArrowIcon aria-hidden="true" />
							</IconButton>
							<IconButton
								v-if="filteredGallery.length > 1"
								label="Next image"
								class="next"
								@click="nextImage()"
							>
								<RightArrowIcon aria-hidden="true" />
							</IconButton>
						</div>
					</div>
				</div>
			</div>
		</div>
	</Teleport>
</template>

<script setup>
import {
	CalendarIcon,
	ContractIcon,
	ExpandIcon,
	ExternalIcon,
	LeftArrowIcon,
	RightArrowIcon,
	XIcon,
} from '@freeplay/assets'
import { ButtonLink, IconButton, useFormatDateTime } from '@freeplay/ui'
import { computed, onMounted, onUnmounted, ref } from 'vue'

const MC_SERVER_BANNER_NAME = '__mc_server_banner__'

const formatDate = useFormatDateTime({
	year: 'numeric',
	month: 'long',
	day: 'numeric',
})

const props = defineProps({
	project: {
		type: Object,
		default: () => ({}),
	},
})

const filteredGallery = computed(
	() => props.project.gallery?.filter((img) => img.title !== MC_SERVER_BANNER_NAME) ?? [],
)

const expandedGalleryItem = ref(null)
const expandedGalleryIndex = ref(0)
const zoomedIn = ref(false)

const hideImage = () => {
	expandedGalleryItem.value = null
}

const nextImage = () => {
	expandedGalleryIndex.value++
	if (expandedGalleryIndex.value >= filteredGallery.value.length) {
		expandedGalleryIndex.value = 0
	}
	expandedGalleryItem.value = filteredGallery.value[expandedGalleryIndex.value]
}

const previousImage = () => {
	expandedGalleryIndex.value--
	if (expandedGalleryIndex.value < 0) {
		expandedGalleryIndex.value = filteredGallery.value.length - 1
	}
	expandedGalleryItem.value = filteredGallery.value[expandedGalleryIndex.value]
}

const expandImage = (item, index) => {
	expandedGalleryItem.value = item
	expandedGalleryIndex.value = index
	zoomedIn.value = false
}

function keyListener(e) {
	if (expandedGalleryItem.value) {
		if (e.key === 'Escape') {
			e.preventDefault()
			hideImage()
		} else if (e.key === 'ArrowLeft') {
			e.preventDefault()
			previousImage()
		} else if (e.key === 'ArrowRight') {
			e.preventDefault()
			nextImage()
		}
	}
}

onMounted(() => {
	document.addEventListener('keydown', keyListener)
})

onUnmounted(() => {
	document.removeEventListener('keydown', keyListener)
})
</script>

<style scoped lang="scss">
.expanded-image-modal {
	position: fixed;
	z-index: 11;
	overflow: auto;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	background-color: #000000;
	background-color: rgba(0, 0, 0, 0.7);
	display: flex;
	justify-content: center;
	align-items: center;

	.content {
		position: relative;
		width: calc(100vw - 2 * var(--gap-lg));
		height: calc(100vh - 2 * var(--gap-lg));

		.circle-button {
			padding: 0.5rem;
			line-height: 1;
			display: flex;
			max-width: 2rem;
			color: var(--color-button-text);
			background-color: var(--color-button-bg);
			border-radius: var(--size-rounded-max);
			margin: 0;
			box-shadow: inset 0px -1px 1px rgb(17 24 39 / 10%);

			&:not(:last-child) {
				margin-right: 0.5rem;
			}

			&:hover {
				background-color: var(--color-button-bg-hover) !important;

				svg {
					color: var(--color-button-text-hover) !important;
				}
			}

			&:active {
				background-color: var(--color-button-bg-active) !important;

				svg {
					color: var(--color-button-text-active) !important;
				}
			}

			svg {
				height: 1rem;
				width: 1rem;
			}
		}

		.image {
			position: absolute;
			left: 50%;
			top: 50%;
			transform: translate(-50%, -50%);
			max-width: calc(100vw - 2 * var(--gap-lg));
			max-height: calc(100vh - 2 * var(--gap-lg));
			border-radius: var(--radius-lg);

			&.zoomed-in {
				object-fit: cover;
				width: auto;
				height: calc(100vh - 2 * var(--gap-lg));
				max-width: calc(100vw - 2 * var(--gap-lg));
			}
		}
		.floating {
			position: absolute;
			left: 50%;
			transform: translateX(-50%);
			bottom: var(--gap-md);
			display: flex;
			flex-direction: column;
			align-items: center;
			gap: var(--gap-md);
			transition: opacity 0.25s ease-in-out;
			opacity: 1;
			padding: 2rem 2rem 0 2rem;

			&:not(&:hover) {
				opacity: 0.4;
				.text {
					transform: translateY(2.5rem) scale(0.8);
					opacity: 0;
				}
				.controls {
					transform: translateY(0.25rem) scale(0.9);
				}
			}

			.text {
				display: flex;
				flex-direction: column;
				max-width: 40rem;
				transition:
					opacity 0.25s ease-in-out,
					transform 0.25s ease-in-out;
				text-shadow: 1px 1px 10px #000000d4;
				margin-bottom: 0.25rem;
				gap: 0.5rem;

				h2 {
					color: var(--dark-color-base);
					font-size: 1.25rem;
					text-align: center;
					margin: 0;
				}

				p {
					color: var(--dark-color-base);
					margin: 0;
				}
			}
			.controls {
				background-color: var(--color-raised-bg);
				padding: var(--gap-md);
				border-radius: var(--radius-md);
				transition:
					opacity 0.25s ease-in-out,
					transform 0.25s ease-in-out;
			}
		}
	}
}

.buttons {
	display: flex;
	gap: 0.5rem;
}
</style>
