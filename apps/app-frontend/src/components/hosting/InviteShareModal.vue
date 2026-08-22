<script setup lang="ts">
import { computed, ref } from 'vue'

const props = defineProps<{
	show: boolean
	publicIp: string
	localPort: number
	engine: string
	version: string
	serverName: string
}>()

const emit = defineEmits<{
	(e: 'close'): void
}>()

const copiedDiscord = ref(false)
const copiedIp = ref(false)

const localLanIp = computed(() => {
	return `192.168.1.100:${props.localPort}`
})

const effectivePublicAddress = computed(() => {
	return props.publicIp || `127.0.0.1:${props.localPort}`
})

const discordTemplate = computed(() => {
	return (
		`🎮 **Join my Minecraft Server!**\n` +
		`• **Server:** ${props.serverName || 'FreePlay Dedicated Server'}\n` +
		`• **Address:** \`${effectivePublicAddress.value}\`\n` +
		`• **Engine / Version:** ${props.engine} ${props.version}\n` +
		`• **LAN Direct:** \`${localLanIp.value}\`\n` +
		`Hop online and join the world!`
	)
})

async function copyToClipboard(text: string, isDiscord = false) {
	try {
		await navigator.clipboard.writeText(text)
		if (isDiscord) {
			copiedDiscord.value = true
			setTimeout(() => (copiedDiscord.value = false), 2500)
		} else {
			copiedIp.value = true
			setTimeout(() => (copiedIp.value = false), 2500)
		}
	} catch (e) {
		console.debug('Clipboard write error', e)
	}
}
</script>

<template>
	<div
		v-if="show"
		class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
	>
		<div
			class="w-full max-w-lg p-6 rounded-3xl bg-surface-1 border border-surface-4 shadow-2xl flex flex-col gap-5 animate-in fade-in zoom-in-95 duration-200"
		>
			<!-- Header -->
			<div class="flex items-center justify-between pb-3 border-b border-surface-4">
				<div class="flex items-center gap-2.5">
					<div
						class="w-8 h-8 rounded-xl bg-brand/15 text-brand flex items-center justify-center border border-brand/30"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-4 h-4"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<circle cx="18" cy="5" r="3" />
							<circle cx="6" cy="12" r="3" />
							<circle cx="18" cy="19" r="3" />
							<line x1="8.59" y1="13.51" x2="15.42" y2="17.49" />
							<line x1="15.41" y1="6.51" x2="8.59" y2="10.49" />
						</svg>
					</div>
					<div class="flex items-center">
						<h3 class="text-base font-bold font-lemon-milk tracking-wide text-contrast m-0">
							Share &amp; Invite Friends
						</h3>
					</div>
				</div>

				<button
					type="button"
					class="p-1.5 rounded-xl bg-surface-3 border border-surface-4 text-secondary hover:text-contrast transition-all cursor-pointer"
					@click="emit('close')"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-4 h-4"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<line x1="18" y1="6" x2="6" y2="18" />
						<line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				</button>
			</div>

			<!-- Public vs Local Addresses -->
			<div class="flex flex-col gap-3">
				<!-- Public Address -->
				<div
					class="p-3.5 rounded-2xl bg-surface-2 border border-surface-4 flex items-center justify-between gap-3"
				>
					<div class="flex flex-col min-w-0">
						<span class="text-[10px] font-bold text-secondary uppercase tracking-wider"
							>Public Anycast Address</span
						>
						<span class="text-xs font-mono font-bold text-contrast truncate">{{
							effectivePublicAddress
						}}</span>
					</div>
					<button
						type="button"
						class="px-3 py-1.5 rounded-xl bg-brand text-brand-inverted text-xs font-bold border-none hover:bg-brand-highlight transition-all cursor-pointer shrink-0"
						@click="copyToClipboard(effectivePublicAddress, false)"
					>
						{{ copiedIp ? '✓ Copied!' : 'Copy IP' }}
					</button>
				</div>

				<!-- Local LAN Address -->
				<div
					class="p-3.5 rounded-2xl bg-surface-2 border border-surface-4 flex items-center justify-between gap-3"
				>
					<div class="flex flex-col min-w-0">
						<span class="text-[10px] font-bold text-secondary uppercase tracking-wider"
							>Local Wi-Fi / LAN Address</span
						>
						<span class="text-xs font-mono font-bold text-contrast truncate">{{ localLanIp }}</span>
					</div>
					<button
						type="button"
						class="px-3 py-1.5 rounded-xl bg-surface-3 border border-surface-4 text-xs font-semibold text-secondary hover:text-contrast transition-all cursor-pointer shrink-0"
						@click="copyToClipboard(localLanIp, false)"
					>
						Copy LAN
					</button>
				</div>
			</div>

			<!-- Discord Formatted Template Preview -->
			<div class="flex flex-col gap-2">
				<div class="flex items-center justify-between">
					<span class="text-xs font-bold text-contrast">Discord Invite Template</span>
					<button
						type="button"
						class="text-xs text-brand hover:underline border-none bg-transparent cursor-pointer font-bold"
						@click="copyToClipboard(discordTemplate, true)"
					>
						{{ copiedDiscord ? '✓ Copied Markdown!' : 'Copy Template' }}
					</button>
				</div>
				<pre
					class="p-3.5 rounded-2xl bg-surface-3 border border-surface-4 text-secondary font-mono text-[11px] whitespace-pre-wrap leading-relaxed m-0"
					>{{ discordTemplate }}</pre
				>
			</div>

			<!-- Footer -->
			<div class="flex items-center justify-end pt-2">
				<button
					type="button"
					class="px-5 py-2 rounded-xl bg-surface-3 border border-surface-4 text-xs font-bold text-contrast hover:bg-surface-4 transition-all cursor-pointer"
					@click="emit('close')"
				>
					Done
				</button>
			</div>
		</div>
	</div>
</template>
