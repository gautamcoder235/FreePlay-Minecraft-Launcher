<template>
	<Transition name="splash-fade" @after-leave="onAfterLeave">
		<div v-if="!doneLoading" class="splash-screen" data-tauri-drag-region>
			<!-- Animated Mesh Gradient Glowing Orbs (Clip Maker New Style) -->
			<div class="splash-bg-glow-orb splash-orb-1"></div>
			<div class="splash-bg-glow-orb splash-orb-2"></div>
			<div class="splash-bg-glow-orb splash-orb-3"></div>
			<div class="splash-bg-glow-orb splash-orb-4"></div>

			<!-- 3D Perspective Cyber Tech Grid Overlay -->
			<div class="splash-bg-grid"></div>

			<!-- Particle Constellation Interactive Canvas -->
			<canvas ref="canvasRef" class="splash-bg-canvas"></canvas>

			<!-- Central Spotlight Glow Halo -->
			<div class="splash-central-halo"></div>

			<!-- Borderless Center Floating Stage -->
			<div class="splash-center-stage" data-tauri-drag-region>
				<!-- Animated 3D Diamond & Holographic Typography -->
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
								<stop offset="0%" stop-color="#e0f2fe" />
								<stop offset="100%" stop-color="#38bdf8" />
							</linearGradient>
							<linearGradient id="fp-gem-left" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#d8b4fe" />
								<stop offset="100%" stop-color="#7c3aed" />
							</linearGradient>
							<linearGradient id="fp-gem-right" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#38bdf8" />
								<stop offset="100%" stop-color="#0284c7" />
							</linearGradient>
							<linearGradient id="fp-gem-front-left" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#c084fc" />
								<stop offset="100%" stop-color="#6366f1" />
							</linearGradient>
							<linearGradient id="fp-gem-front-right" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#22d3ee" />
								<stop offset="100%" stop-color="#0891b2" />
							</linearGradient>
							<!-- Chromatic Liquid Gradient for Text -->
							<linearGradient id="fp-holo-chroma" x1="0%" y1="0%" x2="100%" y2="0%">
								<stop offset="0%" stop-color="#ffffff" />
								<stop offset="20%" stop-color="#c084fc" />
								<stop offset="40%" stop-color="#38bdf8" />
								<stop offset="60%" stop-color="#ffffff" />
								<stop offset="80%" stop-color="#f472b6" />
								<stop offset="100%" stop-color="#38bdf8" />
							</linearGradient>
							<!-- Ambient Text Glow -->
							<filter id="fp-neon-glow" x="-40%" y="-40%" width="180%" height="180%">
								<feGaussianBlur stdDeviation="10" result="blur" />
								<feComposite in="SourceGraphic" in2="blur" operator="over" />
							</filter>
							<filter id="fp-gem-aura" x="-60%" y="-60%" width="220%" height="220%">
								<feGaussianBlur in="SourceGraphic" stdDeviation="6" result="blur1" />
								<feGaussianBlur in="SourceGraphic" stdDeviation="18" result="blur2" />
								<feMerge>
									<feMergeNode in="blur2" />
									<feMergeNode in="blur1" />
									<feMergeNode in="SourceGraphic" />
								</feMerge>
							</filter>
						</defs>

						<!-- 3D Diamond Gem with Prismatic Levitation -->
						<g class="holo-gem" transform="translate(18, 12)" filter="url(#fp-gem-aura)">
							<polygon points="38,4 58,22 38,34 18,22" fill="url(#fp-gem-top)" opacity="0.95" />
							<polygon points="18,22 38,34 38,54 8,36" fill="url(#fp-gem-left)" opacity="0.9" />
							<polygon points="58,22 38,34 38,54 68,36" fill="url(#fp-gem-right)" opacity="0.95" />
							<polygon points="8,36 38,54 38,72" fill="url(#fp-gem-front-left)" opacity="0.85" />
							<polygon points="68,36 38,54 38,72" fill="url(#fp-gem-front-right)" opacity="0.9" />
							<polygon points="38,4 48,22 38,34" fill="#ffffff" opacity="0.5" />
							<polygon points="38,34 38,54 28,32" fill="#ffffff" opacity="0.35" />
							<line
								x1="38"
								y1="4"
								x2="38"
								y2="72"
								stroke="rgba(255,255,255,0.6)"
								stroke-width="1.2"
							/>
						</g>

						<!-- Holographic FREEPLAY Title with Multi-layered Wave -->
						<g transform="translate(108, 0)">
							<!-- Background Chromatic Neon Pulse -->
							<text
								x="0"
								y="62"
								class="holo-title-ambient"
								font-family="'Inter', 'Montserrat', system-ui, -apple-system, sans-serif"
								font-size="46"
								font-weight="900"
								letter-spacing="0.14em"
								fill="#a855f7"
								opacity="0.4"
								filter="url(#fp-neon-glow)"
							>
								FREEPLAY
							</text>
							<!-- Foreground Shimmering Wave Text -->
							<text
								x="0"
								y="62"
								class="holo-title-text"
								font-family="'Inter', 'Montserrat', system-ui, -apple-system, sans-serif"
								font-size="46"
								font-weight="900"
								letter-spacing="0.14em"
								fill="url(#fp-holo-chroma)"
							>
								FREEPLAY
							</text>
						</g>
					</svg>
				</div>

				<!-- Sleek Minimalistic Neon Progress Bar -->
				<div class="splash-loader-wrapper">
					<div class="splash-loader-bar" :style="{ width: `${Math.min(loadingProgress, 100)}%` }">
						<div class="splash-loader-sparkle"></div>
					</div>
				</div>

				<!-- Clean Subtitle Status Message -->
				<span class="splash-loading-text">{{ message }}</span>
			</div>
		</div>
	</Transition>
