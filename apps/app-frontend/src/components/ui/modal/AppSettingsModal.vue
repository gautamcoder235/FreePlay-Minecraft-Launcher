<script setup lang="ts">
import {
	CoffeeIcon,
	FreePlayIcon,
	GameIcon,
	GaugeIcon,
	LanguagesIcon,
	PaintbrushIcon,
	Settings2Icon,
	ShieldIcon,
	ToggleRightIcon,
	UserIcon,
} from '@freeplay/assets'
import {
	commonMessages,
	commonSettingsMessages,
	defineMessage,
	defineMessages,
	TabbedModal,
	UnsavedChangesPopup,
	useVIntl,
} from '@freeplay/ui'
import { getVersion } from '@tauri-apps/api/app'
import { platform as getOsPlatform, version as getOsVersion } from '@tauri-apps/plugin-os'
import { computed, provide, ref, watch } from 'vue'

import AccountsManagerSettings from '@/components/ui/settings/account/AccountsManagerSettings.vue'
import PrivacySettings from '@/components/ui/settings/account/PrivacySettings.vue'
import AppearanceSettings from '@/components/ui/settings/display/AppearanceSettings.vue'
import BehaviorSettings from '@/components/ui/settings/display/BehaviorSettings.vue'
import FeatureFlagSettings from '@/components/ui/settings/display/FeatureFlagSettings.vue'
import LanguageSettings from '@/components/ui/settings/display/LanguageSettings.vue'
import DefaultInstanceSettings from '@/components/ui/settings/instances/DefaultInstanceSettings.vue'
import JavaSettings from '@/components/ui/settings/instances/JavaSettings.vue'
import ResourceManagementSettings from '@/components/ui/settings/instances/ResourceManagementSettings.vue'
import { get, set } from '@/helpers/settings.ts'
import {
	appSettingsModalContextKey,
	type UnsavedChangesController,
} from '@/providers/app-settings-modal'
import { useTheming } from '@/store/state'

const themeStore = useTheming()

const { formatMessage } = useVIntl()

const devModeCounter = ref(0)

const developerModeEnabled = defineMessage({
	id: 'app.settings.developer-mode-enabled',
	defaultMessage: 'Developer mode enabled.',
})

const tabCategories = defineMessages({
	app: {
		id: 'app.settings.sidebar.label.app',
		defaultMessage: 'App & Interface',
	},
	game: {
		id: 'app.settings.sidebar.label.game',
		defaultMessage: 'Game & Engine',
	},
	storage: {
		id: 'app.settings.sidebar.label.storage',
		defaultMessage: 'Storage & Network',
	},
	accounts: {
		id: 'app.settings.sidebar.label.accounts',
		defaultMessage: 'Accounts & Privacy',
	},
	developer: {
		id: 'app.settings.sidebar.label.developer',
		defaultMessage: 'Developer',
	},
})

const tabs = [
	// 1. App & Interface
	{
		name: defineMessage({
			id: 'app.settings.tabs.appearance',
			defaultMessage: 'Appearance',
		}),
		category: tabCategories.app,
		icon: PaintbrushIcon,
		content: AppearanceSettings,
	},
	{
		name: defineMessage({
			id: 'app.settings.tabs.behavior',
			defaultMessage: 'Behavior',
		}),
		category: tabCategories.app,
		icon: Settings2Icon,
		content: BehaviorSettings,
	},
	{
		name: defineMessage({
			id: 'app.settings.tabs.language',
			defaultMessage: 'Language',
		}),
		category: tabCategories.app,
		icon: LanguagesIcon,
		content: LanguageSettings,
		badge: commonMessages.beta,
	},

	// 2. Game & Engine
	{
		name: defineMessage({
			id: 'app.settings.tabs.default-instance-options',
			defaultMessage: 'Default game options',
		}),
		category: tabCategories.game,
		icon: GameIcon,
		content: DefaultInstanceSettings,
	},
	{
		name: defineMessage({
			id: 'app.settings.tabs.java-installations',
			defaultMessage: 'Java installations',
		}),
		category: tabCategories.game,
		icon: CoffeeIcon,
		content: JavaSettings,
	},

	// 3. Storage & Network
	{
		name: defineMessage({
			id: 'app.settings.tabs.resource-management',
			defaultMessage: 'Storage & Downloads',
		}),
		category: tabCategories.storage,
		icon: GaugeIcon,
		content: ResourceManagementSettings,
	},

	// 4. Accounts & Privacy
	{
		name: defineMessage({
			id: 'app.settings.tabs.accounts',
			defaultMessage: 'Accounts & Profiles',
		}),
		category: tabCategories.accounts,
		icon: UserIcon,
		content: AccountsManagerSettings,
	},
	{
		name: defineMessage({
			id: 'app.settings.tabs.privacy',
			defaultMessage: 'Privacy & Integrations',
		}),
		category: tabCategories.accounts,
		icon: ShieldIcon,
		content: PrivacySettings,
	},

	// 5. Developer (Dev Mode Only)
	{
		name: commonSettingsMessages.featureFlags,
		category: tabCategories.developer,
		icon: ToggleRightIcon,
		content: FeatureFlagSettings,
		developerOnly: true,
	},
]

const availableTabs = computed(() => tabs.filter((tab) => !tab.developerOnly || themeStore.devMode))

