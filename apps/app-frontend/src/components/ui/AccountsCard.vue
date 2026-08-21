<template>
	<div
		v-if="accounts.length === 0"
		class="flex flex-col gap-3.5 bg-surface-2 border border-surface-4 rounded-2xl p-4 shadow-sm select-none"
	>
		<!-- Dual Auth Switcher Header -->
		<div class="flex flex-col gap-1">
			<span class="font-bold text-contrast text-sm">Choose How You Want to Play</span>
			<span class="text-xs text-secondary"
				>Play for free with an offline nickname or sign in with your Microsoft account.</span
			>
		</div>

		<!-- Dual Auth Tab Switcher -->
		<div
			class="grid grid-cols-2 gap-1.5 p-1 bg-[var(--surface-1)] rounded-xl border border-[var(--border-subtle)]"
		>
			<button
				type="button"
				class="flex items-center justify-center gap-1.5 py-1.5 px-2 rounded-lg text-xs font-bold cursor-pointer transition-all duration-200 border border-transparent"
				:class="
					activeAuthTab === 'microsoft'
						? 'bg-sky-500/20 text-sky-300 !border-sky-500/30 shadow-sm'
						: 'text-zinc-400 hover:text-zinc-200 hover:bg-white/5'
				"
				@click="activeAuthTab = 'microsoft'"
			>
				<LogInIcon class="w-3.5 h-3.5 text-sky-400" />
				Microsoft
			</button>
			<button
				type="button"
				class="flex items-center justify-center gap-1.5 py-1.5 px-2 rounded-lg text-xs font-bold cursor-pointer transition-all duration-200 border border-transparent"
				:class="
					activeAuthTab === 'offline'
						? 'bg-emerald-500/20 text-emerald-300 !border-emerald-500/30 shadow-sm'
						: 'text-zinc-400 hover:text-zinc-200 hover:bg-white/5'
				"
				@click="activeAuthTab = 'offline'"
			>
				<UserIcon class="w-3.5 h-3.5 text-emerald-400" />
				Offline
			</button>
		</div>

		<!-- Auth Tab Actions -->
		<div v-if="activeAuthTab === 'microsoft'" class="flex flex-col gap-2">
			<button
				type="button"
				class="w-full py-2.5 rounded-xl btn-accent-primary font-extrabold text-xs flex items-center justify-center gap-2 cursor-pointer border-none shadow-md transition-all active:scale-95"
				:disabled="loginDisabled"
				@click="login()"
			>
				<LogInIcon v-if="!loginDisabled" class="w-4 h-4" />
				<SpinnerIcon v-else class="animate-spin w-4 h-4" />
				{{ formatMessage(messages.signInToMinecraft) }}
			</button>
		</div>
		<div v-else class="flex flex-col gap-2">
			<button
				type="button"
				class="w-full py-2.5 rounded-xl btn-accent-secondary font-extrabold text-xs flex items-center justify-center gap-2 cursor-pointer transition-all active:scale-95"
				@click="offlineModal?.show()"
			>
				<UserIcon class="w-4 h-4" />
				Create Offline Profile
			</button>
		</div>
	</div>
	<Accordion
		v-else
		class="w-full bg-surface-2 border border-surface-4 rounded-2xl overflow-clip shadow-sm select-none"
		button-class="button-base w-full bg-transparent px-3.5 py-3 border-0 cursor-pointer hover:bg-surface-3 transition-colors"
		:open-by-default="true"
	>
		<template #title>
			<div class="flex gap-3 w-full min-w-0 items-center">
				<div class="relative shrink-0">
					<img
						:src="selectedAccount ? avatarUrl : 'https://mc-heads.net/avatar/Steve/64'"
						alt="Player Avatar"
						class="w-9 h-9 rounded-xl border border-surface-4 shadow-sm object-cover bg-surface-3"
					/>
					<span class="absolute -bottom-0.5 -right-0.5 flex h-3 w-3">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
						></span>
						<span
							class="relative inline-flex rounded-full h-3 w-3 bg-emerald-400 border-2 border-[var(--surface-2)]"
						></span>
					</span>
				</div>
				<div class="flex flex-col items-start w-full min-w-0">
					<div class="flex items-center gap-2 w-full min-w-0">
						<span class="truncate text-left font-bold text-contrast text-xs">{{
							selectedAccount ? selectedAccount.profile.name : formatMessage(messages.selectAccount)
						}}</span>
						<span
							class="text-[9px] font-extrabold px-1.5 py-0.5 rounded bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 uppercase tracking-widest font-mono shrink-0"
							>Active</span
						>
					</div>
					<span class="text-secondary text-[11px] font-medium truncate">
						{{
							isSelectedOffline
								? 'Offline Player Profile'
								: formatMessage(messages.minecraftAccount)
						}}
					</span>
				</div>
			</div>
		</template>
		<div class="bg-surface-1/60 pt-2 pb-3 px-2 border-t border-surface-4 flex flex-col gap-1.5">
			<template v-if="accounts.length > 0">
				<div v-for="account in accounts" :key="account.profile.id" class="flex gap-1 items-center">
					<button
						type="button"
						class="flex items-center flex-shrink flex-grow overflow-clip gap-2.5 p-2 rounded-xl border transition-all duration-200 min-w-0 cursor-pointer text-left"
						:class="[
							selectedAccount &&
							(selectedAccount.profile.id === account.profile.id ||
								selectedAccount.profile.name.toLowerCase() === account.profile.name.toLowerCase())
								? 'bg-surface-3 border-[var(--color-brand-shadow)] text-contrast shadow-sm font-semibold'
								: 'bg-transparent hover:bg-surface-3/60 border-transparent text-secondary hover:text-contrast',
						]"
						@click="setAccount(account)"
					>
						<RadioButtonCheckedIcon
							v-if="
								selectedAccount &&
								(selectedAccount.profile.id === account.profile.id ||
									selectedAccount.profile.name.toLowerCase() === account.profile.name.toLowerCase())
							"
							class="w-4 h-4 text-[var(--color-brand-highlight,var(--color-brand))] shrink-0"
						/>
						<RadioButtonIcon v-else class="w-4 h-4 text-secondary/60 shrink-0" />
						<img
							:src="getAccountAvatarUrl(account)"
							class="w-6 h-6 rounded-lg border border-surface-4 shrink-0 bg-surface-3"
						/>
						<p class="m-0 truncate min-w-0 text-xs flex-1">
							{{ account.profile.name }}
						</p>
					</button>
					<IconButton
						v-tooltip="formatMessage(messages.removeAccount)"
						type="quiet"
						color="red"
						:label="formatMessage(messages.removeAccount)"
						class="mr-1 hover:!bg-red-500/20 !text-red-400 hover:!text-red-300 cursor-pointer transition-colors"
						@click="logout(account.profile.id)"
					>
						<TrashIcon />
					</IconButton>
				</div>
			</template>
			<div class="grid grid-cols-2 gap-2 pt-2 border-t border-surface-4/60">
				<button
					type="button"
					class="w-full flex items-center justify-center gap-1.5 p-2 rounded-xl btn-accent-secondary text-xs font-bold cursor-pointer transition-all duration-200 active:scale-95 shadow-sm"
					@click="offlineModal?.show()"
				>
					<PlusIcon class="w-3.5 h-3.5" />
					Offline
				</button>
				<button
					type="button"
					class="w-full flex items-center justify-center gap-1.5 p-2 rounded-xl btn-accent-primary text-xs font-bold cursor-pointer transition-all duration-200 active:scale-95 shadow-sm"
					:disabled="loginDisabled"
					@click="login()"
				>
					<LogInIcon class="w-3.5 h-3.5" />
					Microsoft
				</button>
			</div>
		</div>
	</Accordion>
	<OfflineAccountModal ref="offlineModal" @created="refreshValues" />
