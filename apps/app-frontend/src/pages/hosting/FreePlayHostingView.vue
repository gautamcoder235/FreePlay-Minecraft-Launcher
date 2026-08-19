<template>
	<div class="freeplay-control-room min-h-full flex flex-col gap-6 p-6 max-w-7xl mx-auto select-none text-zinc-100 font-sans">
		<!-- Header Hero & Server Status Banner -->
		<div class="relative overflow-hidden rounded-2xl bg-zinc-900/80 border border-white/10 shadow-2xl backdrop-blur-md p-6 lg:p-7 flex flex-col lg:flex-row items-start lg:items-center justify-between gap-6">
			<!-- Background Ambient Glow Accent -->
			<div
				class="absolute -right-24 -top-24 w-96 h-96 rounded-full blur-3xl pointer-events-none transition-colors duration-700 opacity-25"
				:class="{
					'bg-emerald-500': serverState.status === 'online',
					'bg-amber-500': serverState.status === 'starting',
					'bg-cyan-500': serverState.status === 'tunneling',
					'bg-rose-500': serverState.status === 'offline',
				}"
			/>

			<!-- Server Identity & Public IP Section -->
			<div class="flex flex-col gap-3 z-10 max-w-2xl">
				<div class="flex flex-wrap items-center gap-3">
					<h1 class="text-2xl lg:text-3xl font-extrabold tracking-tight text-white m-0 flex items-center gap-2.5">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-7 h-7 text-emerald-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
							<rect width="20" height="8" x="2" y="2" rx="2" ry="2"/>
							<rect width="20" height="8" x="2" y="14" rx="2" ry="2"/>
							<line x1="6" x2="6.01" y1="6" y2="6"/>
							<line x1="6" x2="6.01" y1="18" y2="18"/>
						</svg>
						FreePlay Server Control Room
					</h1>

					<!-- Status Badge -->
					<div
						class="inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider transition-all duration-300 border backdrop-blur-md"
						:class="{
							'bg-emerald-500/15 text-emerald-300 border-emerald-500/40 shadow-[0_0_15px_rgba(16,185,129,0.25)]': serverState.status === 'online',
							'bg-amber-500/15 text-amber-300 border-amber-500/40 animate-pulse': serverState.status === 'starting',
							'bg-cyan-500/15 text-cyan-300 border-cyan-500/40 shadow-[0_0_15px_rgba(6,182,212,0.25)]': serverState.status === 'tunneling',
							'bg-zinc-800/80 text-zinc-400 border-zinc-700': serverState.status === 'offline',
						}"
					>
						<span
							class="w-2 h-2 rounded-full"
							:class="{
								'bg-emerald-400 animate-ping': serverState.status === 'online',
								'bg-amber-400 animate-spin': serverState.status === 'starting',
								'bg-cyan-400 animate-bounce': serverState.status === 'tunneling',
								'bg-zinc-500': serverState.status === 'offline',
							}"
						/>
						{{ serverState.status }}
					</div>
				</div>

				<!-- Public IP Address Join Card -->
				<div class="flex flex-wrap items-center gap-3">
					<div class="flex items-center gap-2.5 bg-zinc-950/80 border border-white/10 rounded-xl px-3.5 py-2 shadow-inner group hover:border-emerald-500/40 transition-colors duration-200">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-emerald-400 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="12" cy="12" r="10"/>
							<path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/>
							<path d="M2 12h20"/>
						</svg>
						<span class="text-xs font-semibold text-zinc-400 uppercase tracking-wide">Public IP:</span>
						<code class="font-mono text-sm font-bold text-white tracking-wide select-all">
							{{ serverState.tunnel_enabled ? serverState.public_ip : `127.0.0.1:${serverState.local_port}` }}
						</code>
						<button
							type="button"
							class="inline-flex items-center justify-center p-1.5 rounded-lg bg-zinc-800 hover:bg-emerald-500 text-zinc-300 hover:text-zinc-950 transition-all duration-200 cursor-pointer border-none active:scale-95 focus-visible:ring-2 focus-visible:ring-emerald-500 focus-visible:outline-none"
							title="1-Click Copy Public IP for Friends"
							@click="copyPublicIp"
						>
							<svg v-if="!copied" xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
								<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
							</svg>
							<svg v-else xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-zinc-950" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
								<polyline points="20 6 9 17 4 12"/>
							</svg>
						</button>
					</div>

					<transition name="fade">
						<span v-if="copied" class="text-xs font-bold text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 px-2.5 py-1 rounded-lg">
							Copied to clipboard!
						</span>
					</transition>

					<div class="flex items-center gap-1.5 text-xs text-zinc-400 bg-zinc-950/40 px-3 py-1.5 rounded-xl border border-white/5">
						<span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
						<span>Ping: <strong class="text-white font-mono">18 ms</strong></span>
					</div>
				</div>
			</div>

			<!-- Quick Server Action Controls -->
			<div class="flex flex-wrap items-center gap-3 z-10 w-full lg:w-auto shrink-0">
				<!-- Start Button -->
				<button
					v-if="serverState.status === 'offline'"
					type="button"
					class="flex-1 lg:flex-none inline-flex items-center justify-center gap-2.5 px-6 py-3 rounded-xl bg-gradient-to-r from-emerald-600 to-emerald-500 hover:from-emerald-500 hover:to-emerald-400 text-zinc-950 font-extrabold text-sm shadow-lg shadow-emerald-950/50 hover:shadow-emerald-500/20 active:scale-[0.98] transition-all duration-200 cursor-pointer border-none focus-visible:ring-2 focus-visible:ring-emerald-400 focus-visible:outline-none"
					:disabled="actionLoading"
					@click="startServer"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
						<path d="M8 5v14l11-7z"/>
					</svg>
					<span>Start Server</span>
				</button>

				<!-- Stop Button -->
				<button
					v-else
					type="button"
					class="flex-1 lg:flex-none inline-flex items-center justify-center gap-2.5 px-6 py-3 rounded-xl bg-rose-500/15 hover:bg-rose-500/25 text-rose-300 font-extrabold text-sm border border-rose-500/40 shadow-lg shadow-rose-950/40 active:scale-[0.98] transition-all duration-200 cursor-pointer focus-visible:ring-2 focus-visible:ring-rose-500 focus-visible:outline-none"
					:disabled="actionLoading"
					@click="stopServer"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
						<rect x="6" y="6" width="12" height="12" rx="1.5"/>
					</svg>
					<span>Stop Server</span>
				</button>

				<!-- Restart Button -->
				<button
					type="button"
					class="inline-flex items-center justify-center gap-2 px-4.5 py-3 rounded-xl bg-zinc-800/80 hover:bg-zinc-700 text-zinc-100 font-semibold text-sm border border-zinc-700 hover:border-zinc-600 active:scale-[0.98] transition-all duration-200 cursor-pointer focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:outline-none"
					:disabled="serverState.status === 'offline' || actionLoading"
					@click="restartServer"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-indigo-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
						<path d="M3 3v5h5"/>
						<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/>
						<path d="M16 21h5v-5"/>
					</svg>
					<span>Restart</span>
				</button>

				<!-- Open Folder Button -->
				<button
					type="button"
					class="inline-flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 hover:text-white font-semibold text-sm border border-zinc-700 hover:border-zinc-600 active:scale-[0.98] transition-all duration-200 cursor-pointer focus-visible:ring-2 focus-visible:ring-amber-500 focus-visible:outline-none"
					title="Open Local Server Folder"
					@click="openServerFolder"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-amber-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>
					</svg>
					<span>Folder</span>
				</button>
			</div>
		</div>

		<!-- Metrics Dashboard Cards Grid -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
			<!-- CPU Load % Card -->
			<div class="p-5 rounded-2xl bg-zinc-900/70 border border-white/5 hover:border-white/10 transition-all duration-200 flex flex-col gap-2.5 shadow-lg backdrop-blur-sm">
				<div class="flex items-center justify-between">
					<span class="text-xs font-semibold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-indigo-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<rect width="16" height="16" x="4" y="4" rx="2"/>
							<rect width="6" height="6" x="9" y="9" rx="1"/>
							<path d="M15 2v2"/><path d="M15 20v2"/><path d="M2 15h2"/><path d="M2 9h2"/><path d="M20 15h2"/><path d="M20 9h2"/><path d="M9 2v2"/><path d="M9 20v2"/>
						</svg>
						CPU Load
					</span>
					<span class="text-xs text-indigo-400 font-mono font-bold bg-indigo-500/10 px-2 py-0.5 rounded-md border border-indigo-500/20">4 Cores</span>
				</div>
				<div class="flex items-baseline justify-between">
					<span class="text-2xl font-black text-white font-mono tracking-tight">
						{{ serverState.status === 'online' ? `${serverState.cpu_percent}%` : '0%' }}
					</span>
					<span class="text-xs text-zinc-400">Target &lt; 85%</span>
				</div>
				<div class="w-full bg-zinc-800 rounded-full h-2 overflow-hidden p-0.5 border border-white/5">
					<div
						class="bg-gradient-to-r from-indigo-500 to-cyan-400 h-full rounded-full transition-all duration-500 ease-out shadow-[0_0_8px_rgba(99,102,241,0.5)]"
						:style="{ width: `${serverState.status === 'online' ? serverState.cpu_percent : 0}%` }"
					/>
				</div>
			</div>

			<!-- RAM Utilization Slider Card -->
			<div class="p-5 rounded-2xl bg-zinc-900/70 border border-white/5 hover:border-white/10 transition-all duration-200 flex flex-col gap-2.5 shadow-lg backdrop-blur-sm">
				<div class="flex items-center justify-between">
					<span class="text-xs font-semibold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-emerald-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M6 19v-3"/><path d="M10 19v-3"/><path d="M14 19v-3"/><path d="M18 19v-3"/><path d="M8 11V9"/><path d="M16 11V9"/><rect width="18" height="12" x="3" y="4" rx="2"/>
						</svg>
						RAM Utilization
					</span>
					<span class="text-xs text-zinc-400 font-mono">Max {{ serverState.ram_gb }} GB</span>
				</div>
				<div class="flex items-baseline justify-between">
					<span class="text-2xl font-black text-white font-mono tracking-tight">
						{{ serverState.status === 'online' ? `${(serverState.ram_used_mb / 1024).toFixed(1)} GB` : '0 GB' }}
					</span>
					<span class="text-xs text-emerald-400 font-bold font-mono">
						{{ serverState.status === 'online' ? `${Math.round((serverState.ram_used_mb / (serverState.ram_gb * 1024)) * 100)}%` : '0%' }}
					</span>
				</div>
				<div class="w-full bg-zinc-800 rounded-full h-2 overflow-hidden p-0.5 border border-white/5">
					<div
						class="bg-gradient-to-r from-emerald-500 to-teal-400 h-full rounded-full transition-all duration-500 ease-out shadow-[0_0_8px_rgba(16,185,129,0.5)]"
						:style="{ width: `${serverState.status === 'online' ? (serverState.ram_used_mb / (serverState.ram_gb * 1024)) * 100 : 0}%` }"
					/>
				</div>
			</div>

			<!-- Online Players List Card -->
			<div class="p-5 rounded-2xl bg-zinc-900/70 border border-white/5 hover:border-white/10 transition-all duration-200 flex flex-col gap-2.5 shadow-lg backdrop-blur-sm">
				<div class="flex items-center justify-between">
					<span class="text-xs font-semibold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-cyan-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
							<circle cx="9" cy="7" r="4"/>
							<path d="M22 21v-2a4 4 0 0 0-3-3.87"/>
							<path d="M16 3.13a4 4 0 0 1 0 7.75"/>
						</svg>
						Online Players
					</span>
					<span class="text-xs font-bold text-emerald-400 flex items-center gap-1">
						<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
						Active
					</span>
				</div>
				<div class="flex items-baseline justify-between">
					<span class="text-2xl font-black text-white font-mono tracking-tight">
						{{ serverState.status === 'online' ? onlinePlayers.length : 0 }}
						<span class="text-xs font-normal text-zinc-400">/ 20 Max</span>
					</span>
				</div>
				<!-- Player Avatar Heads Preview Stack -->
				<div class="flex items-center gap-1.5 overflow-x-auto py-0.5">
					<template v-if="serverState.status === 'online' && onlinePlayers.length > 0">
						<div
							v-for="p in onlinePlayers"
							:key="p.name"
							class="relative group/avatar cursor-pointer"
							:title="`${p.name} (${p.latency}ms)`"
						>
							<img
								:src="`https://mc-heads.net/avatar/${p.name}/28`"
								:alt="p.name"
								class="w-7 h-7 rounded-lg border border-white/20 bg-zinc-800 object-cover hover:scale-110 transition-transform duration-200 shadow-md"
								@error="(e) => handleAvatarError(e, p.name)"
							/>
							<span
								class="absolute -top-1 -right-1 w-2.5 h-2.5 rounded-full border border-zinc-950"
								:class="p.is_op ? 'bg-amber-400' : 'bg-emerald-400'"
							/>
						</div>
					</template>
					<span v-else class="text-xs text-zinc-500 italic">No players connected</span>
				</div>
			</div>

			<!-- Session Uptime & TPS Card -->
			<div class="p-5 rounded-2xl bg-zinc-900/70 border border-white/5 hover:border-white/10 transition-all duration-200 flex flex-col gap-2.5 shadow-lg backdrop-blur-sm">
				<div class="flex items-center justify-between">
					<span class="text-xs font-semibold text-zinc-400 uppercase tracking-wider flex items-center gap-1.5">
						<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-amber-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="12" cy="12" r="10"/>
							<polyline points="12 6 12 12 16 14"/>
						</svg>
						Session Uptime
					</span>
					<span class="text-xs font-mono font-bold text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded-md border border-emerald-500/20">TPS 20.0</span>
				</div>
				<div class="flex items-baseline justify-between">
					<span class="text-2xl font-black text-white font-mono tracking-tight">
						{{ serverState.status === 'online' ? formattedUptime : '00:00:00' }}
					</span>
				</div>
				<div class="text-xs text-zinc-400 truncate">
					Engine: <strong class="text-zinc-200 font-semibold">{{ serverState.engine }} {{ serverState.version }}</strong>
				</div>
			</div>
		</div>

		<!-- Main 2-Column Grid: Configuration & Console -->
		<div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
			<!-- Left Column: Server Configuration & Connected Players (5 cols) -->
			<div class="lg:col-span-5 flex flex-col gap-6">
				<!-- Server Configuration Card -->
				<div class="p-6 rounded-2xl bg-zinc-900/80 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-5">
					<div class="flex items-center justify-between border-b border-white/10 pb-4">
						<h2 class="text-base font-extrabold text-white m-0 flex items-center gap-2.5">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-emerald-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
								<circle cx="12" cy="12" r="3"/>
							</svg>
							Server Settings
						</h2>
						<span class="text-xs font-semibold px-2.5 py-1 rounded-full bg-zinc-800 text-zinc-300 border border-white/5">Instant Hot-Reload</span>
					</div>

					<!-- Version Selector -->
					<div class="flex flex-col gap-2.5">
						<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider">Minecraft Version</label>
						<div class="grid grid-cols-3 gap-2.5">
							<button
								v-for="ver in ['1.21.4', '1.20.1', '1.16.5']"
								:key="ver"
								type="button"
								class="px-3.5 py-2.5 rounded-xl text-xs font-bold border transition-all duration-200 text-center cursor-pointer active:scale-95 focus-visible:ring-2 focus-visible:ring-emerald-500 focus-visible:outline-none"
								:class="serverState.version === ver
									? 'bg-emerald-500/20 border-emerald-500 text-emerald-300 shadow-md shadow-emerald-950/40'
									: 'bg-zinc-800/60 border-zinc-700/80 text-zinc-300 hover:text-white hover:bg-zinc-800 hover:border-zinc-600'"
								@click="updateVersion(ver)"
							>
								{{ ver }}
								<span v-if="ver === '1.21.4'" class="block text-[9px] font-normal text-emerald-400">Latest</span>
							</button>
						</div>
					</div>

					<!-- Engine Selector -->
					<div class="flex flex-col gap-2.5">
						<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider">Server Engine</label>
						<div class="grid grid-cols-3 gap-2.5">
							<button
								v-for="eng in ['PaperMC', 'Fabric', 'Vanilla']"
								:key="eng"
								type="button"
								class="px-3.5 py-2.5 rounded-xl text-xs font-bold border transition-all duration-200 text-center cursor-pointer active:scale-95 focus-visible:ring-2 focus-visible:ring-emerald-500 focus-visible:outline-none"
								:class="serverState.engine === eng
									? 'bg-emerald-500/20 border-emerald-500 text-emerald-300 shadow-md shadow-emerald-950/40'
									: 'bg-zinc-800/60 border-zinc-700/80 text-zinc-300 hover:text-white hover:bg-zinc-800 hover:border-zinc-600'"
								@click="updateEngine(eng)"
							>
								{{ eng }}
								<span v-if="eng === 'PaperMC'" class="block text-[9px] font-normal text-emerald-400">Fastest</span>
							</button>
						</div>
					</div>

					<!-- Dedicated RAM Allocation Slider (2GB to 8GB) -->
					<div class="flex flex-col gap-2.5">
						<div class="flex items-center justify-between">
							<label class="text-xs font-bold text-zinc-300 uppercase tracking-wider">Dedicated RAM Allocation</label>
							<span class="text-sm font-extrabold text-white font-mono bg-zinc-800 px-2.5 py-0.5 rounded-lg border border-white/10">{{ serverState.ram_gb }} GB Dedicated</span>
						</div>
						<input
							type="range"
							min="2"
							max="8"
							step="1"
							:value="serverState.ram_gb"
							class="w-full accent-emerald-500 cursor-pointer h-2 bg-zinc-800 rounded-lg appearance-none focus:outline-none"
							@input="onRamChange"
						/>
						<div class="flex justify-between text-[10px] text-zinc-400 font-mono px-1">
							<span>2 GB (Lite)</span>
							<span>4 GB (Balanced)</span>
							<span>6 GB</span>
							<span>8 GB (Max)</span>
						</div>
					</div>

					<!-- playit.gg Free Tunnel Toggle -->
					<div class="p-4 rounded-xl bg-zinc-950/60 border border-white/10 flex items-center justify-between gap-4">
						<div class="flex flex-col gap-1">
							<div class="flex items-center gap-2">
								<span class="text-sm font-bold text-white">Free playit.gg Tunnel</span>
								<span class="text-[10px] font-bold px-1.5 py-0.5 rounded bg-cyan-500/20 text-cyan-300 border border-cyan-500/30 uppercase">Anycast</span>
							</div>
							<p class="text-xs text-zinc-400 m-0">Zero port-forwarding required. Friends can join with custom public address.</p>
						</div>

						<!-- Toggle Switch -->
						<button
							type="button"
							class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus-visible:ring-2 focus-visible:ring-emerald-500 focus-visible:outline-none"
							:class="serverState.tunnel_enabled ? 'bg-emerald-500' : 'bg-zinc-700'"
							@click="toggleTunnel"
						>
							<span
								class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-zinc-950 shadow-lg ring-0 transition duration-200 ease-in-out"
								:class="serverState.tunnel_enabled ? 'translate-x-5' : 'translate-x-0'"
							/>
						</button>
					</div>
				</div>

				<!-- Connected Players Card -->
				<div class="p-6 rounded-2xl bg-zinc-900/80 border border-white/10 shadow-xl backdrop-blur-md flex flex-col gap-4">
					<div class="flex items-center justify-between border-b border-white/10 pb-4">
						<div class="flex items-center gap-2.5">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-emerald-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
								<circle cx="9" cy="7" r="4"/>
								<path d="M22 21v-2a4 4 0 0 0-3-3.87"/>
								<path d="M16 3.13a4 4 0 0 1 0 7.75"/>
							</svg>
							<h2 class="text-base font-extrabold text-white m-0">Connected Players</h2>
						</div>
						<span class="text-xs font-semibold px-2.5 py-1 rounded-full bg-zinc-800 text-zinc-300 border border-white/5">
							{{ onlinePlayers.length }} / {{ serverState.players.length }} Online
						</span>
					</div>

					<div class="flex flex-col gap-2.5">
						<div
							v-for="player in serverState.players"
							:key="player.name"
							class="flex items-center justify-between p-3 rounded-xl bg-zinc-950/60 border border-white/5 hover:border-white/15 transition-all duration-200"
						>
							<div class="flex items-center gap-3">
								<div class="relative w-9 h-9 rounded-xl overflow-hidden bg-zinc-800 border border-white/10 shrink-0">
									<img
										:src="`https://mc-heads.net/avatar/${player.name}/36`"
										:alt="player.name"
										class="w-full h-full object-cover"
										@error="(e) => handleAvatarError(e, player.name)"
									/>
								</div>
								<div class="flex flex-col">
									<div class="flex items-center gap-2">
										<span class="text-sm font-bold text-white leading-none">{{ player.name }}</span>
										<span v-if="player.is_op" class="text-[9px] font-extrabold px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-300 border border-amber-500/30 uppercase">OP</span>
									</div>
									<span class="text-[11px] text-zinc-400 mt-1 flex items-center gap-1.5 font-mono">
										<span
											class="w-1.5 h-1.5 rounded-full"
											:class="player.online ? 'bg-emerald-400' : 'bg-zinc-600'"
										/>
										{{ player.online ? `${player.latency}ms ping` : 'Offline' }}
									</span>
								</div>
							</div>

							<!-- Moderation Quick Actions -->
							<div class="flex items-center gap-1.5">
								<button
									type="button"
									class="px-2.5 py-1 rounded-lg text-xs font-bold bg-zinc-800 hover:bg-zinc-700 text-zinc-300 hover:text-white border border-white/5 active:scale-95 transition-all cursor-pointer focus-visible:ring-2 focus-visible:ring-amber-500 focus-visible:outline-none"
									:title="player.is_op ? 'De-op' : 'Make OP'"
									@click="sendQuickCommand(player.is_op ? `/deop ${player.name}` : `/op ${player.name}`)"
								>
									{{ player.is_op ? 'De-op' : 'Op' }}
								</button>
								<button
									v-if="player.online"
									type="button"
									class="px-2.5 py-1 rounded-lg text-xs font-bold bg-rose-500/10 hover:bg-rose-500/25 text-rose-300 border border-rose-500/30 active:scale-95 transition-all cursor-pointer focus-visible:ring-2 focus-visible:ring-rose-500 focus-visible:outline-none"
									title="Kick Player"
									@click="sendQuickCommand(`/kick ${player.name}`)"
								>
									Kick
								</button>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Right Column: Interactive Live Console (7 cols) -->
			<div class="lg:col-span-7 flex flex-col gap-4">
				<div class="rounded-2xl bg-zinc-900/80 border border-white/10 overflow-hidden shadow-2xl backdrop-blur-md flex flex-col h-[700px]">
					<!-- Terminal Header Bar -->
					<div class="bg-zinc-950/90 px-4 py-3.5 border-b border-white/10 flex items-center justify-between shrink-0">
						<div class="flex items-center gap-3">
							<div class="flex items-center gap-1.5">
								<span class="w-3 h-3 rounded-full bg-rose-500/80 inline-block"></span>
								<span class="w-3 h-3 rounded-full bg-amber-500/80 inline-block"></span>
								<span class="w-3 h-3 rounded-full bg-emerald-500/80 inline-block"></span>
							</div>
							<span class="text-xs font-extrabold text-white font-mono flex items-center gap-2 tracking-wide">
								<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-emerald-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
									<polyline points="4 17 10 11 4 5"/>
									<line x1="12" y1="19" x2="20" y2="19"/>
								</svg>
								Interactive Live Console
							</span>
						</div>

						<div class="flex items-center gap-2">
							<button
								type="button"
								class="text-xs px-3 py-1.5 rounded-lg border border-white/10 bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 hover:text-white font-mono cursor-pointer transition-colors focus-visible:ring-2 focus-visible:ring-emerald-500 focus-visible:outline-none"
								:class="{ '!bg-emerald-500/20 !border-emerald-500/50 !text-emerald-300 font-bold': autoScroll }"
								@click="autoScroll = !autoScroll"
							>
								Auto-Scroll: {{ autoScroll ? 'ON' : 'OFF' }}
							</button>
							<button
								type="button"
								class="text-xs px-3 py-1.5 rounded-lg border border-white/10 bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 hover:text-white font-mono cursor-pointer transition-colors focus-visible:ring-2 focus-visible:ring-emerald-500 focus-visible:outline-none"
								@click="clearLogs"
							>
								Clear
							</button>
						</div>
					</div>

					<!-- Terminal Log Viewer -->
					<div
						ref="terminalLogContainer"
						class="flex-1 p-4 bg-[#090d12] overflow-y-auto font-mono text-xs leading-relaxed space-y-1.5 select-text scrollbar-thin scrollbar-thumb-zinc-800"
					>
						<div
							v-for="(log, idx) in serverState.logs"
							:key="idx"
							class="whitespace-pre-wrap break-all transition-colors duration-150"
						>
							<!-- Highlight Syntax -->
							<span v-if="log.includes('[INFO]')" class="text-zinc-300">
								<span class="text-cyan-400 font-semibold">{{ log.substring(0, log.indexOf('[INFO]:') + 7) }}</span>
								<span class="text-zinc-200">{{ log.substring(log.indexOf('[INFO]:') + 7) }}</span>
							</span>
							<span v-else-if="log.includes('[WARN]')" class="text-amber-400 font-medium">
								{{ log }}
							</span>
							<span v-else-if="log.includes('[ERROR]')" class="text-rose-400 font-bold">
								{{ log }}
							</span>
							<span v-else-if="log.includes('[DONE]')" class="text-emerald-400 font-bold">
								{{ log }}
							</span>
							<span v-else-if="log.includes('[Console]')" class="text-emerald-300 font-semibold">
								{{ log }}
							</span>
							<span v-else class="text-zinc-300">
								{{ log }}
							</span>
						</div>
					</div>

					<!-- Quick Command Suggestion Chips -->
					<div class="px-4 py-2.5 bg-zinc-950/80 border-t border-white/10 flex flex-wrap items-center gap-2 shrink-0">
						<span class="text-xs font-bold text-zinc-400 uppercase tracking-wide mr-1 flex items-center gap-1">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-amber-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>
							</svg>
							Quick:
						</span>
						<button
							v-for="cmd in ['/op Alex', '/gamemode creative', '/time set day', '/whitelist off', '/weather clear', '/tps']"
							:key="cmd"
							type="button"
							class="text-xs font-mono px-2.5 py-1 rounded-lg bg-zinc-800/80 hover:bg-emerald-500 hover:text-zinc-950 text-zinc-300 font-semibold transition-all duration-150 cursor-pointer border border-white/5 active:scale-95 focus-visible:ring-2 focus-visible:ring-emerald-500 focus-visible:outline-none"
							@click="sendQuickCommand(cmd)"
						>
							{{ cmd }}
						</button>
					</div>

					<!-- Command Input Bar -->
					<form class="p-3.5 bg-zinc-950 border-t border-white/10 flex items-center gap-3 shrink-0" @submit.prevent="submitCommand">
						<span class="text-emerald-400 font-mono font-black text-base pl-2 select-none">&gt;</span>
						<input
							v-model="commandInput"
							type="text"
							placeholder="Type a server command... (e.g. /gamemode creative)"
							class="flex-1 bg-zinc-900 border border-white/10 focus:border-emerald-500/80 rounded-xl px-4 py-2.5 text-sm text-white font-mono outline-none shadow-inner transition-colors duration-200 focus-visible:ring-2 focus-visible:ring-emerald-500/40"
						/>
						<button
							type="submit"
							class="px-5 py-2.5 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-zinc-950 font-extrabold text-xs shadow-md shadow-emerald-950/50 active:scale-95 transition-all duration-200 cursor-pointer border-none flex items-center gap-2 focus-visible:ring-2 focus-visible:ring-emerald-400 focus-visible:outline-none"
						>
							<span>Send</span>
							<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
								<line x1="22" y1="2" x2="11" y2="13"/>
								<polygon points="22 2 15 22 11 13 2 9 22 2"/>
							</svg>
						</button>
					</form>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, nextTick, onMounted, onUnmounted, ref } from 'vue'

