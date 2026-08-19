<template>
	<div
		v-if="accounts.length === 0"
		class="flex flex-col gap-4 bg-zinc-950/80 backdrop-blur-md border border-white/10 rounded-2xl p-4 shadow-xl mt-2 select-none"
	>
		<!-- Dual Auth Switcher Header -->
		<div class="flex flex-col gap-1">
			<span class="font-bold text-white text-base">Choose How You Want to Play</span>
			<span class="text-xs text-zinc-400">Play for free with a custom offline nickname or sign in with your official Microsoft account.</span>
		</div>

		<!-- Dual Auth Tab Switcher -->
		<div class="grid grid-cols-2 gap-2 p-1 bg-zinc-900/90 rounded-xl border border-white/10">
			<button
				class="flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-semibold cursor-pointer transition-all duration-200"
				:class="activeAuthTab === 'microsoft' ? 'bg-indigo-600 text-white shadow-[0_0_12px_rgba(99,102,241,0.4)]' : 'text-zinc-400 hover:text-white hover:bg-white/5'"
				@click="activeAuthTab = 'microsoft'"
			>
				<LogInIcon class="w-3.5 h-3.5" />
				Microsoft Auth
			</button>
			<button
				class="flex items-center justify-center gap-2 py-2 px-3 rounded-lg text-xs font-semibold cursor-pointer transition-all duration-200"
				:class="activeAuthTab === 'offline' ? 'bg-emerald-600 text-white shadow-[0_0_12px_rgba(16,185,129,0.4)]' : 'text-zinc-400 hover:text-white hover:bg-white/5'"
				@click="activeAuthTab = 'offline'"
			>
				<UserIcon class="w-3.5 h-3.5" />
				1-Click Offline
			</button>
		</div>

		<!-- Auth Tab Actions -->
		<div v-if="activeAuthTab === 'microsoft'" class="flex flex-col gap-2">
			<Button
				type="colored"
				color="brand"
				class="!bg-indigo-600 hover:!bg-indigo-500 !font-bold cursor-pointer transition-all duration-200 shadow-lg shadow-indigo-950/50"
				:disabled="loginDisabled"
				@click="login()"
			>
				<LogInIcon v-if="!loginDisabled" class="w-4 h-4" />
				<SpinnerIcon v-else class="animate-spin w-4 h-4" />
				{{ formatMessage(messages.signInToMinecraft) }}
			</Button>
		</div>
		<div v-else class="flex flex-col gap-2">
			<Button
				type="colored"
				color="brand"
				class="!bg-emerald-600 hover:!bg-emerald-500 !font-bold cursor-pointer transition-all duration-200 shadow-lg shadow-emerald-950/50"
				@click="offlineModal?.show()"
			>
				<UserIcon class="w-4 h-4" />
				Create 1-Click Offline Profile
			</Button>
		</div>
	</div>
	<Accordion
		v-else
		class="w-full mt-2 bg-zinc-950/80 backdrop-blur-md border border-white/10 rounded-2xl overflow-clip shadow-xl select-none"
		button-class="button-base w-full bg-transparent px-3.5 py-2.5 border-0 cursor-pointer hover:bg-white/5 transition-colors"
		:open-by-default="false"
	>
		<template #title>
			<div class="flex gap-3 w-full min-w-0 items-center">
				<div class="relative shrink-0">
					<img
						:src="selectedAccount ? avatarUrl : 'https://mc-heads.net/avatar/Steve/64'"
						alt="Player Avatar"
						class="w-10 h-10 rounded-xl border border-white/20 shadow-md object-cover"
					/>
					<span class="absolute -bottom-1 -right-1 flex h-3 w-3">
						<span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
						<span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500 border border-zinc-950"></span>
					</span>
				</div>
				<div class="flex flex-col items-start w-full min-w-0">
					<div class="flex items-center gap-2 w-full min-w-0">
						<span class="truncate text-left font-bold text-white text-sm">{{
							selectedAccount ? selectedAccount.profile.name : formatMessage(messages.selectAccount)
						}}</span>
						<span class="text-[9px] font-extrabold px-1.5 py-0.5 rounded bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 uppercase tracking-widest shrink-0">Active</span>
					</div>
					<span class="text-zinc-400 text-xs">
						{{ isSelectedOffline ? '1-Click Offline Profile' : formatMessage(messages.minecraftAccount) }}
					</span>
				</div>
			</div>
		</template>
		<div class="bg-zinc-900/90 pt-2 pb-3 px-2 border-t border-white/10 flex flex-col gap-2">
			<template v-if="accounts.length > 0">
				<div v-for="account in accounts" :key="account.profile.id" class="flex gap-1 items-center">
					<button
						class="flex items-center flex-shrink flex-grow overflow-clip gap-2.5 p-2 rounded-xl border border-transparent hover:border-white/10 bg-transparent hover:bg-white/5 cursor-pointer transition-all duration-200 min-w-0"
						@click="setAccount(account)"
					>
						<RadioButtonCheckedIcon
							v-if="selectedAccount && selectedAccount.profile.id === account.profile.id"
							class="w-4 h-4 text-indigo-400 shrink-0"
						/>
						<RadioButtonIcon v-else class="w-4 h-4 text-zinc-500 shrink-0" />
						<img :src="getAccountAvatarUrl(account)" class="w-6 h-6 rounded-lg border border-white/10 shrink-0" />
						<p
							class="m-0 truncate min-w-0 text-xs"
							:class="
								selectedAccount && selectedAccount.profile.id === account.profile.id
									? 'text-white font-bold'
									: 'text-zinc-400'
							"
						>
							{{ account.profile.name }}
						</p>
					</button>
					<IconButton
						v-tooltip="formatMessage(messages.removeAccount)"
						type="quiet"
						color="red"
						:label="formatMessage(messages.removeAccount)"
						class="mr-1 !bg-red-500/10 hover:!bg-red-500/20 !text-red-400 hover:!text-red-300 cursor-pointer transition-colors"
						@click="logout(account.profile.id)"
					>
						<TrashIcon />
					</IconButton>
				</div>
			</template>
			<div class="grid grid-cols-2 gap-2 pt-2 border-t border-white/10">
				<Button
					class="w-full !bg-emerald-600/20 hover:!bg-emerald-600/30 !text-emerald-300 border border-emerald-500/30 !text-xs font-semibold cursor-pointer transition-all duration-200"
					@click="offlineModal?.show()"
				>
					<PlusIcon class="w-3.5 h-3.5" />
					Offline Profile
				</Button>
				<Button
					class="w-full !bg-indigo-600/20 hover:!bg-indigo-600/30 !text-indigo-300 border border-indigo-500/30 !text-xs font-semibold cursor-pointer transition-all duration-200"
					:disabled="loginDisabled"
					@click="login()"
				>
					<LogInIcon class="w-3.5 h-3.5" />
					Microsoft Auth
				</Button>
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
} from '@modrinth/assets'
import {
	Accordion,
	Avatar,
	Button,
	defineMessages,
	IconButton,
	injectNotificationManager,
	useVIntl,
} from '@modrinth/ui'
import type { Ref } from 'vue'
import { computed, ref, useTemplateRef } from 'vue'

