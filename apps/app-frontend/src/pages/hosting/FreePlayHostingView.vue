<template>
	<div
		class="server-control-room min-h-full flex flex-col gap-6 p-4 sm:p-6 max-w-7xl mx-auto select-none text-zinc-100 font-sans"
	>
		<!-- Sticky Top Header & Navigation Hub (Stays pinned at the top on scroll) -->
		<header
			class="sticky top-0 z-40 flex flex-col gap-3 -mt-2 -mx-2 px-2 pt-2 pb-3 bg-[#090B0F]/95 backdrop-blur-2xl border-b border-white/5"
		>
			<!-- Top Server HUD Header Bar -->
			<div
				class="relative overflow-hidden rounded-2xl bg-[#141923]/95 border border-white/10 shadow-2xl backdrop-blur-2xl p-5 sm:p-6 lg:p-7 flex flex-col lg:flex-row items-start lg:items-center justify-between gap-6 transition-all duration-300"
			>
				<!-- Ambient Glow Backdrop -->
				<div
					class="absolute -right-20 -top-20 w-80 h-80 rounded-full blur-3xl pointer-events-none transition-all duration-700 opacity-20"
					:class="{
						'bg-sky-500': serverState.status === 'online',
						'bg-amber-500': serverState.status === 'starting',
						'bg-indigo-500': serverState.status === 'tunneling',
						'bg-rose-500/60': serverState.status === 'offline',
					}"
				/>

				<!-- Server Title & Live Status Pills -->
				<div class="flex flex-col gap-3.5 z-10 max-w-2xl">
					<div class="flex flex-wrap items-center gap-3">
						<div class="flex items-center gap-2.5">
							<div
								class="w-9 h-9 rounded-xl bg-sky-500/10 border border-sky-500/30 flex items-center justify-center text-sky-400 shadow-[0_0_15px_rgba(56,189,248,0.2)]"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-5 h-5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2.2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<rect width="20" height="8" x="2" y="2" rx="2" ry="2" />
									<rect width="20" height="8" x="2" y="14" rx="2" ry="2" />
									<line x1="6" x2="6.01" y1="6" y2="6" />
									<line x1="6" x2="6.01" y1="18" y2="18" />
								</svg>
							</div>
							<h1
								class="text-xl sm:text-2xl lg:text-3xl font-extrabold tracking-tight text-white m-0"
							>
								{{ activeServer?.name || 'Server Control Room' }}
							</h1>
						</div>

						<!-- Server Switcher / Quick Dropdown -->
						<div class="relative">
							<button
								type="button"
								class="inline-flex items-center gap-1.5 px-3 py-1 rounded-xl bg-zinc-800/80 hover:bg-zinc-700/80 border border-white/10 text-xs font-semibold text-zinc-300 transition-colors cursor-pointer"
								@click="showServerListModal = true"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5 text-cyan-400"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<rect width="7" height="7" x="3" y="3" rx="1" />
									<rect width="7" height="7" x="14" y="3" rx="1" />
									<rect width="7" height="7" x="14" y="14" rx="1" />
									<rect width="7" height="7" x="3" y="14" rx="1" />
								</svg>
								<span>Servers ({{ serverList.length }})</span>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3 h-3 text-zinc-400"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<polyline points="6 9 12 15 18 9" />
								</svg>
							</button>
						</div>

						<!-- Glowing Status Pill -->
						<div
							class="inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider transition-all duration-300 border backdrop-blur-md shadow-sm"
							:class="{
								'bg-sky-500/15 text-sky-300 border-sky-500/40 shadow-[0_0_16px_rgba(56,189,248,0.3)]':
									serverState.status === 'online',
								'bg-amber-500/15 text-amber-300 border-amber-500/40 animate-pulse shadow-[0_0_16px_rgba(245,158,11,0.25)]':
									serverState.status === 'starting',
								'bg-indigo-500/15 text-indigo-300 border-indigo-500/40 shadow-[0_0_16px_rgba(99,102,241,0.25)]':
									serverState.status === 'tunneling',
								'bg-zinc-800/80 text-zinc-400 border-zinc-700/80': serverState.status === 'offline',
							}"
						>
							<span class="relative flex h-2 w-2">
								<span
									v-if="serverState.status === 'online'"
									class="animate-ping absolute inline-flex h-full w-full rounded-full bg-sky-400 opacity-75"
								/>
								<span
									class="relative inline-flex rounded-full h-2 w-2"
									:class="{
										'bg-sky-400': serverState.status === 'online',
										'bg-amber-400 animate-spin': serverState.status === 'starting',
										'bg-indigo-400 animate-bounce': serverState.status === 'tunneling',
										'bg-zinc-500': serverState.status === 'offline',
									}"
								/>
							</span>
							<span>{{ serverState.status }}</span>
						</div>

						<!-- Engine & Version Badge -->
						<span
							class="text-xs font-mono font-semibold px-2.5 py-0.5 rounded-lg bg-zinc-800/90 text-zinc-300 border border-white/5"
						>
							{{ serverState.engine }} {{ serverState.version }}
						</span>
					</div>

					<!-- Connection Pill, Ping Counter, & Active Players Preview -->
					<div class="flex flex-wrap items-center gap-2.5 text-xs">
						<!-- 1-Click IP Address Copy Pill -->
						<div
							class="flex items-center gap-2 bg-[#090B0F]/90 border border-white/10 rounded-xl px-3 py-1.5 shadow-inner hover:border-sky-500/40 transition-colors duration-200"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-3.5 h-3.5 text-sky-400 shrink-0"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<circle cx="12" cy="12" r="10" />
								<path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
								<path d="M2 12h20" />
							</svg>
							<span class="font-semibold text-zinc-400 uppercase tracking-wide text-[10px]"
								>Host:</span
							>
							<code
								class="font-mono text-xs font-bold tracking-wide select-all"
								:class="serverState.claim_url ? 'text-amber-300' : 'text-white'"
							>
								{{
									serverState.tunnel_enabled &&
									serverState.public_ip &&
									serverState.public_ip !== 'Not Active'
										? serverState.public_ip
										: serverState.claim_url
											? 'Claim Required'
											: serverState.tunnel_enabled
												? isTunnelLoading
													? 'Connecting Public Tunnel...'
													: serverState.status === 'online'
														? 'Routing Anycast...'
														: serverState.status === 'offline'
															? 'Tunnel Auto-Routes On Start'
															: `127.0.0.1:${serverState.local_port}`
												: `127.0.0.1:${serverState.local_port}`
								}}
							</code>
							<a
								v-if="serverState.claim_url"
								:href="serverState.claim_url"
								target="_blank"
								rel="noreferrer"
								class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md bg-amber-400 hover:bg-amber-300 text-zinc-950 font-bold text-[10px] no-underline shadow-sm transition-all cursor-pointer"
								title="Click to claim agent on Playit.gg"
							>
								<span>Claim</span>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-2.5 h-2.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2.5"
								>
									<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
									<polyline points="15 3 21 3 21 9" />
									<line x1="10" y1="14" x2="21" y2="3" />
								</svg>
							</a>
							<button
								v-else
								type="button"
								class="inline-flex items-center justify-center p-1 rounded-md bg-zinc-800 hover:bg-sky-500 text-zinc-300 hover:text-zinc-950 transition-all duration-200 cursor-pointer border-none active:scale-95 focus-visible:ring-2 focus-visible:ring-sky-500 focus-visible:outline-none"
								title="1-Click Copy Public IP for Friends"
								@click="copyPublicIp"
							>
								<svg
									v-if="!copied"
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
									<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
								</svg>
								<svg
									v-else
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5 text-zinc-950"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2.5"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<polyline points="20 6 9 17 4 12" />
								</svg>
							</button>
						</div>

						<transition name="fade">
							<span
								v-if="copied"
								class="font-bold text-sky-400 bg-sky-500/10 border border-sky-500/30 px-2.5 py-1 rounded-lg flex items-center gap-1.5"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2.5"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<polyline points="20 6 9 17 4 12" />
								</svg>
								Copied to clipboard!
							</span>
						</transition>

						<!-- Port Counter -->
						<div
							class="flex items-center gap-1.5 bg-[#090B0F]/70 px-2.5 py-1.5 rounded-xl border border-white/5 text-zinc-400"
						>
							<span
								class="w-2 h-2 rounded-full"
								:class="
									serverState.status === 'online' ? 'bg-sky-400 animate-pulse' : 'bg-zinc-600'
								"
							/>
							<span
								>Local Port:
								<strong class="text-white font-mono">{{ serverState.local_port }}</strong></span
							>
						</div>

						<!-- Online Player Head Avatars Preview -->
						<div
							class="flex items-center gap-2 bg-[#090B0F]/70 px-2.5 py-1.5 rounded-xl border border-white/5"
						>
							<span class="text-zinc-400">Players:</span>
							<strong class="text-white font-mono">{{ onlinePlayers.length }}</strong>
						</div>
					</div>
				</div>

				<!-- Power Controls Bar -->
				<div class="flex flex-wrap items-center gap-2.5 z-10 w-full lg:w-auto shrink-0">
					<!-- Start Button -->
					<button
						v-if="serverState.status === 'offline'"
						type="button"
						class="flex-1 lg:flex-none inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl bg-gradient-to-r from-sky-400 to-blue-500 hover:from-sky-300 hover:to-blue-400 text-zinc-950 font-black text-xs uppercase tracking-wider shadow-[0_0_20px_rgba(56,189,248,0.35)] hover:shadow-[0_0_25px_rgba(56,189,248,0.5)] active:scale-[0.98] transition-all duration-200 cursor-pointer border-none focus-visible:ring-2 focus-visible:ring-sky-400 focus-visible:outline-none"
						:disabled="actionLoading"
						@click="startServer"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-4 h-4"
							viewBox="0 0 24 24"
							fill="currentColor"
						>
							<path d="M8 5v14l11-7z" />
						</svg>
						<span>Start Server</span>
					</button>

					<!-- Stop Button -->
					<button
						v-else
						type="button"
						class="flex-1 lg:flex-none inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl bg-rose-500/15 hover:bg-rose-500/25 text-rose-300 font-bold text-xs uppercase tracking-wider border border-rose-500/40 shadow-[0_0_15px_rgba(244,63,94,0.2)] active:scale-[0.98] transition-all duration-200 cursor-pointer focus-visible:ring-2 focus-visible:ring-rose-500 focus-visible:outline-none"
						:disabled="actionLoading"
						@click="stopServer"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-4 h-4"
							viewBox="0 0 24 24"
							fill="currentColor"
						>
							<rect x="6" y="6" width="12" height="12" rx="1.5" />
						</svg>
						<span>Stop Server</span>
					</button>

					<!-- Restart Button -->
					<button
						type="button"
						class="inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-amber-500/10 hover:bg-amber-500/20 text-amber-300 font-bold text-xs uppercase tracking-wider border border-amber-500/30 hover:border-amber-500/50 shadow-[0_0_15px_rgba(245,158,11,0.15)] active:scale-[0.98] transition-all duration-200 cursor-pointer focus-visible:ring-2 focus-visible:ring-amber-500 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none"
						:disabled="serverState.status === 'offline' || actionLoading"
						@click="restartServer"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
							<path d="M3 3v5h5" />
							<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
							<path d="M16 21h5v-5" />
						</svg>
						<span>Restart</span>
					</button>

					<!-- Force Kill Dropdown Option -->
					<button
						v-if="serverState.status !== 'offline'"
						type="button"
						class="inline-flex items-center justify-center gap-1.5 px-3 py-2.5 rounded-xl bg-zinc-800/80 hover:bg-rose-500/20 text-zinc-400 hover:text-rose-300 font-bold text-xs border border-white/5 hover:border-rose-500/30 active:scale-[0.98] transition-all duration-200 cursor-pointer focus-visible:ring-2 focus-visible:ring-rose-500 focus-visible:outline-none"
						title="Force Kill Server Process"
						@click="killServer"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5 text-rose-400"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="m18 6-12 12" />
							<path d="m6 6 12 12" />
						</svg>
						<span>Kill</span>
					</button>

					<!-- Open Folder in File Explorer Button -->
					<button
						type="button"
						class="inline-flex items-center justify-center gap-1.5 px-3.5 py-2.5 rounded-xl bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 hover:text-white font-bold text-xs border border-white/5 hover:border-white/10 active:scale-[0.98] transition-all duration-200 cursor-pointer focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:outline-none shadow-sm"
						title="Open Server Directory in OS File Manager"
						@click="openServerFolder"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5 text-indigo-400"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path
								d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
							/>
						</svg>
						<span>Open Folder</span>
					</button>

					<!-- Share & Invite Modal Trigger -->
					<button
						type="button"
						class="inline-flex items-center justify-center gap-1.5 px-3.5 py-2.5 rounded-xl bg-emerald-500/15 hover:bg-emerald-500/25 text-emerald-300 border border-emerald-500/40 font-bold text-xs active:scale-[0.98] transition-all duration-200 cursor-pointer focus-visible:ring-2 focus-visible:ring-emerald-400 focus-visible:outline-none shadow-sm"
						title="Share invite and LAN connection details"
						@click="showInviteShareModal = true"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5 text-emerald-400"
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
						<span>Share</span>
					</button>
				</div>
			</div>

			<!-- Playit Claim URL Alert Banner (When Playit requests claim) -->
			<div
				v-if="serverState.claim_url"
				class="p-4 rounded-2xl bg-amber-500/10 border border-amber-500/30 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 text-amber-200"
			>
				<div class="flex items-center gap-3">
					<div
						class="w-8 h-8 rounded-lg bg-amber-500/20 flex items-center justify-center text-amber-300 shrink-0"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-4 h-4"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<circle cx="12" cy="12" r="10" />
							<line x1="12" y1="8" x2="12" y2="12" />
							<line x1="12" y1="16" x2="12.01" y2="16" />
						</svg>
					</div>
					<div class="text-xs">
						<p class="font-bold text-white m-0">Playit.gg Tunnel Setup Required</p>
						<p class="text-zinc-300 m-0 mt-0.5">
							Click to link this launcher to your Playit account and get your custom domain.
						</p>
					</div>
				</div>
				<a
					:href="serverState.claim_url"
					target="_blank"
					rel="noreferrer"
					class="px-4 py-2 rounded-xl bg-amber-500 hover:bg-amber-400 text-zinc-950 font-bold text-xs no-underline inline-flex items-center gap-1.5 shrink-0"
				>
					<span>Claim Agent on Playit.gg</span>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-3.5 h-3.5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
						<polyline points="15 3 21 3 21 9" />
						<line x1="10" y1="14" x2="21" y2="3" />
					</svg>
				</a>
			</div>

			<!-- Tabbed Navigation Bar -->
			<div
				class="flex items-center overflow-x-auto gap-2 p-1.5 rounded-2xl bg-[#141923]/80 border border-white/10 backdrop-blur-md scrollbar-none"
			>
				<button
					v-for="tab in tabs"
					:key="tab.id"
					type="button"
					class="flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-bold transition-all duration-200 whitespace-nowrap cursor-pointer active:scale-[0.98] focus-visible:ring-2 focus-visible:ring-sky-500 focus-visible:outline-none"
					:class="
						activeTab === tab.id
							? 'bg-gradient-to-r from-sky-500/20 to-cyan-500/20 text-white border border-sky-500/40 shadow-[0_0_15px_rgba(56,189,248,0.2)]'
							: 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-800/50 border border-transparent'
					"
					@click="switchTab(tab.id)"
				>
					<component
						:is="tab.icon"
						class="w-4 h-4"
						:class="activeTab === tab.id ? 'text-sky-400' : 'text-zinc-400'"
					/>
					<span>{{ tab.label }}</span>
				</button>
			</div>
		</header>

		<!-- TAB 1: OVERVIEW -->
		<div v-if="activeTab === 'overview'" class="flex flex-col gap-6">
			<!-- Bento Telemetry Cards Grid -->
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
				<!-- Server Status Card -->
				<div
					class="p-5 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-lg backdrop-blur-md flex flex-col gap-3 hover:border-sky-500/30 transition-all duration-200"
				>
					<div class="flex items-center justify-between">
						<span
							class="text-xs font-bold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4 text-sky-400"
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
							Server Process
						</span>
						<span
							class="text-[10px] text-sky-300 font-mono font-bold bg-sky-500/15 px-2 py-0.5 rounded-md border border-sky-500/30"
							>{{ serverState.engine }}</span
						>
					</div>
					<div class="flex items-baseline justify-between">
						<span class="text-2xl font-black text-white font-mono tracking-tight capitalize">
							{{ serverState.status }}
						</span>
						<span
							class="text-xs font-semibold flex items-center gap-1"
							:class="serverState.status === 'online' ? 'text-emerald-400' : 'text-zinc-500'"
						>
							<span
								class="w-1.5 h-1.5 rounded-full"
								:class="serverState.status === 'online' ? 'bg-emerald-400' : 'bg-zinc-600'"
							></span>
							{{ serverState.status === 'online' ? 'Active' : 'Standby' }}
						</span>
					</div>
					<div class="text-[11px] text-zinc-400 truncate">
						Version: <strong class="text-zinc-200">{{ serverState.version }}</strong>
					</div>
				</div>

				<!-- RAM Allocation Card -->
				<div
					class="p-5 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-lg backdrop-blur-md flex flex-col gap-3 hover:border-sky-500/30 transition-all duration-200"
				>
					<div class="flex items-center justify-between">
						<span
							class="text-xs font-bold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4 text-sky-400"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path d="M6 19v-3" />
								<path d="M10 19v-3" />
								<path d="M14 19v-3" />
								<path d="M18 19v-3" />
								<path d="M8 11V9" />
								<path d="M16 11V9" />
								<rect width="18" height="12" x="3" y="4" rx="2" />
							</svg>
							RAM Allocation
						</span>
						<span class="text-[10px] text-zinc-400 font-mono">Dedicated</span>
					</div>
					<div class="flex items-baseline justify-between">
						<span class="text-3xl font-black text-white font-mono tracking-tight">
							{{ serverState.ram_gb }} GB
						</span>
						<span class="text-xs text-sky-400 font-bold font-mono">
							{{ serverState.ram_gb * 1024 }} MB
						</span>
					</div>
					<div
						class="w-full bg-zinc-950 rounded-full h-2 overflow-hidden p-0.5 border border-white/5"
					>
						<div
							class="bg-gradient-to-r from-sky-500 to-cyan-400 h-full rounded-full transition-all duration-500 ease-out"
							:style="{
								width: `${serverState.status === 'online' ? 75 : 0}%`,
							}"
						/>
					</div>
				</div>

				<!-- Port & Network Card -->
				<div
					class="p-5 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-lg backdrop-blur-md flex flex-col gap-3 hover:border-sky-500/30 transition-all duration-200"
				>
					<div class="flex items-center justify-between">
						<span
							class="text-xs font-bold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4 text-sky-400"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<circle cx="12" cy="12" r="10" />
								<path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
								<path d="M2 12h20" />
							</svg>
							Network Port
						</span>
						<span
							class="text-[10px] text-sky-300 font-mono font-bold bg-sky-500/15 px-2 py-0.5 rounded-md border border-sky-500/30"
							>TCP &amp; UDP</span
						>
					</div>
					<div class="flex items-baseline justify-between">
						<span class="text-3xl font-black text-white font-mono tracking-tight">
							{{ serverState.local_port }}
						</span>
						<span class="text-xs font-semibold text-sky-400">Standard</span>
					</div>
					<div class="text-[11px] text-zinc-400 truncate">
						Tunnel:
						<strong class="text-zinc-200">{{
							serverState.tunnel_enabled ? 'Enabled' : 'Disabled'
						}}</strong>
					</div>
				</div>

				<!-- Session Uptime Clock Card -->
				<div
					class="p-5 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-lg backdrop-blur-md flex flex-col gap-3 hover:border-cyan-500/30 transition-all duration-200"
				>
					<div class="flex items-center justify-between">
						<span
							class="text-xs font-bold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4 text-cyan-400"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<circle cx="12" cy="12" r="10" />
								<polyline points="12 6 12 12 16 14" />
							</svg>
							Session Uptime
						</span>
						<span
							class="text-[10px] text-cyan-300 font-mono font-bold bg-cyan-500/15 px-2 py-0.5 rounded-md border border-cyan-500/30"
							>Live Clock</span
						>
					</div>
					<div class="flex items-baseline justify-between">
						<span class="text-3xl font-black text-white font-mono tracking-tight">
							{{ serverState.status === 'online' ? formattedUptime : '00:00:00' }}
						</span>
					</div>
					<div class="text-xs text-zinc-400 truncate">
						Directory:
						<strong class="text-zinc-200 font-mono text-[10px]">{{
							activeServer?.path || 'Default'
						}}</strong>
					</div>
				</div>
			</div>

			<!-- FreePlay LAN & P2P Tunnel Hub Card -->
			<div
				class="p-6 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-5"
			>
				<div
					class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-white/10 pb-5"
				>
					<div class="flex items-center gap-3">
						<div
							class="w-10 h-10 rounded-xl bg-sky-500/10 border border-sky-500/30 flex items-center justify-center text-sky-400 shadow-[0_0_15px_rgba(56,189,248,0.2)]"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-5 h-5"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path d="M5 12.55a11 11 0 0 1 14.08 0" />
								<path d="M1.42 9a16 16 0 0 1 21.16 0" />
								<path d="M8.53 16.11a6 6 0 0 1 6.95 0" />
								<line x1="12" y1="20" x2="12.01" y2="20" />
							</svg>
						</div>
						<div>
							<div class="flex items-center gap-2">
								<h2 class="text-lg font-extrabold text-white m-0">
									FreePlay LAN &amp; Anycast P2P Tunnel
								</h2>
								<span
									class="text-[10px] font-extrabold px-2 py-0.5 rounded-full bg-sky-500/20 text-sky-300 border border-sky-500/30 uppercase tracking-wide"
									>playit.gg Protocol</span
								>
							</div>
							<p class="text-xs text-zinc-400 m-0 mt-0.5">
								Zero router port-forwarding required. Low-latency DDoS-protected multiplayer routing
								for friends anywhere.
							</p>
						</div>
					</div>

					<!-- Tunnel Switch -->
					<div class="flex items-center gap-3">
						<span
							class="text-xs font-semibold"
							:class="serverState.tunnel_enabled ? 'text-sky-300' : 'text-zinc-500'"
						>
							{{ serverState.tunnel_enabled ? 'Tunnel Active' : 'Tunnel Disabled' }}
						</span>
						<button
							type="button"
							class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus-visible:ring-2 focus-visible:ring-sky-400 focus-visible:outline-none p-0.5 items-center"
							:class="
								serverState.tunnel_enabled
									? 'bg-sky-500 shadow-[0_0_14px_rgba(56,189,248,0.5)]'
									: 'bg-zinc-800 border border-white/10'
							"
							@click="toggleTunnel"
						>
							<span
								class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow-md transition duration-200 ease-in-out"
								:class="serverState.tunnel_enabled ? 'translate-x-5' : 'translate-x-0'"
							/>
						</button>
					</div>
				</div>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<!-- Public Invite Link Copy Card -->
					<div class="p-4 rounded-xl bg-[#090B0F]/80 border border-white/5 flex flex-col gap-2">
						<div class="flex items-center justify-between">
							<span class="text-xs font-bold text-zinc-400 uppercase tracking-wider"
								>Shareable Public Address</span
							>
							<span
								v-if="
									serverState.tunnel_enabled &&
									serverState.public_ip &&
									serverState.public_ip !== 'Not Active'
								"
								class="px-2 py-0.5 rounded-full text-[10px] font-bold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 flex items-center gap-1"
							>
								<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
								Online & Routed
							</span>
							<a
								v-else-if="serverState.claim_url"
								:href="serverState.claim_url"
								target="_blank"
								rel="noreferrer"
								class="px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-amber-400 hover:bg-amber-300 text-zinc-950 flex items-center gap-1 shadow-sm no-underline cursor-pointer transition-all"
								title="Click to claim agent on Playit.gg"
							>
								<span class="w-1.5 h-1.5 rounded-full bg-amber-900 animate-ping" />
								<span>Claim on Playit</span>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-2.5 h-2.5 ml-0.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2.5"
								>
									<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
									<polyline points="15 3 21 3 21 9" />
									<line x1="10" y1="14" x2="21" y2="3" />
								</svg>
							</a>
							<span
								v-else-if="serverState.tunnel_enabled"
								class="px-2 py-0.5 rounded-full text-[10px] font-bold bg-sky-500/15 text-sky-300 border border-sky-500/30 flex items-center gap-1"
							>
								<span
									class="w-1.5 h-1.5 rounded-full bg-sky-400"
									:class="serverState.status === 'online' ? 'animate-pulse' : ''"
								/>
								{{
									isTunnelLoading
										? 'Connecting Anycast...'
										: serverState.status === 'offline'
											? 'Standby (Routes on Start)'
											: 'Routing Anycast...'
								}}
							</span>
							<button
								v-else
								type="button"
								class="text-[10px] font-bold text-sky-400 hover:text-sky-300 bg-sky-500/10 hover:bg-sky-500/20 px-2 py-0.5 rounded border border-sky-500/30 transition-all cursor-pointer"
								@click="toggleTunnel"
							>
								Click to Enable
							</button>
						</div>
						<!-- Multi-Tunnel Layout (when multiple ports / tunnels configured on Playit) -->
						<div
							v-if="serverState.tunnel_enabled && activeTunnels.length > 1"
							class="flex flex-col gap-1.5"
						>
							<div
								v-for="tun in activeTunnels"
								:key="tun.domain"
								class="flex items-center justify-between bg-zinc-900/90 px-3 py-2 rounded-lg border border-white/5 group hover:border-sky-500/30 transition-all cursor-pointer"
								:title="`Click to copy ${tun.tunnel_type}`"
								@click="copyTunnel(tun.domain)"
							>
								<div class="flex items-center gap-2 min-w-0">
									<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 shrink-0" />
									<span
										class="text-[10px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded bg-sky-500/10 text-sky-300 border border-sky-500/20 shrink-0"
									>
										{{ tun.tunnel_type }}
									</span>
									<code class="text-xs font-mono font-bold text-zinc-200 select-all truncate">
										{{ tun.domain }}
									</code>
								</div>
								<button
									type="button"
									class="p-1 px-2.5 rounded bg-zinc-800 hover:bg-sky-500 text-zinc-300 hover:text-zinc-950 transition-all duration-200 cursor-pointer border-none ml-2 shrink-0 flex items-center gap-1 text-[11px] font-bold"
									:class="{ '!bg-emerald-500 !text-zinc-950': copiedTunnelAddr === tun.domain }"
									@click.stop="copyTunnel(tun.domain)"
								>
									<span>{{ copiedTunnelAddr === tun.domain ? 'Copied!' : 'Copy' }}</span>
								</button>
							</div>
						</div>

						<!-- Single Tunnel Layout -->
						<div
							v-else
							class="flex items-center justify-between bg-zinc-900/90 px-3 py-2 rounded-lg border border-white/5 group hover:border-sky-500/30 transition-all cursor-pointer"
							:title="
								serverState.tunnel_enabled
									? 'Click to copy address'
									: 'Click to enable public tunnel'
							"
							@click="
								serverState.tunnel_enabled &&
								serverState.public_ip &&
								serverState.public_ip !== 'Not Active'
									? copyPublicIp()
									: toggleTunnel()
							"
						>
							<code
								class="text-xs font-mono font-bold select-all truncate"
								:class="
									serverState.tunnel_enabled &&
									serverState.public_ip &&
									serverState.public_ip !== 'Not Active'
										? 'text-sky-300'
										: serverState.claim_url
											? 'text-amber-300'
											: 'text-sky-400'
								"
							>
								{{
									serverState.tunnel_enabled &&
									serverState.public_ip &&
									serverState.public_ip !== 'Not Active'
										? serverState.public_ip
										: serverState.claim_url
											? 'Setup Required: Click Claim on Playit'
											: serverState.tunnel_enabled
												? isTunnelLoading
													? 'Connecting Public Anycast...'
													: serverState.status === 'offline'
														? 'Tunnel Enabled (Auto-Routes on Server Start)'
														: 'Connecting Public Anycast...'
												: 'Enable Tunnel for Public IP'
								}}
							</code>
							<button
								type="button"
								class="p-1 px-2 rounded bg-zinc-800 hover:bg-sky-500 text-zinc-300 hover:text-zinc-950 transition-all duration-200 cursor-pointer border-none ml-2 shrink-0 flex items-center gap-1 text-xs font-bold"
								:class="{ '!bg-sky-500 !text-zinc-950': copied }"
								title="Copy Invite Address"
								@click.stop="copyPublicIp"
							>
								<svg
									v-if="!copied"
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
									<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
								</svg>
								<svg
									v-else
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2.5"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<polyline points="20 6 9 17 4 12" />
								</svg>
								<span>{{ copied ? 'Copied!' : 'Copy' }}</span>
							</button>
						</div>
						<!-- Claim URL Banner if present -->
						<div
							v-if="serverState.claim_url"
							class="flex items-center justify-between p-2 rounded-lg bg-sky-500/10 border border-sky-500/30 text-xs text-sky-300"
						>
							<span class="truncate">Claim tunnel for custom address:</span>
							<a
								:href="serverState.claim_url"
								target="_blank"
								class="px-2 py-0.5 rounded bg-sky-600 hover:bg-sky-500 text-white font-bold text-[11px] shrink-0 no-underline"
							>
								Claim Setup
							</a>
						</div>
					</div>

					<!-- Direct LAN Address -->
					<div class="p-4 rounded-xl bg-[#090B0F]/80 border border-white/5 flex flex-col gap-2">
						<span class="text-xs font-bold text-zinc-400 uppercase tracking-wider"
							>Local Direct Join</span
						>
						<div
							class="flex items-center justify-between bg-zinc-900/90 px-3 py-2 rounded-lg border border-white/5"
						>
							<code class="text-xs font-mono font-bold text-sky-400 select-all">
								127.0.0.1:{{ serverState.local_port }}
							</code>
							<span class="text-[10px] font-bold text-zinc-500 uppercase">Localhost</span>
						</div>
					</div>
				</div>

				<!-- Tunnel Debug Panel (Collapsible) -->
				<div class="border-t border-white/10 pt-4">
					<button
						type="button"
						class="flex items-center gap-2 text-[11px] font-bold uppercase tracking-wider cursor-pointer bg-transparent border-none transition-colors duration-200 p-0"
						:class="showTunnelDebug ? 'text-amber-300' : 'text-zinc-500 hover:text-zinc-300'"
						@click="showTunnelDebug = !showTunnelDebug"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5 transition-transform duration-200"
							:class="showTunnelDebug ? 'rotate-90 text-amber-400' : 'text-zinc-500'"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<polyline points="9 18 15 12 9 6" />
						</svg>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="M12 20h9" />
							<path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" />
						</svg>
						Tunnel Debug
					</button>

					<transition name="fade">
						<div
							v-if="showTunnelDebug"
							class="mt-3 p-4 rounded-xl bg-[#090B0F] border border-amber-500/20 flex flex-col gap-3 text-xs font-mono"
						>
							<!-- Status Grid -->
							<div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
								<div class="flex flex-col gap-1">
									<span
										class="text-zinc-500 text-[10px] font-sans font-bold uppercase tracking-wider"
										>Tunnel Process Status</span
									>
									<span
										class="font-bold"
										:class="{
											'text-emerald-400': tunnelDebug.rawStatus === 'connected',
											'text-amber-300':
												tunnelDebug.rawStatus === 'starting' ||
												tunnelDebug.rawStatus === 'downloading',
											'text-sky-300': tunnelDebug.rawStatus === 'claiming',
											'text-rose-400': tunnelDebug.rawStatus.startsWith('error'),
											'text-zinc-400':
												tunnelDebug.rawStatus === 'stopped' || tunnelDebug.rawStatus === 'unknown',
										}"
									>
										{{ tunnelDebug.rawStatus }}
									</span>
								</div>

								<div class="flex flex-col gap-1">
									<span
										class="text-zinc-500 text-[10px] font-sans font-bold uppercase tracking-wider"
										>Public Address (Raw)</span
									>
									<span
										:class="
											tunnelDebug.rawPublicAddress ? 'text-emerald-400' : 'text-zinc-500 italic'
										"
									>
										{{ tunnelDebug.rawPublicAddress || 'null (not resolved)' }}
									</span>
								</div>

								<div class="flex flex-col gap-1">
									<span
										class="text-zinc-500 text-[10px] font-sans font-bold uppercase tracking-wider"
										>Claim URL</span
									>
									<span
										:class="
											tunnelDebug.rawClaimUrl ? 'text-amber-300 break-all' : 'text-zinc-500 italic'
										"
									>
										{{ tunnelDebug.rawClaimUrl || 'none' }}
									</span>
								</div>

								<div class="flex flex-col gap-1">
									<span
										class="text-zinc-500 text-[10px] font-sans font-bold uppercase tracking-wider"
										>Last Poll</span
									>
									<span class="text-zinc-300">
										{{ tunnelDebug.lastPollTime || 'never' }}
									</span>
								</div>

								<!-- All Active Tunnels Table -->
								<div
									v-if="activeTunnels.length > 0"
									class="col-span-1 md:col-span-2 flex flex-col gap-1.5 p-2.5 rounded-lg bg-black/40 border border-white/5"
								>
									<span
										class="text-zinc-400 text-[10px] font-sans font-bold uppercase tracking-wider flex items-center justify-between"
									>
										<span>Configured Anycast Tunnels ({{ activeTunnels.length }})</span>
										<span class="text-emerald-400 font-mono text-[10px]">● Playit Edge Active</span>
									</span>
									<div class="flex flex-col gap-1.5 mt-1">
										<div
											v-for="tun in activeTunnels"
											:key="tun.domain"
											class="flex items-center justify-between px-2.5 py-1.5 rounded bg-zinc-900/90 border border-white/5 text-xs font-mono"
										>
											<div class="flex items-center gap-2">
												<span class="text-emerald-400">●</span>
												<span class="text-sky-300 font-bold">{{ tun.domain }}</span>
												<span class="text-zinc-500">=&gt;</span>
												<span class="text-zinc-400">{{ tun.target }}</span>
											</div>
											<span
												class="text-[10px] px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-300 font-sans font-semibold"
											>
												{{ tun.tunnel_type }}
											</span>
										</div>
									</div>
								</div>
							</div>

							<!-- Error Display -->
							<div
								v-if="tunnelDebug.lastError"
								class="p-2 rounded-lg bg-rose-500/10 border border-rose-500/30 text-rose-300"
							>
								<span class="text-[10px] font-sans font-bold uppercase tracking-wider text-rose-400"
									>Last Error</span
								>
								<p class="m-0 mt-1">{{ tunnelDebug.lastError }}</p>
							</div>

							<!-- Frontend State -->
							<div
								class="p-2 rounded-lg bg-zinc-900/80 border border-white/5 flex flex-wrap gap-x-4 gap-y-1 text-[11px]"
							>
								<span class="text-zinc-500"
									>tunnel_enabled:
									<strong
										:class="serverState.tunnel_enabled ? 'text-emerald-400' : 'text-zinc-400'"
										>{{ serverState.tunnel_enabled }}</strong
									></span
								>
								<span class="text-zinc-500"
									>isTunnelLoading:
									<strong :class="isTunnelLoading ? 'text-amber-300' : 'text-zinc-400'">{{
										isTunnelLoading
									}}</strong></span
								>
								<span class="text-zinc-500"
									>public_ip:
									<strong class="text-sky-300">{{ serverState.public_ip }}</strong></span
								>
							</div>

							<!-- Tunnel Logs -->
							<div class="flex flex-col gap-1.5">
								<div class="flex items-center justify-between">
									<span
										class="text-zinc-500 text-[10px] font-sans font-bold uppercase tracking-wider"
										>Playit Agent Logs (last {{ tunnelDebug.tunnelLogs.length }})</span
									>
									<button
										type="button"
										class="text-[10px] font-bold text-sky-400 hover:text-sky-300 bg-sky-500/10 px-2 py-0.5 rounded border border-sky-500/30 cursor-pointer transition-all"
										@click="fetchStatus()"
									>
										Refresh Now
									</button>
								</div>
								<div
									class="p-2.5 rounded-lg bg-zinc-950 border border-white/5 max-h-40 overflow-y-auto space-y-0.5 scrollbar-thin"
								>
									<div
										v-for="(log, idx) in tunnelDebug.tunnelLogs"
										:key="idx"
										class="truncate text-[11px]"
										:class="{
											'text-rose-400 font-bold':
												log.includes('[ERROR]') || log.includes('[STDERR]'),
											'text-amber-300': log.includes('[WARN]') || log.includes('claim'),
											'text-emerald-400 font-semibold':
												log.includes('connected') || log.includes('tunnel ready'),
											'text-sky-400 font-medium': log.includes('playit') || log.includes('[INFO]'),
											'text-sky-400/90':
												!log.includes('[ERROR]') &&
												!log.includes('[WARN]') &&
												!log.includes('[INFO]') &&
												!log.includes('connected') &&
												!log.includes('tunnel ready'),
										}"
									>
										{{ log }}
									</div>
									<div v-if="tunnelDebug.tunnelLogs.length === 0" class="text-zinc-600 italic py-1">
										No tunnel logs yet. Start the tunnel to see output.
									</div>
								</div>
							</div>
						</div>
					</transition>
				</div>
			</div>

			<!-- Real-Time Telemetry & Performance Gauges Hub -->
			<div class="flex flex-col gap-3">
				<ServerTelemetryHub
					:server-status="serverState.status"
					:dedicated-ram-gb="serverState.ram_gb"
				/>
			</div>

			<!-- Live Log Stream Preview -->
			<div
				class="p-6 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-3"
			>
				<div class="flex items-center justify-between border-b border-white/10 pb-3">
					<h3 class="text-sm font-extrabold text-white m-0 flex items-center gap-2">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-4 h-4 text-sky-400"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<polyline points="4 17 10 11 4 5" />
							<line x1="12" y1="19" x2="20" y2="19" />
						</svg>
						Live Server Logs
					</h3>
					<button
						type="button"
						class="text-xs font-bold text-cyan-400 hover:text-cyan-300 flex items-center gap-1 cursor-pointer bg-transparent border-none"
						@click="activeTab = 'console'"
					>
						Open Full Interactive Console &rarr;
					</button>
				</div>

				<div
					class="p-3.5 rounded-xl bg-[#090B0F] border border-white/5 font-mono text-xs space-y-1 overflow-y-auto max-h-48 scrollbar-thin"
				>
					<div
						v-for="(log, idx) in serverState.logs.slice(-8)"
						:key="idx"
						class="truncate"
						:class="{
							'text-rose-400 font-bold': log.includes('[ERROR]') || log.includes('[STDERR]'),
							'text-amber-400': log.includes('[WARN]'),
							'text-emerald-400 font-semibold': log.includes('[DONE]') || log.includes('Done ('),
							'text-sky-400 font-medium': log.includes('[INFO]') || log.includes('playit.gg'),
							'text-sky-400/90':
								!log.includes('[INFO]') &&
								!log.includes('[ERROR]') &&
								!log.includes('[WARN]') &&
								!log.includes('[DONE]'),
						}"
					>
						{{ log }}
					</div>
					<div v-if="serverState.logs.length === 0" class="text-zinc-500 italic py-2">
						No log output yet. Start the server to view output stream.
					</div>
				</div>
			</div>
		</div>

		<!-- TAB 2: INTERACTIVE LIVE CONSOLE -->
		<div v-if="activeTab === 'console'" class="flex flex-col gap-4">
			<div
				class="rounded-2xl bg-[#141923]/90 border border-white/10 overflow-hidden shadow-2xl backdrop-blur-md flex flex-col h-[740px]"
			>
				<!-- Terminal Top Header Bar -->
				<div
					class="bg-[#090B0F] px-4 py-3.5 border-b border-white/10 flex flex-wrap items-center justify-between gap-3 shrink-0"
				>
					<div class="flex items-center gap-3">
						<div class="flex items-center gap-1.5">
							<span
								class="w-3 h-3 rounded-full bg-rose-500 inline-block shadow-[0_0_8px_rgba(244,63,94,0.4)]"
							></span>
							<span
								class="w-3 h-3 rounded-full bg-amber-500 inline-block shadow-[0_0_8px_rgba(245,158,11,0.4)]"
							></span>
							<span
								class="w-3 h-3 rounded-full bg-sky-500 inline-block shadow-[0_0_8px_rgba(56,189,248,0.4)]"
							></span>
						</div>
						<span
							class="text-xs font-black text-white font-mono flex items-center gap-2 tracking-wide uppercase"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4 text-sky-400"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2.2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<polyline points="4 17 10 11 4 5" />
								<line x1="12" y1="19" x2="20" y2="19" />
							</svg>
							Interactive Live Terminal ({{ serverState.logs.length }} lines)
						</span>
					</div>

					<!-- Filter Chips & Action Controls -->
					<div class="flex items-center gap-2">
						<div
							class="hidden sm:flex items-center bg-zinc-900 rounded-lg p-0.5 border border-white/5 text-xs font-mono"
						>
							<button
								v-for="flt in ['all', 'info', 'warn', 'error']"
								:key="flt"
								type="button"
								class="px-2.5 py-1 rounded-md text-[11px] font-bold uppercase transition-colors cursor-pointer border-none"
								:class="
									logFilter === flt
										? 'bg-sky-500/20 text-sky-300 border border-sky-500/40'
										: 'text-zinc-400 hover:text-zinc-200'
								"
								@click="logFilter = flt"
							>
								{{ flt }}
							</button>
						</div>

						<button
							type="button"
							class="text-xs px-3 py-1.5 rounded-lg border border-white/10 bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 hover:text-white font-mono cursor-pointer transition-colors active:scale-95 focus-visible:ring-2 focus-visible:ring-sky-500 focus-visible:outline-none"
							:class="{
								'!bg-sky-500/20 !border-sky-500/50 !text-sky-300 font-bold': autoScroll,
							}"
							@click="autoScroll = !autoScroll"
						>
							Auto-Scroll: {{ autoScroll ? 'ON' : 'OFF' }}
						</button>

						<button
							type="button"
							class="text-xs px-3 py-1.5 rounded-lg border border-white/10 bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 hover:text-white font-mono cursor-pointer transition-colors active:scale-95 flex items-center gap-1.5"
							title="Export Server Logs as .log file"
							@click="downloadLogs"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-3.5 h-3.5 text-cyan-400"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
								<polyline points="7 10 12 15 17 10" />
								<line x1="12" y1="15" x2="12" y2="3" />
							</svg>
							<span>Export</span>
						</button>

						<button
							type="button"
							class="text-xs px-2.5 py-1.5 rounded-lg border border-white/10 bg-zinc-800/80 hover:bg-rose-500/20 text-zinc-400 hover:text-rose-300 font-mono cursor-pointer transition-colors active:scale-95"
							title="Clear Logs Buffer"
							@click="clearLogs"
						>
							Clear
						</button>
					</div>
				</div>

				<!-- Terminal Log View Area -->
				<div
					ref="terminalLogContainer"
					class="flex-1 p-4 bg-[#070a10] overflow-y-auto font-mono text-xs leading-relaxed space-y-1 select-text scrollbar-thin scrollbar-thumb-zinc-800"
				>
					<div
						v-for="(log, idx) in filteredLogs"
						:key="idx"
						class="whitespace-pre-wrap break-all leading-relaxed"
					>
						<span
							v-if="log.includes('[INFO]') || log.includes('INFO')"
							class="text-sky-400 font-medium"
						>
							{{ log }}
						</span>
						<span
							v-else-if="log.includes('[WARN]') || log.includes('WARN')"
							class="text-amber-400 font-medium"
						>
							{{ log }}
						</span>
						<span
							v-else-if="
								log.includes('[ERROR]') || log.includes('ERROR') || log.includes('[STDERR]')
							"
							class="text-rose-400 font-bold bg-rose-500/10 px-1 py-0.5 rounded"
						>
							{{ log }}
						</span>
						<span
							v-else-if="log.includes('[DONE]') || log.includes('Done (')"
							class="text-emerald-400 font-bold bg-emerald-500/10 px-1 py-0.5 rounded"
						>
							{{ log }}
						</span>
						<span v-else-if="log.includes('[Console]')" class="text-sky-300 font-semibold">
							{{ log }}
						</span>
						<span v-else class="text-sky-400 font-medium">
							{{ log }}
						</span>
					</div>
					<div v-if="filteredLogs.length === 0" class="text-zinc-500 italic py-8 text-center">
						No console logs to display. Start the server above to initiate execution.
					</div>
				</div>

				<!-- Quick Command Suggestion Bar -->
				<div
					class="px-4 py-2 bg-[#090B0F] border-t border-white/10 flex flex-wrap items-center gap-2 shrink-0"
				>
					<span
						class="text-xs font-bold text-zinc-400 uppercase tracking-wide mr-1 flex items-center gap-1"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5 text-amber-400"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
						</svg>
						Quick:
					</span>
					<button
						v-for="cmd in quickCommands"
						:key="cmd"
						type="button"
						class="text-xs font-mono px-2.5 py-1 rounded-lg bg-zinc-900 hover:bg-sky-500 hover:text-zinc-950 text-zinc-300 font-semibold transition-all duration-150 cursor-pointer border border-white/5 active:scale-95 focus-visible:ring-2 focus-visible:ring-sky-500 focus-visible:outline-none"
						@click="sendQuickCommand(cmd)"
					>
						{{ cmd }}
					</button>
				</div>

				<!-- Command Input Bar -->
				<form
					class="p-3.5 bg-[#090B0F] border-t border-white/10 flex items-center gap-3 shrink-0"
					@submit.prevent="submitCommand"
				>
					<span class="text-sky-400 font-mono font-black text-base pl-2 select-none">&gt;</span>
					<input
						v-model="commandInput"
						type="text"
						placeholder="Type a server command... (e.g. help, list, save-all, whitelist add Player)"
						class="flex-1 bg-zinc-900 border border-white/10 focus:border-sky-500/80 rounded-xl px-4 py-2.5 text-sm text-white font-mono outline-none shadow-inner transition-colors duration-200 focus-visible:ring-2 focus-visible:ring-sky-500/40"
						@keydown.up="navigateHistory(-1)"
						@keydown.down="navigateHistory(1)"
					/>
					<button
						type="submit"
						class="px-5 py-2.5 rounded-xl bg-gradient-to-r from-sky-400 to-blue-500 hover:from-sky-300 hover:to-blue-400 text-zinc-950 font-extrabold text-xs shadow-md shadow-sky-950/50 active:scale-95 transition-all duration-200 cursor-pointer border-none flex items-center gap-2 focus-visible:ring-2 focus-visible:ring-sky-400 focus-visible:outline-none"
					>
						<span>Send</span>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.5"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<line x1="22" y1="2" x2="11" y2="13" />
							<polygon points="22 2 15 22 11 13 2 9 22 2" />
						</svg>
					</button>
				</form>
			</div>
		</div>

		<!-- TAB: PLAYERS & MODERATION -->
		<div v-if="activeTab === 'players'" class="flex flex-col gap-4">
			<PlayerManagerTab
				:server-status="serverState.status"
				:can-moderate="true"
				@command="executeCommand"
			/>
		</div>

		<!-- TAB: VISUAL SERVER PROPERTIES -->
		<div v-if="activeTab === 'properties'" class="flex flex-col gap-4">
			<ServerPropertiesEditor :server-status="serverState.status" />
		</div>

		<!-- TAB: ADDONS (PLUGINS & MODS) -->
		<div v-if="activeTab === 'addons'" class="flex flex-col gap-6">
			<!-- Header Banner with Engine, Version, Counts, and Restart Alert -->
			<div
				class="p-6 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-4"
			>
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-white/10 pb-4">
					<div class="flex items-center gap-3">
						<div
							class="w-10 h-10 rounded-xl bg-purple-500/10 border border-purple-500/30 flex items-center justify-center text-purple-400 shadow-[0_0_15px_rgba(168,85,247,0.2)]"
						>
							<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<rect width="8" height="8" x="2" y="2" rx="2" />
								<path d="M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
								<path d="M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
								<rect width="8" height="8" x="2" y="14" rx="2" />
								<rect width="8" height="8" x="14" y="14" rx="2" />
							</svg>
						</div>
						<div>
							<div class="flex items-center gap-2">
								<h2 class="text-base font-extrabold text-white m-0">Plugins & Server Mods</h2>
								<span class="text-xs font-mono px-2 py-0.5 rounded-md bg-purple-500/20 text-purple-300 border border-purple-500/30">
									{{ serverState.engine }} {{ serverState.version }}
								</span>
							</div>
							<p class="text-xs text-zinc-400 m-0">
								Manage and install plugins, mods, and datapacks with atomic downloads and compatibility validation.
							</p>
						</div>
					</div>

					<div class="flex items-center flex-wrap gap-2">
						<button
							type="button"
							class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-xl bg-purple-600 hover:bg-purple-500 text-xs font-bold text-white shadow-md shadow-purple-950/40 cursor-pointer transition-all active:scale-95"
							@click="showAddonCatalog = true; searchModrinthAddons()"
						>
							<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2">
								<circle cx="11" cy="11" r="8" />
								<line x1="21" y1="21" x2="16.65" y2="16.65" />
							</svg>
							<span>Browse Modrinth Addons</span>
						</button>

						<button
							type="button"
							class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-xl bg-zinc-800 hover:bg-zinc-700 text-xs font-bold text-zinc-300 hover:text-white border border-white/10 cursor-pointer transition-colors"
							@click="importLocalAddon"
						>
							<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-cyan-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
								<polyline points="17 8 12 3 7 8" />
								<line x1="12" y1="3" x2="12" y2="15" />
							</svg>
							<span>Import .jar / .zip</span>
						</button>

						<button
							type="button"
							class="inline-flex items-center gap-2 px-3 py-1.5 rounded-xl bg-zinc-800 hover:bg-zinc-700 text-xs font-bold text-zinc-300 hover:text-white border border-white/10 cursor-pointer transition-colors"
							@click="fetchServerAddons"
						>
							<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
								<path d="M3 3v5h5" />
								<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
								<path d="M16 21h5v-5" />
							</svg>
							<span>Refresh</span>
						</button>
					</div>
				</div>

				<!-- Restart Required Banner -->
				<div
					v-if="restartRequired && serverState.status === 'online'"
					class="flex items-center justify-between gap-4 p-3.5 rounded-xl bg-amber-500/15 border border-amber-500/30 text-amber-200 text-xs"
				>
					<div class="flex items-center gap-2.5">
						<span class="relative flex h-2.5 w-2.5">
							<span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
							<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-amber-500"></span>
						</span>
						<span class="font-semibold">Addon modifications made while server is live. Restart server to apply all changes.</span>
					</div>
					<button
						type="button"
						class="px-3 py-1 rounded-lg bg-amber-500 hover:bg-amber-400 text-black font-bold text-xs cursor-pointer transition-colors"
						@click="restartServer"
					>
						Restart Server Now
					</button>
				</div>

				<!-- Filter & Search Controls -->
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
					<div class="flex items-center gap-1.5 flex-wrap">
						<button
							v-for="filter in [
								{ id: 'all', label: `All (${serverAddons.length})` },
								{ id: 'plugin', label: `Plugins (${serverAddons.filter(a => a.addon_type === 'plugin').length})` },
								{ id: 'mod', label: `Mods (${serverAddons.filter(a => a.addon_type === 'mod').length})` },
								{ id: 'datapack', label: `Datapacks (${serverAddons.filter(a => a.addon_type === 'datapack').length})` },
								{ id: 'local', label: `Local (${serverAddons.filter(a => a.source === 'local').length})` },
							]"
							:key="filter.id"
							type="button"
							class="px-3 py-1 rounded-lg text-xs font-semibold cursor-pointer transition-colors border"
							:class="addonTypeFilter === filter.id ? 'bg-purple-500/20 text-purple-300 border-purple-500/40' : 'bg-zinc-800/60 text-zinc-400 border-white/5 hover:bg-zinc-800 hover:text-zinc-200'"
							@click="addonTypeFilter = filter.id"
						>
							{{ filter.label }}
						</button>
					</div>

					<div class="w-full sm:w-64">
						<input
							v-model="addonSearchQuery"
							type="text"
							placeholder="Filter installed addons..."
							class="w-full px-3 py-1.5 rounded-xl bg-zinc-900/90 border border-white/10 text-xs text-white placeholder-zinc-500 focus:outline-none focus:border-purple-500/50 font-mono"
						/>
					</div>
				</div>
			</div>

			<!-- Installed Addons Table -->
			<div class="p-6 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-4">
				<div v-if="filteredAddons.length === 0" class="flex flex-col items-center justify-center py-16 text-center gap-3">
					<div class="w-12 h-12 rounded-2xl bg-zinc-800/80 border border-white/10 flex items-center justify-center text-zinc-500">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
							<rect width="8" height="8" x="2" y="2" rx="2" />
							<path d="M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
							<path d="M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
							<rect width="8" height="8" x="2" y="14" rx="2" />
							<rect width="8" height="8" x="14" y="14" rx="2" />
						</svg>
					</div>
					<div class="flex flex-col gap-1 max-w-sm">
						<h3 class="text-sm font-bold text-white m-0">No Addons Installed</h3>
						<p class="text-xs text-zinc-400 m-0">
							Click "Browse Modrinth Addons" to search and 1-click install plugins or server mods, or drop existing .jar files here.
						</p>
					</div>
					<button
						type="button"
						class="mt-2 px-4 py-2 rounded-xl bg-purple-600 hover:bg-purple-500 text-xs font-bold text-white shadow-md cursor-pointer transition-all active:scale-95"
						@click="showAddonCatalog = true; searchModrinthAddons()"
					>
						Browse Addons Catalog
					</button>
				</div>

				<div v-else class="overflow-x-auto">
					<table class="w-full text-left text-xs font-mono">
						<thead>
							<tr class="border-b border-white/10 text-zinc-400 uppercase text-[10px]">
								<th class="py-2.5 px-3">Addon / File</th>
								<th class="py-2.5 px-3">Type</th>
								<th class="py-2.5 px-3">Version</th>
								<th class="py-2.5 px-3">Source</th>
								<th class="py-2.5 px-3">Size</th>
								<th class="py-2.5 px-3">State</th>
								<th class="py-2.5 px-3 text-right">Actions</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-white/5">
							<tr
								v-for="addon in filteredAddons"
								:key="addon.id"
								class="hover:bg-white/5 transition-colors"
								:class="!addon.enabled ? 'opacity-60 bg-zinc-950/20' : ''"
							>
								<td class="py-3 px-3">
									<div class="flex items-center gap-2.5">
										<div class="w-7 h-7 rounded-lg bg-zinc-800/80 border border-white/10 flex items-center justify-center text-purple-400 font-bold shrink-0">
											{{ addon.name.charAt(0).toUpperCase() }}
										</div>
										<div class="flex flex-col">
											<span class="text-white font-bold">{{ addon.name }}</span>
											<span class="text-[11px] text-zinc-400">{{ addon.filename }}</span>
										</div>
									</div>
								</td>
								<td class="py-3 px-3">
									<span
										class="px-2 py-0.5 rounded text-[10px] font-bold uppercase"
										:class="addon.addon_type === 'plugin' ? 'bg-sky-500/20 text-sky-300 border border-sky-500/30' : addon.addon_type === 'mod' ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30' : 'bg-amber-500/20 text-amber-300 border border-amber-500/30'"
									>
										{{ addon.addon_type }}
									</span>
								</td>
								<td class="py-3 px-3 text-zinc-300">
									{{ addon.version_number || addon.game_version || 'Latest' }}
								</td>
								<td class="py-3 px-3">
									<span
										class="px-2 py-0.5 rounded text-[10px] font-semibold"
										:class="addon.source === 'modrinth' ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' : 'bg-zinc-800 text-zinc-400 border border-white/5'"
									>
										{{ addon.source === 'modrinth' ? 'Modrinth' : 'Local File' }}
									</span>
								</td>
								<td class="py-3 px-3 text-zinc-400">
									{{ formatFileSize(addon.file_size) }}
								</td>
								<td class="py-3 px-3">
									<span
										class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-bold"
										:class="addon.enabled ? 'bg-emerald-500/15 text-emerald-300' : 'bg-rose-500/15 text-rose-300'"
									>
										<span class="w-1.5 h-1.5 rounded-full" :class="addon.enabled ? 'bg-emerald-400' : 'bg-rose-400'"></span>
										{{ addon.enabled ? 'Active' : 'Disabled' }}
									</span>
								</td>
								<td class="py-3 px-3 text-right">
									<div class="flex items-center justify-end gap-1.5">
										<button
											type="button"
											class="px-2 py-1 rounded-lg text-xs font-semibold cursor-pointer transition-colors border"
											:class="addon.enabled ? 'bg-zinc-800 text-zinc-300 hover:bg-zinc-700 border-white/10' : 'bg-emerald-500/20 text-emerald-300 hover:bg-emerald-500/30 border-emerald-500/40'"
											:title="addon.enabled ? 'Disable addon' : 'Enable addon'"
											@click="toggleAddon(addon)"
										>
											{{ addon.enabled ? 'Disable' : 'Enable' }}
										</button>
										<button
											type="button"
											class="p-1.5 rounded-lg bg-rose-500/10 hover:bg-rose-500/25 text-rose-400 hover:text-rose-300 border border-rose-500/30 cursor-pointer transition-colors"
											title="Delete addon"
											@click="deleteAddon(addon)"
										>
											<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
												<polyline points="3 6 5 6 21 6" />
												<path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
											</svg>
										</button>
									</div>
								</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>

		<!-- TAB 3: SERVERS & INSTANCES -->
		<div v-if="activeTab === 'servers'" class="flex flex-col gap-5">
			<div
				class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 p-6 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md"
			>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 rounded-xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-[0_0_15px_rgba(6,182,212,0.2)]"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-5 h-5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<rect width="20" height="8" x="2" y="2" rx="2" ry="2" />
							<rect width="20" height="8" x="2" y="14" rx="2" ry="2" />
							<line x1="6" x2="6.01" y1="6" y2="6" />
							<line x1="6" x2="6.01" y1="18" y2="18" />
						</svg>
					</div>
					<div>
						<h2 class="text-base font-extrabold text-white m-0">Server Instances</h2>
						<p class="text-xs text-zinc-400 m-0">
							Create, configure, and switch between dedicated Minecraft servers with custom file
							locations.
						</p>
					</div>
				</div>

				<button
					type="button"
					class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-gradient-to-r from-cyan-500 to-sky-400 hover:from-cyan-400 hover:to-sky-300 text-zinc-950 font-extrabold text-xs shadow-[0_0_15px_rgba(6,182,212,0.3)] active:scale-[0.98] transition-all cursor-pointer border-none"
					@click="showCreateModal = true"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-4 h-4"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
					>
						<line x1="12" y1="5" x2="12" y2="19" />
						<line x1="5" y1="12" x2="19" y2="12" />
					</svg>
					<span>Create New Server</span>
				</button>
			</div>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<div
					v-for="server in serverList"
					:key="server.id"
					class="p-5 rounded-2xl bg-[#141923]/90 border transition-all duration-200 flex flex-col justify-between gap-4"
					:class="
						activeServer?.id === server.id
							? 'border-sky-500/50 shadow-[0_0_20px_rgba(56,189,248,0.15)] bg-[#141923]'
							: 'border-white/10 hover:border-white/20'
					"
				>
					<div class="flex items-start justify-between gap-3">
						<div class="flex items-center gap-3">
							<div
								class="w-10 h-10 rounded-xl bg-zinc-800 border border-white/10 flex items-center justify-center text-sky-400 font-black text-sm"
							>
								MC
							</div>
							<div>
								<div class="flex items-center gap-2">
									<h3 class="text-sm font-bold text-white m-0">{{ server.name }}</h3>
									<span
										v-if="activeServer?.id === server.id"
										class="px-2 py-0.5 rounded-full text-[10px] font-bold bg-sky-500/20 text-sky-300 border border-sky-500/30 uppercase"
										>Active</span
									>
								</div>
								<span class="text-xs text-zinc-400 font-mono"
									>{{ server.engine }} {{ server.version }} &bull; {{ server.ram_gb }} GB RAM &bull;
									Port {{ server.port }}</span
								>
							</div>
						</div>

						<div class="flex items-center gap-1.5">
							<button
								v-if="activeServer?.id !== server.id"
								type="button"
								class="px-3 py-1.5 rounded-lg bg-sky-500/20 hover:bg-sky-500/30 text-sky-300 text-xs font-bold border border-sky-500/30 cursor-pointer"
								@click="selectServer(server)"
							>
								Select
							</button>
							<button
								type="button"
								class="p-1.5 rounded-lg bg-zinc-800 hover:bg-rose-500/20 text-zinc-400 hover:text-rose-300 border border-white/5 cursor-pointer"
								title="Delete Server"
								@click="deleteServer(server.id)"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-4 h-4"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<polyline points="3 6 5 6 21 6" />
									<path
										d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
									/>
								</svg>
							</button>
						</div>
					</div>

					<div
						class="flex items-center justify-between text-[11px] font-mono text-zinc-400 bg-[#090B0F] px-3 py-2 rounded-xl border border-white/5"
					>
						<span class="truncate">Path: {{ server.path }}</span>
					</div>
				</div>
			</div>
		</div>

		<!-- TAB 4: FILE MANAGER -->
		<div
			v-if="activeTab === 'files'"
			class="p-6 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-4"
		>
			<div
				class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 border-b border-white/10 pb-4"
			>
				<div class="flex items-center gap-2.5">
					<div
						class="w-8 h-8 rounded-lg bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400"
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
							<path
								d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
							/>
						</svg>
					</div>
					<div>
						<h2 class="text-base font-extrabold text-white m-0">Server File System</h2>
						<p class="text-xs text-zinc-400 m-0">
							Live files located in:
							<span class="font-mono text-zinc-200">{{ activeServer?.path || 'servers' }}</span>
						</p>
					</div>
				</div>

				<div class="flex items-center gap-2">
					<button
						type="button"
						class="inline-flex items-center gap-2 px-3 py-1.5 rounded-xl bg-zinc-800 hover:bg-zinc-700 text-xs font-bold text-zinc-300 hover:text-white border border-white/10 cursor-pointer transition-colors"
						@click="fetchServerFiles"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
							<path d="M3 3v5h5" />
							<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
							<path d="M16 21h5v-5" />
						</svg>
						<span>Refresh</span>
					</button>

					<button
						type="button"
						class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-xl bg-indigo-500/20 hover:bg-indigo-500/30 text-xs font-bold text-indigo-300 border border-indigo-500/40 cursor-pointer transition-colors"
						@click="openServerFolder"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-3.5 h-3.5 text-indigo-400"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path
								d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
							/>
						</svg>
						<span>Open in File Explorer</span>
					</button>
				</div>
			</div>

			<!-- Breadcrumb Directory Navigation Bar with Back, Forward & Up Controls -->
			<div
				class="flex items-center justify-between flex-wrap gap-2 px-3.5 py-2 rounded-xl bg-zinc-900/90 border border-white/10 text-xs font-mono"
			>
				<div class="flex items-center gap-2 flex-wrap">
					<!-- History & Up Navigation Group -->
					<div class="flex items-center gap-1 bg-zinc-800/90 p-1 rounded-lg border border-white/5">
						<!-- Backward Button -->
						<button
							type="button"
							class="w-7 h-7 flex items-center justify-center rounded-md text-zinc-300 hover:text-white hover:bg-zinc-700/80 disabled:opacity-25 disabled:pointer-events-none transition-all cursor-pointer border-none"
							:disabled="!canGoBack"
							title="Back"
							@click="goBack"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2.5"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<polyline points="15 18 9 12 15 6" />
							</svg>
						</button>

						<!-- Forward Button -->
						<button
							type="button"
							class="w-7 h-7 flex items-center justify-center rounded-md text-zinc-300 hover:text-white hover:bg-zinc-700/80 disabled:opacity-25 disabled:pointer-events-none transition-all cursor-pointer border-none"
							:disabled="!canGoForward"
							title="Forward"
							@click="goForward"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2.5"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<polyline points="9 18 15 12 9 6" />
							</svg>
						</button>

						<!-- Up to Parent Directory Button -->
						<button
							type="button"
							class="w-7 h-7 flex items-center justify-center rounded-md text-zinc-300 hover:text-white hover:bg-zinc-700/80 disabled:opacity-25 disabled:pointer-events-none transition-all cursor-pointer border-none"
							:disabled="!canGoUp"
							title="Up to parent folder"
							@click="navigateUp"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-4 h-4"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2.5"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<polyline points="18 15 12 9 6 15" />
							</svg>
						</button>
					</div>

					<!-- Breadcrumb Trail -->
					<div class="flex items-center gap-1.5 flex-wrap pl-1">
						<span
							class="cursor-pointer hover:underline flex items-center gap-1 font-bold"
							:class="!currentSubDir ? 'text-cyan-400' : 'text-zinc-400 hover:text-white'"
							@click="navigateToDir('')"
						>
							📁 root
						</span>

						<template v-for="(crumb, idx) in breadcrumbs" :key="idx">
							<span class="text-zinc-600 font-normal">/</span>
							<span
								class="cursor-pointer hover:underline"
								:class="
									idx === breadcrumbs.length - 1
										? 'text-cyan-400 font-bold'
										: 'text-zinc-400 hover:text-white'
								"
								@click="navigateToDir(crumb.path)"
							>
								{{ crumb.name }}
							</span>
						</template>
					</div>
				</div>

				<span class="text-[11px] text-zinc-500 font-normal"> {{ serverFiles.length }} items </span>
			</div>

			<!-- Files Table -->
			<div class="overflow-x-auto">
				<table class="w-full text-left text-xs font-mono">
					<thead>
						<tr class="border-b border-white/10 text-zinc-400 uppercase text-[10px]">
							<th class="py-2.5 px-3">File / Directory</th>
							<th class="py-2.5 px-3">Type</th>
							<th class="py-2.5 px-3">Size</th>
							<th class="py-2.5 px-3 text-right">Action</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-white/5">
						<tr
							v-for="file in serverFiles"
							:key="file.name"
							class="hover:bg-white/5 transition-colors cursor-pointer"
							:class="file.is_dir ? 'hover:bg-amber-500/10' : 'hover:bg-cyan-500/5'"
							@click="file.is_dir ? navigateToDir(file.path) : openFileEditor(file)"
						>
							<td class="py-3 px-3 flex items-center gap-2 text-white font-bold">
								<svg
									v-if="file.is_dir"
									xmlns="http://www.w3.org/2000/svg"
									class="w-4 h-4 text-amber-400 shrink-0"
									viewBox="0 0 24 24"
									fill="currentColor"
								>
									<path
										d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
									/>
								</svg>
								<svg
									v-else
									xmlns="http://www.w3.org/2000/svg"
									class="w-4 h-4 text-cyan-400 shrink-0"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
									<polyline points="14 2 14 8 20 8" />
								</svg>
								<span :class="file.is_dir ? 'text-amber-200' : 'text-zinc-100'">{{
									file.name
								}}</span>
							</td>
							<td class="py-3 px-3 text-zinc-400">
								<span
									v-if="file.is_dir"
									class="px-2 py-0.5 rounded bg-amber-500/20 text-amber-300 text-[10px] font-bold"
								>
									Folder
								</span>
								<span v-else class="text-zinc-400"> File </span>
							</td>
							<td class="py-3 px-3 text-zinc-400">
								{{ file.is_dir ? '--' : formatFileSize(file.size) }}
							</td>
							<td class="py-3 px-3 text-right" @click.stop>
								<button
									v-if="file.is_dir"
									type="button"
									class="inline-flex items-center gap-1.5 px-3 py-1 rounded-lg bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 font-bold transition-all cursor-pointer border border-amber-500/30 text-xs shadow-sm hover:scale-105 active:scale-95"
									@click="navigateToDir(file.path)"
								>
									<span>Open</span>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										class="w-3.5 h-3.5"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
									>
										<path d="M5 12h14" />
										<path d="m12 5 7 7-7 7" />
									</svg>
								</button>
								<button
									v-else
									type="button"
									class="inline-flex items-center gap-1 px-3 py-1 rounded-lg bg-zinc-800 hover:bg-cyan-500 hover:text-zinc-950 text-cyan-300 font-bold transition-all cursor-pointer border border-cyan-500/30 text-xs shadow-sm hover:scale-105 active:scale-95"
									@click="openFileEditor(file)"
								>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										class="w-3 h-3"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
									>
										<path d="M12 20h9" />
										<path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" />
									</svg>
									<span>Edit</span>
								</button>
							</td>
						</tr>
						<tr v-if="serverFiles.length === 0">
							<td colspan="4" class="py-8 text-center text-zinc-500 italic font-sans">
								No files or folders found in this directory.
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>

		<!-- TAB 5: BACKUPS -->
		<div
			v-if="activeTab === 'backups'"
			class="p-6 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-5"
		>
			<div
				class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-white/10 pb-5"
			>
				<div class="flex items-center gap-3">
					<div
						class="w-10 h-10 rounded-xl bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400 shadow-[0_0_15px_rgba(245,158,11,0.2)]"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-5 h-5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
							<polyline points="17 21 17 13 7 13 7 21" />
							<polyline points="7 3 7 8 15 8" />
						</svg>
					</div>
					<div>
						<h2 class="text-base font-extrabold text-white m-0">World Snapshot Backups</h2>
						<p class="text-xs text-zinc-400 m-0">
							Create compressed zip archives of your world directory and restore anytime.
						</p>
					</div>
				</div>

				<button
					type="button"
					class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-gradient-to-r from-amber-500 to-amber-400 hover:from-amber-400 hover:to-amber-300 text-zinc-950 font-extrabold text-xs shadow-[0_0_15px_rgba(245,158,11,0.3)] active:scale-[0.98] transition-all cursor-pointer border-none"
					@click="createBackup"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="w-4 h-4"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<line x1="12" y1="5" x2="12" y2="19" />
						<line x1="5" y1="12" x2="19" y2="12" />
					</svg>
					<span>Create World Snapshot</span>
				</button>
			</div>

			<div class="flex flex-col gap-3">
				<div
					v-for="backup in backupsList"
					:key="backup.id"
					class="flex items-center justify-between p-4 rounded-xl bg-[#090B0F]/80 border border-white/5 hover:border-white/15 transition-all duration-200"
				>
					<div class="flex items-center gap-3.5">
						<div
							class="w-9 h-9 rounded-xl bg-zinc-800 flex items-center justify-center text-amber-400"
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
								<path
									d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
								/>
							</svg>
						</div>
						<div class="flex flex-col">
							<span class="text-sm font-bold text-white font-mono">{{ backup.name }}</span>
							<span class="text-xs text-zinc-400 font-mono mt-0.5"
								>{{ backup.date }} &bull; {{ backup.size }}</span
							>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<button
							type="button"
							class="px-3.5 py-1.5 rounded-lg bg-zinc-800 hover:bg-amber-500 hover:text-zinc-950 text-zinc-300 font-bold text-xs transition-colors cursor-pointer border border-white/5 active:scale-95"
							@click="restoreBackup(backup)"
						>
							Restore
						</button>
					</div>
				</div>

				<div v-if="backupsList.length === 0" class="py-8 text-center text-zinc-500 italic text-xs">
					No backups found. Click "Create World Snapshot" to back up your current world.
				</div>
			</div>
		</div>

		<!-- TAB 6: SETTINGS & PORTS -->
		<div
			v-if="activeTab === 'settings'"
			class="p-6 rounded-2xl bg-[#141923]/90 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-6"
		>
			<div class="flex items-center justify-between border-b border-white/10 pb-4">
				<div class="flex items-center gap-2.5">
					<div
						class="w-8 h-8 rounded-lg bg-sky-500/10 border border-sky-500/30 flex items-center justify-center text-sky-400"
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
							<path
								d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
							/>
							<circle cx="12" cy="12" r="3" />
						</svg>
					</div>
					<div>
						<h2 class="text-base font-extrabold text-white m-0">
							Server Engine &amp; Performance Settings
						</h2>
						<p class="text-xs text-zinc-400 m-0">
							Configure dedicated memory, loader version, ports, and custom storage location.
						</p>
					</div>
				</div>
			</div>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<!-- Active Minecraft Version Display -->
				<div class="flex flex-col gap-2.5">
					<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider"
						>Minecraft Version</label
					>
					<div
						class="flex items-center justify-between px-4 py-2.5 rounded-xl bg-zinc-900 border border-white/10"
					>
						<div class="flex items-center gap-2.5">
							<span class="text-base font-extrabold text-white font-mono">{{
								serverState.version || activeServer?.version || '1.21.4'
							}}</span>
							<span
								class="text-[10px] font-semibold text-sky-400 bg-sky-500/10 border border-sky-500/20 px-2 py-0.5 rounded-md uppercase tracking-wider font-mono"
							>
								Installed
							</span>
						</div>
						<span class="text-[11px] text-zinc-400 font-mono">Game Release</span>
					</div>
				</div>

				<!-- Active Server Engine Core Display -->
				<div class="flex flex-col gap-2.5">
					<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider"
						>Server Engine Core</label
					>
					<div
						class="flex items-center justify-between px-4 py-2.5 rounded-xl bg-zinc-900 border border-white/10"
					>
						<div class="flex items-center gap-2.5">
							<span class="text-base font-extrabold text-white font-mono">{{
								serverState.engine || activeServer?.engine || 'PaperMC'
							}}</span>
							<span
								class="text-[10px] font-semibold text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 rounded-md uppercase tracking-wider font-mono"
							>
								Active Core
							</span>
						</div>
						<span class="text-[11px] text-zinc-400 font-mono">Server Software</span>
					</div>
				</div>

				<!-- Dedicated RAM Allocation Slider (2GB to 16GB) -->
				<div class="flex flex-col gap-3 md:col-span-2">
					<div class="flex items-center justify-between">
						<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider"
							>Dedicated RAM Allocation</label
						>
						<div class="flex items-center gap-2">
							<span
								class="text-sm font-extrabold text-white font-mono bg-zinc-800 px-3 py-0.5 rounded-lg border border-white/10"
								>{{ serverState.ram_gb }} GB Dedicated</span
							>
							<span class="text-xs text-sky-400 font-mono font-bold"
								>({{ serverState.ram_gb * 1024 }} MB)</span
							>
						</div>
					</div>

					<!-- Slider Bar Container with margin for 0% and 100% thumb clearance -->
					<div class="px-3 pt-2 pb-1">
						<!-- Track Bar & Custom Thumb Container -->
						<div class="relative w-full h-7 flex items-center">
							<!-- Track Background -->
							<div
								class="w-full h-2.5 bg-zinc-800/90 rounded-full border border-white/10 relative overflow-hidden"
							>
								<!-- Progress Active Fill -->
								<div
									class="absolute left-0 top-0 bottom-0 bg-gradient-to-r from-sky-600 via-sky-500 to-cyan-400 rounded-full transition-all duration-75"
									:style="{ width: `${((serverState.ram_gb - 2) / 14) * 100}%` }"
								/>
							</div>

							<!-- Vertical Tick Lines at each preset -->
							<div class="absolute inset-0 pointer-events-none flex items-center">
								<!-- 1GB Subtle Step Ticks -->
								<div
									v-for="gb in [3, 5, 6, 7, 9, 10, 11, 13, 14, 15]"
									:key="'step-' + gb"
									class="w-[1px] h-2 rounded-full transition-colors duration-150"
									:style="{
										left: `${((gb - 2) / 14) * 100}%`,
										position: 'absolute',
										transform: 'translateX(-50%)',
									}"
									:class="serverState.ram_gb >= gb ? 'bg-white/40' : 'bg-zinc-700/60'"
								/>

								<!-- Main Preset Tick Lines -->
								<div
									v-for="preset in ramPresets"
									:key="'tick-' + preset.val"
									class="w-[2px] h-4 rounded-full transition-all duration-150"
									:style="{
										left: `${((preset.val - 2) / 14) * 100}%`,
										position: 'absolute',
										transform: 'translateX(-50%)',
									}"
									:class="
										serverState.ram_gb >= preset.val
											? 'bg-white shadow-[0_0_6px_rgba(255,255,255,0.8)]'
											: 'bg-zinc-500'
									"
								/>
							</div>

							<!-- Custom Pixel-Perfect Thumb -->
							<div
								class="absolute top-1/2 -translate-y-1/2 -translate-x-1/2 w-4 h-4 rounded-full bg-sky-400 border-2 border-white shadow-[0_0_10px_rgba(56,189,248,0.9)] pointer-events-none z-20 transition-all duration-75"
								:style="{ left: `${((serverState.ram_gb - 2) / 14) * 100}%` }"
							/>

							<!-- Transparent Range Input Overlay for Drag / Touch / Keyboard -->
							<input
								type="range"
								min="2"
								max="16"
								step="1"
								:value="serverState.ram_gb"
								class="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-30"
								@input="onRamChange"
							/>
						</div>

						<!-- Pointing Arrows & Values Row (Exact same coordinate formula) -->
						<div class="relative w-full h-11 text-[10px] font-mono select-none mt-1">
							<div
								v-for="preset in ramPresets"
								:key="preset.val"
								class="absolute top-0 flex flex-col items-center cursor-pointer group"
								:style="{
									left: `${((preset.val - 2) / 14) * 100}%`,
									transform: 'translateX(-50%)',
								}"
								@click="setRamGb(preset.val)"
							>
								<!-- Pointing Arrow: Points directly to the tick and thumb -->
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5 transition-all duration-150 mb-0.5"
									:class="
										serverState.ram_gb === preset.val
											? 'text-sky-400 scale-125 drop-shadow-[0_0_8px_rgba(56,189,248,1)]'
											: 'text-zinc-600 group-hover:text-zinc-400'
									"
									viewBox="0 0 24 24"
									fill="currentColor"
								>
									<path d="M12 4l-8 14h16l-8-14z" />
								</svg>

								<!-- Label & Tag -->
								<span
									class="font-bold transition-colors duration-150 whitespace-nowrap"
									:class="
										serverState.ram_gb === preset.val
											? 'text-sky-300 font-extrabold scale-105'
											: 'text-zinc-400 group-hover:text-zinc-200'
									"
								>
									{{ preset.val }} GB
									<span
										class="font-normal text-[9px]"
										:class="serverState.ram_gb === preset.val ? 'text-sky-400' : 'text-zinc-500'"
									>
										({{ preset.tag }})
									</span>
								</span>
							</div>
						</div>
					</div>
				</div>

				<!-- Server Port -->
				<div class="flex flex-col gap-2.5">
					<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider"
						>Local Server Port</label
					>
					<input
						v-model.number="serverState.local_port"
						type="number"
						class="w-full bg-zinc-900 border border-white/10 rounded-xl px-4 py-2.5 text-xs text-white font-mono outline-none focus:border-sky-500"
						@change="updateConfig"
					/>
				</div>

				<!-- Server MOTD -->
				<div class="flex flex-col gap-2.5">
					<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider"
						>Server MOTD (Message of the Day)</label
					>
					<input
						v-model="serverState.motd"
						type="text"
						class="w-full bg-zinc-900 border border-white/10 rounded-xl px-4 py-2.5 text-xs text-white font-mono outline-none focus:border-sky-500"
						@change="updateConfig"
					/>
				</div>

				<!-- Storage Path Option -->
				<div class="flex flex-col gap-2.5 md:col-span-2">
					<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider"
						>Server Files Location</label
					>
					<div class="flex items-center gap-2">
						<input
							:value="activeServer?.path || 'Default config directory'"
							type="text"
							readonly
							class="flex-1 bg-zinc-900 border border-white/10 rounded-xl px-4 py-2.5 text-xs text-zinc-300 font-mono outline-none"
						/>
						<button
							type="button"
							class="px-4 py-2.5 rounded-xl bg-zinc-800 hover:bg-zinc-700 text-xs font-bold text-white border border-white/10 cursor-pointer"
							@click="openServerFolder"
						>
							Open Folder
						</button>
					</div>
				</div>
			</div>
		</div>

		<!-- CREATE SERVER MODAL -->
		<div
			v-if="showCreateModal"
			class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
		>
			<div
				class="w-full max-w-lg rounded-2xl bg-[#141923] border border-white/15 shadow-2xl p-6 flex flex-col gap-5"
			>
				<div class="flex items-center justify-between border-b border-white/10 pb-4">
					<h3 class="text-base font-extrabold text-white m-0">Create New Minecraft Server</h3>
					<button
						type="button"
						class="p-1 rounded-lg text-zinc-400 hover:text-white bg-transparent border-none cursor-pointer"
						@click="showCreateModal = false"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-5 h-5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<line x1="18" y1="6" x2="6" y2="18" />
							<line x1="6" y1="6" x2="18" y2="18" />
						</svg>
					</button>
				</div>

				<div class="flex flex-col gap-4 text-xs">
					<div class="flex flex-col gap-1.5">
						<label class="font-bold text-zinc-300">Server Name</label>
						<input
							v-model="newServerForm.name"
							type="text"
							placeholder="e.g. My Survival Server"
							class="bg-zinc-900 border border-white/10 rounded-xl px-3.5 py-2.5 text-white font-mono outline-none focus:border-cyan-500"
						/>
					</div>

					<div class="grid grid-cols-2 gap-3">
						<!-- Custom Engine Core Dropdown -->
						<div class="relative flex flex-col gap-1.5">
							<label class="font-bold text-zinc-300">Engine Core</label>
							<button
								type="button"
								class="flex items-center justify-between bg-zinc-900 border border-white/10 hover:border-cyan-500/50 rounded-xl px-3.5 py-2.5 text-white font-mono text-xs cursor-pointer transition-all focus-visible:ring-2 focus-visible:ring-cyan-500 focus-visible:outline-none"
								@click="toggleEngineDropdown"
							>
								<div class="flex items-center gap-2 min-w-0">
									<span class="font-bold truncate">{{ newServerForm.engine }}</span>
									<span
										class="px-1.5 py-0.2 rounded text-[10px] font-bold bg-cyan-500/20 text-cyan-300 border border-cyan-500/30"
									>
										{{
											availableEngines.find((e) => e.id === newServerForm.engine)?.tag || 'Engine'
										}}
									</span>
								</div>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5 text-zinc-400 transition-transform duration-200"
									:class="{ 'rotate-180 text-cyan-400': isEngineDropdownOpen }"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<polyline points="6 9 12 15 18 9" />
								</svg>
							</button>

							<!-- Engine Dropdown Menu -->
							<transition name="fade">
								<div
									v-if="isEngineDropdownOpen"
									class="absolute top-[calc(100%+4px)] left-0 right-0 z-50 p-1.5 rounded-2xl bg-[#0e121a] border border-white/15 shadow-2xl backdrop-blur-xl flex flex-col gap-1 max-h-56 overflow-y-auto scrollbar-thin"
								>
									<div
										v-for="eng in availableEngines"
										:key="eng.id"
										class="p-2 rounded-xl flex items-center justify-between gap-2 transition-all cursor-pointer select-none"
										:class="
											newServerForm.engine === eng.id
												? 'bg-cyan-500/20 text-cyan-200 border border-cyan-500/40'
												: 'hover:bg-zinc-800/80 text-zinc-300 hover:text-white border border-transparent'
										"
										@click="selectEngine(eng.id)"
									>
										<div class="flex flex-col min-w-0">
											<div class="flex items-center gap-1.5">
												<span class="font-bold text-xs">{{ eng.name }}</span>
												<span
													class="px-1.5 py-0.2 rounded text-[9px] font-extrabold bg-zinc-800 text-zinc-400 border border-white/5"
												>
													{{ eng.tag }}
												</span>
											</div>
											<span class="text-[10px] text-zinc-400 truncate">{{ eng.desc }}</span>
										</div>
										<svg
											v-if="newServerForm.engine === eng.id"
											xmlns="http://www.w3.org/2000/svg"
											class="w-4 h-4 text-cyan-400 shrink-0"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2.5"
											stroke-linecap="round"
											stroke-linejoin="round"
										>
											<polyline points="20 6 9 17 4 12" />
										</svg>
									</div>
								</div>
							</transition>
						</div>

						<!-- Custom Minecraft Version Dropdown -->
						<div class="relative flex flex-col gap-1.5">
							<label class="font-bold text-zinc-300">Minecraft Version</label>
							<button
								type="button"
								class="flex items-center justify-between bg-zinc-900 border border-white/10 hover:border-cyan-500/50 rounded-xl px-3.5 py-2.5 text-white font-mono text-xs cursor-pointer transition-all focus-visible:ring-2 focus-visible:ring-cyan-500 focus-visible:outline-none"
								:class="{
									'border-cyan-500 shadow-[0_0_12px_rgba(6,182,212,0.25)]': isVersionDropdownOpen,
								}"
								@click="toggleVersionDropdown"
							>
								<div class="flex items-center gap-2 min-w-0">
									<span class="font-bold truncate">{{ newServerForm.version }}</span>
									<span
										class="px-1.5 py-0.2 rounded text-[10px] font-bold bg-cyan-500/20 text-cyan-300 border border-cyan-500/30"
									>
										{{
											availableVersions.find((v) => v.id === newServerForm.version)?.tag || 'Custom'
										}}
									</span>
								</div>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="w-3.5 h-3.5 text-zinc-400 transition-transform duration-200"
									:class="{ 'rotate-180 text-cyan-400': isVersionDropdownOpen }"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<polyline points="6 9 12 15 18 9" />
								</svg>
							</button>

							<!-- Version Dropdown Menu with Search & Full Release Catalog -->
							<transition name="fade">
								<div
									v-if="isVersionDropdownOpen"
									class="absolute top-[calc(100%+4px)] left-0 right-0 z-50 p-2 rounded-2xl bg-[#0e121a] border border-white/15 shadow-2xl backdrop-blur-xl flex flex-col gap-1.5 max-h-72 overflow-y-auto scrollbar-thin"
								>
									<div class="p-1 pb-1.5 border-b border-white/10 sticky top-0 bg-[#0e121a] z-10">
										<input
											v-model="versionSearchQuery"
											type="text"
											placeholder="Search all versions (e.g. 1.20.1, 1.12.2)..."
											class="w-full bg-zinc-900 border border-white/10 rounded-lg px-2.5 py-1.5 text-xs text-white font-mono outline-none focus:border-cyan-500"
											@click.stop
										/>
									</div>

									<!-- Custom version option if typed query not in list -->
									<div
										v-if="
											versionSearchQuery.trim() &&
											!filteredVersions.some((v) => v.id === versionSearchQuery.trim())
										"
										class="p-2 rounded-xl flex items-center justify-between gap-2 transition-all cursor-pointer bg-cyan-500/10 hover:bg-cyan-500/20 text-cyan-300 border border-cyan-500/30 text-xs font-mono select-none"
										@click="selectVersion(versionSearchQuery.trim())"
									>
										<span
											>Select Custom: <strong>{{ versionSearchQuery.trim() }}</strong></span
										>
										<span class="text-[9px] px-1.5 py-0.5 rounded bg-cyan-500/20 text-cyan-200"
											>Custom</span
										>
									</div>

									<div
										v-for="ver in filteredVersions"
										:key="ver.id"
										class="p-2 rounded-xl flex items-center justify-between gap-2 transition-all cursor-pointer select-none"
										:class="
											newServerForm.version === ver.id
												? 'bg-cyan-500/20 text-cyan-200 border border-cyan-500/40'
												: 'hover:bg-zinc-800/80 text-zinc-300 hover:text-white border border-transparent'
										"
										@click="selectVersion(ver.id)"
									>
										<div class="flex items-center gap-2">
											<span class="font-bold text-xs font-mono">{{ ver.label }}</span>
											<span
												v-if="ver.tag"
												class="px-1.5 py-0.2 rounded text-[9px] font-bold bg-zinc-800 text-zinc-400 border border-white/5"
											>
												{{ ver.tag }}
											</span>
										</div>
										<svg
											v-if="newServerForm.version === ver.id"
											xmlns="http://www.w3.org/2000/svg"
											class="w-4 h-4 text-cyan-400 shrink-0"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2.5"
											stroke-linecap="round"
											stroke-linejoin="round"
										>
											<polyline points="20 6 9 17 4 12" />
										</svg>
									</div>

									<div
										v-if="filteredVersions.length === 0 && !versionSearchQuery.trim()"
										class="text-zinc-500 text-xs text-center py-3"
									>
										No versions found
									</div>
								</div>
							</transition>
						</div>
					</div>

					<div class="grid grid-cols-2 gap-3">
						<div class="flex flex-col gap-1.5">
							<label class="font-bold text-zinc-300">RAM (GB)</label>
							<input
								v-model.number="newServerForm.ram_gb"
								type="number"
								min="2"
								max="32"
								class="bg-zinc-900 border border-white/10 rounded-xl px-3.5 py-2.5 text-white font-mono outline-none focus:border-cyan-500"
							/>
						</div>

						<div class="flex flex-col gap-1.5">
							<label class="font-bold text-zinc-300">Port</label>
							<input
								v-model.number="newServerForm.port"
								type="number"
								class="bg-zinc-900 border border-white/10 rounded-xl px-3.5 py-2.5 text-white font-mono outline-none focus:border-cyan-500"
							/>
						</div>
					</div>

					<div class="flex flex-col gap-1.5">
						<label class="font-bold text-zinc-300">Custom Storage Path (Optional)</label>
						<div class="flex items-center gap-2">
							<input
								v-model="newServerForm.custom_path"
								type="text"
								placeholder="Leave empty for default location"
								class="flex-1 bg-zinc-900 border border-white/10 rounded-xl px-3.5 py-2.5 text-white font-mono outline-none focus:border-cyan-500 text-xs"
							/>
							<button
								type="button"
								class="px-3 py-2.5 rounded-xl bg-zinc-800 hover:bg-zinc-700 text-zinc-200 text-xs font-bold border border-white/10 cursor-pointer shrink-0"
								@click="browseCustomPath"
							>
								Browse...
							</button>
						</div>
					</div>
				</div>

				<div class="flex items-center justify-end gap-3 border-t border-white/10 pt-4">
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-zinc-800 text-zinc-300 text-xs font-bold cursor-pointer border-none"
						@click="showCreateModal = false"
					>
						Cancel
					</button>
					<button
						type="button"
						class="px-5 py-2 rounded-xl bg-gradient-to-r from-sky-400 to-blue-500 hover:from-sky-300 hover:to-blue-400 text-zinc-950 text-xs font-extrabold cursor-pointer border-none shadow-lg"
						@click="submitCreateServer"
					>
						Create Server
					</button>
				</div>
			</div>
		</div>

		<!-- SERVER LIST QUICK MODAL -->
		<div
			v-if="showServerListModal"
			class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
		>
			<div
				class="w-full max-w-md rounded-2xl bg-[#141923] border border-white/15 shadow-2xl p-6 flex flex-col gap-4"
			>
				<div class="flex items-center justify-between border-b border-white/10 pb-3">
					<h3 class="text-base font-extrabold text-white m-0">Your Minecraft Servers</h3>
					<button
						type="button"
						class="p-1 rounded-lg text-zinc-400 hover:text-white bg-transparent border-none cursor-pointer"
						@click="showServerListModal = false"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-5 h-5"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<line x1="18" y1="6" x2="6" y2="18" />
							<line x1="6" y1="6" x2="18" y2="18" />
						</svg>
					</button>
				</div>

				<div class="flex flex-col gap-2 max-h-80 overflow-y-auto scrollbar-thin">
					<div
						v-for="s in serverList"
						:key="s.id"
						class="p-3 rounded-xl border flex items-center justify-between transition-all cursor-pointer"
						:class="
							activeServer?.id === s.id
								? 'bg-sky-500/10 border-sky-500/40 text-sky-300'
								: 'bg-zinc-900/80 border-white/5 text-zinc-300 hover:border-white/20'
						"
						@click="handleSelectServerModal(s)"
					>
						<div class="flex flex-col">
							<span class="font-bold text-sm text-white">{{ s.name }}</span>
							<span class="text-[11px] text-zinc-400 font-mono"
								>{{ s.engine }} {{ s.version }} &bull; Port {{ s.port }}</span
							>
						</div>
						<span v-if="activeServer?.id === s.id" class="text-xs font-bold text-sky-400"
							>Selected</span
						>
					</div>
				</div>

				<div class="flex items-center justify-between border-t border-white/10 pt-3">
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-cyan-500 text-zinc-950 font-bold text-xs cursor-pointer border-none"
						@click="handleOpenCreateModalFromServerList"
					>
						+ Create New
					</button>
					<button
						type="button"
						class="px-4 py-2 rounded-xl bg-zinc-800 text-zinc-300 text-xs font-bold cursor-pointer border-none"
						@click="showServerListModal = false"
					>
						Close
					</button>
				</div>
			</div>
		</div>

		<!-- Share & Invite Modal -->
		<InviteShareModal
			:show="showInviteShareModal"
			:public-ip="serverState.public_ip"
			:local-port="serverState.local_port"
			:engine="serverState.engine"
			:version="serverState.version"
			:server-name="activeServer?.name || 'FreePlay Dedicated Server'"
			@close="showInviteShareModal = false"
		/>

		<!-- Completely Floating Centered File Editor Modal Window -->
		<teleport to="body">
			<transition name="fade">
				<div
					v-if="editingFile"
					class="fixed inset-0 z-[999] bg-black/85 backdrop-blur-md flex items-center justify-center p-4 sm:p-6"
					@click.self="closeFileEditor"
				>
					<div
						class="w-full max-w-4xl max-h-[88vh] bg-[#0d121c] border border-cyan-500/30 rounded-2xl shadow-2xl flex flex-col overflow-hidden animate-fadeIn"
					>
						<!-- Modal Header -->
						<div
							class="flex items-center justify-between px-5 py-3.5 border-b border-white/10 bg-[#131a27]"
						>
							<div class="flex items-center gap-3 min-w-0">
								<div
									class="w-9 h-9 rounded-xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shrink-0"
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
										<path
											d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"
										/>
										<polyline points="14 2 14 8 20 8" />
									</svg>
								</div>
								<div class="flex flex-col min-w-0">
									<div class="flex items-center gap-2">
										<span class="text-sm font-bold text-white font-mono truncate">{{
											editingFile.name
										}}</span>
										<span
											class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-cyan-500/20 text-cyan-300 border border-cyan-500/30 font-mono"
										>
											{{ formatFileSize(editingFile.size) }}
										</span>
									</div>
									<span class="text-[11px] text-zinc-400 font-mono truncate">{{
										editingFile.path
									}}</span>
								</div>
							</div>

							<div class="flex items-center gap-2.5">
								<button
									type="button"
									class="px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-400 to-sky-500 hover:from-cyan-300 hover:to-sky-400 text-zinc-950 font-bold text-xs cursor-pointer border-none shadow-md hover:shadow-cyan-500/20 transition-all flex items-center gap-1.5 active:scale-95 disabled:opacity-50"
									:disabled="isFileSaving"
									@click="saveFileEditor"
								>
									<span v-if="fileSaveSuccess" class="text-zinc-950 font-bold">✓ Saved!</span>
									<span v-else-if="isFileSaving">Saving...</span>
									<span v-else>Save Changes</span>
								</button>

								<button
									type="button"
									class="w-8 h-8 rounded-xl bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-white flex items-center justify-center cursor-pointer border border-white/5 transition-colors"
									@click="closeFileEditor"
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
						</div>

						<!-- Editor Body -->
						<div class="p-4 flex-1 overflow-hidden flex flex-col bg-[#080b11]">
							<textarea
								v-model="fileEditorContent"
								rows="18"
								class="w-full flex-1 bg-transparent p-4 rounded-xl border border-white/10 font-mono text-xs text-zinc-200 outline-none focus:border-cyan-500/60 leading-relaxed resize-none overflow-y-auto scrollbar-thin"
								placeholder="File content..."
								spellcheck="false"
							/>
						</div>

						<!-- Modal Footer Info -->
						<div
							class="flex items-center justify-between px-5 py-3 border-t border-white/10 bg-[#0f1420] text-xs text-zinc-400 font-mono"
						>
							<div class="flex items-center gap-4 text-[11px]">
								<span
									>Lines:
									<strong class="text-zinc-200">{{
										fileEditorContent.split('\n').length
									}}</strong></span
								>
								<span
									>Characters:
									<strong class="text-zinc-200">{{ fileEditorContent.length }}</strong></span
								>
							</div>
							<div class="flex items-center gap-2">
								<button
									type="button"
									class="px-3 py-1.5 rounded-lg bg-zinc-800 hover:bg-zinc-700 text-zinc-300 hover:text-white text-xs cursor-pointer border border-white/5 transition-colors"
									@click="closeFileEditor"
								>
									Cancel
								</button>
								<button
									type="button"
									class="px-4 py-1.5 rounded-lg bg-cyan-500 hover:bg-cyan-400 text-zinc-950 font-bold text-xs cursor-pointer border-none transition-colors"
									:disabled="isFileSaving"
									@click="saveFileEditor"
								>
									Save File
								</button>
							</div>
						</div>
					</div>
				</div>
			</transition>
		</teleport>

		<!-- MODAL: BROWSE MODRINTH ADDONS -->
		<teleport to="body">
			<transition name="fade">
				<div
					v-if="showAddonCatalog"
					class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-md"
				>
					<div
						class="w-full max-w-4xl max-h-[85vh] flex flex-col rounded-2xl bg-[#141923] border border-white/10 shadow-2xl overflow-hidden font-sans text-zinc-100"
					>
						<!-- Modal Header -->
						<div class="flex items-center justify-between p-5 border-b border-white/10 bg-[#090B0F]/80">
							<div class="flex items-center gap-3">
								<div class="w-9 h-9 rounded-xl bg-purple-500/10 border border-purple-500/30 flex items-center justify-center text-purple-400">
									<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<rect width="8" height="8" x="2" y="2" rx="2" />
										<path d="M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
										<path d="M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
										<rect width="8" height="8" x="2" y="14" rx="2" />
										<rect width="8" height="8" x="14" y="14" rx="2" />
									</svg>
								</div>
								<div>
									<h3 class="text-lg font-bold text-white m-0">Modrinth Addon Catalog</h3>
									<p class="text-xs text-zinc-400 m-0">
										Compatible with <span class="text-purple-300 font-semibold">{{ serverState.engine }} {{ serverState.version }}</span>
									</p>
								</div>
							</div>

							<button
								type="button"
								class="p-2 rounded-xl bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-white cursor-pointer transition-colors border-none"
								@click="showAddonCatalog = false"
							>
								<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
									<line x1="18" y1="6" x2="6" y2="18" />
									<line x1="6" y1="6" x2="18" y2="18" />
								</svg>
							</button>
						</div>

						<!-- Search & Category Filters -->
						<div class="p-4 border-b border-white/10 bg-[#090B0F]/40 flex flex-col gap-3">
							<div class="flex items-center gap-2">
								<input
									v-model="browseCatalogQuery"
									type="text"
									placeholder="Search plugins & mods (e.g. LuckPerms, EssentialsX, WorldEdit, Geyser)..."
									class="flex-1 px-4 py-2.5 rounded-xl bg-zinc-900 border border-white/10 text-sm text-white placeholder-zinc-500 focus:outline-none focus:border-purple-500/50"
									@keyup.enter="searchModrinthAddons"
								/>
								<button
									type="button"
									class="px-5 py-2.5 rounded-xl bg-purple-600 hover:bg-purple-500 text-sm font-bold text-white cursor-pointer transition-colors"
									@click="searchModrinthAddons"
								>
									Search
								</button>
							</div>
						</div>

						<!-- Catalog Items List -->
						<div class="flex-1 overflow-y-auto p-4 flex flex-col gap-3 min-h-[350px]">
							<div v-if="searchingCatalog" class="flex flex-col items-center justify-center py-20 gap-3 text-zinc-400">
								<svg class="animate-spin w-8 h-8 text-purple-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
								</svg>
								<span class="text-sm">Fetching compatible addons from Modrinth...</span>
							</div>

							<div v-else-if="catalogResults.length === 0" class="flex flex-col items-center justify-center py-20 text-zinc-400 text-center">
								<p class="text-sm font-semibold">No addons found for query.</p>
								<p class="text-xs text-zinc-500">Try searching for generic terms like "essentials", "permissions", "performance", or "protection".</p>
							</div>

							<div
								v-for="item in catalogResults"
								:key="item.project_id"
								class="p-4 rounded-xl bg-[#181E2B] border border-white/5 hover:border-purple-500/30 flex items-start justify-between gap-4 transition-all"
							>
								<div class="flex items-start gap-3.5 flex-1 min-w-0">
									<img
										v-if="item.icon_url"
										:src="item.icon_url"
										:alt="item.title"
										class="w-12 h-12 rounded-xl object-cover bg-zinc-800 shrink-0 border border-white/10"
									/>
									<div
										v-else
										class="w-12 h-12 rounded-xl bg-purple-500/20 text-purple-300 font-bold flex items-center justify-center shrink-0 border border-purple-500/30 text-lg"
									>
										{{ item.title.charAt(0) }}
									</div>

									<div class="flex flex-col gap-1 min-w-0 flex-1">
										<div class="flex items-center gap-2 flex-wrap">
											<h4 class="text-sm font-bold text-white m-0 truncate">{{ item.title }}</h4>
											<span class="text-[10px] text-zinc-400">by {{ item.author }}</span>
											<span class="text-[10px] px-2 py-0.5 rounded bg-zinc-800 text-zinc-300 border border-white/5">
												📥 {{ (item.downloads || 0).toLocaleString() }}
											</span>
										</div>
										<p class="text-xs text-zinc-400 m-0 line-clamp-2">{{ item.description }}</p>
										<div class="flex items-center gap-1.5 flex-wrap mt-1">
											<span
												v-for="cat in (item.display_categories || item.categories || []).slice(0, 4)"
												:key="cat"
												class="text-[10px] px-1.5 py-0.5 rounded bg-purple-500/10 text-purple-300 border border-purple-500/20 uppercase font-mono"
											>
												{{ cat }}
											</span>
										</div>
									</div>
								</div>

								<div class="flex flex-col items-end gap-2 shrink-0">
									<button
										type="button"
										class="px-4 py-2 rounded-xl font-bold text-xs cursor-pointer transition-all active:scale-95 flex items-center gap-1.5"
										:class="
											isAddonInstalled(item.project_id)
												? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 pointer-events-none'
												: installingAddonId === item.project_id
												? 'bg-purple-700 text-white cursor-wait'
												: 'bg-purple-600 hover:bg-purple-500 text-white shadow-md shadow-purple-950/40'
										"
										:disabled="isAddonInstalled(item.project_id) || installingAddonId === item.project_id"
										@click="installModrinthAddon(item)"
									>
										<svg v-if="installingAddonId === item.project_id" class="animate-spin w-3.5 h-3.5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
											<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
											<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
										</svg>
										<span>{{ isAddonInstalled(item.project_id) ? 'Installed' : installingAddonId === item.project_id ? 'Installing...' : 'Install' }}</span>
									</button>
								</div>
							</div>
						</div>
					</div>
				</div>
			</transition>
		</teleport>
	</div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { computed, h, nextTick, onMounted, onUnmounted, ref } from 'vue'