</template>

<script setup>
import { injectLoadingState } from '@freeplay/ui'
import { onMounted, onUnmounted, ref, watch } from 'vue'

import { useAppEvent } from '@/composables/use-app-event'

const doneLoading = ref(false)
const loadingProgress = ref(15)
const message = ref('Starting FreePlay...')
const canvasRef = ref(null)

const MIN_DISPLAY_MS = 1600
const mountedAt = Date.now()

const loading = injectLoadingState()

function onAfterLeave() {
	loading.setEnabled(true)
}

let progressInterval = null
let cleanupCanvas = null

function startSmoothProgress() {
	if (progressInterval) clearInterval(progressInterval)
	progressInterval = setInterval(() => {
		if (loadingProgress.value < 88) {
			loadingProgress.value += 1
		} else if (loadingProgress.value < 94) {
			loadingProgress.value += 0.3
		}
	}, 35)
}

function initParticleCanvas() {
	const canvas = canvasRef.value
	if (!canvas) return null
	const ctx = canvas.getContext('2d')
	if (!ctx) return null

	let animId = null
	let width = (canvas.width = window.innerWidth)
	let height = (canvas.height = window.innerHeight)

	const handleResize = () => {
		if (!canvas) return
		width = canvas.width = window.innerWidth
		height = canvas.height = window.innerHeight
	}
	window.addEventListener('resize', handleResize)

	const particleCount = 45
	const particles = []
	for (let p = 0; p < particleCount; p++) {
		particles.push({
			x: Math.random() * width,
			y: Math.random() * height,
			vx: (Math.random() - 0.5) * 0.4,
			vy: (Math.random() - 0.5) * 0.4,
			radius: Math.random() * 2 + 0.8,
			alpha: Math.random() * 0.6 + 0.2,
			color: Math.random() > 0.5 ? 'rgba(168, 85, 247,' : 'rgba(6, 182, 212,',
		})
	}

	function render() {
		ctx.clearRect(0, 0, width, height)
		for (let i = 0; i < particleCount; i++) {
			for (let j = i + 1; j < particleCount; j++) {
				const dx = particles[i].x - particles[j].x
				const dy = particles[i].y - particles[j].y
				const dist = Math.sqrt(dx * dx + dy * dy)
				if (dist < 120) {
					ctx.beginPath()
					ctx.moveTo(particles[i].x, particles[i].y)
					ctx.lineTo(particles[j].x, particles[j].y)
					ctx.strokeStyle = `rgba(168, 85, 247, ${0.15 * (1 - dist / 120)})`
					ctx.lineWidth = 0.6
					ctx.stroke()
				}
			}
		}
		for (let k = 0; k < particles.length; k++) {
			const part = particles[k]
			part.x += part.vx
			part.y += part.vy
			if (part.x < 0) part.x = width
			if (part.x > width) part.x = 0
			if (part.y < 0) part.y = height
			if (part.y > height) part.y = 0
			ctx.beginPath()
			ctx.arc(part.x, part.y, part.radius, 0, Math.PI * 2)
			ctx.fillStyle = `${part.color} ${part.alpha})`
			ctx.shadowColor = part.color.includes('168') ? '#a855f7' : '#06b6d4'
			ctx.shadowBlur = 8
			ctx.fill()
		}
		animId = requestAnimationFrame(render)
	}
	render()

	return () => {
		window.removeEventListener('resize', handleResize)
		if (animId) cancelAnimationFrame(animId)
	}
}