interface Player {
	name: string
	uuid: string
	latency: number
	is_op: boolean
	online: boolean
}

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
	players: Player[]
	logs: string[]
}

const serverState = ref<ServerState>({
	status: 'offline',
	version: '1.21.4',
	engine: 'PaperMC',
	ram_gb: 4,
	tunnel_enabled: true,
	public_ip: 'Not Active',
	local_port: 25565,
	motd: 'A FreePlay Minecraft Server',
	uptime_seconds: 0,
	cpu_percent: 0,
	ram_used_mb: 0,
	players: [],
	logs: ['[INFO] FreePlay Server Control Room ready. Click "Start Server" to launch.'],
})

const copied = ref(false)
const actionLoading = ref(false)
const autoScroll = ref(true)
const commandInput = ref('')
const terminalLogContainer = ref<HTMLElement | null>(null)

const onlinePlayers = computed(() => serverState.value.players.filter((p) => p.online))

const formattedUptime = computed(() => {
	const total = serverState.value.uptime_seconds
	const hours = Math.floor(total / 3600).toString().padStart(2, '0')
	const minutes = Math.floor((total % 3600) / 60).toString().padStart(2, '0')
	const seconds = (total % 60).toString().padStart(2, '0')
	return `${hours}:${minutes}:${seconds}`
})

