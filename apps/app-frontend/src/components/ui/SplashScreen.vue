<template>
	<Transition name="splash-fade" @after-leave="onAfterLeave">
		<div v-if="!doneLoading" class="splash-screen dark">
			<div class="app-logo-wrapper" data-tauri-drag-region>
				<div class="logo-container">
					<svg
						class="app-logo"
						viewBox="0 0 460 100"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<defs>
							<!-- Diamond Facet Gradients -->
							<linearGradient id="fp-gem-top" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#a5f3fc" />
								<stop offset="100%" stop-color="#38bdf8" />
							</linearGradient>
							<linearGradient id="fp-gem-left" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#818cf8" />
								<stop offset="100%" stop-color="#4f46e5" />
							</linearGradient>
							<linearGradient id="fp-gem-right" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#22d3ee" />
								<stop offset="100%" stop-color="#0891b2" />
							</linearGradient>
							<linearGradient id="fp-gem-front-left" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#6366f1" />
								<stop offset="100%" stop-color="#3730a3" />
							</linearGradient>
							<linearGradient id="fp-gem-front-right" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#06b6d4" />
								<stop offset="100%" stop-color="#0e7490" />
							</linearGradient>
							<linearGradient id="fp-text-grad" x1="0%" y1="0%" x2="100%" y2="0%">
								<stop offset="0%" stop-color="#818cf8" />
								<stop offset="50%" stop-color="#67e8f9" />
								<stop offset="100%" stop-color="#22d3ee" />
							</linearGradient>
							<!-- Glow Filters -->
							<filter id="fp-glow" x="-30%" y="-30%" width="160%" height="160%">
								<feGaussianBlur stdDeviation="8" result="blur" />
								<feComposite in="SourceGraphic" in2="blur" operator="over" />
							</filter>
							<filter id="fp-diamond-glow" x="-40%" y="-40%" width="180%" height="180%">
								<feGaussianBlur in="SourceGraphic" stdDeviation="6" result="blur1" />
								<feGaussianBlur in="SourceGraphic" stdDeviation="14" result="blur2" />
								<feMerge>
									<feMergeNode in="blur2" />
									<feMergeNode in="blur1" />
									<feMergeNode in="SourceGraphic" />
								</feMerge>
							</filter>
						</defs>

						<!-- 3D Diamond Gem with Electric Indigo & Cyan Glow -->
						<g transform="translate(18, 12)" filter="url(#fp-diamond-glow)">
							<!-- Background Ambient Halo -->
							<circle cx="38" cy="38" r="32" fill="#6366f1" opacity="0.25" filter="url(#fp-glow)" />

							<!-- 3D Diamond Geometry -->
							<!-- Top Crown / Table -->
							<polygon points="38,4 58,22 38,34 18,22" fill="url(#fp-gem-top)" opacity="0.95" />
							<!-- Upper Left Bezel -->
							<polygon points="18,22 38,34 38,54 8,36" fill="url(#fp-gem-left)" opacity="0.9" />
							<!-- Upper Right Bezel -->
							<polygon points="58,22 38,34 38,54 68,36" fill="url(#fp-gem-right)" opacity="0.95" />
							<!-- Lower Left Pavilion -->
							<polygon points="8,36 38,54 38,72" fill="url(#fp-gem-front-left)" opacity="0.85" />
							<!-- Lower Right Pavilion -->
							<polygon points="68,36 38,54 38,72" fill="url(#fp-gem-front-right)" opacity="0.9" />

							<!-- Specular Highlights / Inner Shimmer Facets -->
							<polygon points="38,4 48,22 38,34" fill="#ffffff" opacity="0.4" />
							<polygon points="38,34 38,54 28,32" fill="#ffffff" opacity="0.25" />
							<line x1="38" y1="4" x2="38" y2="72" stroke="rgba(255,255,255,0.4)" stroke-width="1" />
						</g>

						<!-- FREEPLAY Bold Title -->
						<g transform="translate(108, 0)">
							<!-- Ambient Text Shadow/Glow -->
							<text
								x="0"
								y="62"
								font-family="'Inter', 'Montserrat', system-ui, -apple-system, sans-serif"
								font-size="44"
								font-weight="900"
								letter-spacing="0.08em"
								fill="#6366f1"
								opacity="0.3"
								filter="url(#fp-glow)"
							>FREEPLAY</text>
							<text
								x="0"
								y="62"
								font-family="'Inter', 'Montserrat', system-ui, -apple-system, sans-serif"
								font-size="44"
								font-weight="900"
								letter-spacing="0.08em"
								fill="url(#fp-text-grad)"
							>FREEPLAY</text>
						</g>
					</svg>
				</div>

				<!-- Cyber-Obsidian Loading Bar -->
				<div class="cyber-loading-container">
					<div class="cyber-loading-track">
						<div
							class="cyber-loading-fill"
							:style="{ width: `${Math.min(loadingProgress, 100)}%` }"
						>
							<div class="cyber-loading-sparkle"></div>
						</div>
					</div>
					<span v-if="message" class="cyber-loading-message">{{ message }}</span>
				</div>
			</div>

			<div class="gradient-bg" data-tauri-drag-region></div>
			<div class="cyber-ambient-glow"></div>
			<div class="base-bg"></div>
		</div>
	</Transition>