onMounted(() => {
	cleanupCanvas = initParticleCanvas()
})

onUnmounted(() => {
	if (cleanupCanvas) cleanupCanvas()
	if (progressInterval) clearInterval(progressInterval)
})

watch(
	[loading.barEnabled, loading.pending],
	([barEnabled, pending]) => {
		if (barEnabled) {
			return
		}

		if (pending) {
			startSmoothProgress()
			return
		}

		if (progressInterval) {
			clearInterval(progressInterval)
			progressInterval = null
		}
		loadingProgress.value = 100
		message.value = 'Ready'

		const elapsed = Date.now() - mountedAt
		const delay = Math.max(250, MIN_DISPLAY_MS - elapsed)

		setTimeout(() => {
			if (loading.pending.value) {
				return
			}
			doneLoading.value = true
		}, delay)
	},
	{ immediate: true },
)

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
	background: radial-gradient(circle at 50% 50%, #0c0618 0%, #05020a 60%, #020104 100%);
	display: flex;
	align-items: center;
	justify-content: center;
	overflow: hidden;
	user-select: none;
}

.splash-fade-leave-active {
	transition:
		opacity 0.5s cubic-bezier(0.76, 0, 0.24, 1),
		transform 0.5s cubic-bezier(0.76, 0, 0.24, 1);
}

.splash-fade-leave-to {
	opacity: 0;
	transform: scale(1.04);
	pointer-events: none;
}

/* ==========================================================================
   Animated Mesh Gradient Glowing Orbs (Clip Maker New Style)
   ========================================================================== */
.splash-bg-glow-orb {
	position: absolute;
	border-radius: 50%;
	filter: blur(90px);
	pointer-events: none;
	opacity: 0.65;
	will-change: transform;
}

.splash-orb-1 {
	width: 550px;
	height: 550px;
	background: radial-gradient(
		circle,
		rgba(168, 85, 247, 0.7) 0%,
		rgba(147, 51, 234, 0.25) 55%,
		transparent 80%
	);
	top: -15%;
	left: 10%;
	animation: splashOrbFloat1 11s infinite alternate ease-in-out;
}

.splash-orb-2 {
	width: 650px;
	height: 650px;
	background: radial-gradient(
		circle,
		rgba(6, 182, 212, 0.65) 0%,
		rgba(59, 130, 246, 0.2) 60%,
		transparent 80%
	);
	bottom: -20%;
	right: 5%;
	animation: splashOrbFloat2 14s infinite alternate ease-in-out;
}

.splash-orb-3 {
	width: 480px;
	height: 480px;
	background: radial-gradient(
		circle,
		rgba(236, 72, 153, 0.55) 0%,
		rgba(217, 70, 239, 0.2) 55%,
		transparent 80%
	);
	top: 30%;
	right: 25%;
	animation: splashOrbFloat3 9s infinite alternate ease-in-out;
}

