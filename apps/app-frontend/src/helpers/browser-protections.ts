function isEditableElement(target: EventTarget | null): boolean {
	if (!target || !(target instanceof HTMLElement)) return false

	if (target instanceof HTMLInputElement) {
		const nonTextTypes = new Set([
			'button',
			'checkbox',
			'file',
			'hidden',
			'image',
			'radio',
			'range',
			'reset',
			'submit',
		])
		return !nonTextTypes.has(target.type)
	}

	if (target instanceof HTMLTextAreaElement) return true

	return target.isContentEditable || target.closest('[contenteditable="true"]') !== null
}

function isAllowedEditingShortcut(event: KeyboardEvent): boolean {
	const modKey = event.ctrlKey || event.metaKey
	if (!modKey) return false

	const key = event.key.toLowerCase()

	// Clipboard & Undo/Redo: C, V, X, A, Z, Y
	if (['c', 'v', 'x', 'a', 'z', 'y'].includes(key)) {
		return true
	}

	return false
}

function isAppCustomShortcut(event: KeyboardEvent): boolean {
	const key = event.key.toLowerCase()

	// In-Game Overlay toggle: F8
	if (event.key === 'F8' || event.key === 'f8') {
		return true
	}

	// In-Game Overlay toggle and backward form navigation: Shift+Tab
	if (event.shiftKey && key === 'tab') {
		return true
	}

	const modKey = event.ctrlKey || event.metaKey
	if (!modKey) return false

	// Ctrl+K: Command Palette, Ctrl+B: Toggle Sidebar
	if (['k', 'b'].includes(key)) {
		return true
	}

	return false
}

function isStandardNavigationKey(event: KeyboardEvent): boolean {
	const key = event.key

	// Allow standalone navigation and editing keys
	const standardKeys = new Set([
		'Tab',
		'Enter',
		'Escape',
		'ArrowUp',
		'ArrowDown',
		'ArrowLeft',
		'ArrowRight',
		'Home',
		'End',
		'PageUp',
		'PageDown',
		'Delete',
	])

	if (standardKeys.has(key)) {
		// Disallow Alt+ArrowLeft / Alt+ArrowRight (browser history traversal)
		if (event.altKey && (key === 'ArrowLeft' || key === 'ArrowRight')) {
			return false
		}
		return true
	}

	// Backspace is strictly allowed only when focus is inside an editable input/textarea
	if (key === 'Backspace') {
		return isEditableElement(event.target)
	}

	// Space key allowed when not combined with Cmd/Ctrl
	if (key === ' ' || key === 'Spacebar') {
		return !event.ctrlKey && !event.metaKey
	}

	return false
}

export function handleGlobalKeyDown(event: KeyboardEvent): void {
	// Allowed app custom shortcuts (Ctrl+B, Ctrl+K, F8, Shift+Tab)
	if (isAppCustomShortcut(event)) {
		return
	}

	// Function keys F1-F12 (except F8 which is handled above)
	if (/^F\d+$/i.test(event.key)) {
		event.preventDefault()
		event.stopPropagation()
		return
	}

	// Allowed text editing shortcuts: Ctrl+C, Ctrl+V, Ctrl+X, Ctrl+A, Ctrl+Z, Ctrl+Y
	if (isAllowedEditingShortcut(event)) {
		return
	}

	// Standard navigation keys (Arrows, Enter, Tab, Escape, Delete, Backspace in inputs)
	if (isStandardNavigationKey(event)) {
		return
	}

	// Block any remaining Ctrl/Meta or Alt modified browser shortcuts (e.g. Ctrl+R, Ctrl+P, Ctrl+S, Ctrl+U, Ctrl+H, Ctrl+J, Ctrl+N, Ctrl+T, Ctrl+W, Ctrl+O, Ctrl+D, Ctrl+G, Alt+Left, etc.)
	if (event.ctrlKey || event.metaKey || event.altKey) {
		// If typing characters inside an editable input without modifier or with simple AltGr, allow
		if (isEditableElement(event.target) && !event.ctrlKey && !event.metaKey) {
			return
		}

		event.preventDefault()
		event.stopPropagation()
	}
}

export function handleGlobalContextMenu(event: MouseEvent): void {
	// Prevent native Chromium/WebView2 context menu everywhere
	event.preventDefault()
}

export function handleGlobalWheelZoom(event: WheelEvent): void {
	// Prevent Ctrl+MouseWheel zooming
	if (event.ctrlKey || event.metaKey) {
		event.preventDefault()
	}
}

export function installBrowserProtections(): () => void {
	window.addEventListener('keydown', handleGlobalKeyDown, { capture: true })
	window.addEventListener('contextmenu', handleGlobalContextMenu, { capture: true })
	window.addEventListener('wheel', handleGlobalWheelZoom, { passive: false, capture: true })

	return () => {
		window.removeEventListener('keydown', handleGlobalKeyDown, { capture: true })
		window.removeEventListener('contextmenu', handleGlobalContextMenu, { capture: true })
		window.removeEventListener('wheel', handleGlobalWheelZoom, { capture: true })
	}
}
