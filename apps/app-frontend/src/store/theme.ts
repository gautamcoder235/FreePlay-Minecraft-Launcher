import { defineStore } from 'pinia'

let systemThemeMq: MediaQueryList | null = null

export const DEFAULT_FEATURE_FLAGS = {
	project_background: false,
	page_path: false,
	worlds_in_home: true,
	server_project_qa: false,
	show_version_environment_column: false,
	server_ram_as_bytes_always_on: false,
	always_show_app_controls: false,
	skip_non_essential_warnings: false,
	skip_unknown_pack_warning: false,
	pride_fundraiser: true,
	i18n_debug: false,
	show_instance_play_time: true,
	advanced_filters_collapsed: true,
	always_show_copy_details: false,
	hide_installed_modpacks: false,
	friends_active_collapsed: false,
	friends_online_collapsed: false,
	friends_offline_collapsed: true,
	friends_pending_collapsed: true,
	dismissed_photosensitivity_filter_warning: false,
	high_contrast_borders: false,
}

export const THEME_OPTIONS = ['dark', 'light', 'oled', 'retro', 'system'] as const
export const ACCENT_OPTIONS = [
	'sky',
	'emerald',
	'amethyst',
	'crimson',
	'amber',
	'neon',
	'cyan',
	'rose',
] as const

export type FeatureFlag = keyof typeof DEFAULT_FEATURE_FLAGS
export type FeatureFlags = Record<FeatureFlag, boolean>
export type ColorTheme = (typeof THEME_OPTIONS)[number]
export type AccentColor = (typeof ACCENT_OPTIONS)[number]

export interface AccentPresetInfo {
	id: AccentColor
	name: string
	flavor: string
	color: string
	highlight: string
	contrast: string
	bg: string
	shadow: string
	gradient: string
	gradientClasses: string
	glow: string
}

export const ACCENT_PRESETS: Record<AccentColor, AccentPresetInfo> = {
	sky: {
		id: 'sky',
		name: 'Diamond Sky',
		flavor: 'Cyber Diamond',
		color: '#0ea5e9',
		highlight: '#38bdf8',
		contrast: '#ffffff',
		bg: 'rgba(14, 165, 233, 0.15)',
		shadow: 'rgba(14, 165, 233, 0.35)',
		gradient: 'linear-gradient(135deg, #0ea5e9 0%, #38bdf8 50%, #06b6d4 100%)',
		gradientClasses: 'from-sky-500 via-blue-500 to-cyan-400',
		glow: '0 0 24px rgba(56, 189, 248, 0.4)',
	},
	emerald: {
		id: 'emerald',
		name: 'Emerald Creeper',
		flavor: 'Classic Gamer Green',
		color: '#10b981',
		highlight: '#34d399',
		contrast: '#ffffff',
		bg: 'rgba(16, 185, 129, 0.15)',
		shadow: 'rgba(16, 185, 129, 0.35)',
		gradient: 'linear-gradient(135deg, #059669 0%, #10b981 50%, #34d399 100%)',
		gradientClasses: 'from-emerald-600 via-emerald-500 to-teal-400',
		glow: '0 0 24px rgba(16, 185, 129, 0.4)',
	},
	amethyst: {
		id: 'amethyst',
		name: 'Amethyst Shard',
		flavor: 'Mystic Violet',
		color: '#8b5cf6',
		highlight: '#a78bfa',
		contrast: '#ffffff',
		bg: 'rgba(139, 92, 246, 0.15)',
		shadow: 'rgba(139, 92, 246, 0.35)',
		gradient: 'linear-gradient(135deg, #7c3aed 0%, #8b5cf6 50%, #c084fc 100%)',
		gradientClasses: 'from-violet-600 via-purple-500 to-fuchsia-400',
		glow: '0 0 24px rgba(139, 92, 246, 0.4)',
	},
	crimson: {
		id: 'crimson',
		name: 'Netherite Crimson',
		flavor: 'Fiery Ruby',
		color: '#f43f5e',
		highlight: '#fb7185',
		contrast: '#ffffff',
		bg: 'rgba(244, 63, 94, 0.15)',
		shadow: 'rgba(244, 63, 94, 0.35)',
		gradient: 'linear-gradient(135deg, #e11d48 0%, #f43f5e 50%, #fb7185 100%)',
		gradientClasses: 'from-rose-600 via-red-500 to-orange-400',
		glow: '0 0 24px rgba(244, 63, 94, 0.4)',
	},
	amber: {
		id: 'amber',
		name: 'Sunset Amber',
		flavor: 'Warm Gold',
		color: '#f59e0b',
		highlight: '#fbbf24',
		contrast: '#ffffff',
		bg: 'rgba(245, 158, 11, 0.15)',
		shadow: 'rgba(245, 158, 11, 0.35)',
		gradient: 'linear-gradient(135deg, #d97706 0%, #f59e0b 50%, #fbbf24 100%)',
		gradientClasses: 'from-amber-600 via-amber-500 to-yellow-400',
		glow: '0 0 24px rgba(245, 158, 11, 0.4)',
	},
	neon: {
		id: 'neon',
		name: 'Electric Lime',
		flavor: 'High-Vis Neon',
		color: '#84cc16',
		highlight: '#a3e635',
		contrast: '#000000',
		bg: 'rgba(132, 204, 22, 0.15)',
		shadow: 'rgba(132, 204, 22, 0.35)',
		gradient: 'linear-gradient(135deg, #65a30d 0%, #84cc16 50%, #bef264 100%)',
		gradientClasses: 'from-lime-600 via-lime-500 to-emerald-400',
		glow: '0 0 24px rgba(132, 204, 22, 0.4)',
	},
	cyan: {
		id: 'cyan',
		name: 'Midnight Cyan',
		flavor: 'Deep Ocean Teal',
		color: '#06b6d4',
		highlight: '#22d3ee',
		contrast: '#ffffff',
		bg: 'rgba(6, 182, 212, 0.15)',
		shadow: 'rgba(6, 182, 212, 0.35)',
		gradient: 'linear-gradient(135deg, #0891b2 0%, #06b6d4 50%, #67e8f9 100%)',
		gradientClasses: 'from-cyan-600 via-teal-500 to-blue-400',
		glow: '0 0 24px rgba(6, 182, 212, 0.4)',
	},
	rose: {
		id: 'rose',
		name: 'Cherry Blossom',
		flavor: 'Vibrant Pink',
		color: '#ec4899',
		highlight: '#f472b6',
		contrast: '#ffffff',
		bg: 'rgba(236, 72, 153, 0.15)',
		shadow: 'rgba(236, 72, 153, 0.35)',
		gradient: 'linear-gradient(135deg, #db2777 0%, #ec4899 50%, #f472b6 100%)',
		gradientClasses: 'from-pink-600 via-pink-500 to-rose-400',
		glow: '0 0 24px rgba(236, 72, 153, 0.4)',
	},
}