.splash-orb-4 {
	width: 420px;
	height: 420px;
	background: radial-gradient(
		circle,
		rgba(99, 102, 241, 0.6) 0%,
		rgba(79, 70, 229, 0.2) 60%,
		transparent 80%
	);
	bottom: 15%;
	left: -5%;
	animation: splashOrbFloat4 13s infinite alternate ease-in-out;
}

@keyframes splashOrbFloat1 {
	0% {
		transform: translate(0, 0) scale(1) rotate(0deg);
	}
	50% {
		transform: translate(90px, 70px) scale(1.18) rotate(45deg);
	}
	100% {
		transform: translate(-50px, 100px) scale(0.92) rotate(-30deg);
	}
}

@keyframes splashOrbFloat2 {
	0% {
		transform: translate(0, 0) scale(1) rotate(0deg);
	}
	50% {
		transform: translate(-100px, -80px) scale(1.22) rotate(-60deg);
	}
	100% {
		transform: translate(60px, -120px) scale(0.88) rotate(30deg);
	}
}

@keyframes splashOrbFloat3 {
	0% {
		transform: translate(0, 0) scale(0.88);
	}
	50% {
		transform: translate(-110px, 60px) scale(1.28);
	}
	100% {
		transform: translate(70px, -90px) scale(1.02);
	}
}

@keyframes splashOrbFloat4 {
	0% {
		transform: translate(0, 0) scale(1.12);
	}
	50% {
		transform: translate(80px, -60px) scale(0.84);
	}
	100% {
		transform: translate(-40px, 70px) scale(1.18);
	}
}

/* ==========================================================================
   3D Perspective Cyber Tech Grid Layer
   ========================================================================== */
.splash-bg-grid {
	position: absolute;
	inset: -60%;
	background-image:
		linear-gradient(to right, rgba(255, 255, 255, 0.04) 1px, transparent 1px),
		linear-gradient(to bottom, rgba(255, 255, 255, 0.04) 1px, transparent 1px);
	background-size: 54px 54px;
	transform: perspective(700px) rotateX(28deg) translateY(-30px);
	mask-image: radial-gradient(circle at 50% 50%, black 15%, transparent 70%);
	-webkit-mask-image: radial-gradient(circle at 50% 50%, black 15%, transparent 70%);
	pointer-events: none;
	animation: splashGridPulse 7s infinite alternate ease-in-out;
}

@keyframes splashGridPulse {
	0% {
		opacity: 0.35;
		transform: perspective(700px) rotateX(28deg) translateY(-30px) scale(1);
	}
	100% {
		opacity: 0.75;
		transform: perspective(700px) rotateX(28deg) translateY(-30px) scale(1.04);
	}
}

/* Floating Constellation Particle Canvas */
.splash-bg-canvas {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	pointer-events: none;
	z-index: 1;
}

/* Central Spotlight Halo */
.splash-central-halo {
	position: absolute;
	top: 50%;
	left: 50%;
	transform: translate(-50%, -50%);
	width: 520px;
	height: 520px;
	background: radial-gradient(
		circle,
		rgba(168, 85, 247, 0.28) 0%,
		rgba(6, 182, 212, 0.12) 45%,
		transparent 70%
	);
	border-radius: 50%;
	pointer-events: none;
	filter: blur(45px);
	animation: splashHaloPulse 3.8s infinite alternate ease-in-out;
	z-index: 1;
}

@keyframes splashHaloPulse {
	0% {
		transform: translate(-50%, -50%) scale(0.85);
		opacity: 0.55;
	}
	100% {
		transform: translate(-50%, -50%) scale(1.3);
		opacity: 0.95;
	}
}

/* ==========================================================================
   Center Floating Stage (No Capsule Box)
   ========================================================================== */
