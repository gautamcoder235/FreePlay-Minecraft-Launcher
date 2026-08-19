import type { AbstractFreePlayClient } from '@freeplay/api-client'
import { provideUserCountry } from '@freeplay/ui'
import { ref } from 'vue'

export function setupUserCountryProvider(client: AbstractFreePlayClient) {
	const country = ref('US')

	void client.labrinth.geoip
		.getCountry()
		.then((detectedCountry) => {
			country.value = detectedCountry ?? country.value
		})
		.catch(() => {})

	return provideUserCountry(country)
}