import InviteShareModal from '@/components/hosting/InviteShareModal.vue'
import PlayerManagerTab from '@/components/hosting/PlayerManagerTab.vue'
import ServerPropertiesEditor from '@/components/hosting/ServerPropertiesEditor.vue'
import ServerTelemetryHub from '@/components/hosting/ServerTelemetryHub.vue'
import { get_game_versions } from '@/helpers/tags'

interface ServerState {
	status: 'offline' | 'starting' | 'online' | 'tunneling'
	version: string
	engine: string
	ram_gb: number
	tunnel_enabled: boolean
	public_ip: string
	local_port: number
	motd: string
	uptime_seconds: number
	cpu_percent: number
	ram_used_mb: number
	players: Array<{ name: string; latency?: number; online?: boolean }>
	logs: string[]
	claim_url?: string | null
}

interface ServerEntry {
	id: string
	name: string
	path: string
	engine: string
	version: string
	port: number
	ram_gb: number
}

interface ServerFileEntry {
	name: string
	path: string
	is_dir: boolean
	size: number
}

interface BackupEntry {
	id: string
	name: string
	date: string
	size: string
}

interface ServerAddonEntry {
	id: string
	name: string
	source: string
	project_id?: string | null
	version_id?: string | null
	version_number?: string | null
	filename: string
	addon_type: string
	game_version?: string | null
	file_size: number
	installed_at: string
	enabled: boolean
	dependencies: string[]
}

