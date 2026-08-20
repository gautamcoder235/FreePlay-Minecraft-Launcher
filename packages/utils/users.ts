// noinspection JSUnusedGlobalSymbols

export const getUserLink = (user) => {
	return `/user/${user.username}`
}

export const isStaff = (user) => {
	return !!user && STAFF_ROLES.includes(user.role)
}

export const isAdmin = (user) => {
	return user && user.role === 'admin'
}

export const STAFF_ROLES = ['moderator', 'admin']

export const FREEPLAY_USER_ID = '2REoufqX'
export const MODRINTH_USER_ID = FREEPLAY_USER_ID
export const AUTOMOD_USER_ID = ''
export const FREEPLAY_ARCHIVES_USER_ID = 'GVFjtWTf'
export const MODRINTH_ARCHIVES_USER_ID = FREEPLAY_ARCHIVES_USER_ID

export const OFFICIAL_ACCOUNT_IDS = [FREEPLAY_USER_ID, AUTOMOD_USER_ID, FREEPLAY_ARCHIVES_USER_ID]

export const isFreePlayUser = (userId) => {
	return userId === FREEPLAY_USER_ID
}

export const isModrinthUser = isFreePlayUser

export const isOfficialAccount = (userId) => {
	return OFFICIAL_ACCOUNT_IDS.includes(userId)
}