</template>

<script setup lang="ts">
import {
	LogInIcon,
	PlusIcon,
	RadioButtonCheckedIcon,
	RadioButtonIcon,
	SpinnerIcon,
	TrashIcon,
	UserIcon,
} from '@freeplay/assets'
import { Accordion, defineMessages, IconButton, useVIntl } from '@freeplay/ui'
import { computed, onMounted, onUnmounted, ref, useTemplateRef } from 'vue'

import OfflineAccountModal from '@/components/ui/modal/OfflineAccountModal.vue'
import { useAppEvent } from '@/composables/use-app-event'
import { trackEvent } from '@/helpers/analytics'
import { login as login_flow } from '@/helpers/auth'
import { getPlayerHeadUrl } from '@/helpers/rendering/batch-skin-renderer.ts'
import type { Skin } from '@/helpers/skins'
import { get_available_skins } from '@/helpers/skins'
import { useAccountStore } from '@/store/account.ts'
import { handleSevereError } from '@/store/error.js'

const { formatMessage } = useVIntl()
const accountStore = useAccountStore()

const offlineModal = useTemplateRef('offlineModal')

const emit = defineEmits<{
	change: []
}>()

type MinecraftCredential = {
	profile: {
		id: string
		name: string
	}
	access_token?: string
	type?: string
}