let uptimeInterval: ReturnType<typeof setInterval> | null = null

async function fetchStatus() {
	try {
		const res = await invoke<ServerState>('host_get_status')
		if (res) {
			serverState.value = res
			scrollToBottom()
		}
	} catch (e) {
		console.error('Failed to get host status', e)
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

function handleAvatarError(event: Event, name: string) {
	const target = event.target as HTMLImageElement
	if (target) {
		target.style.display = 'none'
	}
}

async function copyPublicIp() {
	const ip = serverState.value.tunnel_enabled
		? serverState.value.public_ip
		: `127.0.0.1:${serverState.value.local_port}`
	try {
		await navigator.clipboard.writeText(ip)
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
	try {
		await invoke('host_start_server')
		await fetchStatus()
		setTimeout(async () => {
			await fetchStatus()
			actionLoading.value = false
		}, 900)
	} catch (e) {
		console.error(e)
		actionLoading.value = false
	}
}

async function stopServer() {
	actionLoading.value = true
	try {
		await invoke('host_stop_server')
		await fetchStatus()
		actionLoading.value = false
	} catch (e) {
		console.error(e)
		actionLoading.value = false
	}
}

async function restartServer() {
	actionLoading.value = true
	try {
		await invoke('host_restart_server')
		await fetchStatus()
		setTimeout(async () => {
			await fetchStatus()
			actionLoading.value = false
		}, 1100)
	} catch (e) {
		console.error(e)
		actionLoading.value = false
	}
}

async function openServerFolder() {
	try {
		await invoke('host_open_server_dir')
	} catch (e) {
		console.error(e)
	}
}

async function toggleTunnel() {
	try {
		if (serverState.value.tunnel_enabled) {
			await invoke('host_stop_tunnel')
			serverState.value.tunnel_enabled = false
		} else {
			await invoke('host_start_tunnel')
			serverState.value.tunnel_enabled = true
		}
		await fetchStatus()
	} catch (e) {
		console.error(e)
	}
}

async function updateVersion(ver: string) {
	serverState.value.version = ver
	try {
		await invoke('host_update_config', { version: ver })
	} catch (e) {
		console.error(e)
	}
}

async function updateEngine(eng: string) {
	serverState.value.engine = eng
	try {
		await invoke('host_update_config', { engine: eng })
	} catch (e) {
		console.error(e)
	}
}

async function onRamChange(e: Event) {
	const val = Number((e.target as HTMLInputElement).value)
	serverState.value.ram_gb = val
	try {
		await invoke('host_update_config', { ram_gb: val })
	} catch (err) {
		console.error(err)
	}
}

async function submitCommand() {
	const cmd = commandInput.value.trim()
	if (!cmd) return
	commandInput.value = ''
	try {
		await invoke('host_send_command', { command: cmd })
		await fetchStatus()
	} catch (e) {
		console.error(e)
	}
}

async function sendQuickCommand(cmd: string) {
	try {
		await invoke('host_send_command', { command: cmd })
		await fetchStatus()
	} catch (e) {
		console.error(e)
	}
}

function clearLogs() {
	serverState.value.logs = []
}

onMounted(async () => {
	await fetchStatus()
	uptimeInterval = setInterval(() => {
		if (serverState.value.status === 'online') {
			serverState.value.uptime_seconds++
		}
	}, 1000)
})

onUnmounted(() => {
	if (uptimeInterval) clearInterval(uptimeInterval)
})
</script>

<style scoped>
.freeplay-control-room {
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
