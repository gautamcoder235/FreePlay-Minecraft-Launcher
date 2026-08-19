import type { CrowdinMessages } from '@freeplay/ui'

export const moderationLocaleModules = import.meta.glob<{ default: CrowdinMessages }>(
	'./locales/*/index.json',
	{ eager: false },
)
