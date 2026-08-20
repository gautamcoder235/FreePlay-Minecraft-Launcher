<script setup lang="ts">
import { Button, Toggle } from '@freeplay/ui'
import { ref, watch } from 'vue'

import { get as getSettings, set as setSettings } from '@/helpers/settings.ts'
import { useTheming } from '@/store/state'
import { DEFAULT_FEATURE_FLAGS, type FeatureFlag } from '@/store/theme.ts'

const themeStore = useTheming()

const settings = ref(await getSettings())
const options = ref<FeatureFlag[]>(Object.keys(DEFAULT_FEATURE_FLAGS))

function setFeatureFlag(key: string, value: boolean) {
	themeStore.featureFlags[key] = value
	settings.value.feature_flags[key] = value
}

watch(
	settings,
	async () => {
		await setSettings(settings.value)
	},
	{ deep: true },
)
</script>
<template>
	<div class="flex flex-col gap-6">
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Developer Feature Flags</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Toggle experimental features and developer debug tooling.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-4"
			>
				<template v-for="(option, index) in options" :key="option">
					<div class="grid grid-cols-[1fr_auto] items-center gap-4">
						<span class="text-sm font-semibold text-contrast capitalize">
							{{ option.replaceAll('_', ' ') }}
						</span>
						<div class="flex items-center gap-2">
							<Button
								type="quiet"
								size="sm"
								:disabled="themeStore.getFeatureFlag(option) === DEFAULT_FEATURE_FLAGS[option]"
								@click="setFeatureFlag(option, DEFAULT_FEATURE_FLAGS[option])"
							>
								Reset
							</Button>
							<Toggle
								:id="`flag-${option}`"
								:model-value="themeStore.getFeatureFlag(option)"
								@update:model-value="
									() => setFeatureFlag(option, !themeStore.getFeatureFlag(option))
								"
							/>
						</div>
					</div>
					<div v-if="index < options.length - 1" class="h-px bg-surface-4/60 -mx-5" />
				</template>
			</div>
		</section>
	</div>
</template>