.splash-center-stage {
	display: flex;
	flex-direction: column;
	align-items: center;
	position: relative;
	z-index: 10;
	animation: stageReveal 1.2s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

@keyframes stageReveal {
	0% {
		opacity: 0;
		transform: translateY(20px) scale(0.95);
	}
	100% {
		opacity: 1;
		transform: translateY(0) scale(1);
	}
}

.logo-container {
	position: relative;
	display: flex;
	justify-content: center;
	align-items: center;
	margin-bottom: 28px;
}

.app-logo {
	height: 4.2rem;
	width: auto;
	max-width: 90vw;
	overflow: visible;
}

/* 3D Diamond Levitation & Prismatic Lighting */
.holo-gem {
	animation: gemLevitate 3.6s ease-in-out infinite alternate;
	transform-origin: 38px 38px;
}

@keyframes gemLevitate {
	0% {
		transform: translate(18px, 14px) rotate(-1.5deg) scale(0.98);
		filter: drop-shadow(0 0 12px rgba(99, 102, 241, 0.6))
			drop-shadow(0 0 24px rgba(168, 85, 247, 0.4));
	}
	50% {
		transform: translate(18px, 9px) rotate(1deg) scale(1.04);
		filter: drop-shadow(0 0 20px rgba(6, 182, 212, 0.9))
			drop-shadow(0 0 35px rgba(168, 85, 247, 0.7));
	}
	100% {
		transform: translate(18px, 6px) rotate(-1deg) scale(1.02);
		filter: drop-shadow(0 0 26px rgba(56, 189, 248, 0.85))
			drop-shadow(0 0 45px rgba(192, 132, 252, 0.6));
	}
}

/* FREEPLAY Ambient & Foreground Shimmer */
.holo-title-ambient {
	animation: titleAmbientPulse 3.6s ease-in-out infinite alternate;
}

@keyframes titleAmbientPulse {
	0% {
		opacity: 0.25;
		fill: #818cf8;
	}
	50% {
		opacity: 0.55;
		fill: #c084fc;
	}
	100% {
		opacity: 0.35;
		fill: #38bdf8;
	}
}

.holo-title-text {
	animation: titleFloat 4s ease-in-out infinite alternate;
	filter: drop-shadow(0 4px 20px rgba(168, 85, 247, 0.45));
}

@keyframes titleFloat {
	0% {
		transform: translateY(0px);
	}
	100% {
		transform: translateY(-2.5px);
	}
}

/* Minimalist Neon Progress Track */
.splash-loader-wrapper {
	position: relative;
	width: 280px;
	height: 4px;
	background: rgba(255, 255, 255, 0.08);
	border-radius: 9999px;
	overflow: hidden;
	margin-bottom: 16px;
	box-shadow:
		0 0 20px rgba(0, 0, 0, 0.8),
		inset 0 1px 2px rgba(0, 0, 0, 0.6);
}

.splash-loader-bar {
	position: relative;
	height: 100%;
	background: linear-gradient(90deg, #ec4899, #a855f7, #6366f1, #06b6d4, #38bdf8);
	background-size: 250% auto;
	border-radius: 9999px;
	box-shadow:
		0 0 16px rgba(168, 85, 247, 0.9),
		0 0 26px rgba(6, 182, 212, 0.7);
	transition: width 0.25s cubic-bezier(0.4, 0, 0.2, 1);
	animation: loaderGlowSweep 2.8s infinite linear;
}

@keyframes loaderGlowSweep {
	0% {
		background-position: 0% center;
	}
	100% {
		background-position: 250% center;
	}
}

.splash-loader-sparkle {
	position: absolute;
	top: 0;
	right: 0;
	height: 100%;
	width: 20px;
	background: #ffffff;
	box-shadow:
		0 0 10px #ffffff,
		0 0 20px #a855f7,
		0 0 35px #06b6d4;
	filter: blur(1.5px);
	opacity: 0.95;
}

.splash-loading-text {
	font-size: 11px;
	font-weight: 700;
	color: rgba(255, 255, 255, 0.85);
	letter-spacing: 0.12em;
	text-transform: uppercase;
	transition: opacity 0.25s ease;
	text-shadow:
		0 2px 10px rgba(0, 0, 0, 0.9),
		0 0 14px rgba(168, 85, 247, 0.3);
}
</style>