const serverState = ref<ServerState>({
	status: 'offline',
	version: '1.21.4',
	engine: 'PaperMC',
	ram_gb: 4,
	tunnel_enabled: localStorage.getItem('freeplay-tunnel-enabled') !== 'false',
	public_ip: 'Not Active',
	local_port: 25565,
	motd: 'A FreePlay Minecraft Server',
	uptime_seconds: 0,
	cpu_percent: 0,
	ram_used_mb: 0,
	players: [],
	logs: [],
	claim_url: null,
})

const activeTab = ref<
	| 'overview'
	| 'console'
	| 'players'
	| 'properties'
	| 'addons'
	| 'servers'
	| 'files'
	| 'backups'
	| 'settings'
>('overview')
const copied = ref(false)
const copiedTunnelAddr = ref<string | null>(null)
const activeTunnels = ref<Array<{ domain: string; target: string; tunnel_type: string }>>([])
const isTunnelLoading = ref(false)
const showTunnelDebug = ref(false)
const tunnelDebug = ref<{
	rawStatus: string
	rawPublicAddress: string | null
	rawClaimUrl: string | null
	lastError: string | null
	tunnelLogs: string[]
	lastPollTime: string | null
}>({
	rawStatus: 'unknown',
	rawPublicAddress: null,
	rawClaimUrl: null,
	lastError: null,
	tunnelLogs: [],
	lastPollTime: null,
})
const actionLoading = ref(false)
const autoScroll = ref(true)
const logFilter = ref('all')
const commandInput = ref('')
const commandHistory = ref<string[]>([])
const historyIndex = ref(-1)
const terminalLogContainer = ref<HTMLElement | null>(null)