</template>

<script setup>
import { injectLoadingState } from '@freeplay/ui'
import { ref, watch } from 'vue'

import { useAppEvent } from '@/composables/use-app-event'

const doneLoading = ref(false)
const loadingProgress = ref(0)
const message = ref()

const MIN_DISPLAY_MS = 500
const mountedAt = Date.now()

const loading = injectLoadingState()

function onAfterLeave() {
	loading.setEnabled(true)
}

watch(
	[loading.barEnabled, loading.pending],
	([barEnabled, pending]) => {
		if (barEnabled) {
			return
		}

		if (pending) {
			loadingProgress.value = 0
			fakeLoadingIncrease()
			return
		}

		const elapsed = Date.now() - mountedAt
		const delay = Math.max(0, MIN_DISPLAY_MS - elapsed)

		setTimeout(() => {
			if (loading.pending.value) {
				return
			}
			doneLoading.value = true
		}, delay)
	},
	{ immediate: true },
)

function fakeLoadingIncrease() {
	if (loadingProgress.value < 95) {
		setTimeout(() => {
			loadingProgress.value += 2
			fakeLoadingIncrease()
		}, 5)
	}
}

useAppEvent('loading', (e) => {
	if (e.event.type === 'directory_move') {
		loadingProgress.value = 100 * (e.fraction ?? 1)
		message.value = 'Updating app directory...'
	}
})
</script>

<style scoped lang="scss">
.splash-screen {
	position: fixed;
	inset: 0;
	z-index: 10000;
	background: #090a0f;
	user-select: none;
}

.splash-fade-leave-active {
	transition: opacity 0.35s cubic-bezier(0.4, 0, 0.2, 1);
}

.splash-fade-leave-to {
	opacity: 0;
}

.app-logo-wrapper {
	position: absolute;
	height: 100vh;
	width: 100%;

	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;

	gap: 2rem;

	z-index: 9998;
}

.logo-container {
	display: flex;
	justify-content: center;
	align-items: center;
	filter: drop-shadow(0 0 35px rgba(99, 102, 241, 0.35)) drop-shadow(0 0 60px rgba(6, 182, 212, 0.2));
	animation: logo-float 4s ease-in-out infinite alternate;
}

@keyframes logo-float {
	0% {
		transform: translateY(0px);
	}
	100% {
		transform: translateY(-4px);
	}
}

.app-logo {
	height: 3.5rem;
	width: auto;
	max-width: 90vw;
}

.cyber-loading-container {
	display: flex;
	flex-direction: column;
	align-items: center;
	gap: 0.75rem;
	width: 100%;
	max-width: 22rem;
	padding: 0 1rem;
}

.cyber-loading-track {
	width: 100%;
	height: 0.5rem;
	background: rgba(15, 23, 42, 0.9);
	border-radius: 9999px;
	border: 1px solid rgba(255, 255, 255, 0.08);
	box-shadow:
		inset 0 2px 4px rgba(0, 0, 0, 0.6),
		0 0 10px rgba(99, 102, 241, 0.1);
	overflow: hidden;
	position: relative;
}

.cyber-loading-fill {
	height: 100%;
	background: linear-gradient(90deg, #6366f1 0%, #818cf8 40%, #06b6d4 100%);
	border-radius: 9999px;
	box-shadow:
		0 0 12px rgba(6, 182, 212, 0.7),
		0 0 24px rgba(99, 102, 241, 0.4);
	transition: width 0.25s cubic-bezier(0.4, 0, 0.2, 1);
	position: relative;
}

.cyber-loading-sparkle {
	position: absolute;
	right: 0;
	top: 0;
	bottom: 0;
	width: 1rem;
	background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.8));
	border-radius: 9999px;
	filter: blur(1px);
}

.cyber-loading-message {
	font-size: 0.8125rem;
	font-weight: 500;
	color: #94a3b8;
	letter-spacing: 0.02em;
	text-shadow: 0 0 8px rgba(0, 0, 0, 0.8);
}

.gradient-bg {
	position: absolute;
	inset: 0;
	height: 100vh;
	width: 100vw;
	background:
		radial-gradient(circle at 50% 45%, rgba(99, 102, 241, 0.12) 0%, rgba(6, 182, 212, 0.06) 35%, transparent 70%),
		linear-gradient(180deg, rgba(10, 11, 16, 0.7) 0%, #090a0f 100%);
	z-index: 9997;
}

.cyber-ambient-glow {
	position: absolute;
	left: 50%;
	top: 50%;
	transform: translate(-50%, -50%);
	width: 40rem;
	height: 25rem;
	background: radial-gradient(ellipse, rgba(99, 102, 241, 0.15), rgba(6, 182, 212, 0.05) 50%, transparent 80%);
	filter: blur(40px);
	pointer-events: none;
	z-index: 9996;
}

.base-bg {
	position: absolute;
	inset: 0;
	width: 100%;
	height: 100%;
	background: #090a0f;
	z-index: 9995;
}
</style>