const activeAuthTab = ref<'microsoft' | 'offline'>('offline')
const loginDisabled = ref(false)
const equippedSkin = ref<Skin | null>(null)
const headUrlCache = ref(new Map<string, string>())

const accounts = computed<MinecraftCredential[]>(() =>
	accountStore.accounts.map((acc) => ({
		profile: {
			id: acc.id,
			name: acc.name,
		},
		access_token: acc.isOffline ? '0' : 'token',
		type: acc.type,
	})),
)

async function refreshValues() {
	await accountStore.refresh()

	try {
		const skins = await get_available_skins()
		equippedSkin.value = skins.find((skin) => skin.is_equipped) ?? null

		if (equippedSkin.value) {
			try {
				const headUrl = await getPlayerHeadUrl(equippedSkin.value)
				headUrlCache.value = new Map(headUrlCache.value).set(
					equippedSkin.value.texture_key,
					headUrl,
				)
			} catch (error) {
				console.warn('Failed to get head render for equipped skin:', error)
			}
		}
	} catch {
		equippedSkin.value = null
	}
}

async function setEquippedSkin(skin: Skin) {
	equippedSkin.value = skin

	try {
		const headUrl = await getPlayerHeadUrl(skin)
		headUrlCache.value = new Map(headUrlCache.value).set(skin.texture_key, headUrl)
	} catch (error) {
		console.warn('Failed to get head render for equipped skin:', error)
	}
}

function setLoginDisabled(value: boolean) {
	loginDisabled.value = value
}

defineExpose({
	refreshValues,
	setEquippedSkin,
	setLoginDisabled,
	login,
	loginDisabled,
})

await refreshValues()

const selectedAccount = computed(() => {
	const active = accountStore.activeAccount
	if (!active) return null
	return {
		profile: {
			id: active.id,
			name: active.name,
		},
		access_token: active.isOffline ? '0' : 'token',
		type: active.type,
	}
})

const isSelectedOffline = computed(() => {
	return accountStore.isActiveOffline
})

const avatarUrl = computed(() => {
	if (equippedSkin.value?.texture_key) {
		const cachedUrl = headUrlCache.value.get(equippedSkin.value.texture_key)
		if (cachedUrl) {
			return cachedUrl
		}
		return `https://mc-heads.net/avatar/${equippedSkin.value.texture_key}/128`
	}
	return accountStore.activePlayerAvatar
})

function getAccountAvatarUrl(account: MinecraftCredential) {
	if (
		account.profile.id === selectedAccount.value?.profile?.id &&
		equippedSkin.value?.texture_key
	) {
		const cachedUrl = headUrlCache.value.get(equippedSkin.value.texture_key)
		if (cachedUrl) {
			return cachedUrl
		}
	}
	return `https://mc-heads.net/avatar/${account.profile.name || account.profile.id}/128`
}

async function setAccount(account: MinecraftCredential) {
	const activeId = account.profile?.id
	if (!activeId) return
	await accountStore.setActiveAccount(activeId)
	emit('change')
}

async function login() {
	loginDisabled.value = true
	const loggedIn = await login_flow().catch(handleSevereError)

	if (loggedIn) {
		await setAccount(loggedIn)
	}

	trackEvent('AccountLogIn')
	loginDisabled.value = false
}

async function logout(id: string) {
	await accountStore.removeAccount(id)
	emit('change')
	trackEvent('AccountLogOut')
}

function handleExternalAccountChange() {
	void refreshValues()
}

onMounted(() => {
	window.addEventListener('freeplay-account-changed', handleExternalAccountChange)
	window.addEventListener('storage', handleExternalAccountChange)
})

onUnmounted(() => {
	window.removeEventListener('freeplay-account-changed', handleExternalAccountChange)
	window.removeEventListener('storage', handleExternalAccountChange)
})

useAppEvent('process', async (e) => {
	if (e.event === 'launched') {
		await refreshValues()
	}
})

const messages = defineMessages({
	notSignedIn: {
		id: 'minecraft-account.not-signed-in',
		defaultMessage: 'Not signed in',
	},
	addAccount: {
		id: 'minecraft-account.add-account',
		defaultMessage: 'Add account',
	},
	removeAccount: {
		id: 'minecraft-account.remove-account',
		defaultMessage: 'Remove account',
	},
	selectAccount: {
		id: 'minecraft-account.select-account',
		defaultMessage: 'Select account',
	},
	minecraftAccount: {
		id: 'minecraft-account.label',
		defaultMessage: 'Minecraft account',
	},
	signInToMinecraft: {
		id: 'minecraft-account.sign-in',
		defaultMessage: 'Sign in to Minecraft',
	},
})
</script>
