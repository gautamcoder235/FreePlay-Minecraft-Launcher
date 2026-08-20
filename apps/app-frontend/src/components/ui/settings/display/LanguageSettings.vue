<script setup lang="ts">
import {
	Admonition,
	AutoLink,
	commonSettingsMessages,
	IntlFormatted,
	LanguageSelector,
	languageSelectorMessages,
	LOCALES,
	useVIntl,
} from '@freeplay/ui'
import { computed, ref, watch } from 'vue'

import { get, set } from '@/helpers/settings.ts'
import i18n from '@/i18n.config'

const { formatMessage } = useVIntl()

const platform = computed(() => formatMessage(languageSelectorMessages.platformApp))

const settings = ref(await get())

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)

const $isChanging = ref(false)

async function onLocaleChange(newLocale: string) {
	if (settings.value.locale === newLocale) return

	$isChanging.value = true
	try {
		i18n.global.locale.value = newLocale
		settings.value.locale = newLocale
	} finally {
		$isChanging.value = false
	}
}
</script>

<template>
	<div class="flex flex-col gap-6">
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(commonSettingsMessages.language) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Select your preferred launcher display language.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-4"
			>
				<Admonition type="warning" class="m-0">
					{{ formatMessage(languageSelectorMessages.languageWarning, { platform }) }}
				</Admonition>

				<p class="m-0 text-xs text-secondary leading-relaxed">
					<IntlFormatted
						:message-id="languageSelectorMessages.languagesDescription"
						:values="{ platform }"
					>
						<template #~crowdin-link="{ children }">
							<AutoLink to="https://translate.freeplay.app" class="text-brand hover:underline">
								<component :is="() => children" />
							</AutoLink>
						</template>
					</IntlFormatted>
				</p>

				<div class="pt-2 border-t border-surface-4/60">
					<LanguageSelector
						:current-locale="settings.locale"
						:locales="LOCALES"
						:on-locale-change="onLocaleChange"
						:is-changing="$isChanging"
					/>
				</div>
			</div>
		</section>
	</div>
</template>
