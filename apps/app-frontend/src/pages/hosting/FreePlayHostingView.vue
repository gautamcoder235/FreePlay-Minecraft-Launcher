<template>
	<div class="freeplay-control-room min-h-full flex flex-col gap-6 p-6 max-w-7xl mx-auto select-none">
		<!-- Header Hero & Status -->
		<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 p-6 rounded-2xl bg-surface-2/80 border border-surface-4 shadow-lg backdrop-blur-sm relative overflow-hidden">
			<!-- Background Glow Accent -->
			<div
				class="absolute -right-20 -top-20 w-80 h-80 rounded-full blur-3xl pointer-events-none transition-colors duration-700 opacity-20"
				:class="{
					'bg-emerald-500': serverState.status === 'online',
					'bg-amber-500': serverState.status === 'starting',
					'bg-cyan-500': serverState.status === 'tunneling',
					'bg-rose-500': serverState.status === 'offline',
				}"
			/>

			<!-- Server Identity & Public IP -->
			<div class="flex flex-col gap-2 z-10">
				<div class="flex items-center gap-3">
					<div class="flex items-center gap-2">
						<h1 class="text-2xl font-bold tracking-tight text-contrast m-0 flex items-center gap-2">
							FreePlay Server Control Room
						</h1>
					</div>

					<!-- Status Badge -->
					<div
						class="inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider transition-all duration-300 border"
						:class="{
							'bg-emerald-500/10 text-emerald-400 border-emerald-500/30 shadow-[0_0_12px_rgba(16,185,129,0.2)]': serverState.status === 'online',
							'bg-amber-500/10 text-amber-400 border-amber-500/30 animate-pulse': serverState.status === 'starting',
							'bg-cyan-500/10 text-cyan-400 border-cyan-500/30 shadow-[0_0_12px_rgba(6,182,212,0.2)]': serverState.status === 'tunneling',
							'bg-surface-4 text-secondary border-surface-5': serverState.status === 'offline',
						}"
					>
						<span
							class="w-2 h-2 rounded-full"
							:class="{
								'bg-emerald-400 animate-ping': serverState.status === 'online',
								'bg-amber-400 animate-spin': serverState.status === 'starting',
								'bg-cyan-400 animate-bounce': serverState.status === 'tunneling',
								'bg-secondary': serverState.status === 'offline',
							}"
						/>
						{{ serverState.status }}
					</div>
				</div>

				<!-- Public IP Address Card -->
				<div class="flex flex-wrap items-center gap-3 mt-1">
					<div class="flex items-center gap-2 bg-surface-3/90 border border-surface-4/80 rounded-xl px-3 py-1.5 shadow-inner">
						<span class="text-xs font-medium text-secondary">Public Address:</span>
						<code class="font-mono text-sm font-bold text-contrast tracking-wide">
							{{ serverState.tunnel_enabled ? serverState.public_ip : `127.0.0.1:${serverState.local_port}` }}
						</code>
						<button
							type="button"
							class="ml-1 inline-flex items-center justify-center p-1.5 rounded-lg bg-surface-4 hover:bg-brand hover:text-black text-primary transition-all duration-200 cursor-pointer border-none"
							title="Copy IP for friends"
							@click="copyPublicIp"
						>
							<svg v-if="!copied" xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
								<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
							</svg>
							<svg v-else xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-emerald-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
								<polyline points="20 6 9 17 4 12"/>
							</svg>
						</button>
					</div>

					<span v-if="copied" class="text-xs font-semibold text-emerald-400 transition-opacity">
						Copied to clipboard!
					</span>

					<div class="flex items-center gap-1.5 text-xs text-secondary bg-surface-3/60 px-2.5 py-1 rounded-lg border border-surface-4">
						<span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
						<span>Ping: <strong class="text-contrast">18 ms</strong></span>
					</div>
				</div>
			</div>

			<!-- Quick Server Actions -->
			<div class="flex flex-wrap items-center gap-2.5 z-10 w-full lg:w-auto mt-2 lg:mt-0">
				<button
					v-if="serverState.status === 'offline'"
					type="button"
					class="flex-1 lg:flex-none inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl bg-brand hover:bg-brand-highlight text-black font-bold text-sm shadow-md hover:shadow-brand/20 transition-all cursor-pointer border-none"
					:disabled="actionLoading"
					@click="startServer"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
						<path d="M8 5v14l11-7z"/>
					</svg>
					Start Server
				</button>

				<button
					v-else
					type="button"
					class="flex-1 lg:flex-none inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl bg-rose-500/20 hover:bg-rose-500/30 text-rose-300 font-bold text-sm border border-rose-500/40 transition-all cursor-pointer"
					:disabled="actionLoading"
					@click="stopServer"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
						<rect x="6" y="6" width="12" height="12" rx="1"/>
					</svg>
					Stop Server
				</button>

				<button
					type="button"
					class="inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-surface-3 hover:bg-surface-4 text-primary font-semibold text-sm border border-surface-4 transition-all cursor-pointer"
					:disabled="serverState.status === 'offline' || actionLoading"
					@click="restartServer"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
						<path d="M3 3v5h5"/>
						<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/>
						<path d="M16 21h5v-5"/>
					</svg>
					Restart
				</button>

				<button
					type="button"
					class="inline-flex items-center justify-center gap-2 px-3.5 py-2.5 rounded-xl bg-surface-3 hover:bg-surface-4 text-secondary hover:text-contrast font-medium text-sm border border-surface-4 transition-all cursor-pointer"
					title="Open Server Directory"
					@click="openServerFolder"
				>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>
					</svg>
					Folder
				</button>
			</div>
		</div>

		<!-- Live Metrics Row -->
		<div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
			<!-- CPU Usage Metric -->
			<div class="p-4 rounded-xl bg-surface-2 border border-surface-4 flex flex-col gap-1.5">
				<span class="text-xs text-secondary font-medium">CPU Load</span>
				<div class="flex items-baseline justify-between">
					<span class="text-xl font-bold text-contrast">{{ serverState.status === 'online' ? `${serverState.cpu_percent}%` : '0%' }}</span>
					<span class="text-xs text-brand font-mono">4 Cores</span>
				</div>
				<div class="w-full bg-surface-4 rounded-full h-1.5 mt-1 overflow-hidden">
					<div
						class="bg-brand h-full rounded-full transition-all duration-500"
						:style="{ width: `${serverState.status === 'online' ? serverState.cpu_percent : 0}%` }"
					/>
				</div>
			</div>

			<!-- RAM Allocation Metric -->
			<div class="p-4 rounded-xl bg-surface-2 border border-surface-4 flex flex-col gap-1.5">
				<span class="text-xs text-secondary font-medium">RAM Utilization</span>
				<div class="flex items-baseline justify-between">
					<span class="text-xl font-bold text-contrast font-mono">
						{{ serverState.status === 'online' ? `${(serverState.ram_used_mb / 1024).toFixed(1)} GB` : '0 GB' }}
					</span>
					<span class="text-xs text-secondary">of {{ serverState.ram_gb }} GB</span>
				</div>
				<div class="w-full bg-surface-4 rounded-full h-1.5 mt-1 overflow-hidden">
					<div
						class="bg-emerald-400 h-full rounded-full transition-all duration-500"
						:style="{ width: `${serverState.status === 'online' ? (serverState.ram_used_mb / (serverState.ram_gb * 1024)) * 100 : 0}%` }"
					/>
				</div>
			</div>

			<!-- Active Players Metric -->
			<div class="p-4 rounded-xl bg-surface-2 border border-surface-4 flex flex-col gap-1.5">
				<span class="text-xs text-secondary font-medium">Players Online</span>
				<div class="flex items-baseline justify-between">
					<span class="text-xl font-bold text-contrast">
						{{ serverState.status === 'online' ? onlinePlayers.length : 0 }} <span class="text-xs font-normal text-secondary">/ 20 Max</span>
					</span>
					<span class="text-xs text-emerald-400 font-semibold flex items-center gap-1">
						<span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span> Active
					</span>
				</div>
				<div class="text-xs text-secondary truncate mt-1">
					{{ serverState.status === 'online' && onlinePlayers.length > 0 ? onlinePlayers.map(p => p.name).join(', ') : 'No players currently' }}
				</div>
			</div>

			<!-- Uptime Metric -->
			<div class="p-4 rounded-xl bg-surface-2 border border-surface-4 flex flex-col gap-1.5">
				<span class="text-xs text-secondary font-medium">Session Uptime</span>
				<div class="flex items-baseline justify-between">
					<span class="text-xl font-bold text-contrast font-mono">
						{{ serverState.status === 'online' ? formattedUptime : '00:00:00' }}
					</span>
					<span class="text-xs text-secondary">TPS: <strong class="text-contrast">20.0</strong></span>
				</div>
				<div class="text-xs text-secondary mt-1">
					Engine: <strong class="text-contrast">{{ serverState.engine }} {{ serverState.version }}</strong>
				</div>
			</div>
		</div>

		<!-- Main 2-Column Grid: Configuration & Console -->
		<div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
			<!-- Left Column: Server Configuration & Connected Players (5 cols) -->
			<div class="lg:col-span-5 flex flex-col gap-6">
				<!-- Server Configuration Card -->
				<div class="p-5 rounded-2xl bg-surface-2 border border-surface-4 flex flex-col gap-5 shadow-sm">
					<div class="flex items-center justify-between border-b border-surface-4 pb-3">
						<h2 class="text-base font-bold text-contrast m-0 flex items-center gap-2">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-brand" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
								<circle cx="12" cy="12" r="3"/>
							</svg>
							Server Settings
						</h2>
						<span class="text-xs text-secondary">Instant Hot-Reload</span>
					</div>

					<!-- Version Selector -->
					<div class="flex flex-col gap-2">
						<label class="text-xs font-semibold text-primary uppercase tracking-wider">Minecraft Version</label>
						<div class="grid grid-cols-3 gap-2">
							<button
								v-for="ver in ['1.21.4', '1.20.1', '1.16.5']"
								:key="ver"
								type="button"
								class="px-3 py-2 rounded-xl text-xs font-bold border transition-all text-center cursor-pointer"
								:class="serverState.version === ver
									? 'bg-brand/20 border-brand text-brand shadow-sm'
									: 'bg-surface-3 border-surface-4 text-secondary hover:text-contrast hover:bg-surface-4'"
								@click="updateVersion(ver)"
							>
								{{ ver }}
								<span v-if="ver === '1.21.4'" class="block text-[9px] font-normal text-emerald-400">Latest</span>
							</button>
						</div>
					</div>

					<!-- Engine Selector -->
					<div class="flex flex-col gap-2">
						<label class="text-xs font-semibold text-primary uppercase tracking-wider">Server Engine</label>
						<div class="grid grid-cols-3 gap-2">
							<button
								v-for="eng in ['PaperMC', 'Fabric', 'Vanilla']"
								:key="eng"
								type="button"
								class="px-3 py-2 rounded-xl text-xs font-bold border transition-all text-center cursor-pointer"
								:class="serverState.engine === eng
									? 'bg-brand/20 border-brand text-brand shadow-sm'
									: 'bg-surface-3 border-surface-4 text-secondary hover:text-contrast hover:bg-surface-4'"
								@click="updateEngine(eng)"
							>
								{{ eng }}
								<span v-if="eng === 'PaperMC'" class="block text-[9px] font-normal text-emerald-400">Fastest</span>
							</button>
						</div>
					</div>

					<!-- RAM Allocation Slider -->
					<div class="flex flex-col gap-2">
						<div class="flex items-center justify-between">
							<label class="text-xs font-semibold text-primary uppercase tracking-wider">RAM Allocation</label>
							<span class="text-sm font-bold text-contrast font-mono">{{ serverState.ram_gb }} GB Dedicated</span>
						</div>
						<input
							type="range"
							min="2"
							max="8"
							step="1"
							:value="serverState.ram_gb"
							class="w-full accent-brand cursor-pointer h-2 bg-surface-4 rounded-lg"
							@input="onRamChange"
						/>
						<div class="flex justify-between text-[10px] text-secondary font-mono px-1">
							<span>2 GB (Lite)</span>
							<span>4 GB (Balanced)</span>
							<span>6 GB</span>
							<span>8 GB (Max)</span>
						</div>
					</div>

					<!-- playit.gg Free Tunnel Toggle -->
					<div class="p-4 rounded-xl bg-surface-3/80 border border-surface-4 flex items-center justify-between gap-4">
						<div class="flex flex-col gap-0.5">
							<div class="flex items-center gap-1.5">
								<span class="text-sm font-bold text-contrast">Free playit.gg Tunnel</span>
								<span class="text-[10px] font-bold px-1.5 py-0.2 rounded bg-cyan-500/20 text-cyan-400 uppercase">Anycast</span>
							</div>
							<p class="text-xs text-secondary m-0">Zero port-forwarding required. Friends can join with custom public address.</p>
						</div>

						<!-- Toggle Switch -->
						<button
							type="button"
							class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none"
							:class="serverState.tunnel_enabled ? 'bg-brand' : 'bg-surface-4'"
							@click="toggleTunnel"
						>
							<span
								class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-black shadow-lg ring-0 transition duration-200 ease-in-out"
								:class="serverState.tunnel_enabled ? 'translate-x-5' : 'translate-x-0'"
							/>
						</button>
					</div>
				</div>

				<!-- Connected Players Card -->
				<div class="p-5 rounded-2xl bg-surface-2 border border-surface-4 flex flex-col gap-4 shadow-sm">
					<div class="flex items-center justify-between border-b border-surface-4 pb-3">
						<div class="flex items-center gap-2">
							<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-brand" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
								<circle cx="9" cy="7" r="4"/>
								<path d="M22 21v-2a4 4 0 0 0-3-3.87"/>
								<path d="M16 3.13a4 4 0 0 1 0 7.75"/>
							</svg>
							<h2 class="text-base font-bold text-contrast m-0">Connected Players</h2>
						</div>
						<span class="text-xs font-semibold px-2 py-0.5 rounded-full bg-surface-3 text-secondary">
							{{ onlinePlayers.length }} / {{ serverState.players.length }} Online
						</span>
					</div>

					<div class="flex flex-col gap-2">
						<div
							v-for="player in serverState.players"
							:key="player.name"
							class="flex items-center justify-between p-2.5 rounded-xl bg-surface-3/70 border border-surface-4/60 hover:bg-surface-3 transition-colors"
						>
							<div class="flex items-center gap-3">
								<div class="relative w-8 h-8 rounded-lg overflow-hidden bg-surface-4 border border-surface-5">
									<img
										:src="`https://mc-heads.net/avatar/${player.name}/32`"
										:alt="player.name"
										class="w-full h-full object-cover"
										@error="(e) => (e.target as HTMLElement).style.display = 'none'"
									/>
								</div>
								<div class="flex flex-col">
									<div class="flex items-center gap-1.5">
										<span class="text-sm font-semibold text-contrast leading-none">{{ player.name }}</span>
										<span v-if="player.is_op" class="text-[9px] font-bold px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-300 uppercase">OP</span>
									</div>
									<span class="text-[11px] text-secondary mt-0.5 flex items-center gap-1">
										<span
											class="w-1.5 h-1.5 rounded-full"
											:class="player.online ? 'bg-emerald-400' : 'bg-surface-5'"
										/>
										{{ player.online ? `${player.latency}ms ping` : 'Offline' }}
									</span>
								</div>
							</div>

							<!-- Moderation Quick Actions -->
							<div class="flex items-center gap-1">
								<button
									type="button"
									class="px-2 py-1 rounded text-xs font-semibold bg-surface-4 hover:bg-surface-5 text-secondary hover:text-contrast border-none cursor-pointer"
									:title="player.is_op ? 'De-op' : 'Make OP'"
									@click="sendQuickCommand(player.is_op ? `/deop ${player.name}` : `/op ${player.name}`)"
								>
									{{ player.is_op ? 'De-op' : 'Op' }}
								</button>
								<button
									v-if="player.online"
									type="button"
									class="px-2 py-1 rounded text-xs font-semibold bg-surface-4 hover:bg-rose-500/20 text-secondary hover:text-rose-300 border-none cursor-pointer"
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
				<div class="rounded-2xl bg-surface-2 border border-surface-4 overflow-hidden shadow-lg flex flex-col h-[680px]">
					<!-- Terminal Header Bar -->
					<div class="bg-surface-3/90 px-4 py-3 border-b border-surface-4 flex items-center justify-between">
						<div class="flex items-center gap-2">
							<div class="flex items-center gap-1.5">
								<span class="w-3 h-3 rounded-full bg-rose-500/80 inline-block"></span>
								<span class="w-3 h-3 rounded-full bg-amber-500/80 inline-block"></span>
								<span class="w-3 h-3 rounded-full bg-emerald-500/80 inline-block"></span>
							</div>
							<span class="text-xs font-bold text-contrast ml-2 font-mono flex items-center gap-1.5">
								<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-brand" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<polyline points="4 17 10 11 4 5"/>
									<line x1="12" y1="19" x2="20" y2="19"/>
								</svg>
								Live Interactive Console
							</span>
						</div>

						<div class="flex items-center gap-2">
							<button
								type="button"
								class="text-xs px-2.5 py-1 rounded-lg border border-surface-4 bg-surface-3 hover:bg-surface-4 text-secondary hover:text-contrast cursor-pointer"
								:class="{ '!bg-brand/20 !border-brand !text-brand': autoScroll }"
								@click="autoScroll = !autoScroll"
							>
								Auto-Scroll: {{ autoScroll ? 'ON' : 'OFF' }}
							</button>
							<button
								type="button"
								class="text-xs px-2.5 py-1 rounded-lg border border-surface-4 bg-surface-3 hover:bg-surface-4 text-secondary hover:text-contrast cursor-pointer"
								@click="clearLogs"
							>
								Clear
							</button>
						</div>
					</div>

					<!-- Terminal Log Viewer -->
					<div
						ref="terminalLogContainer"
						class="flex-1 p-4 bg-[#0a0f14] overflow-y-auto font-mono text-xs leading-relaxed space-y-1 select-text scrollbar-thin scrollbar-thumb-surface-4"
					>
						<div
							v-for="(log, idx) in serverState.logs"
							:key="idx"
							class="whitespace-pre-wrap break-all"
						>
							<!-- Highlight Syntax -->
							<span v-if="log.includes('[INFO]')" class="text-slate-400">
								<span class="text-cyan-400 font-medium">{{ log.split('[INFO]:')[0] }}[INFO]:</span>
								<span class="text-slate-200">{{ log.split('[INFO]:')[1] }}</span>
							</span>
							<span v-else-if="log.includes('[WARN]')" class="text-amber-400">
								<span class="font-bold">{{ log }}</span>
							</span>
							<span v-else-if="log.includes('[ERROR]')" class="text-rose-400">
								<span class="font-bold">{{ log }}</span>
							</span>
							<span v-else-if="log.includes('[DONE]')" class="text-emerald-400 font-bold">
								{{ log }}
							</span>
							<span v-else-if="log.includes('[Console]')" class="text-brand font-semibold">
								{{ log }}
							</span>
							<span v-else class="text-slate-300">
								{{ log }}
							</span>
						</div>
					</div>

					<!-- Quick Command Suggestion Chips -->
					<div class="px-4 py-2.5 bg-surface-3/60 border-t border-surface-4 flex flex-wrap items-center gap-1.5">
						<span class="text-[11px] font-semibold text-secondary mr-1">Quick:</span>
						<button
							v-for="cmd in ['/op Alex', '/gamemode creative', '/time set day', '/whitelist off', '/weather clear', '/tps']"
							:key="cmd"
							type="button"
							class="text-[11px] font-mono px-2 py-0.5 rounded-md bg-surface-4/80 hover:bg-brand hover:text-black text-secondary hover:font-bold transition-colors cursor-pointer border-none"
							@click="sendQuickCommand(cmd)"
						>
							{{ cmd }}
						</button>
					</div>

					<!-- Command Input Bar -->
					<form class="p-3 bg-surface-3 border-t border-surface-4 flex items-center gap-2" @submit.prevent="submitCommand">
						<span class="text-brand font-mono font-bold text-sm pl-2">&gt;</span>
						<input
							v-model="commandInput"
							type="text"
							placeholder="Type a server command... (e.g. /gamemode creative)"
							class="flex-1 bg-surface-2 border border-surface-4 focus:border-brand rounded-xl px-3.5 py-2 text-sm text-contrast font-mono outline-none shadow-inner transition-colors"
						/>
						<button
							type="submit"
							class="px-4 py-2 rounded-xl bg-brand hover:bg-brand-highlight text-black font-bold text-xs shadow-sm transition-all cursor-pointer border-none flex items-center gap-1.5"
						>
							<span>Send</span>
							<svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
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
	status: 'online',
	version: '1.21.4',
	engine: 'PaperMC',
	ram_gb: 4,
	tunnel_enabled: true,
	public_ip: 'freeplay-game.gl.joinmc.link:25565',
	local_port: 25565,
	motd: 'A FreePlay Minecraft Server',
	uptime_seconds: 9840,
	cpu_percent: 14.5,
	ram_used_mb: 2480,
	players: [],
	logs: [],
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
</style>