import OfflineAccountModal from '@/components/ui/modal/OfflineAccountModal.vue'
import { useAppEvent } from '@/composables/use-app-event'
import { trackEvent } from '@/helpers/analytics'
import {
	get_default_user,
	login as login_flow,
	remove_user,
	set_default_user,
	users,
} from '@/helpers/auth'
import { getPlayerHeadUrl } from '@/helpers/rendering/batch-skin-renderer.ts'
import type { Skin } from '@/helpers/skins'
import { get_available_skins } from '@/helpers/skins'
import { handleSevereError } from '@/store/error.js'

const { formatMessage } = useVIntl()
const { handleError } = injectNotificationManager()

const offlineModal = useTemplateRef('offlineModal')

const emit = defineEmits<{
	change: []
}>()

type MinecraftCredential = {
	profile: {
		id: string
		name: string
	}
}

const activeAuthTab = ref<'microsoft' | 'offline'>('offline')
const accounts: Ref<MinecraftCredential[]> = ref([])
const loginDisabled = ref(false)
const defaultUser = ref<string | undefined>()
const equippedSkin = ref<Skin | null>(null)
const headUrlCache = ref(new Map<string, string>())

async function refreshValues() {
	defaultUser.value = await get_default_user().catch(handleError)
	const userList = await users().catch(handleError)
	accounts.value = Array.isArray(userList) ? [...userList] : []
	accounts.value.sort((a, b) => (a.profile?.name ?? '').localeCompare(b.profile?.name ?? ''))

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

const selectedAccount = computed(() =>
	accounts.value.find((account) => account.profile.id === defaultUser.value),
)

const isSelectedOffline = computed(() => {
	const acc = selectedAccount.value as any
	return acc && (!acc.access_token || acc.access_token === '0')
})

const avatarUrl = computed(() => {
	if (equippedSkin.value?.texture_key) {
		const cachedUrl = headUrlCache.value.get(equippedSkin.value.texture_key)
		if (cachedUrl) {
			return cachedUrl
		}
		return `https://mc-heads.net/avatar/${equippedSkin.value.texture_key}/128`
	}
	if (selectedAccount.value?.profile?.id) {
		return `https://mc-heads.net/avatar/${selectedAccount.value.profile.id}/128`
	}
	return 'https://mc-heads.net/avatar/Steve/128'
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
	return `https://mc-heads.net/avatar/${account.profile.id}/128`
}

async function setAccount(account: MinecraftCredential) {
	defaultUser.value = account.profile.id
	await set_default_user(account.profile.id).catch(handleError)
	await refreshValues()
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
	await remove_user(id).catch(handleError)
	await refreshValues()
	if (!selectedAccount.value && accounts.value.length > 0) {
		await setAccount(accounts.value[0])
	} else {
		emit('change')
	}
	trackEvent('AccountLogOut')
}

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
