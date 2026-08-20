<template>
	<div class="relative flex w-full justify-center p-3">
		<RouterLink
			to="/hosting/manage"
			class="flex flex-col gap-3 p-4 w-full rounded-2xl bg-gradient-to-br from-indigo-950/60 to-purple-950/40 border border-indigo-500/20 hover:border-indigo-500/40 transition-all text-decoration-none shadow-lg group cursor-pointer"
		>
			<div class="flex items-center gap-2">
				<div
					class="w-8 h-8 rounded-lg bg-indigo-600/30 flex items-center justify-center text-indigo-400 group-hover:scale-110 transition-transform"
				>
					<svg
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<rect width="20" height="8" x="2" y="2" rx="2" ry="2" />
						<rect width="20" height="8" x="2" y="14" rx="2" ry="2" />
						<line x1="6" x2="6.01" y1="6" y2="6" />
						<line x1="6" x2="6.01" y1="18" y2="18" />
					</svg>
				</div>
				<div class="flex flex-col">
					<span class="text-xs font-bold text-indigo-300 uppercase tracking-wider"
						>FreePlay Tunnel</span
					>
					<span class="text-sm font-semibold text-contrast">Host Free Server</span>
				</div>
			</div>

			<p class="text-xs text-secondary m-0 leading-relaxed">
				Spin up a local Paper/Fabric server and share a free
				<span class="text-indigo-300 font-mono">playit.gg</span> link with friends instantly.
			</p>

			<div
				class="flex items-center justify-between pt-1 border-t border-white/5 text-xs text-indigo-400 font-medium"
			>
				<span>Open Control Room</span>
				<span class="group-hover:translate-x-1 transition-transform">→</span>
			</div>
		</RouterLink>
	</div>
</template>

<script setup>
import { RouterLink } from 'vue-router'
</script>