const serverList = ref<ServerEntry[]>([])
const activeServer = ref<ServerEntry | null>(null)
const showCreateModal = ref(false)
const showServerListModal = ref(false)
const isEngineDropdownOpen = ref(false)
const isVersionDropdownOpen = ref(false)

const availableEngines = [
	{
		id: 'PaperMC',
		name: 'PaperMC',
		tag: 'Recommended',
		desc: 'High performance engine with plugin support (Spigot / Paper)',
	},
	{
		id: 'Purpur',
		name: 'Purpur',
		tag: 'Extended',
		desc: 'Fork of Paper with extra configurable gameplay mechanics',
	},
	{
		id: 'Fabric',
		name: 'Fabric',
		tag: 'Mods',
		desc: 'Lightweight, modular modding engine for Fabric server mods',
	},
	{
		id: 'Vanilla',
		name: 'Vanilla',
		tag: 'Official',
		desc: 'Official Mojang server with pure unmodded mechanics',
	},
]

const versionSearchQuery = ref('')

const availableVersions = ref<Array<{ id: string; label: string; tag?: string }>>([
	{ id: '1.21.4', label: '1.21.4', tag: 'Latest' },
	{ id: '1.21.3', label: '1.21.3' },
	{ id: '1.21.1', label: '1.21.1', tag: 'Stable' },
	{ id: '1.21.0', label: '1.21.0' },
	{ id: '1.20.6', label: '1.20.6' },
	{ id: '1.20.4', label: '1.20.4', tag: 'Popular' },
	{ id: '1.20.2', label: '1.20.2' },
	{ id: '1.20.1', label: '1.20.1', tag: 'Modpacks' },
	{ id: '1.19.4', label: '1.19.4', tag: 'Legacy' },
	{ id: '1.19.2', label: '1.19.2' },
	{ id: '1.18.2', label: '1.18.2', tag: 'Caves & Cliffs' },
	{ id: '1.16.5', label: '1.16.5', tag: 'Nether Update' },
	{ id: '1.12.2', label: '1.12.2', tag: 'Classic Mods' },
	{ id: '1.7.10', label: '1.7.10', tag: 'Golden Age' },
])

