<template>
	<div class="app-sidebar-nav bg-zinc-950/80 backdrop-blur-md border-r border-white/10 flex flex-col p-2 gap-2.5 w-[--left-bar-width] select-none">
		<!-- Home / Library -->
		<NavButton
			v-tooltip.right="formatMessage(messages.home)"
			to="/"
			:is-primary="(route) => route.path === '/'"
			:is-subpage="
				() =>
					(route.path.startsWith('/browse') || route.path.startsWith('/project')) && route.query.i
			"
		>
			<PlayIcon />
		</NavButton>

		<!-- Discover Content -->
		<NavButton
			v-tooltip.right="formatMessage(commonMessages.discoverContentLabel)"
			to="/browse/modpack"
			:is-primary="() => route.path.startsWith('/browse') && !route.query.i && !route.query.sid"
			:is-subpage="
				(route) => route.path.startsWith('/project') && !route.query.i && !route.query.sid
			"
		>
			<CompassIcon />
		</NavButton>

		<!-- Skin Selector -->
		<NavButton v-tooltip.right="formatMessage(appMessages.skinSelectorLabel)" to="/skins">
			<ShirtIcon />
		</NavButton>

		<!-- FreePlay Server Hosting -->
		<NavButton
			v-tooltip.right="formatMessage(messages.hosting)"
			to="/hosting/manage"
			:is-primary="(r) => r.path === '/hosting/manage' || r.path === '/hosting/manage/'"
			:is-subpage="
				(r) =>
					(r.path.startsWith('/hosting/manage/') && r.path !== '/hosting/manage/') ||
					((r.path.startsWith('/browse') || r.path.startsWith('/project')) && r.query.sid)
			"
		>
			<ServerStackIcon />
		</NavButton>

		<!-- Quick Instance Switcher -->
		<Suspense>
			<QuickInstanceSwitcher />
		</Suspense>

		<!-- Create New Instance -->
		<NavButton
			v-tooltip.right="formatMessage(messages.createNewInstance)"
			:to="() => emit('showCreationModal')"
			:disabled="offline"
		>
			<PlusIcon />
		</NavButton>

		<div class="flex flex-grow"></div>

		<!-- App Settings -->
		<NavButton
			v-tooltip.right="formatMessage(commonMessages.settingsLabel)"
			:to="() => emit('showAppSettings')"
		>
			<SettingsIcon />
		</NavButton>

		<!-- FreePlay User Profile / Sign In -->
		<TeleportOverflowMenu
			v-if="credentials?.user"
			v-tooltip.right="formatMessage(messages.account)"
			type="quiet"
			size="xl"
			:label="formatMessage(messages.moreOptions)"
			:options="[
				{
					id: 'view-profile',
					label: formatMessage(messages.signedInAs, {
						username: credentials.user.username,
					}),
					action: () => router.push(`/user/${encodeURIComponent(credentials.user.username)}`),
				},
				{
					id: 'sign-out',
					label: formatMessage(commonMessages.signOutButton),
					tone: 'red',
					action: () => emit('logout'),
				},
			]"
			placement="right-end"
			:distance="4"
		>
			<Avatar :src="credentials?.user?.avatar_url" alt="" size="32px" circle />
			<template #view-profile>
				<UserIcon />
				<span class="inline-flex items-center gap-1">
					<IntlFormatted
						:message-id="messages.signedInAs"
						:values="{ username: credentials?.user?.username }"
					>
						<template #user="{ children }">
							<span class="inline-flex items-center gap-1 text-contrast font-semibold">
								<Avatar :src="credentials?.user?.avatar_url" alt="" size="20px" circle />
								<component :is="() => children" />
							</span>
						</template>
					</IntlFormatted>
				</span>
			</template>
			<template #sign-out>
				<LogOutIcon />
				{{ formatMessage(commonMessages.signOutButton) }}
			</template>
		</TeleportOverflowMenu>

		<NavButton
			v-else
			v-tooltip.right="formatMessage(messages.signIn)"
			:to="() => emit('signIn')"
		>
			<LogInIcon class="text-brand" />
		</NavButton>
	</div>
</template>

<script setup lang="ts">
import {
	CompassIcon,
	LogInIcon,
	LogOutIcon,
	PlayIcon,
	PlusIcon,
	ServerStackIcon,
	SettingsIcon,
	ShirtIcon,
	UserIcon,
} from '@freeplay/assets'
import {
	Avatar,
	commonMessages,
	defineMessages,
	IntlFormatted,
	TeleportOverflowMenu,
	useVIntl,
} from '@freeplay/ui'
import { useRoute, useRouter } from 'vue-router'

import NavButton from '@/components/ui/NavButton.vue'
import QuickInstanceSwitcher from '@/components/ui/QuickInstanceSwitcher.vue'
import { appMessages } from '@/utils/app-messages'

defineProps<{
	credentials?: any
	offline?: boolean
}>()

const emit = defineEmits<{
	(e: 'showCreationModal'): void
	(e: 'showAppSettings'): void
	(e: 'signIn'): void
	(e: 'logout'): void
}>()

const router = useRouter()
const route = useRoute()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	home: { id: 'app.nav.home', defaultMessage: 'FreePlay Home' },
	hosting: { id: 'app.nav.modrinth-hosting', defaultMessage: 'FreePlay Server Hosting' },
	createNewInstance: { id: 'app.nav.create-new-instance', defaultMessage: 'Create new instance' },
	account: { id: 'app.nav.modrinth-account', defaultMessage: 'FreePlay account' },
	signedInAs: { id: 'app.nav.signed-in-as', defaultMessage: 'Signed in as <user>{username}</user>' },
	signIn: { id: 'app.nav.sign-in-to-modrinth-account', defaultMessage: 'Sign in to a FreePlay account' },
	moreOptions: { id: 'app.navigation.more-options', defaultMessage: 'More options' },
})
</script>

<style scoped>
.app-sidebar-nav {
	grid-area: nav;
	position: relative;
	z-index: 2;
}
</style>