export type ThemeStore = {
	selectedTheme: ColorTheme
	selectedAccent: AccentColor
	highContrastBorders: boolean
	advancedRendering: boolean
	hideNametagSkinsPage: boolean
	toggleSidebar: boolean

	devMode: boolean
	featureFlags: FeatureFlags
}

export const DEFAULT_THEME_STORE: ThemeStore = {
	selectedTheme: 'dark',
	selectedAccent: 'sky',
	highContrastBorders: false,
	advancedRendering: true,
	hideNametagSkinsPage: false,
	toggleSidebar: false,

	devMode: false,
	featureFlags: DEFAULT_FEATURE_FLAGS,
}

export const useTheming = defineStore('themeStore', {
	state: () => {
		let initialTheme: ColorTheme = 'dark'
		let initialAccent: AccentColor = 'sky'
		let initialHighContrast = false

		try {
			const savedTheme = localStorage.getItem('freeplay-theme') as ColorTheme | null
			if (savedTheme && THEME_OPTIONS.includes(savedTheme)) {
				initialTheme = savedTheme
			}

			const savedAccent = localStorage.getItem('freeplay-accent') as AccentColor | null
			if (savedAccent && ACCENT_OPTIONS.includes(savedAccent)) {
				initialAccent = savedAccent
			}

			const savedHighContrast = localStorage.getItem('freeplay-high-contrast')
			if (savedHighContrast === 'true') {
				initialHighContrast = true
			}
		} catch {
			// ignore
		}

		return {
			...DEFAULT_THEME_STORE,
			selectedTheme: initialTheme,
			selectedAccent: initialAccent,
			highContrastBorders: initialHighContrast,
		}
	},
	getters: {
		resolvedTheme(state): 'dark' | 'light' | 'oled' | 'retro' {
			if (state.selectedTheme === 'system') {
				if (typeof window !== 'undefined' && window.matchMedia) {
					return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
				}
				return 'dark'
			}
			return state.selectedTheme === 'retro'
				? 'retro'
				: state.selectedTheme === 'oled'
					? 'oled'
					: state.selectedTheme === 'light'
						? 'light'
						: 'dark'
		},
		isDark(): boolean {
			return this.resolvedTheme !== 'light'
		},
		isOled(): boolean {
			return this.resolvedTheme === 'oled'
		},
		currentAccentPreset(state): AccentPresetInfo {
			return ACCENT_PRESETS[state.selectedAccent] || ACCENT_PRESETS.sky
		},
	},
	actions: {
		setThemeState(newTheme: ColorTheme) {
			if (THEME_OPTIONS.includes(newTheme)) {
				this.selectedTheme = newTheme
				try {
					localStorage.setItem('freeplay-theme', newTheme)
				} catch {
					// ignore
				}
			} else {
				console.warn('Selected theme is not present. Check themeOptions.')
			}

			this.setThemeClass()
		},
		setAccentState(newAccent: AccentColor) {
			if (ACCENT_OPTIONS.includes(newAccent)) {
				this.selectedAccent = newAccent
				try {
					localStorage.setItem('freeplay-accent', newAccent)
				} catch {
					// ignore
				}
			}
			this.setThemeClass()
		},
		setHighContrastBorders(enabled: boolean) {
			this.highContrastBorders = enabled
			try {
				localStorage.setItem('freeplay-high-contrast', String(enabled))
			} catch {
				// ignore
			}
			this.setThemeClass()
		},
		setThemeClass() {
			if (typeof document === 'undefined') return

			const html = document.documentElement
			const body = document.body

			const allThemes = [
				'dark-mode',
				'light-mode',
				'oled-mode',
				'retro-mode',
				'system-mode',
				'dark',
				'light',
				'oled',
				'retro',
			]

			for (const theme of allThemes) {
				html.classList.remove(theme)
				if (body) body.classList.remove(theme)
			}

			if (systemThemeMq) {
				systemThemeMq.removeEventListener('change', this._handleSystemMediaChange)
				systemThemeMq = null
			}

			if (this.selectedTheme === 'system' && typeof window !== 'undefined' && window.matchMedia) {
				systemThemeMq = window.matchMedia('(prefers-color-scheme: dark)')
				systemThemeMq.addEventListener('change', this._handleSystemMediaChange)
			}

			const activeTheme = this.resolvedTheme
			const activeAccent = this.selectedAccent || 'sky'
			const preset = ACCENT_PRESETS[activeAccent] || ACCENT_PRESETS.sky

			html.classList.add(`${activeTheme}-mode`, activeTheme)
			if (body) {
				body.classList.add(`${activeTheme}-mode`, activeTheme)
			}

			html.setAttribute('data-theme', activeTheme)
			html.setAttribute('data-color-mode', activeTheme)
			html.setAttribute('data-accent', activeAccent)
			html.setAttribute('data-high-contrast', String(this.highContrastBorders))
			// Dynamically set accent CSS custom properties on documentElement and body
			const propsToSet: Record<string, string> = {
				'--color-brand': preset.color,
				'--color-brand-highlight': preset.highlight,
				'--color-brand-contrast': preset.contrast,
				'--color-accent-contrast': preset.contrast,
				'--color-brand-bg': preset.bg,
				'--color-brand-shadow': preset.shadow,
				'--color-brand-gradient': preset.gradient,
				'--accent-glow': preset.glow,
				'--accent-primary': preset.highlight,
				'--loading-bar-gradient': `linear-gradient(to right, ${preset.color} 0%, ${preset.highlight} 100%)`,
			}

			for (const [key, value] of Object.entries(propsToSet)) {
				html.style.setProperty(key, value)
				if (body) {
					body.style.setProperty(key, value)
				}
			}
		},
		_handleSystemMediaChange() {
			if (this.selectedTheme === 'system') {
				this.setThemeClass()
			}
		},
		setFeatureFlag(key: FeatureFlag, value: boolean) {
			if (!this.featureFlags) {
				this.featureFlags = { ...DEFAULT_FEATURE_FLAGS }
			}
			this.featureFlags = {
				...this.featureFlags,
				[key]: value,
			}
		},
		setAdvancedRendering(value: boolean) {
			this.advancedRendering = value
		},
		setHideNametag(value: boolean) {
			this.hideNametagSkinsPage = value
		},
		getFeatureFlag(key: FeatureFlag): boolean {
			return this.featureFlags?.[key] ?? DEFAULT_FEATURE_FLAGS[key]
		},
		getThemeOptions() {
			return THEME_OPTIONS
		},
		getAccentOptions() {
			return ACCENT_OPTIONS
		},
	},
})
