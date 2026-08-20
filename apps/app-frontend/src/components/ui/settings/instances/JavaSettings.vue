<script setup>
import { defineMessages, injectNotificationManager, useVIntl } from '@freeplay/ui'
import { ref } from 'vue'

import JavaSelector from '@/components/ui/JavaSelector.vue'
import { get_java_versions, set_java_version } from '@/helpers/jre'

const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	javaLocation: {
		id: 'app.settings.java-installations.location.title',
		defaultMessage: 'Java {version, number} location',
	},
})

const javaVersions = ref(await get_java_versions().catch(handleError))
async function updateJavaVersion(version) {
	if (version?.path === '') {
		version.path = undefined
	}

	if (version?.path) {
		version.path = version.path.replace('java.exe', 'javaw.exe')
	}

	await set_java_version(version).catch(handleError)
}
</script>
<template>
	<div class="flex flex-col gap-6">
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Java Runtimes</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					FreePlay automatically detects and manages Java runtimes required by various Minecraft
					versions.
				</p>
			</div>

			<div class="flex flex-col gap-4">
				<div
					v-for="javaVersion in [25, 21, 17, 8]"
					:key="`java-${javaVersion}`"
					class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-3"
				>
					<div class="flex items-center justify-between">
						<h3 class="m-0 text-sm font-bold text-contrast">
							{{ formatMessage(messages.javaLocation, { version: javaVersion }) }}
						</h3>
						<span
							class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-surface-3 border border-surface-4 text-secondary"
						>
							Java {{ javaVersion }}
						</span>
					</div>
					<JavaSelector
						:id="'java-selector-' + javaVersion"
						v-model="javaVersions[javaVersion]"
						:version="javaVersion"
						@update:model-value="updateJavaVersion"
					/>
				</div>
			</div>
		</section>
	</div>
</template>