const modal = ref<InstanceType<typeof TabbedModal> | null>(null)
const unsavedChangesPopup = ref<{ nudge: () => void } | null>(null)
const unsavedChangesController = ref<UnsavedChangesController | null>(null)
const emptyUnsavedChangesState: Record<string, unknown> = {}
const originalUnsavedChangesState = computed(
	() => unsavedChangesController.value?.getOriginal() ?? emptyUnsavedChangesState,
)
const modifiedUnsavedChangesState = computed(
	() => unsavedChangesController.value?.getModified() ?? emptyUnsavedChangesState,
)
const savingUnsavedChanges = computed(() => unsavedChangesController.value?.isSaving() ?? false)
const hasUnsavedChanges = computed(() => unsavedChangesController.value?.hasChanges() ?? false)

function canLeaveCurrentTab(): boolean {
	if (!unsavedChangesController.value?.hasChanges()) return true
	unsavedChangesPopup.value?.nudge()
	return false
}

function close(): boolean {
	return modal.value?.hide() ?? false
}

function registerUnsavedChangesController(controller: UnsavedChangesController | null): void {
	unsavedChangesController.value = controller
}

provide(appSettingsModalContextKey, {
	close,
	registerUnsavedChangesController,
})

function resetUnsavedChanges(): void {
	unsavedChangesController.value?.reset()
}

function saveUnsavedChanges(): void {
	void unsavedChangesController.value?.save()
}

function show() {
	modal.value?.show()
}

function showProfile(): void {
	const profileTabIndex = availableTabs.value.findIndex(
		(tab) => tab.content === AccountsManagerSettings,
	)
	if (profileTabIndex >= 0) {
		modal.value?.setTab(profileTabIndex)
	}
	modal.value?.show()
}

defineExpose({ show, showProfile })

const version = await getVersion()
const osPlatform = getOsPlatform()
const osVersion = getOsVersion()
const settings = ref(await get())

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)

function devModeCount() {
	devModeCounter.value++
	if (devModeCounter.value > 5) {
		const selectedTab = modal.value ? availableTabs.value[modal.value.selectedTab] : undefined

		themeStore.devMode = !themeStore.devMode
		settings.value.developer_mode = !!themeStore.devMode
		devModeCounter.value = 0

		if (modal.value) {
			const selectedTabIndex = selectedTab ? availableTabs.value.indexOf(selectedTab) : -1
			modal.value.setTab(selectedTabIndex >= 0 ? selectedTabIndex : 0)
		}
	}
}

const messages = defineMessages({
	appVersion: {
		id: 'app.settings.app-version',
		defaultMessage: 'FreePlay Launcher {version}',
	},
	macos: {
		id: 'app.settings.operating-system.macos',
		defaultMessage: 'macOS',
	},
	developerModeButtonLabel: {
		id: 'app.settings.developer-mode-button.label',
		defaultMessage: 'Toggle developer mode',
	},
})
</script>
<template>
	<TabbedModal
		ref="modal"
		:tabs="availableTabs"
		:width="'min(960px, calc(95vw - 8rem))'"
		:before-hide="canLeaveCurrentTab"
		:before-tab-change="canLeaveCurrentTab"
		:floating-action-bar-shown="hasUnsavedChanges"
	>
		<template #title>
			<span class="text-2xl font-bold tracking-tight text-contrast">
				{{ formatMessage(commonMessages.settingsLabel) }}
			</span>
		</template>
		<template #floating-action-bar>
			<UnsavedChangesPopup
				ref="unsavedChangesPopup"
				:original="originalUnsavedChangesState"
				:modified="modifiedUnsavedChangesState"
				:saving="savingUnsavedChanges"
				inline
				@reset="resetUnsavedChanges"
				@save="saveUnsavedChanges"
			/>
		</template>
		<template #footer>
			<div
				class="mt-auto p-3 rounded-2xl bg-surface-2 border border-surface-4 shadow-sm flex flex-col gap-2"
			>
				<p
					v-if="themeStore.devMode"
					class="text-brand text-xs font-bold m-0 flex items-center gap-1.5"
				>
					<span class="size-2 rounded-full bg-brand animate-pulse" />
					{{ formatMessage(developerModeEnabled) }}
				</p>
				<div class="flex items-center gap-2.5">
					<button
						:aria-label="formatMessage(messages.developerModeButtonLabel)"
						class="p-1.5 rounded-xl bg-surface-3/80 hover:bg-surface-3 hover:scale-105 active:scale-95 border border-surface-4 cursor-pointer transition-all duration-150 shrink-0"
						:class="{
							'text-brand shadow-[0_0_12px_rgba(27,217,106,0.3)]': themeStore.devMode,
							'text-secondary': !themeStore.devMode,
						}"
						@click="devModeCount"
					>
						<FreePlayIcon aria-hidden="true" class="size-4" />
					</button>
					<div class="flex flex-col min-w-0 leading-tight">
						<p class="m-0 text-xs font-semibold text-contrast truncate">
							{{ formatMessage(messages.appVersion, { version }) }}
						</p>
						<p class="m-0 text-[11px] text-secondary truncate">
							<span v-if="osPlatform === 'macos'">{{ formatMessage(messages.macos) }}</span>
							<span v-else class="capitalize">{{ osPlatform }}</span>
							{{ osVersion }}
						</p>
					</div>
				</div>
			</div>
		</template>
	</TabbedModal>
</template>
