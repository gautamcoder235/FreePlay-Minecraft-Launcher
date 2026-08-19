import type { AbstractFreePlayClient } from '@freeplay/api-client'

import { createContext } from './create-context'

export const [injectFreePlayClient, provideFreePlayClient] = createContext<AbstractFreePlayClient>(
	'root',
	'freeplayClient',
)