const filteredVersions = computed(() => {
	const q = versionSearchQuery.value.trim().toLowerCase()
	if (!q) return availableVersions.value
	return availableVersions.value.filter(
		(v) => v.id.toLowerCase().includes(q) || (v.tag && v.tag.toLowerCase().includes(q)),
	)
})

async function loadGameVersions() {
	try {
		const versions = (await get_game_versions().catch(() => [])) as Array<{
			version?: string
			version_type?: string
		}>
		if (Array.isArray(versions) && versions.length > 0) {
			const popularTags: Record<string, string> = {
				'1.21.4': 'Latest',
				'1.21.1': 'Stable',
				'1.20.4': 'Popular',
				'1.20.1': 'Modpacks',
				'1.19.4': 'Legacy',
				'1.18.2': 'Caves & Cliffs',
				'1.16.5': 'Nether Update',
				'1.12.2': 'Classic Mods',
				'1.7.10': 'Golden Age',
			}
			const mapped = versions
				.filter((v) => v.version && (!v.version_type || v.version_type === 'release'))
				.map((v) => ({
					id: v.version as string,
					label: v.version as string,
					tag: v.version ? popularTags[v.version] : undefined,
				}))
			if (mapped.length > 0) {
				availableVersions.value = mapped
			}
		}
	} catch (e) {
		console.debug('Failed to fetch dynamic game versions', e)
	}
}

