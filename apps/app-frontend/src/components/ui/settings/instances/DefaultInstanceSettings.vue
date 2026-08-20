<script setup lang="ts">
import {
	defineMessages,
	injectNotificationManager,
	Slider,
	StyledInput,
	Toggle,
	useVIntl,
} from '@freeplay/ui'
import { ref, watch } from 'vue'

import useMemorySlider from '@/composables/useMemorySlider'
import { get, parseEnvVars, serializeEnvVars, set } from '@/helpers/settings.ts'

const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	windowSectionTitle: {
		id: 'app.settings.default-instance-options.window-section.title',
		defaultMessage: 'Game Window & Resolution',
	},
	fullscreenTitle: {
		id: 'app.settings.default-instance-options.fullscreen.title',
		defaultMessage: 'Fullscreen',
	},
	fullscreenDescription: {
		id: 'app.settings.default-instance-options.fullscreen.description',
		defaultMessage: 'Start instances in fullscreen by updating their options.txt file.',
	},
	widthTitle: {
		id: 'app.settings.default-instance-options.width.title',
		defaultMessage: 'Width',
	},
	widthDescription: {
		id: 'app.settings.default-instance-options.width.description',
		defaultMessage: 'The width of the game window when launched.',
	},
	widthPlaceholder: {
		id: 'app.settings.default-instance-options.width.placeholder',
		defaultMessage: 'Enter width...',
	},
	heightTitle: {
		id: 'app.settings.default-instance-options.height.title',
		defaultMessage: 'Height',
	},
	heightDescription: {
		id: 'app.settings.default-instance-options.height.description',
		defaultMessage: 'The height of the game window when launched.',
	},
	heightPlaceholder: {
		id: 'app.settings.default-instance-options.height.placeholder',
		defaultMessage: 'Enter height...',
	},
	memoryAllocationTitle: {
		id: 'app.settings.default-instance-options.memory-allocation.title',
		defaultMessage: 'Memory allocation',
	},
	memoryAllocationDescription: {
		id: 'app.settings.default-instance-options.memory-allocation.description',
		defaultMessage: 'Maximum memory available to each instance.',
	},
	javaArgumentsTitle: {
		id: 'app.settings.default-instance-options.java-arguments.title',
		defaultMessage: 'Java arguments',
	},
	javaArgumentsPlaceholder: {
		id: 'app.settings.default-instance-options.java-arguments.placeholder',
		defaultMessage: 'Enter Java arguments...',
	},
	javaArgumentsDescription: {
		id: 'app.settings.default-instance-options.java-arguments.description',
		defaultMessage: 'Arguments passed to Java when launching an instance.',
	},
	environmentVariablesTitle: {
		id: 'app.settings.default-instance-options.environment-variables.title',
		defaultMessage: 'Environment variables',
	},
	environmentVariablesPlaceholder: {
		id: 'app.settings.default-instance-options.environment-variables.placeholder',
		defaultMessage: 'Enter environment variables...',
	},
	environmentVariablesDescription: {
		id: 'app.settings.default-instance-options.environment-variables.description',
		defaultMessage: 'Environment variables set when launching an instance.',
	},
	preLaunchHookTitle: {
		id: 'app.settings.default-instance-options.pre-launch-hook.title',
		defaultMessage: 'Pre-launch hook',
	},
	preLaunchHookPlaceholder: {
		id: 'app.settings.default-instance-options.pre-launch-hook.placeholder',
		defaultMessage: 'Enter pre-launch command...',
	},
	preLaunchHookDescription: {
		id: 'app.settings.default-instance-options.pre-launch-hook.description',
		defaultMessage: 'Runs before the instance starts.',
	},
	wrapperHookTitle: {
		id: 'app.settings.default-instance-options.wrapper-hook.title',
		defaultMessage: 'Wrapper hook',
	},
	wrapperHookPlaceholder: {
		id: 'app.settings.default-instance-options.wrapper-hook.placeholder',
		defaultMessage: 'Enter wrapper command...',
	},
	wrapperHookDescription: {
		id: 'app.settings.default-instance-options.wrapper-hook.description',
		defaultMessage: 'Command used to wrap the Minecraft launch process.',
	},
	postExitHookTitle: {
		id: 'app.settings.default-instance-options.post-exit-hook.title',
		defaultMessage: 'Post-exit hook',
	},
	postExitHookPlaceholder: {
		id: 'app.settings.default-instance-options.post-exit-hook.placeholder',
		defaultMessage: 'Enter post-exit command...',
	},
	postExitHookDescription: {
		id: 'app.settings.default-instance-options.post-exit-hook.description',
		defaultMessage: 'Runs after the game closes.',
	},
	hookVariablesDescription: {
		id: 'instance.settings.tabs.hooks.variables.description',
		defaultMessage:
			'Hooks run in the working directory of the instance, with the following variables:',
	},
	instanceNameDescription: {
		id: 'instance.settings.tabs.hooks.variables.inst-name.description',
		defaultMessage: '$INST_NAME: The name of the instance',
	},
	instanceIdDescription: {
		id: 'instance.settings.tabs.hooks.variables.inst-id.description',
		defaultMessage: "$INST_ID: The name of the instance's folder",
	},
	instanceDirDescription: {
		id: 'instance.settings.tabs.hooks.variables.inst-dir.description',
		defaultMessage: "$INST_DIR: The absolute path to the instance's folder",
	},
	instanceMcDirDescription: {
		id: 'instance.settings.tabs.hooks.variables.inst-mc-dir.description',
		defaultMessage: '$INST_MC_DIR: An alias for $INST_DIR',
	},
	instanceJavaDescription: {
		id: 'instance.settings.tabs.hooks.variables.inst-java.description',
		defaultMessage: '$INST_JAVA: The absolute path to the java binary',
	},
	instanceJavaArgsDescription: {
		id: 'instance.settings.tabs.hooks.variables.inst-java-args.description',
		defaultMessage: '$INST_JAVA_ARGS: The JVM Arguments provided to the game',
	},
})

