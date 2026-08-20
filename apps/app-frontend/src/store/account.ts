import { defineStore } from 'pinia'

import {
	create_offline_account,
	get_default_user,
	login as login_flow,
	remove_user,
	set_default_user,
	users as getAuthUsers,
} from '@/helpers/auth.js'
import { generateOfflineUuid } from '@/helpers/offline-uuid.ts'

export interface PlayerAccount {
	id: string
	name: string
	type: 'offline' | 'microsoft'
	avatarUrl: string
	active: boolean
	isOffline: boolean
	raw?: Record<string, unknown>
}

interface RawCredential {
	profile?: {
		id?: string
		name?: string
		username?: string
	}
	id?: string
	name?: string
	username?: string
	access_token?: string
	type?: string
	active?: boolean
}

function isSameUuid(a?: string | null, b?: string | null): boolean {
	if (!a || !b) return false
	return a.replace(/-/g, '').toLowerCase() === b.replace(/-/g, '').toLowerCase()
}

export const useAccountStore = defineStore('accountStore', {
	state: () => ({
		accounts: [] as PlayerAccount[],
		activeAccountId: null as string | null,
		isLoading: false,
		isLoggingIn: false,
		isInitialized: false,
	}),

	getters: {
		activeAccount(state): PlayerAccount | null {
			if (state.accounts.length === 0) return null
			if (state.activeAccountId) {
				const found = state.accounts.find(
					(a) =>
						isSameUuid(a.id, state.activeAccountId) ||
						a.name.toLowerCase() === state.activeAccountId?.toLowerCase(),
				)
				if (found) return found
			}
			return state.accounts.find((a) => a.active) || state.accounts[0] || null
		},

		activePlayerName(): string {
			return this.activeAccount?.name || 'Player'
		},

		activePlayerAvatar(): string {
			if (this.activeAccount?.avatarUrl) {
				return this.activeAccount.avatarUrl
			}
			const name = this.activePlayerName
			return `https://mc-heads.net/avatar/${encodeURIComponent(name)}/128`
		},

		isActiveOffline(): boolean {
			return this.activeAccount?.isOffline ?? true
		},

		accountCount(state): number {
			return state.accounts.length
		},
	},

	actions: {
		init() {
			if (this.isInitialized) return
			this.isInitialized = true

			// Global cross-window and in-app event listeners
			window.addEventListener('freeplay-account-changed', (e: Event) => {
				const custom = e as CustomEvent<{ id?: string }>
				if (custom.detail?.id) {
					void this.refresh(custom.detail.id)
				} else {
					void this.refresh()
				}
			})

			window.addEventListener('storage', (e) => {
				if (
					e.key === 'freeplay-offline-accounts' ||
					e.key === 'freeplay-active-player' ||
					e.key === 'freeplay-default-user'
				) {
					void this.refresh()
				}
			})

			void this.refresh()
		},

		async refresh(preferredActiveId?: string) {
			this.isLoading = true
			try {
				const localActive = localStorage.getItem('freeplay-active-player')
				let localActiveId: string | undefined
				try {
					if (localActive) {
						localActiveId = JSON.parse(localActive)?.id
					}
				} catch {
					// ignore
				}
				const localDefault = localStorage.getItem('freeplay-default-user') || undefined

				let defaultId: string | undefined
				try {
					defaultId = await get_default_user()
				} catch {
					defaultId = undefined
				}

				// Priority: explicit parameter -> active in store -> active in localStorage -> local default -> backend default
				const resolvedTarget =
					preferredActiveId || this.activeAccountId || localActiveId || localDefault || defaultId

				let rawBackendUsers: RawCredential[] = []
				try {
					const list = await getAuthUsers()
					if (Array.isArray(list)) {
						rawBackendUsers = list
					}
				} catch {
					rawBackendUsers = []
				}

				let localOfflineList: Array<{
					id?: string
					name?: string
					username?: string
					type?: string
				}> = []
				try {
					const saved = localStorage.getItem('freeplay-offline-accounts')
					if (saved) {
						const parsed = JSON.parse(saved)
						if (Array.isArray(parsed)) {
							localOfflineList = parsed
						}
					}
				} catch {
					localOfflineList = []
				}

				const normalizedMap = new Map<string, PlayerAccount>()

				// 1. Process backend credentials first
				for (const cred of rawBackendUsers) {
					const profileId = cred.profile?.id || cred.id
					const profileName = cred.profile?.name || cred.name || cred.username
					if (!profileId || !profileName) continue

					const isOffline =
						!cred.access_token || cred.access_token === '0' || cred.type === 'offline'

					const account: PlayerAccount = {
						id: profileId,
						name: profileName,
						type: isOffline ? 'offline' : 'microsoft',
						avatarUrl: `https://mc-heads.net/avatar/${encodeURIComponent(profileName)}/128`,
						active: isSameUuid(profileId, resolvedTarget),
						isOffline,
						raw: cred as Record<string, unknown>,
					}
					normalizedMap.set(profileName.toLowerCase(), account)
				}

				// 2. Process local offline accounts
				let storageUpdated = false
				for (const off of localOfflineList) {
					const name = (off.username || off.name || '').trim()
					if (!name) continue

					const standardUuid = generateOfflineUuid(name)
					if (off.id !== standardUuid) {
						off.id = standardUuid
						storageUpdated = true
					}

					const existing = normalizedMap.get(name.toLowerCase())
					if (!existing) {
						normalizedMap.set(name.toLowerCase(), {
							id: standardUuid,
							name,
							type: 'offline',
							avatarUrl: `https://mc-heads.net/avatar/${encodeURIComponent(name)}/128`,
							active: isSameUuid(standardUuid, resolvedTarget),
							isOffline: true,
						})
					}
				}

				if (storageUpdated) {
					try {
						localStorage.setItem('freeplay-offline-accounts', JSON.stringify(localOfflineList))
					} catch {
						// ignore
					}
				}

				const mergedList = Array.from(normalizedMap.values())
				mergedList.sort((a, b) => a.name.localeCompare(b.name))

				let activeAcc = mergedList.find((a) => isSameUuid(a.id, resolvedTarget))
				if (!activeAcc && resolvedTarget) {
					activeAcc = mergedList.find((a) => a.name.toLowerCase() === resolvedTarget.toLowerCase())
				}
				if (!activeAcc && this.activeAccountId) {
					activeAcc = mergedList.find((a) => isSameUuid(a.id, this.activeAccountId))
				}
				if (!activeAcc && mergedList.length > 0) {
					const nonPlayer = mergedList.find((a) => a.name.toLowerCase() !== 'player')
					activeAcc = nonPlayer || mergedList[0]
				}

				const activeId = activeAcc?.id || null
				for (const acc of mergedList) {
					acc.active = isSameUuid(acc.id, activeId)
				}

				this.accounts = mergedList
				this.activeAccountId = activeId

				if (activeAcc) {
					try {
						if (activeAcc.isOffline && activeAcc.name) {
							await create_offline_account(activeAcc.name)
						} else if (activeAcc.id) {
							await set_default_user(activeAcc.id)
						}
					} catch {
						// ignore
					}

					try {
						const profileObj = {
							id: activeAcc.id,
							username: activeAcc.name,
							name: activeAcc.name,
							type: activeAcc.type,
							avatar_url: activeAcc.avatarUrl,
						}
						localStorage.setItem('freeplay-active-player', JSON.stringify(profileObj))
						localStorage.setItem('freeplay-default-user', activeAcc.id)
					} catch {
						// ignore
					}
				}
			} finally {
				this.isLoading = false
			}
		},

		async createOfflineAccount(rawUsername: string): Promise<PlayerAccount> {
			const cleanName = rawUsername
				.trim()
				.replace(/[^a-zA-Z0-9_]/g, '')
				.slice(0, 16)
			if (!cleanName) {
				throw new Error(
					'Username must be 1-16 characters containing letters, numbers, and underscores.',
				)
			}

			const standardUuid = generateOfflineUuid(cleanName)
			let createdId = standardUuid

			try {
				const created = (await create_offline_account(cleanName)) as RawCredential | null
				if (created?.profile?.id) {
					createdId = created.profile.id
				}
			} catch (ipcErr) {
				console.warn('create_offline_account IPC fallback note:', ipcErr)
			}

			try {
				await set_default_user(createdId)
			} catch {
				// ignore
			}

			// Update localStorage list
			try {
				const saved = localStorage.getItem('freeplay-offline-accounts') || '[]'
				const list = JSON.parse(saved) as Array<{ username?: string; name?: string; id?: string }>
				const profileObj = {
					id: createdId,
					username: cleanName,
					name: cleanName,
					type: 'offline',
					avatar_url: `https://mc-heads.net/avatar/${encodeURIComponent(cleanName)}/128`,
				}
				const existingIdx = list.findIndex(
					(a) => (a.username || a.name)?.toLowerCase() === cleanName.toLowerCase(),
				)
				if (existingIdx >= 0) {
					list[existingIdx] = profileObj
				} else {
					list.unshift(profileObj)
				}
				localStorage.setItem('freeplay-offline-accounts', JSON.stringify(list))
				localStorage.setItem('freeplay-active-player', JSON.stringify(profileObj))
				localStorage.setItem('freeplay-default-user', createdId)
				window.dispatchEvent(new CustomEvent('freeplay-account-changed', { detail: profileObj }))
			} catch {
				// ignore
			}

			await this.refresh(createdId)

			const active = this.activeAccount || {
				id: createdId,
				name: cleanName,
				type: 'offline' as const,
				avatarUrl: `https://mc-heads.net/avatar/${encodeURIComponent(cleanName)}/128`,
				active: true,
				isOffline: true,
			}
			return active
		},

		async setActiveAccount(accountId: string) {
			const target = this.accounts.find(
				(a) => isSameUuid(a.id, accountId) || a.name.toLowerCase() === accountId.toLowerCase(),
			)
			if (!target) return

			const activeId = target.id
			this.activeAccountId = activeId
			for (const acc of this.accounts) {
				acc.active = isSameUuid(acc.id, activeId)
			}

			try {
				if (target.isOffline && target.name) {
					await create_offline_account(target.name)
				} else {
					await set_default_user(activeId)
				}
			} catch (err) {
				console.warn('setActiveAccount set_default_user fallback notice:', err)
			}

			try {
				const profileObj = {
					id: activeId,
					username: target.name,
					name: target.name,
					type: target.type,
					avatar_url: target.avatarUrl,
				}
				localStorage.setItem('freeplay-active-player', JSON.stringify(profileObj))
				localStorage.setItem('freeplay-default-user', activeId)
				window.dispatchEvent(new CustomEvent('freeplay-account-changed', { detail: profileObj }))
			} catch {
				// ignore
			}

			await this.refresh(activeId)
		},

		async removeAccount(accountId: string) {
			const target = this.accounts.find((a) => a.id === accountId)
			const targetName = target?.name?.toLowerCase()

			try {
				await remove_user(accountId)
			} catch {
				// ignore
			}

			try {
				const saved = localStorage.getItem('freeplay-offline-accounts')
				if (saved) {
					const list = JSON.parse(saved) as Array<{ id?: string; username?: string; name?: string }>
					if (Array.isArray(list)) {
						const filtered = list.filter((a) => {
							const name = (a.username || a.name)?.toLowerCase()
							return a.id !== accountId && name !== targetName
						})
						localStorage.setItem('freeplay-offline-accounts', JSON.stringify(filtered))
					}
				}
			} catch {
				// ignore
			}

			await this.refresh()
			if (
				this.accounts.length > 0 &&
				(!this.activeAccount || this.activeAccount.id === accountId)
			) {
				await this.setActiveAccount(this.accounts[0].id)
			}
		},

		async loginMicrosoft() {
			this.isLoggingIn = true
			try {
				const loggedIn = (await login_flow()) as RawCredential | null
				if (loggedIn?.profile?.id) {
					await this.setActiveAccount(loggedIn.profile.id)
				}
			} finally {
				this.isLoggingIn = false
			}
		},
	},
})