const ramPresets = [
	{ val: 2, label: '2 GB', tag: 'Lite' },
	{ val: 4, label: '4 GB', tag: 'Standard' },
	{ val: 8, label: '8 GB', tag: 'Balanced' },
	{ val: 12, label: '12 GB', tag: 'Heavy' },
	{ val: 16, label: '16 GB', tag: 'Max' },
]

function toggleEngineDropdown() {
	isEngineDropdownOpen.value = !isEngineDropdownOpen.value
	isVersionDropdownOpen.value = false
}

function toggleVersionDropdown() {
	isVersionDropdownOpen.value = !isVersionDropdownOpen.value
	isEngineDropdownOpen.value = false
}

function selectEngine(engineId: string) {
	newServerForm.value.engine = engineId
	isEngineDropdownOpen.value = false
}

function selectVersion(versionId: string) {
	newServerForm.value.version = versionId
	isVersionDropdownOpen.value = false
}

const newServerForm = ref({
	name: '',
	engine: 'PaperMC',
	version: '1.21.4',
	ram_gb: 4,
	port: 25565,
	custom_path: '',
})

const serverFiles = ref<ServerFileEntry[]>([])
const currentSubDir = ref<string>('')
const dirHistory = ref<string[]>([''])
const dirHistoryIndex = ref(0)

