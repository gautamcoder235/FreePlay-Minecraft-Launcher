<script setup lang="ts">
import {
	CheckIcon,
	EditIcon,
	EyeIcon,
	RotateCounterClockwiseIcon,
	ShirtIcon,
	SpinnerIcon,
} from '@freeplay/assets'
import {
	commonMessages,
	ConfirmModal,
	defineMessages,
	injectAuth,
	injectFreePlayClient,
	injectNotificationManager,
	SkinPreviewRenderer,
	Toggle,
	useVIntl,
} from '@freeplay/ui'
import { arrayBufferToBase64 } from '@freeplay/utils'
import { useQuery } from '@tanstack/vue-query'
import { invoke } from '@tauri-apps/api/core'
import { type DragDropEvent, getCurrentWebview } from '@tauri-apps/api/webview'
import { computedAsync } from '@vueuse/core'
import type { Ref } from 'vue'
import { computed, inject, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue'

import EarsModIcon from '@/assets/skins/ears-mod.png'
import type AccountsCard from '@/components/ui/AccountsCard.vue'
import EditSkinModal from '@/components/ui/skin/EditSkinModal.vue'
import VirtualSkinSectionList from '@/components/ui/skin/VirtualSkinSectionList.vue'
import { check_reachable, get_default_user, users } from '@/helpers/auth'
import type { RenderResult } from '@/helpers/rendering/batch-skin-renderer.ts'
import {
	generateSkinPreviews,
	getSkinPreviewKey,
	skinBlobUrlMap,
} from '@/helpers/rendering/batch-skin-renderer.ts'
import type { Cape, Skin, SkinTextureUrl } from '@/helpers/skins.ts'
import {
	equip_skin,
	filterDefaultSkins,
	filterSavedSkins,
	flush_pending_skin_change,
	flush_pending_skin_change_for_profile,
	get_available_capes,
	get_available_skins,
	get_dragged_skin_data,
	get_normalized_skin_texture,
	normalize_skin_texture,
	remove_custom_skin,
	save_custom_skin,
	set_custom_skin_order,
} from '@/helpers/skins.ts'
import { hasPride26Badge } from '@/helpers/user-campaigns.ts'
import { useRootBreadcrumb } from '@/providers/breadcrumbs'
import { useTheming } from '@/store/state'
import { appMessages } from '@/utils/app-messages'

useRootBreadcrumb({
	slot: 'root',
	id: 'skins',
	label: 'Skin selector',
	to: '/skins',
	visual: { type: 'icon', component: ShirtIcon },
})

type UnlistenFn = () => void
type VirtualSkinSectionListExpose = {
	getAddSkinButtonElement: () => HTMLElement | null | undefined
}

const PENDING_SKIN_REFRESH_DELAY_MS = 11_000
const DEFAULT_SKIN_SECTION_SORT_ORDER = ['Default skins', 'FreePlay Pride']
const messages = defineMessages({
	freeplayPrideSection: {
		id: 'app.skins.section.freeplay-pride',
		defaultMessage: 'FreePlay Pride',
	},
	freeplayPrideTooltip: {
		id: 'app.skins.section.freeplay-pride.tooltip',
		defaultMessage:
			'You received these skins for donating to a FreePlay Pride fundraiser during Pride Month.',
	},
	freeplaySection: {
		id: 'app.skins.section.freeplay',
		defaultMessage: 'FreePlay',
	},
	defaultSkinsSection: {
		id: 'app.skins.section.default-skins',
		defaultMessage: 'Default skins',
	},
	mineconEarth2017Section: {
		id: 'app.skins.section.minecon-earth-2017',
		defaultMessage: 'MINECON Earth 2017',
	},
	buildersAndBiomesSection: {
		id: 'app.skins.section.builders-and-biomes',
		defaultMessage: 'Builders & Biomes',
	},
	stridingHeroSection: {
		id: 'app.skins.section.striding-hero',
		defaultMessage: 'Striding Hero',
	},
	theGardenAwakensSection: {
		id: 'app.skins.section.the-garden-awakens',
		defaultMessage: 'The Garden Awakens',
	},
	chaseTheSkiesSection: {
		id: 'app.skins.section.chase-the-skies',
		defaultMessage: 'Chase the Skies',
	},
	theCopperAgeSection: {
		id: 'app.skins.section.the-copper-age',
		defaultMessage: 'The Copper Age',
	},
	mountsOfMayhemSection: {
		id: 'app.skins.section.mounts-of-mayhem',
		defaultMessage: 'Mounts of Mayhem',
	},
	tinyTakeoverSection: {
		id: 'app.skins.section.tiny-takeover',
		defaultMessage: 'Tiny Takeover',
	},
	chaosCubedSection: {
		id: 'app.skins.section.chaos-cubed',
		defaultMessage: 'Chaos Cubed',
	},
	rateLimitTitle: {
		id: 'app.skins.rate-limit.title',
		defaultMessage: 'Slow down!',
	},
	rateLimitText: {
		id: 'app.skins.rate-limit.text',
		defaultMessage:
			"You're changing your skin too frequently. Mojang's servers have temporarily blocked further requests. Please wait a moment before trying again.",
	},
	droppedFileErrorTitle: {
		id: 'app.skins.dropped-file-error.title',
		defaultMessage: 'Error processing file',
	},
	droppedFileErrorText: {
		id: 'app.skins.dropped-file-error.text',
		defaultMessage: 'Failed to read the dropped file.',
	},
	reorderSkinErrorTitle: {
		id: 'app.skins.reorder-error.title',
		defaultMessage: 'Failed to reorder skins',
	},
	reorderSkinErrorText: {
		id: 'app.skins.reorder-error.text',
		defaultMessage: 'Your skin order could not be saved.',
	},
	deleteSkinTitle: {
		id: 'app.skins.delete-modal.title',
		defaultMessage: 'Are you sure you want to delete this skin?',
	},
	deleteSkinDescription: {
		id: 'app.skins.delete-modal.description',
		defaultMessage: 'This will permanently delete the selected skin. This action cannot be undone.',
	},
	previewingBadge: {
		id: 'app.skins.previewing-badge',
		defaultMessage: 'Previewing',
	},
	applyButton: {
		id: 'app.skins.apply-button',
		defaultMessage: 'Apply',
	},
	demoApplyTooltip: {
		id: 'app.skins.demo.apply-tooltip',
		defaultMessage: 'Sign in to apply skins.',
	},
	editSkinButton: {
		id: 'app.skins.preview.edit-button',
		defaultMessage: 'Edit skin',
	},
	earsFeatureNotice: {
		id: 'app.skins.ears-feature-notice',
		defaultMessage: 'This skin uses features from the {ears} mod',
	},
	toggleEarsFeaturesOff: {
		id: 'app.skins.toggle-ears-features-off',
		defaultMessage: 'Toggle off',
	},
	toggleEarsFeaturesOn: {
		id: 'app.skins.toggle-ears-features-on',
		defaultMessage: 'Toggle on',
	},
	demoTitle: {
		id: 'app.skins.demo.title',
		defaultMessage: 'Editing with a demo account',
	},
	demoDescription: {
		id: 'app.skins.demo.description',
		defaultMessage: 'Sign in to your Minecraft account to save and apply skins!',
	},
	signInButton: {
		id: 'app.skins.sign-in.button',
		defaultMessage: 'Sign in to Microsoft',
	},
})

const editSkinModal = useTemplateRef('editSkinModal')
const addSkinFileInput = useTemplateRef<HTMLInputElement>('addSkinFileInput')
const skinSectionList = useTemplateRef<VirtualSkinSectionListExpose>('skinSectionList')

const { formatMessage } = useVIntl()
const notifications = injectNotificationManager()
const { addNotification, handleError } = notifications
const auth = injectAuth()
const client = injectFreePlayClient()

const themeStore = useTheming()
const skins = ref<Skin[]>([])
const capes = ref<Cape[]>([])
const offline = ref(!navigator.onLine)

const accountsCard = inject('accountsCard') as Ref<typeof AccountsCard>
const currentUser = ref(undefined)
const currentUserId = ref<string | undefined>(undefined)

const username = computed(() => currentUser.value?.profile?.name ?? undefined)
const selectedSkin = ref<Skin | null>(null)
const isApplyingSkin = ref(false)
const earsFeaturesEnabled = ref(true)
const selectedSkinHasEarsFeatures = ref(false)

const originalSelectedSkin = ref<Skin | null>(null)

const savedSkins = computed(() => {
	try {
		return filterSavedSkins(skins.value)
	} catch (error) {
		handleError(error as Error)
		return []
	}
})
const authServerQuery = useQuery({
	queryKey: ['authServerReachability'],
	queryFn: async () => {
		await check_reachable()
		return true
	},
	refetchInterval: 5 * 60 * 1000,
	retry: false,
	refetchOnWindowFocus: false,
})
const { data: freeplayUser } = useQuery({
	queryKey: computed(() => ['authenticated-user', 'campaigns', auth.user.value?.id]),
	queryFn: () => client.labrinth.users_v3.getAuthenticated(),
	enabled: () => !!auth.session_token.value,
	retry: false,
})
const hasFreePlayPrideCampaign = computed(
	() => !!auth.session_token.value && hasPride26Badge(freeplayUser.value?.campaigns?.pride_26),
)
const defaultSkins = computed(() =>
	filterDefaultSkins(skins.value).filter(
		(skin) => skin.section !== 'FreePlay Pride' || hasFreePlayPrideCampaign.value,
	),
)
const defaultSkinSections = computed(() => {
	const sections = new Map<string, Skin[]>()

	for (const skin of defaultSkins.value) {
		const section = skin.section ?? 'Default skins'
		const sectionSkins = sections.get(section)

		if (sectionSkins) {
			sectionSkins.push(skin)
		} else {
			sections.set(section, [skin])
		}
	}

	return Array.from(sections, ([section, skins]) => ({
		section,
		title: getDefaultSkinSectionTitle(section),
		infoTooltip: getDefaultSkinSectionInfoTooltip(section),
		skins,
	})).sort(
		(a, b) => getDefaultSkinSectionSortIndex(a.section) - getDefaultSkinSectionSortIndex(b.section),
	)
})

const searchQuery = ref('')
const activeFilterCategory = ref<'all' | 'saved' | 'default'>('all')

const visibleSavedSkins = computed(() => {
	if (activeFilterCategory.value === 'default') return []
	if (!searchQuery.value.trim()) return savedSkins.value
	const q = searchQuery.value.toLowerCase().trim()
	return savedSkins.value.filter((s) => (s.name || 'custom skin').toLowerCase().includes(q))
})

const visibleDefaultSkinSections = computed(() => {
	if (activeFilterCategory.value === 'saved') return []
	if (!searchQuery.value.trim()) return defaultSkinSections.value
	const q = searchQuery.value.toLowerCase().trim()
	return defaultSkinSections.value
		.map((sec) => ({
			...sec,
			skins: sec.skins.filter((s) => (s.name || '').toLowerCase().includes(q)),
		}))
		.filter((sec) => sec.skins.length > 0)
})

const currentCape = computed(() => {
	if (selectedSkin.value?.cape_id) {
		const overrideCape = capes.value.find((c) => c.id === selectedSkin.value?.cape_id)
		if (overrideCape) {
			return overrideCape
		}
	}
	return undefined
})

const skinTexture = computedAsync(async () => {
	const skin = selectedSkin.value
	if (skin?.texture) {
		try {
			return await get_normalized_skin_texture(skin)
		} catch (error) {
			if (skin.texture.startsWith('data:image/')) {
				return skin.texture
			}

			handleError(error as Error)
			return ''
		}
	} else {
		return ''
	}
})
const capeTexture = computed(() => currentCape.value?.texture)
const skinVariant = computed(() => selectedSkin.value?.variant)
const skinNametag = computed(() => (themeStore.hideNametagSkinsPage ? undefined : username.value))
const isSkinManagementReadOnly = computed(
	() =>
		!!currentUser.value &&
		(offline.value || (authServerQuery.isError.value && !authServerQuery.isLoading.value)),
)
const hasPendingSkinChange = computed(
	() => !skinsMatch(selectedSkin.value, originalSelectedSkin.value),
)

let userCheckInterval: number | null = null
let pendingSkinRefreshTimeout: number | null = null
let unlistenAddSkinDragDrop: UnlistenFn | null = null
let isUnmounted = false

const isDraggingSkinFile = ref(false)
const isAddSkinButtonDragActive = ref(false)

const deleteSkinModal = ref()
const skinToDelete = ref<Skin | null>(null)

function confirmDeleteSkin(skin: Skin) {
	if (isSkinManagementReadOnly.value) return

	skinToDelete.value = skin
	deleteSkinModal.value?.show()
}

async function deleteSkin() {
	if (isSkinManagementReadOnly.value) return

	const deletedSkin = skinToDelete.value
	if (!deletedSkin) return

	try {
		await remove_custom_skin(deletedSkin)
		removeLocalSkin(deletedSkin)
	} catch (error) {
		handleError(error as Error)
	} finally {
		skinToDelete.value = null
	}
}

async function loadCapes() {
	try {
		capes.value = (await get_available_capes()) ?? []
	} catch (error) {
		if (currentUser.value && error instanceof Error) {
			handleError(error)
		}
	}
}

async function loadSkins() {
	try {
		const loadedSkins = (await get_available_skins()) ?? []
		const loadedEquippedSkin = loadedSkins.find((s) => s.is_equipped)
		const locallyKnownEquippedSkin =
			originalSelectedSkin.value &&
			(loadedSkins.find((skin) => skinsMatch(skin, originalSelectedSkin.value)) ??
				(originalSelectedSkin.value.texture.startsWith('data:image/')
					? originalSelectedSkin.value
					: undefined))
		const shouldPreserveKnownEquippedSkin =
			isSkinManagementReadOnly.value &&
			locallyKnownEquippedSkin &&
			!skinsMatch(loadedEquippedSkin, locallyKnownEquippedSkin)

		skins.value =
			shouldPreserveKnownEquippedSkin && locallyKnownEquippedSkin
				? mergeEquippedSkin(loadedSkins, locallyKnownEquippedSkin)
				: loadedSkins
		generateSkinPreviews(skins.value, capes.value)
		selectedSkin.value = skins.value.find((s) => s.is_equipped) ?? null
		originalSelectedSkin.value = selectedSkin.value
	} catch (error) {
		if (currentUser.value && error instanceof Error) {
			handleError(error)
		}
	}
}

function mergeEquippedSkin(list: Skin[], equippedSkin: Skin) {
	let foundEquippedSkin = false
	const mergedSkins = list.map((skin) => {
		const isEquipped = skinsMatch(skin, equippedSkin)
		foundEquippedSkin ||= isEquipped

		return {
			...skin,
			is_equipped: isEquipped,
		}
	})

	if (!foundEquippedSkin) {
		mergedSkins.unshift({
			...equippedSkin,
			is_equipped: true,
		})
	}

	return mergedSkins
}

function skinsMatch(a?: Skin | null, b?: Skin | null) {
	return (
		a?.source === b?.source &&
		a?.texture_key === b?.texture_key &&
		a?.variant === b?.variant &&
		(a?.cape_id ?? null) === (b?.cape_id ?? null)
	)
}

function skinsMatchIgnoringSource(a?: Skin | null, b?: Skin | null) {
	return (
		a?.texture_key === b?.texture_key &&
		a?.variant === b?.variant &&
		(a?.cape_id ?? null) === (b?.cape_id ?? null)
	)
}

function isSkinSelected(skin: Skin) {
	return skinsMatch(selectedSkin.value, skin)
}

function isSkinActive(skin: Skin) {
	return hasPendingSkinChange.value && skinsMatch(originalSelectedSkin.value, skin)
}

function getErrorMessage(error: unknown) {
	return error instanceof Error ? error.message : String(error)
}

function isMinecraftSkinRateLimitError(error: unknown) {
	const message = getErrorMessage(error)
	return message.includes('429 Too Many Requests') || message.includes('client error (429')
}

function getDefaultSkinSectionTitle(section?: string) {
	switch (section) {
		case 'FreePlay Pride':
			return formatMessage(messages.freeplayPrideSection)
		case 'FreePlay':
			return formatMessage(messages.freeplaySection)
		case 'MINECON Earth 2017':
			return formatMessage(messages.mineconEarth2017Section)
		case 'Builders & Biomes':
			return formatMessage(messages.buildersAndBiomesSection)
		case 'Striding Hero':
			return formatMessage(messages.stridingHeroSection)
		case 'The Garden Awakens':
			return formatMessage(messages.theGardenAwakensSection)
		case 'Chase the Skies':
			return formatMessage(messages.chaseTheSkiesSection)
		case 'The Copper Age':
			return formatMessage(messages.theCopperAgeSection)
		case 'Mounts of Mayhem':
			return formatMessage(messages.mountsOfMayhemSection)
		case 'Tiny Takeover':
			return formatMessage(messages.tinyTakeoverSection)
		case 'Chaos Cubed':
			return formatMessage(messages.chaosCubedSection)
		case 'Default skins':
			return formatMessage(messages.defaultSkinsSection)
		default:
			return section ?? formatMessage(messages.defaultSkinsSection)
	}
}

function getDefaultSkinSectionInfoTooltip(section: string) {
	switch (section) {
		case 'FreePlay Pride':
			return formatMessage(messages.freeplayPrideTooltip)
		default:
			return undefined
	}
}

function getDefaultSkinSectionSortIndex(section: string) {
	const index = DEFAULT_SKIN_SECTION_SORT_ORDER.indexOf(section)
	return index === -1 ? DEFAULT_SKIN_SECTION_SORT_ORDER.length : index
}

function changeSkin(newSkin: Skin) {
	if (isSkinManagementReadOnly.value) return

	selectedSkin.value = newSkin
}

function resetSelectedSkin() {
	selectedSkin.value =
		skins.value.find((skin) => skinsMatch(skin, originalSelectedSkin.value)) ??
		originalSelectedSkin.value
}

function removeLocalSkin(deletedSkin: Skin) {
	const nextSkins = skins.value.filter((skin) => !skinsMatch(skin, deletedSkin))
	skins.value = nextSkins

	if (selectedSkin.value && skinsMatch(selectedSkin.value, deletedSkin)) {
		selectedSkin.value =
			nextSkins.find((skin) => skinsMatch(skin, originalSelectedSkin.value)) ??
			nextSkins.find((skin) => skin.is_equipped) ??
			null
	}

	if (originalSelectedSkin.value && skinsMatch(originalSelectedSkin.value, deletedSkin)) {
		originalSelectedSkin.value = nextSkins.find((skin) => skin.is_equipped) ?? null
	}

	generateSkinPreviews(skins.value, capes.value)
}

function setLocallyEquippedSkin(skinToApply: Skin) {
	skins.value = skins.value.map((skin) => ({
		...skin,
		is_equipped: skinsMatch(skin, skinToApply),
	}))
	originalSelectedSkin.value =
		skins.value.find((skin) => skinsMatch(skin, skinToApply)) ?? skinToApply
	selectedSkin.value = originalSelectedSkin.value
	void accountsCard.value?.setEquippedSkin(originalSelectedSkin.value)
}

function insertLocalSkin(savedSkin: Skin) {
	const firstNonCustomSkinIndex = skins.value.findIndex((skin) => skin.source !== 'custom')

	if (firstNonCustomSkinIndex === -1) {
		skins.value = [...skins.value, savedSkin]
		return
	}

	const nextSkins = [...skins.value]
	nextSkins.splice(firstNonCustomSkinIndex, 0, savedSkin)
	skins.value = nextSkins
}

function updateLocalSkin(savedSkin: Skin, applied: boolean, previousSkin?: Skin) {
	let foundSkin = false
	const replacesSelectedSkin =
		selectedSkin.value?.texture_key === savedSkin.texture_key ||
		(previousSkin ? skinsMatch(selectedSkin.value, previousSkin) : false)
	const replacesOriginalSkin =
		originalSelectedSkin.value?.texture_key === savedSkin.texture_key ||
		(previousSkin ? skinsMatch(originalSelectedSkin.value, previousSkin) : false)

	skins.value = skins.value.map((skin) => {
		const isUpdatedSkin = skin.texture_key === savedSkin.texture_key
		const isPreviousSkin = previousSkin && skinsMatch(skin, previousSkin)

		if (isUpdatedSkin || isPreviousSkin) {
			foundSkin = true
			return {
				...savedSkin,
				is_equipped: applied || savedSkin.is_equipped,
			}
		}

		return {
			...skin,
			is_equipped: applied ? false : skin.is_equipped,
		}
	})

	if (!foundSkin) {
		insertLocalSkin({
			...savedSkin,
			is_equipped: applied || savedSkin.is_equipped,
		})
	}

	if (applied) {
		const locallyEquippedSkin =
			skins.value.find((skin) => skin.texture_key === savedSkin.texture_key) ?? savedSkin

		originalSelectedSkin.value = locallyEquippedSkin
		selectedSkin.value = locallyEquippedSkin
		void accountsCard.value?.setEquippedSkin(locallyEquippedSkin)
	} else {
		const locallySavedSkin =
			skins.value.find((skin) => skin.texture_key === savedSkin.texture_key) ?? savedSkin

		if (replacesSelectedSkin) {
			selectedSkin.value = locallySavedSkin
		}

		if (replacesOriginalSkin) {
			originalSelectedSkin.value = locallySavedSkin
		}
	}

	generateSkinPreviews(skins.value, capes.value)
}

async function reorderSavedSkins(orderedSkins: Skin[]) {
	const previousSkins = skins.value
	const previousSelectedSkin = selectedSkin.value
	const previousOriginalSelectedSkin = originalSelectedSkin.value
	const orderedTextureKeys = orderedSkins.map((skin) => skin.texture_key)
	const orderedTextureKeySet = new Set(orderedTextureKeys)
	const remainingSavedSkins = previousSkins.filter(
		(skin) => skin.source !== 'default' && !orderedTextureKeySet.has(skin.texture_key),
	)
	const defaultSkins = previousSkins.filter((skin) => skin.source === 'default')
	const nextSavedSkins = [...orderedSkins, ...remainingSavedSkins]

	skins.value = [...nextSavedSkins, ...defaultSkins]
	generateSkinPreviews(skins.value, capes.value)

	try {
		const persistedSavedSkins = await preserveExternalSkins(nextSavedSkins)

		if (persistedSavedSkins.some((skin, index) => skin !== nextSavedSkins[index])) {
			skins.value = [...persistedSavedSkins, ...defaultSkins]
			generateSkinPreviews(skins.value, capes.value)
		}

		await set_custom_skin_order(
			persistedSavedSkins
				.filter((skin) => skin.source === 'custom')
				.map((skin) => skin.texture_key),
		)
	} catch (error) {
		skins.value = previousSkins
		selectedSkin.value = previousSelectedSkin
		originalSelectedSkin.value = previousOriginalSelectedSkin
		generateSkinPreviews(skins.value, capes.value)
		addNotification({
			type: 'error',
			title: formatMessage(messages.reorderSkinErrorTitle),
			text: error instanceof Error ? error.message : formatMessage(messages.reorderSkinErrorText),
		})
		await loadSkins()
	}
}

async function preserveExternalSkins(skinsToPersist: Skin[]) {
	const preservedSkins: Skin[] = []

	for (const skin of skinsToPersist) {
		if (skin.source !== 'custom_external') {
			preservedSkins.push(skin)
			continue
		}

		const textureBlob = await normalize_skin_texture(skin.texture)
		const capeId = skin.cape_id ? capes.value.find((cape) => cape.id === skin.cape_id) : undefined
		const savedSkin = await save_custom_skin(skin, textureBlob, skin.variant, capeId, false)
		const preservedSkin: Skin = {
			...savedSkin,
			source: 'custom',
			is_equipped: skin.is_equipped,
		}

		if (skinsMatchIgnoringSource(selectedSkin.value, skin)) {
			selectedSkin.value = preservedSkin
		}

		if (skinsMatchIgnoringSource(originalSelectedSkin.value, skin)) {
			originalSelectedSkin.value = preservedSkin
			void accountsCard.value?.setEquippedSkin(preservedSkin)
		}

		preservedSkins.push(preservedSkin)
	}

	return preservedSkins
}

function schedulePendingSkinRefresh() {
	if (pendingSkinRefreshTimeout !== null) {
		window.clearTimeout(pendingSkinRefreshTimeout)
	}

	const pendingProfileId = currentUserId.value

	pendingSkinRefreshTimeout = window.setTimeout(async () => {
		pendingSkinRefreshTimeout = null

		if (isUnmounted) {
			return
		}

		try {
			if (pendingProfileId) {
				await flush_pending_skin_change_for_profile(pendingProfileId)
			} else {
				await flush_pending_skin_change()
			}
		} catch (error) {
			handleError(error as Error)
			schedulePendingSkinRefresh()
			return
		}

		if (accountsCard.value) {
			await accountsCard.value.refreshValues()
		}

		await loadCapes()
		await loadSkins()
	}, PENDING_SKIN_REFRESH_DELAY_MS)
}

async function applySelectedSkin() {
	const skinToApply = selectedSkin.value
	if (
		!skinToApply ||
		!hasPendingSkinChange.value ||
		isApplyingSkin.value ||
		isSkinManagementReadOnly.value
	)
		return

	isApplyingSkin.value = true
	try {
		if (currentUser.value) {
			await equip_skin(skinToApply)
			setLocallyEquippedSkin(skinToApply)
			schedulePendingSkinRefresh()
		} else {
			const usernameToUse = username.value || 'Player'
			if (skinToApply.texture) {
				try {
					const texData = await normalize_skin_texture(skinToApply.texture)
					await invoke('plugin:minecraft-skins|set_offline_user_skin', {
						username: usernameToUse,
						pngBytes: Array.from(new Uint8Array(texData)),
						isSlim: skinToApply.variant === 'SLIM',
					})
				} catch (err) {
					console.debug('Offline skin persist:', err)
				}
			}
			setLocallyEquippedSkin(skinToApply)
			notifications.addNotification({
				type: 'success',
				title: 'Skin Equipped',
				text: `Equipped ${skinToApply.name || 'skin'} for ${usernameToUse}!`,
			})
		}
	} catch (error) {
		if (isMinecraftSkinRateLimitError(error)) {
			notifications.addNotification({
				type: 'error',
				title: formatMessage(messages.rateLimitTitle),
				text: formatMessage(messages.rateLimitText),
			})
		} else {
			handleError(error as Error)
		}
	} finally {
		isApplyingSkin.value = false
	}
}

async function onSkinSaved(options: { applied: boolean; skin?: Skin; previousSkin?: Skin }) {
	if (options.skin) {
		updateLocalSkin(options.skin, options.applied, options.previousSkin)
	}

	if (!options.skin) {
		await loadCapes()
		await loadSkins()
	}

	if (options.applied) {
		schedulePendingSkinRefresh()
	}
}

async function loadCurrentUser() {
	try {
		const defaultId = await get_default_user()
		currentUserId.value = defaultId

		const allAccounts = await users()
		currentUser.value = allAccounts.find((acc) => acc.profile.id === defaultId)
	} catch (e) {
		handleError(e as Error)
		currentUser.value = undefined
		currentUserId.value = undefined
	}
}

function getBakedSkinTextures(skin: Skin): RenderResult | undefined {
	return skinBlobUrlMap.get(getSkinPreviewKey(skin))
}

function openAddSkinFileBrowser() {
	if (isSkinManagementReadOnly.value) return

	addSkinFileInput.value?.click()
}

async function onAddSkinFileInputChange(e: Event) {
	if (isSkinManagementReadOnly.value) return

	const files = (e.target as HTMLInputElement).files
	const file = files?.[0]

	if (!file) {
		return
	}

	await processSkinFileBuffer(await file.arrayBuffer())

	if (addSkinFileInput.value) {
		addSkinFileInput.value.value = ''
	}
}

function isSkinImagePath(path: string) {
	return path.toLowerCase().endsWith('.png')
}

function isSkinFileDrag(event: DragEvent) {
	const items = Array.from(event.dataTransfer?.items ?? [])
	const files = Array.from(event.dataTransfer?.files ?? [])

	return (
		items.some((item) => item.kind === 'file' && item.type === 'image/png') ||
		files.some((file) => file.type === 'image/png' || isSkinImagePath(file.name))
	)
}

function isPositionOverAddSkinButton(position: { x: number; y: number }) {
	const element = skinSectionList.value?.getAddSkinButtonElement()

	if (!element) {
		return false
	}

	const { x, y } = position
	const rect = element.getBoundingClientRect()

	return x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom
}

async function handleAddSkinNativeDragDrop(event: { payload: DragDropEvent }) {
	if (isSkinManagementReadOnly.value) return

	const payload = event.payload

	if (payload.type === 'leave') {
		isDraggingSkinFile.value = false
		isAddSkinButtonDragActive.value = false
		return
	}

	if (payload.type === 'enter') {
		isDraggingSkinFile.value = payload.paths.some(isSkinImagePath)
	}

	if (payload.type === 'enter' || payload.type === 'over') {
		isAddSkinButtonDragActive.value =
			isDraggingSkinFile.value && isPositionOverAddSkinButton(payload.position)
		return
	}

	const hasSkinPath = payload.paths.some(isSkinImagePath)
	const shouldUpload =
		(isDraggingSkinFile.value || hasSkinPath) && isPositionOverAddSkinButton(payload.position)

	isDraggingSkinFile.value = false
	isAddSkinButtonDragActive.value = false

	if (!shouldUpload) {
		return
	}

	const skinPath = payload.paths.find(isSkinImagePath)

	if (!skinPath) {
		return
	}

	try {
		const data = await get_dragged_skin_data(skinPath)
		await processSkinFileBuffer(data)
	} catch (error) {
		addNotification({
			title: formatMessage(messages.droppedFileErrorTitle),
			text: error instanceof Error ? error.message : formatMessage(messages.droppedFileErrorText),
			type: 'error',
		})
	}
}

function onAddSkinDragOver(event: DragEvent) {
	if (isSkinManagementReadOnly.value) return

	if (!isSkinFileDrag(event)) {
		return
	}

	isAddSkinButtonDragActive.value = true
}

function onAddSkinDragLeave() {
	if (isSkinManagementReadOnly.value) return

	isAddSkinButtonDragActive.value = false
}

async function onAddSkinDrop(event: DragEvent) {
	if (isSkinManagementReadOnly.value) return

	isAddSkinButtonDragActive.value = false

	const file = Array.from(event.dataTransfer?.files ?? []).find(
		(file) => file.type === 'image/png' || isSkinImagePath(file.name),
	)

	if (!file) {
		return
	}

	await processSkinFileBuffer(await file.arrayBuffer())
}

async function setupAddSkinDragDropListener() {
	try {
		const unlisten = await getCurrentWebview().onDragDropEvent(handleAddSkinNativeDragDrop)

		if (isUnmounted) {
			unlisten()
			return
		}

		unlistenAddSkinDragDrop = unlisten
	} catch (error) {
		handleError(error as Error)
	}
}

async function processSkinFileBuffer(buffer: Uint8Array | ArrayBuffer) {
	if (isSkinManagementReadOnly.value) return

	const fakeEvent = new MouseEvent('click')
	const originalSkinTexUrl = `data:image/png;base64,` + arrayBufferToBase64(buffer)
	try {
		const skinTextureNormalized = await normalize_skin_texture(originalSkinTexUrl)
		const skinTexUrl: SkinTextureUrl = {
			original: originalSkinTexUrl,
			normalized: `data:image/png;base64,` + arrayBufferToBase64(skinTextureNormalized),
		}
		editSkinModal.value?.showNew(fakeEvent, skinTexUrl)
	} catch (error) {
		handleError(error as Error)
	}
}

watch(
	() => selectedSkin.value?.cape_id,
	() => {},
)

watch(selectedSkin, () => {
	earsFeaturesEnabled.value = true
})

watch(isSkinManagementReadOnly, (readOnly) => {
	if (readOnly) {
		isDraggingSkinFile.value = false
		isAddSkinButtonDragActive.value = false
	}
})

onMounted(async () => {
	window.addEventListener('offline', onOffline)
	window.addEventListener('online', onOnline)
	window.addEventListener('freeplay-account-changed', async () => {
		await accountsCard.value?.refreshValues()
		await loadCurrentUser()
		await loadCapes()
		await loadSkins()
	})
	userCheckInterval = window.setInterval(checkUserChanges, 1000)
	void setupAddSkinDragDropListener()

	if (skins.value.length === 0) {
		await Promise.all([loadCapes(), loadCurrentUser()])
		await loadSkins()
	}
})

onUnmounted(() => {
	isUnmounted = true
	window.removeEventListener('offline', onOffline)
	window.removeEventListener('online', onOnline)

	if (userCheckInterval !== null) {
		window.clearInterval(userCheckInterval)
	}

	if (pendingSkinRefreshTimeout !== null) {
		window.clearTimeout(pendingSkinRefreshTimeout)
		pendingSkinRefreshTimeout = null
	}

	if (unlistenAddSkinDragDrop) {
		unlistenAddSkinDragDrop()
		unlistenAddSkinDragDrop = null
	}
})

function onOffline() {
	offline.value = true
}

function onOnline() {
	offline.value = false
	void authServerQuery.refetch()
}

async function checkUserChanges() {
	try {
		const defaultId = await get_default_user()
		if (defaultId !== currentUserId.value) {
			await accountsCard.value?.refreshValues()
			await loadCurrentUser()
			await loadCapes()
			await loadSkins()
		}
	} catch (error) {
		if (currentUser.value && error instanceof Error) {
			handleError(error)
		}
	}
}

await Promise.all([loadCapes(), loadCurrentUser()])
await loadSkins()
</script>

<template>
	<EditSkinModal
		ref="editSkinModal"
		:capes="capes"
		:demo="!currentUser"
		@saved="onSkinSaved"
		@deleted="() => loadSkins()"
	/>
	<input
		ref="addSkinFileInput"
		type="file"
		accept="image/png"
		class="hidden"
		@change="onAddSkinFileInputChange"
	/>
	<ConfirmModal
		ref="deleteSkinModal"
		:title="formatMessage(messages.deleteSkinTitle)"
		:description="formatMessage(messages.deleteSkinDescription)"
		:proceed-label="formatMessage(commonMessages.deleteLabel)"
		@proceed="deleteSkin"
	/>

	<div
		class="skin-control-panel flex flex-col gap-6 p-4 sm:p-6 max-w-7xl mx-auto w-full select-none text-zinc-100 font-sans"
	>
		<!-- Top Wardrobe Header Bar -->
		<div
			class="relative overflow-hidden rounded-2xl bg-[#141923]/90 border border-white/10 shadow-2xl backdrop-blur-xl p-5 sm:p-6 flex flex-col md:flex-row items-start md:items-center justify-between gap-5 transition-all duration-300"
		>
			<div class="flex items-center gap-4 z-10">
				<div
					class="w-12 h-12 rounded-2xl bg-sky-500/10 border border-sky-500/30 flex items-center justify-center text-sky-400 shadow-[0_0_20px_rgba(56,189,248,0.25)]"
				>
					<ShirtIcon class="w-6 h-6" />
				</div>
				<div>
					<h1
						class="m-0 text-2xl font-extrabold tracking-tight text-white flex items-center gap-2.5"
					>
						{{ formatMessage(appMessages.skinSelectorLabel) }}
					</h1>
				</div>
			</div>

			<div class="flex flex-wrap items-center gap-3 z-10 w-full md:w-auto">
				<!-- Search Filter -->
				<div class="relative flex-1 md:w-64">
					<input
						v-model="searchQuery"
						type="text"
						placeholder="Search skins..."
						class="w-full bg-white/5 hover:bg-white/10 focus:bg-[#181e2b] border border-white/10 focus:border-sky-500/50 rounded-xl px-3.5 py-2 text-sm text-white placeholder-zinc-500 focus:outline-none transition-all"
					/>
					<span
						v-if="searchQuery"
						class="absolute right-3 top-2.5 text-xs text-zinc-500 hover:text-white cursor-pointer"
						@click="searchQuery = ''"
					>
						✕
					</span>
				</div>

				<!-- Category Filters -->
				<div class="flex items-center p-1 bg-white/5 border border-white/10 rounded-xl">
					<button
						class="px-3 py-1 text-xs font-semibold rounded-lg transition-all"
						:class="
							activeFilterCategory === 'all'
								? 'bg-sky-500 text-zinc-950 shadow-md shadow-sky-500/20'
								: 'text-zinc-400 hover:text-white'
						"
						@click="activeFilterCategory = 'all'"
					>
						All
					</button>
					<button
						class="px-3 py-1 text-xs font-semibold rounded-lg transition-all"
						:class="
							activeFilterCategory === 'saved'
								? 'bg-sky-500 text-zinc-950 shadow-md shadow-sky-500/20'
								: 'text-zinc-400 hover:text-white'
						"
						@click="activeFilterCategory = 'saved'"
					>
						Saved
					</button>
					<button
						class="px-3 py-1 text-xs font-semibold rounded-lg transition-all"
						:class="
							activeFilterCategory === 'default'
								? 'bg-[var(--color-brand)] text-[var(--color-accent-contrast,#ffffff)] shadow-md'
								: 'text-zinc-400 hover:text-white'
						"
						@click="activeFilterCategory = 'default'"
					>
						Default
					</button>
				</div>

				<!-- Upload Button -->
				<button
					class="px-4 py-2 rounded-xl btn-accent-primary font-bold text-sm transition-all flex items-center gap-2 cursor-pointer border-none active:scale-95"
					@click="openAddSkinFileBrowser"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-4 h-4"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
						<polyline points="17 8 12 3 7 8" />
						<line x1="12" y1="3" x2="12" y2="15" />
					</svg>
					<span>Upload PNG</span>
				</button>
			</div>
		</div>

		<!-- Main 2-Column Studio & Vault Grid -->
		<div class="skin-layout grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
			<!-- Left 3D Studio Pod (col 1 to 5) -->
			<div class="lg:col-span-5 sticky top-4 flex flex-col gap-4">
				<div
					class="relative overflow-hidden rounded-3xl bg-[var(--surface-2)] border border-white/10 shadow-2xl backdrop-blur-2xl p-5 flex flex-col items-center gap-4 transition-all duration-300"
				>
					<!-- Ambient Studio Aura -->
					<div
						class="absolute -right-20 -top-20 w-64 h-64 rounded-full blur-3xl pointer-events-none opacity-25 transition-all duration-700"
						:class="hasPendingSkinChange ? 'bg-amber-500' : 'bg-[var(--color-brand)]'"
					/>

					<!-- Studio Header Meta Bar -->
					<div
						class="w-full flex items-center justify-between gap-2 z-10 pb-3 border-b border-white/10"
					>
						<div class="flex items-center gap-2 min-w-0">
							<span class="text-xs uppercase tracking-wider font-bold text-zinc-400">Selected</span>
							<span class="text-sm font-bold text-white truncate max-w-[150px]">
								{{ selectedSkin?.name || 'Steve / Custom' }}
							</span>
						</div>
						<div class="flex items-center gap-1.5">
							<span
								class="px-2 py-0.5 rounded-md text-[11px] font-mono font-semibold"
								:class="
									skinVariant === 'SLIM'
										? 'bg-purple-500/20 text-purple-300 border border-purple-500/30'
										: 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
								"
							>
								{{ skinVariant === 'SLIM' ? 'SLIM (3px)' : 'CLASSIC (4px)' }}
							</span>
							<span
								v-if="currentCape"
								class="px-2 py-0.5 rounded-md text-[11px] font-mono font-semibold bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border border-[var(--color-brand-shadow)]"
							>
								Cape: {{ currentCape.name }}
							</span>
						</div>
					</div>

					<!-- 3D Canvas Stage -->
					<div
						class="relative w-full h-[400px] max-[700px]:h-[300px] flex items-center justify-center"
					>
						<SkinPreviewRenderer
							:cape-src="capeTexture"
							:texture-src="skinTexture || ''"
							:ears-texture-src="selectedSkin?.texture"
							:variant="skinVariant"
							:nametag="skinNametag"
							:initial-rotation="Math.PI / 8"
							:ears-enabled="earsFeaturesEnabled"
							@ears-features-detected="selectedSkinHasEarsFeatures = $event"
						>
							<template v-if="hasPendingSkinChange" #nametag-badge>
								<div
									class="flex items-center justify-center gap-1.5 rounded-full border border-solid border-[var(--color-brand-shadow)] bg-[var(--color-brand-bg)] px-3 py-1 text-xs font-bold leading-none text-[var(--color-brand-highlight,var(--color-brand))] backdrop-blur-md shadow-lg"
								>
									<EyeIcon class="size-4 shrink-0" />
									{{ formatMessage(messages.previewingBadge) }}
								</div>
							</template>
						</SkinPreviewRenderer>
					</div>

					<!-- Studio Control Deck -->
					<div class="w-full flex flex-col gap-3 z-10 pt-3 border-t border-white/10">
						<div class="flex items-center gap-2">
							<button
								v-if="hasPendingSkinChange"
								class="flex-1 px-4 py-2.5 rounded-xl bg-white/10 hover:bg-white/15 active:scale-95 text-white font-semibold text-sm border border-white/10 transition-all flex items-center justify-center gap-2 cursor-pointer"
								:disabled="isApplyingSkin || isSkinManagementReadOnly"
								@click="resetSelectedSkin"
							>
								<RotateCounterClockwiseIcon class="w-4 h-4" />
								<span>{{ formatMessage(commonMessages.resetButton) }}</span>
							</button>

							<button
								class="flex-1 px-5 py-2.5 rounded-xl font-bold text-sm transition-all flex items-center justify-center gap-2 cursor-pointer border-none"
								:class="
									hasPendingSkinChange
										? 'btn-accent-primary'
										: 'bg-[var(--color-brand-bg)] text-[var(--color-brand-highlight,var(--color-brand))] border border-[var(--color-brand-shadow)] cursor-default'
								"
								:disabled="isApplyingSkin || isSkinManagementReadOnly || !hasPendingSkinChange"
								@click="applySelectedSkin"
							>
								<SpinnerIcon v-if="isApplyingSkin" class="animate-spin w-4 h-4" />
								<CheckIcon v-else class="w-4 h-4" />
								<span>{{
									hasPendingSkinChange ? formatMessage(messages.applyButton) : 'Equipped'
								}}</span>
							</button>

							<button
								class="px-3.5 py-2.5 rounded-xl bg-white/10 hover:bg-white/15 active:scale-95 text-white border border-white/10 transition-all flex items-center justify-center cursor-pointer"
								:disabled="!selectedSkin || isSkinManagementReadOnly"
								title="Edit skin properties"
								@click="(e: MouseEvent) => selectedSkin && editSkinModal?.show(e, selectedSkin)"
							>
								<EditIcon class="w-4 h-4" />
							</button>
						</div>

						<!-- Ears Mod Support Pill -->
						<div
							v-if="selectedSkinHasEarsFeatures"
							class="w-full flex items-center justify-between gap-3 p-3 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md"
						>
							<div class="flex items-center gap-2.5 min-w-0">
								<img
									:src="EarsModIcon"
									alt=""
									class="size-8 shrink-0 rounded-lg border border-white/10 object-cover"
								/>
								<div class="flex flex-col min-w-0">
									<span class="text-xs font-semibold text-white truncate">Ears 3D Features</span>
									<span class="text-[11px] text-zinc-400">Custom tails, snouts, & wings</span>
								</div>
							</div>
							<Toggle
								v-model="earsFeaturesEnabled"
								small
								class="shrink-0"
								:aria-label="
									formatMessage(
										earsFeaturesEnabled
											? messages.toggleEarsFeaturesOff
											: messages.toggleEarsFeaturesOn,
									)
								"
							/>
						</div>
					</div>
				</div>
			</div>

			<!-- Right Skins Vault & Sections (col 6 to 12) -->
			<div class="lg:col-span-7 flex flex-col gap-4">
				<VirtualSkinSectionList
					ref="skinSectionList"
					:saved-skins="visibleSavedSkins"
					:default-skin-sections="visibleDefaultSkinSections"
					:get-baked-skin-textures="getBakedSkinTextures"
					:is-skin-selected="isSkinSelected"
					:is-skin-active="isSkinActive"
					:is-add-skin-button-drag-active="isAddSkinButtonDragActive"
					:read-only="isSkinManagementReadOnly"
					@select="changeSkin"
					@edit="(skin, event) => editSkinModal?.show(event, skin)"
					@delete="confirmDeleteSkin"
					@reorder-saved-skins="reorderSavedSkins"
					@add-skin="openAddSkinFileBrowser"
					@add-skin-dragenter="onAddSkinDragOver"
					@add-skin-dragover="onAddSkinDragOver"
					@add-skin-dragleave="onAddSkinDragLeave"
					@add-skin-drop="onAddSkinDrop"
				/>
			</div>
		</div>
	</div>
</template>

<style scoped>
.skin-control-panel {
	animation: fadeIn 0.25s ease-out;
}

@keyframes fadeIn {
	from {
		opacity: 0;
		transform: translateY(6px);
	}
	to {
		opacity: 1;
		transform: translateY(0);
	}
}
</style>
