import { attributionQuickReplies } from '@freeplay/moderation'
import { provideAttributionModeration } from '@freeplay/ui'

export function setupAttributionModerationProvider() {
	provideAttributionModeration({ attributionQuickReplies })
}