const canGoBack = computed(() => dirHistoryIndex.value > 0)
const canGoForward = computed(() => dirHistoryIndex.value < dirHistory.value.length - 1)
const canGoUp = computed(() => !!currentSubDir.value)

const editingFile = ref<ServerFileEntry | null>(null)
const fileEditorContent = ref('')
const isFileSaving = ref(false)
const fileSaveSuccess = ref(false)

const breadcrumbs = computed(() => {
	if (!currentSubDir.value) return []
	const parts = currentSubDir.value.split('/').filter(Boolean)
	const crumbs: Array<{ name: string; path: string }> = []
	let acc = ''
	for (const part of parts) {
		acc = acc ? `${acc}/${part}` : part
		crumbs.push({ name: part, path: acc })
	}
	return crumbs
})

const backupsList = ref<BackupEntry[]>([])

const quickCommands = [
	'help',
	'list',
	'tps',
	'gamemode creative',
	'gamemode survival',
	'time set day',
	'weather clear',
	'save-all',
	'whitelist on',
	'whitelist off',
]

const overviewIcon = () =>
	h(
		'svg',
		{
			xmlns: 'http://www.w3.org/2000/svg',
			viewBox: '0 0 24 24',
			fill: 'none',
			stroke: 'currentColor',
			strokeWidth: '2',
			strokeLinecap: 'round',
			strokeLinejoin: 'round',
		},
		[
			h('rect', { width: '7', height: '9', x: '3', y: '3', rx: '1' }),
			h('rect', { width: '7', height: '5', x: '14', y: '3', rx: '1' }),
			h('rect', { width: '7', height: '9', x: '14', y: '12', rx: '1' }),
			h('rect', { width: '7', height: '5', x: '3', y: '16', rx: '1' }),
		],
	)

const consoleIcon = () =>
	h(
		'svg',
		{
			xmlns: 'http://www.w3.org/2000/svg',
			viewBox: '0 0 24 24',
			fill: 'none',
			stroke: 'currentColor',
			strokeWidth: '2',
			strokeLinecap: 'round',
			strokeLinejoin: 'round',
		},
		[
			h('polyline', { points: '4 17 10 11 4 5' }),
			h('line', { x1: '12', y1: '19', x2: '20', y2: '19' }),
		],
	)

const serversIcon = () =>
	h(
		'svg',
		{
			xmlns: 'http://www.w3.org/2000/svg',
			viewBox: '0 0 24 24',
			fill: 'none',
			stroke: 'currentColor',
			strokeWidth: '2',
			strokeLinecap: 'round',
			strokeLinejoin: 'round',
		},
		[
			h('rect', { width: '20', height: '8', x: '2', y: '2', rx: '2' }),
			h('rect', { width: '20', height: '8', x: '2', y: '14', rx: '2' }),
			h('line', { x1: '6', x2: '6.01', y1: '6', y2: '6' }),
			h('line', { x1: '6', x2: '6.01', y1: '18', y2: '18' }),
		],
	)

const filesIcon = () =>
	h(
		'svg',
		{
			xmlns: 'http://www.w3.org/2000/svg',
			viewBox: '0 0 24 24',
			fill: 'none',
			stroke: 'currentColor',
			strokeWidth: '2',
			strokeLinecap: 'round',
			strokeLinejoin: 'round',
		},
		[
			h('path', {
				d: 'M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z',
			}),
		],
	)

const backupsIcon = () =>
	h(
		'svg',
		{
			xmlns: 'http://www.w3.org/2000/svg',
			viewBox: '0 0 24 24',
			fill: 'none',
			stroke: 'currentColor',
			strokeWidth: '2',
			strokeLinecap: 'round',
			strokeLinejoin: 'round',
		},
		[
			h('path', { d: 'M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z' }),
			h('polyline', { points: '17 21 17 13 7 13 7 21' }),
			h('polyline', { points: '7 3 7 8 15 8' }),
		],
	)

const settingsIcon = () =>
	h(
		'svg',
		{
			xmlns: 'http://www.w3.org/2000/svg',
			viewBox: '0 0 24 24',
			fill: 'none',
			stroke: 'currentColor',
			strokeWidth: '2',
			strokeLinecap: 'round',
			strokeLinejoin: 'round',
		},
		[
			h('path', {
				d: 'M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z',
			}),
			h('circle', { cx: '12', cy: '12', r: '3' }),
		],
	)

const playersIcon = () =>
	h(
		'svg',
		{
			xmlns: 'http://www.w3.org/2000/svg',
			viewBox: '0 0 24 24',
			fill: 'none',
			stroke: 'currentColor',
			strokeWidth: '2',
			strokeLinecap: 'round',
			strokeLinejoin: 'round',
		},
		[
			h('path', { d: 'M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2' }),
			h('circle', { cx: '9', cy: '7', r: '4' }),
			h('path', { d: 'M22 21v-2a4 4 0 0 0-3-3.87' }),
			h('path', { d: 'M16 3.13a4 4 0 0 1 0 7.75' }),
		],
	)

const propertiesIcon = () =>
	h(
		'svg',
		{
			xmlns: 'http://www.w3.org/2000/svg',
			viewBox: '0 0 24 24',
			fill: 'none',
			stroke: 'currentColor',
			strokeWidth: '2',
			strokeLinecap: 'round',
			strokeLinejoin: 'round',
		},
		[
			h('path', { d: 'M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z' }),
			h('polyline', { points: '14 2 14 8 20 8' }),
			h('line', { x1: '16', y1: '13', x2: '8', y2: '13' }),
			h('line', { x1: '16', y1: '17', x2: '8', y2: '17' }),
			h('polyline', { points: '10 9 9 9 8 9' }),
		],
	)

const showInviteShareModal = ref(false)

const tabs = computed(() => [
	{ id: 'overview' as const, label: 'Overview', icon: overviewIcon },
	{ id: 'console' as const, label: 'Live Console', icon: consoleIcon },
	{
		id: 'players' as const,
		label: `Players (${serverState.value.players?.length ?? 0})`,
		icon: playersIcon,
	},
	{ id: 'properties' as const, label: 'Server Properties', icon: propertiesIcon },
	{ id: 'servers' as const, label: 'Server Instances', icon: serversIcon },
	{ id: 'files' as const, label: 'File Manager', icon: filesIcon },
	{ id: 'backups' as const, label: 'Backups', icon: backupsIcon },
	{ id: 'settings' as const, label: 'Settings & Ports', icon: settingsIcon },
])

const onlinePlayers = computed(() => serverState.value.players.filter((p) => p.online !== false))

const filteredLogs = computed(() => {
	if (logFilter.value === 'all') return serverState.value.logs
	if (logFilter.value === 'info')
		return serverState.value.logs.filter((l) => l.includes('[INFO]') || l.includes('INFO'))
	if (logFilter.value === 'warn')
		return serverState.value.logs.filter((l) => l.includes('[WARN]') || l.includes('WARN'))
	if (logFilter.value === 'error')
		return serverState.value.logs.filter(
			(l) => l.includes('[ERROR]') || l.includes('ERROR') || l.includes('[STDERR]'),
		)
	return serverState.value.logs
})

const formattedUptime = computed(() => {
	const total = serverState.value.uptime_seconds
	const hours = Math.floor(total / 3600)
		.toString()
		.padStart(2, '0')
	const minutes = Math.floor((total % 3600) / 60)
		.toString()
		.padStart(2, '0')
	const seconds = (total % 60).toString().padStart(2, '0')
	return `${hours}:${minutes}:${seconds}`
})

const statusPollInterval: ReturnType<typeof setInterval> | null = null

interface HostStatusIpc {
	server_running?: boolean
	server_status?: string | { running?: { port?: number } }
	status?: 'offline' | 'starting' | 'online' | 'tunneling'
	server_version?: string
	version?: string
	server_type?: string
	engine?: string
	server_ram_mb?: number
	ram_gb?: number
	server_port?: number
	local_port?: number
	public_address?: string
	public_ip?: string
	tunnels?: Array<{ domain: string; target: string; tunnel_type: string }>
	claim_url?: string | null
	tunnel_status?:
		| string
		| { connected?: { public_address?: string } }
		| { claiming?: { claim_url?: string } }
		| { error?: { message?: string } }
	tunnel_logs?: string[]
	server_logs?: string[]
	logs?: string[]
	uptime_seconds?: number
	players?: Array<{
		name: string
		uuid: string
		ip?: string
		joined_at?: number
		is_op?: boolean
		ping?: number
		gamemode?: string
	}>
}

function copyTunnel(domain: string) {
	navigator.clipboard.writeText(domain)
	copiedTunnelAddr.value = domain
	setTimeout(() => {
		copiedTunnelAddr.value = null
	}, 2000)
}

async function fetchStatus() {
	try {
		const res = await invoke<HostStatusIpc>('host_get_status')
		if (res) {
			if (Array.isArray(res.tunnels) && res.tunnels.length > 0) {
				activeTunnels.value = res.tunnels
			}
			let normalizedStatus: 'offline' | 'starting' | 'online' | 'tunneling' = 'offline'
			if (res.server_running) {
				normalizedStatus = 'online'
			} else if (res.server_status) {
				if (typeof res.server_status === 'string') {
					if (res.server_status === 'running') normalizedStatus = 'online'
					else if (res.server_status === 'starting' || res.server_status === 'preparing')
						normalizedStatus = 'starting'
					else normalizedStatus = 'offline'
				} else if (res.server_status.running) {
					normalizedStatus = 'online'
				}
			} else if (res.status) {
				normalizedStatus = res.status
			}

			serverState.value.status = normalizedStatus
			if (typeof res.uptime_seconds === 'number') {
				serverState.value.uptime_seconds = res.uptime_seconds
			}
			if (res.server_version || res.version)
				serverState.value.version = res.server_version || res.version || '1.21.4'
			if (res.server_type || res.engine)
				serverState.value.engine = res.server_type || res.engine || 'PaperMC'
			if (res.server_ram_mb && res.server_ram_mb >= 1024)
				serverState.value.ram_gb = Math.round(res.server_ram_mb / 1024)
			else if (res.ram_gb && res.ram_gb >= 1) serverState.value.ram_gb = res.ram_gb

			if (res.server_port || res.local_port)
				serverState.value.local_port = res.server_port || res.local_port || 25565

			if (Array.isArray(res.players)) {
				serverState.value.players = res.players
			}

			// Parse tunnel_status — Rust PlayitAgentStatus is an externally-tagged serde enum:
			//   unit variants → string:  "stopped", "downloading", "starting"
			//   struct variants → object: {"connected": {"public_address": "..."}},
			//                              {"claiming": {"claim_url": "..."}},
			//                              {"error": {"message": "..."}}
			const ts = res.tunnel_status as string | Record<string, Record<string, string>> | undefined
			let tunnelStatusLabel = 'unknown'

			if (ts) {
				if (typeof ts === 'string') {
					tunnelStatusLabel = ts
					const activeStates = ['starting', 'downloading', 'connected']
					if (activeStates.includes(ts)) {
						serverState.value.tunnel_enabled = true
					}
				} else if (typeof ts === 'object') {
					if ('connected' in ts) {
						tunnelStatusLabel = 'connected'
						const connectedAddr = ts.connected?.public_address
						if (connectedAddr) {
							serverState.value.public_ip = connectedAddr
							isTunnelLoading.value = false
						}
						serverState.value.tunnel_enabled = true
						serverState.value.claim_url = null
					} else if ('claiming' in ts) {
						tunnelStatusLabel = 'claiming'
						serverState.value.claim_url = ts.claiming?.claim_url || null
						serverState.value.tunnel_enabled = true
						isTunnelLoading.value = false
					} else if ('error' in ts) {
						tunnelStatusLabel = `error: ${ts.error?.message || 'unknown'}`
						tunnelDebug.value.lastError = ts.error?.message || 'unknown'
						isTunnelLoading.value = false
					}
				}
			}

			// Also check top-level public_address / public_ip / claim_url fields
			const addr = res.public_address || res.public_ip
			if (addr && addr !== 'Not Active') {
				serverState.value.public_ip = addr
				serverState.value.tunnel_enabled = true
				isTunnelLoading.value = false
			} else if (!serverState.value.tunnel_enabled && !isTunnelLoading.value) {
				serverState.value.public_ip = 'Not Active'
			}

			if (res.claim_url) {
				serverState.value.claim_url = res.claim_url
				isTunnelLoading.value = false
			}

			// Update tunnel debug info
			tunnelDebug.value.rawStatus = tunnelStatusLabel
			tunnelDebug.value.rawPublicAddress = res.public_address || null
			tunnelDebug.value.rawClaimUrl = res.claim_url || null
			tunnelDebug.value.lastPollTime = new Date().toLocaleTimeString()
			if (Array.isArray(res.tunnel_logs)) {
				tunnelDebug.value.tunnelLogs = res.tunnel_logs.slice(-20)
			}

			const combinedLogs: string[] = []
			if (Array.isArray(res.tunnel_logs)) combinedLogs.push(...res.tunnel_logs)
			if (Array.isArray(res.server_logs)) combinedLogs.push(...res.server_logs)
			const incomingLogs =
				combinedLogs.length > 0
					? combinedLogs
					: Array.isArray(res.logs) && res.logs.length > 0
						? res.logs
						: []

			if (incomingLogs.length > 0) {
				const currentLogs = serverState.value.logs
				if (
					incomingLogs.length !== currentLogs.length ||
					incomingLogs[incomingLogs.length - 1] !== currentLogs[currentLogs.length - 1]
				) {
					serverState.value.logs = incomingLogs
					scrollToBottom()
				}
			}
		}
	} catch (e) {
		console.debug('Failed to get host status from IPC', e)
	}
}