const fetchSettings = await get()
fetchSettings.launchArgs = fetchSettings.extra_launch_args.join(' ')
fetchSettings.envVars = serializeEnvVars(fetchSettings.custom_env_vars)

const settings = ref(fetchSettings)

const { maxMemory, snapPoints } = (await useMemorySlider().catch(handleError)) as unknown as {
	maxMemory: number
	snapPoints: number[]
}

watch(
	settings,
	async () => {
		const setSettings = JSON.parse(JSON.stringify(settings.value))

		setSettings.extra_launch_args = setSettings.launchArgs.trim().split(/\s+/).filter(Boolean)
		setSettings.custom_env_vars = parseEnvVars(setSettings.envVars)
		delete setSettings.launchArgs
		delete setSettings.envVars

		if (!setSettings.custom_dir) {
			setSettings.custom_dir = null
		}

		await set(setSettings).catch(handleError)
	},
	{ deep: true },
)
</script>

<template>
	<div class="flex flex-col gap-6">
		<!-- Window & Resolution -->
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">
					{{ formatMessage(messages.windowSectionTitle) }}
				</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Configure default resolution and fullscreen launch mode.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="grid grid-cols-[1fr_auto] items-center gap-6">
					<div class="flex flex-col gap-0.5">
						<label for="fullscreen" class="text-sm font-semibold text-contrast cursor-pointer">
							{{ formatMessage(messages.fullscreenTitle) }}
						</label>
						<p class="m-0 text-xs text-secondary leading-relaxed">
							{{ formatMessage(messages.fullscreenDescription) }}
						</p>
					</div>
					<Toggle id="fullscreen" v-model="settings.force_fullscreen" />
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
					<div class="flex flex-col gap-1.5">
						<label for="width" class="text-sm font-semibold text-contrast">
							{{ formatMessage(messages.widthTitle) }}
						</label>
						<StyledInput
							id="width"
							v-model="settings.game_resolution[0]"
							:disabled="settings.force_fullscreen"
							autocomplete="off"
							type="number"
							:placeholder="formatMessage(messages.widthPlaceholder)"
						/>
						<p class="m-0 text-xs text-secondary leading-tight">
							{{ formatMessage(messages.widthDescription) }}
						</p>
					</div>

					<div class="flex flex-col gap-1.5">
						<label for="height" class="text-sm font-semibold text-contrast">
							{{ formatMessage(messages.heightTitle) }}
						</label>
						<StyledInput
							id="height"
							v-model="settings.game_resolution[1]"
							:disabled="settings.force_fullscreen"
							autocomplete="off"
							type="number"
							:placeholder="formatMessage(messages.heightPlaceholder)"
						/>
						<p class="m-0 text-xs text-secondary leading-tight">
							{{ formatMessage(messages.heightDescription) }}
						</p>
					</div>
				</div>
			</div>
		</section>

		<!-- Memory & Launch Arguments -->
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Memory & Java Configuration</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Set default RAM allocation, launch flags, and environment variables.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="flex flex-col gap-2">
					<label for="max-memory" class="text-sm font-semibold text-contrast">
						{{ formatMessage(messages.memoryAllocationTitle) }}
					</label>
					<Slider
						id="max-memory"
						v-model="settings.memory.maximum"
						:min="512"
						:max="maxMemory"
						:step="64"
						:snap-points="snapPoints"
						:snap-range="512"
						unit="MB"
					/>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.memoryAllocationDescription) }}
					</p>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="flex flex-col gap-2">
					<label for="java-args" class="text-sm font-semibold text-contrast">
						{{ formatMessage(messages.javaArgumentsTitle) }}
					</label>
					<StyledInput
						id="java-args"
						v-model="settings.launchArgs"
						autocomplete="off"
						type="text"
						:placeholder="formatMessage(messages.javaArgumentsPlaceholder)"
						wrapper-class="w-full"
					/>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.javaArgumentsDescription) }}
					</p>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="flex flex-col gap-2">
					<label for="env-vars" class="text-sm font-semibold text-contrast">
						{{ formatMessage(messages.environmentVariablesTitle) }}
					</label>
					<StyledInput
						id="env-vars"
						v-model="settings.envVars"
						autocomplete="off"
						type="text"
						:placeholder="formatMessage(messages.environmentVariablesPlaceholder)"
						wrapper-class="w-full"
					/>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.environmentVariablesDescription) }}
					</p>
				</div>
			</div>
		</section>

		<!-- Lifecycle Hooks -->
		<section class="flex flex-col gap-3">
			<div class="flex flex-col">
				<h2 class="m-0 text-base font-bold text-contrast">Lifecycle Hooks</h2>
				<p class="m-0 text-xs text-secondary mt-0.5">
					Execute automated custom scripts before or after game launch.
				</p>
			</div>

			<div
				class="rounded-2xl bg-surface-2 border border-surface-4 p-5 shadow-sm flex flex-col gap-5"
			>
				<div class="flex flex-col gap-2">
					<label for="pre-launch" class="text-sm font-semibold text-contrast">
						{{ formatMessage(messages.preLaunchHookTitle) }}
					</label>
					<StyledInput
						id="pre-launch"
						v-model="settings.hooks.pre_launch"
						autocomplete="off"
						type="text"
						:placeholder="formatMessage(messages.preLaunchHookPlaceholder)"
						wrapper-class="w-full"
					/>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.preLaunchHookDescription) }}
					</p>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="flex flex-col gap-2">
					<label for="wrapper" class="text-sm font-semibold text-contrast">
						{{ formatMessage(messages.wrapperHookTitle) }}
					</label>
					<StyledInput
						id="wrapper"
						v-model="settings.hooks.wrapper"
						autocomplete="off"
						type="text"
						:placeholder="formatMessage(messages.wrapperHookPlaceholder)"
						wrapper-class="w-full"
					/>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.wrapperHookDescription) }}
					</p>
				</div>

				<div class="h-px bg-surface-4/60 -mx-5" />

				<div class="flex flex-col gap-2">
					<label for="post-exit" class="text-sm font-semibold text-contrast">
						{{ formatMessage(messages.postExitHookTitle) }}
					</label>
					<StyledInput
						id="post-exit"
						v-model="settings.hooks.post_exit"
						autocomplete="off"
						type="text"
						:placeholder="formatMessage(messages.postExitHookPlaceholder)"
						wrapper-class="w-full"
					/>
					<p class="m-0 text-xs text-secondary leading-relaxed">
						{{ formatMessage(messages.postExitHookDescription) }}
					</p>
				</div>

				<div
					class="p-3.5 rounded-xl bg-surface-3 border border-surface-4/80 text-xs text-secondary leading-relaxed"
				>
					<p class="m-0 font-semibold text-contrast mb-1.5">
						{{ formatMessage(messages.hookVariablesDescription) }}
					</p>
					<ul class="m-0 pl-4 space-y-1">
						<li>{{ formatMessage(messages.instanceNameDescription) }}</li>
						<li>{{ formatMessage(messages.instanceIdDescription) }}</li>
						<li>{{ formatMessage(messages.instanceDirDescription) }}</li>
						<li>{{ formatMessage(messages.instanceMcDirDescription) }}</li>
						<li>{{ formatMessage(messages.instanceJavaDescription) }}</li>
						<li>{{ formatMessage(messages.instanceJavaArgsDescription) }}</li>
					</ul>
				</div>
			</div>
		</section>
	</div>
</template>