async function loadServerList() {
	try {
		const list = await invoke<ServerEntry[]>('host_list_servers')
		if (Array.isArray(list) && list.length > 0) {
			serverList.value = list
			if (!activeServer.value) {
				const savedActiveId = localStorage.getItem('freeplay-active-server-id')
				const matched = savedActiveId ? list.find((s) => s.id === savedActiveId) : list[0]
				activeServer.value = matched || list[0]
				serverState.value.ram_gb = activeServer.value.ram_gb || 4
				serverState.value.version = activeServer.value.version || '1.21.4'
				serverState.value.engine = activeServer.value.engine || 'PaperMC'
				serverState.value.local_port = activeServer.value.port || 25565
				try {
					await invoke('host_select_server', {
						serverId: activeServer.value.id,
						server_id: activeServer.value.id,
					})
				} catch {
					// ignore
				}
			}
		} else {
			// default server entry
			const defaultEntry: ServerEntry = {
				id: 'default',
				name: 'Default Server',
				path: 'servers/default',
				engine: 'PaperMC',
				version: '1.21.4',
				port: 25565,
				ram_gb: 4,
			}
			serverList.value = [defaultEntry]
			activeServer.value = defaultEntry
		}
	} catch (e) {
		console.debug('Load server list error', e)
	}
}

async function selectServer(server: ServerEntry) {
	activeServer.value = server
	localStorage.setItem('freeplay-active-server-id', server.id)
	serverState.value.ram_gb = server.ram_gb || 4
	serverState.value.version = server.version || '1.21.4'
	serverState.value.engine = server.engine || 'PaperMC'
	serverState.value.local_port = server.port || 25565
	try {
		await invoke('host_select_server', { serverId: server.id, server_id: server.id })
		await fetchServerFiles()
		await fetchBackups()
	} catch (e) {
		console.debug('Failed to select server', e)
	}
	await fetchStatus()
}

async function submitCreateServer() {
	if (!newServerForm.value.name.trim()) return
	try {
		const res = await invoke<ServerEntry>('host_create_server', {
			name: newServerForm.value.name.trim(),
			engine: newServerForm.value.engine,
			version: newServerForm.value.version.trim() || '1.21.4',
			port: Number(newServerForm.value.port) || 25565,
			ramGb: Number(newServerForm.value.ram_gb) || 4,
			ram_gb: Number(newServerForm.value.ram_gb) || 4,
			customPath: newServerForm.value.custom_path ? newServerForm.value.custom_path.trim() : null,
			custom_path: newServerForm.value.custom_path ? newServerForm.value.custom_path.trim() : null,
		})
		if (res) {
			serverList.value.push(res)
			await selectServer(res)
		}
		showCreateModal.value = false
		newServerForm.value.name = ''
		newServerForm.value.custom_path = ''
	} catch (e) {
		console.error('Failed to create server', e)
	}
}

function handleSelectServerModal(s: ServerEntry) {
	selectServer(s)
	showServerListModal.value = false
}

function handleOpenCreateModalFromServerList() {
	showServerListModal.value = false
	showCreateModal.value = true
}

async function browseCustomPath() {
	try {
		const selected = await open({
			directory: true,
			multiple: false,
			title: 'Select Minecraft Server Directory',
		})
		if (typeof selected === 'string') {
			newServerForm.value.custom_path = selected
		}
	} catch (e) {
		console.debug('Folder picker cancelled or unsupported', e)
	}
}

async function deleteServer(serverId: string) {
	try {
		await invoke('host_delete_server', { serverId: serverId, server_id: serverId })
		serverList.value = serverList.value.filter((s) => s.id !== serverId)
		if (activeServer.value?.id === serverId && serverList.value.length > 0) {
			await selectServer(serverList.value[0])
		}
	} catch (e) {
		console.error('Failed to delete server', e)
	}
}

function scrollToBottom() {
	if (!autoScroll.value) return
	nextTick(() => {
		if (terminalLogContainer.value) {
			terminalLogContainer.value.scrollTop = terminalLogContainer.value.scrollHeight
		}
	})
}

async function copyPublicIp() {
	let ip = ''
	if (
		serverState.value.tunnel_enabled &&
		serverState.value.public_ip &&
		serverState.value.public_ip !== 'Not Active'
	) {
		ip = serverState.value.public_ip
	} else {
		if (!serverState.value.tunnel_enabled) {
			toggleTunnel()
		}
		ip = `127.0.0.1:${serverState.value.local_port || 25565}`
	}
	try {
		if (navigator.clipboard && navigator.clipboard.writeText) {
			await navigator.clipboard.writeText(ip)
		} else {
			const el = document.createElement('textarea')
			el.value = ip
			document.body.appendChild(el)
			el.select()
			document.execCommand('copy')
			document.body.removeChild(el)
		}
		copied.value = true
		setTimeout(() => {
			copied.value = false
		}, 2000)
	} catch {
		copied.value = true
		setTimeout(() => {
			copied.value = false
		}, 2000)
	}
}

async function startServer() {
	actionLoading.value = true
	serverState.value.status = 'starting'
	const time = new Date().toLocaleTimeString()
	serverState.value.logs.push(
		`[${time}] [Server thread/INFO]: Starting server core (${serverState.value.engine} ${serverState.value.version})...`,
	)
	try {
		await invoke('host_start_server', {
			version: serverState.value.version,
			serverType: serverState.value.engine,
			server_type: serverState.value.engine,
			ramMb: serverState.value.ram_gb * 1024,
			ram_mb: serverState.value.ram_gb * 1024,
			port: serverState.value.local_port,
		})
		if (serverState.value.tunnel_enabled) {
			isTunnelLoading.value = true
			try {
				const res = await invoke<HostStatusIpc>('host_start_tunnel', {
					port: serverState.value.local_port,
				})
				if (res) {
					const addr = res.public_address || res.public_ip
					if (addr && addr !== 'Not Active') {
						serverState.value.public_ip = addr
						isTunnelLoading.value = false
					}
					if (res.claim_url) {
						serverState.value.claim_url = res.claim_url
						isTunnelLoading.value = false
					}
				}
			} catch (te) {
				console.debug('Auto-start tunnel error', te)
			}
			// Poll aggressively for tunnel address resolution
			for (let i = 0; i < 10; i++) {
				await fetchStatus()
				if (serverState.value.public_ip && serverState.value.public_ip !== 'Not Active') {
					break
				}
				await new Promise((r) => setTimeout(r, 2000))
			}
			isTunnelLoading.value = false
		}
		await fetchStatus()
	} catch (e: unknown) {
		const errStr = e ? String(e) : 'Unknown error'
		serverState.value.logs.push(`[ERROR] Server start failed: ${errStr}`)
		serverState.value.status = 'offline'
	} finally {
		actionLoading.value = false
	}
}

async function stopServer() {
	actionLoading.value = true
	try {
		await invoke('host_stop_server')
		await fetchStatus()
	} catch (e: unknown) {
		serverState.value.logs.push(`[WARN] Stop server error: ${String(e)}`)
		serverState.value.status = 'offline'
	} finally {
		actionLoading.value = false
	}
}

async function killServer() {
	actionLoading.value = true
	try {
		await invoke('host_kill_server')
		await fetchStatus()
	} catch (e: unknown) {
		serverState.value.logs.push(`[WARN] Kill server: ${String(e)}`)
		serverState.value.status = 'offline'
	} finally {
		actionLoading.value = false
	}
}

async function restartServer() {
	actionLoading.value = true
	serverState.value.status = 'starting'
	try {
		await invoke('host_stop_server')
	} catch {
		// ignore
	}
	try {
		await invoke('host_start_server', {
			version: serverState.value.version,
			serverType: serverState.value.engine,
			server_type: serverState.value.engine,
			ramMb: serverState.value.ram_gb * 1024,
			ram_mb: serverState.value.ram_gb * 1024,
			port: serverState.value.local_port,
		})
		if (serverState.value.tunnel_enabled) {
			isTunnelLoading.value = true
			try {
				const res = await invoke<HostStatusIpc>('host_start_tunnel', {
					port: serverState.value.local_port,
				})
				if (res) {
					const addr = res.public_address || res.public_ip
					if (addr && addr !== 'Not Active') {
						serverState.value.public_ip = addr
						isTunnelLoading.value = false
					}
				}
			} catch (te) {
				console.debug('Auto restart tunnel error', te)
			}
			for (let i = 0; i < 10; i++) {
				await fetchStatus()
				if (serverState.value.public_ip && serverState.value.public_ip !== 'Not Active') {
					break
				}
				await new Promise((r) => setTimeout(r, 2000))
			}
			isTunnelLoading.value = false
		}
		await fetchStatus()
	} catch (e: unknown) {
		serverState.value.logs.push(`[ERROR] Restart failed: ${String(e)}`)
		serverState.value.status = 'offline'
	} finally {
		actionLoading.value = false
	}
}

async function openServerFolder() {
	try {
		await invoke('host_open_server_dir')
	} catch (e) {
		console.error('Failed to open server folder', e)
	}
}

async function toggleTunnel() {
	const next = !serverState.value.tunnel_enabled
	serverState.value.tunnel_enabled = next
	localStorage.setItem('freeplay-tunnel-enabled', String(next))
	if (next) {
		isTunnelLoading.value = true
		try {
			const res = await invoke<HostStatusIpc>('host_start_tunnel', {
				port: serverState.value.local_port,
			})
			if (res) {
				const addr = res.public_address || res.public_ip
				if (addr && addr !== 'Not Active') {
					serverState.value.public_ip = addr
					isTunnelLoading.value = false
				}
				if (res.claim_url) {
					serverState.value.claim_url = res.claim_url
					isTunnelLoading.value = false
				}
			}
		} catch (e) {
			console.error('Tunnel start error', e)
		}
		// Active polling while resolving domain
		for (let i = 0; i < 6; i++) {
			await fetchStatus()
			if (serverState.value.public_ip && serverState.value.public_ip !== 'Not Active') {
				break
			}
			await new Promise((r) => setTimeout(r, 1000))
		}
		isTunnelLoading.value = false
	} else {
		isTunnelLoading.value = false
		try {
			await invoke('host_stop_tunnel')
			serverState.value.public_ip = 'Not Active'
			serverState.value.claim_url = null
		} catch (e) {
			console.error('Tunnel stop error', e)
		}
		await fetchStatus()
	}
}

async function _updateVersion(ver: string) {
	serverState.value.version = ver
	if (activeServer.value) {
		activeServer.value.version = ver
		const idx = serverList.value.findIndex((s) => s.id === activeServer.value?.id)
		if (idx >= 0) {
			serverList.value[idx].version = ver
		}
	}
	localStorage.setItem('freeplay-server-version', ver)
	await updateConfig()
}

async function _updateEngine(eng: string) {
	serverState.value.engine = eng
	if (activeServer.value) {
		activeServer.value.engine = eng
		const idx = serverList.value.findIndex((s) => s.id === activeServer.value?.id)
		if (idx >= 0) {
			serverList.value[idx].engine = eng
		}
	}
	localStorage.setItem('freeplay-server-engine', eng)
	await updateConfig()
}

async function setRamGb(val: number) {
	if (val && !isNaN(val)) {
		serverState.value.ram_gb = val
		if (activeServer.value) {
			activeServer.value.ram_gb = val
			const idx = serverList.value.findIndex((s) => s.id === activeServer.value?.id)
			if (idx >= 0) {
				serverList.value[idx].ram_gb = val
			}
		}
		localStorage.setItem('freeplay-server-ram-gb', String(val))
		await updateConfig()
	}
}

async function onRamChange(e: Event) {
	const val = Number((e.target as HTMLInputElement).value)
	await setRamGb(val)
}

async function updateConfig() {
	try {
		await invoke('host_update_config', {
			serverId: activeServer.value?.id,
			server_id: activeServer.value?.id,
			version: serverState.value.version,
			engine: serverState.value.engine,
			ramGb: serverState.value.ram_gb,
			ram_gb: serverState.value.ram_gb,
			motd: serverState.value.motd,
			port: serverState.value.local_port,
		})
	} catch (e) {
		console.debug(e)
	}
}

async function submitCommand() {
	const cmd = commandInput.value.trim()
	if (!cmd) return
	commandHistory.value.push(cmd)
	historyIndex.value = commandHistory.value.length
	commandInput.value = ''
	await executeCommand(cmd)
}

async function sendQuickCommand(cmd: string) {
	await executeCommand(cmd)
}

async function executeCommand(cmd: string) {
	const time = new Date().toLocaleTimeString()
	serverState.value.logs.push(`[${time}] [Console]: > ${cmd}`)
	try {
		await invoke('host_send_command', { command: cmd })
		await fetchStatus()
	} catch (e: unknown) {
		serverState.value.logs.push(`[${time}] [Server thread/WARN]: Command dispatch: ${String(e)}`)
	}
	scrollToBottom()
}

function navigateHistory(direction: number) {
	if (commandHistory.value.length === 0) return
	historyIndex.value = Math.max(
		0,
		Math.min(commandHistory.value.length, historyIndex.value + direction),
	)
	if (historyIndex.value < commandHistory.value.length) {
		commandInput.value = commandHistory.value[historyIndex.value]
	} else {
		commandInput.value = ''
	}
}

function clearLogs() {
	serverState.value.logs = []
}

function downloadLogs() {
	const text = serverState.value.logs.join('\n')
	const blob = new Blob([text], { type: 'text/plain;charset=utf-8' })
	const url = URL.createObjectURL(blob)
	const link = document.createElement('a')
	link.href = url
	link.download = `freeplay-server-${Date.now()}.log`
	link.click()
	URL.revokeObjectURL(url)
}

async function navigateToDir(relPath: string, pushHistory = true) {
	const clean = relPath.replace(/^\/+/, '')
	if (pushHistory && clean !== currentSubDir.value) {
		dirHistory.value = dirHistory.value.slice(0, dirHistoryIndex.value + 1)
		dirHistory.value.push(clean)
		dirHistoryIndex.value = dirHistory.value.length - 1
	}
	currentSubDir.value = clean
	await fetchServerFiles()
}

async function goBack() {
	if (!canGoBack.value) return
	dirHistoryIndex.value--
	const target = dirHistory.value[dirHistoryIndex.value]
	await navigateToDir(target, false)
}

async function goForward() {
	if (!canGoForward.value) return
	dirHistoryIndex.value++
	const target = dirHistory.value[dirHistoryIndex.value]
	await navigateToDir(target, false)
}

async function navigateUp() {
	if (!canGoUp.value) return
	const parts = currentSubDir.value.split('/').filter(Boolean)
	parts.pop()
	const parent = parts.join('/')
	await navigateToDir(parent, true)
}

async function fetchServerFiles() {
	try {
		const files = await invoke<ServerFileEntry[]>('host_get_files', {
			path: currentSubDir.value || null,
		})
		if (Array.isArray(files)) {
			serverFiles.value = files
		}
	} catch (e) {
		console.debug('Failed to get server files', e)
	}
}

async function openFileEditor(file: ServerFileEntry) {
	if (file.is_dir) return
	editingFile.value = file
	fileSaveSuccess.value = false
	try {
		const content = await invoke<string>('host_read_file', { path: file.path })
		fileEditorContent.value = content || ''
	} catch (e) {
		fileEditorContent.value = `# Unable to load file: ${String(e)}`
	}
}

async function saveFileEditor() {
	if (!editingFile.value) return
	isFileSaving.value = true
	fileSaveSuccess.value = false
	try {
		await invoke('host_save_file', {
			path: editingFile.value.path,
			content: fileEditorContent.value,
		})
		fileSaveSuccess.value = true
		setTimeout(() => {
			fileSaveSuccess.value = false
		}, 2500)
		await fetchServerFiles()
	} catch (e) {
		console.error('Failed to save file', e)
	} finally {
		isFileSaving.value = false
	}
}

function closeFileEditor() {
	editingFile.value = null
	fileSaveSuccess.value = false
}

async function fetchBackups() {
	try {
		const backups = await invoke<BackupEntry[]>('host_get_backups')
		if (Array.isArray(backups)) {
			backupsList.value = backups
		}
	} catch (e) {
		console.debug('Failed to fetch backups', e)
	}
}

async function createBackup() {
	const now = new Date()
	const name = `world_backup_${now.toISOString().split('T')[0]}_${Date.now()}.zip`
	try {
		const entry = await invoke<BackupEntry>('host_create_backup', { name })
		if (entry) {
			backupsList.value.unshift(entry)
		}
	} catch (e: unknown) {
		serverState.value.logs.push(`[ERROR] Backup creation failed: ${String(e)}`)
	}
}

async function restoreBackup(backup: BackupEntry) {
	try {
		await invoke('host_restore_backup', {
			backupId: backup.id || backup.name,
			backup_id: backup.id || backup.name,
		})
		serverState.value.logs.push(
			`[${new Date().toLocaleTimeString()}] [Server thread/INFO]: World restored from ${backup.name}`,
		)
	} catch (e: unknown) {
		serverState.value.logs.push(`[ERROR] Restore backup failed: ${String(e)}`)
	}
}

function switchTab(tabId: typeof activeTab.value) {
	activeTab.value = tabId
	if (tabId === 'files') fetchServerFiles()
	if (tabId === 'backups') fetchBackups()
	if (tabId === 'servers') loadServerList()
}

function formatFileSize(bytes: number): string {
	if (!bytes) return '0 B'
	if (bytes < 1024) return `${bytes} B`
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
	return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

let pollLoopRunning = true

async function fastStatusLoop() {
	while (pollLoopRunning) {
		const isRunning =
			serverState.value.status === 'online' ||
			serverState.value.status === 'starting' ||
			isTunnelLoading.value
		const delay = isRunning || activeTab.value === 'console' ? 250 : 1500

		try {
			await fetchStatus()
		} catch (err) {
			console.debug('Status loop poll error', err)
		}

		await new Promise((resolve) => setTimeout(resolve, delay))
	}
}

onMounted(async () => {
	await loadGameVersions()
	await loadServerList()
	await fetchStatus()
	pollLoopRunning = true
	void fastStatusLoop()
})

onUnmounted(() => {
	pollLoopRunning = false
	if (statusPollInterval) clearInterval(statusPollInterval)
})
</script>

<style scoped>
.server-control-room {
	animation: fadeIn 0.3s ease-out;
}

@keyframes fadeIn {
	from {
		opacity: 0;
		transform: translateY(6px);
	}
	to {
		opacity: 1;
		transform: translateY(0);
	}
}

.fade-enter-active,
.fade-leave-active {
	transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}
</style>
