<script setup lang="ts">
import {
	CrownIcon,
	GripVerticalIcon,
	HeartIcon,
	ShieldAlertIcon,
	ShieldIcon,
	UsersIcon,
	UserXIcon,
	XIcon,
} from '@freeplay/assets'
import { Button } from '@freeplay/ui'
import { computed, ref } from 'vue'

import type { TrackedPlayer } from '@/helpers/hosting'
import { useOverlayStore } from '@/store/overlay'

const overlayStore = useOverlayStore()
const emit = defineEmits<{
	(e: 'drag-start', event: PointerEvent): void
}>()

const selectedPlayer = ref<TrackedPlayer | null>(null)
const actionReason = ref('')
const confirmDialogAction = ref<'kick' | 'ban' | null>(null)

const onlinePlayers = computed(() => overlayStore.server.players)

function openConfirmDialog(action: 'kick' | 'ban', player: TrackedPlayer) {
	selectedPlayer.value = player
	confirmDialogAction.value = action
	actionReason.value = action === 'kick' ? 'Kicked by operator' : 'Banned by operator'
}

function closeConfirmDialog() {
	selectedPlayer.value = null
	confirmDialogAction.value = null
	actionReason.value = ''
}

async function executeConfirmedAction() {
	if (!selectedPlayer.value || !confirmDialogAction.value) return
	const action = confirmDialogAction.value
	const player = selectedPlayer.value.name
	const reason = actionReason.value

	await overlayStore.performPlayerAction(action, player, reason)
	closeConfirmDialog()
}

async function quickHeal(player: TrackedPlayer) {
	await overlayStore.performPlayerAction('heal', player.name)
}

async function toggleOp(player: TrackedPlayer) {
	const action = player.isOp ? 'deop' : 'op'
	await overlayStore.performPlayerAction(action, player.name)
}
</script>

<template>
	<div
		class="w-[420px] rounded-[24px] bg-slate-900/90 backdrop-blur-2xl border border-white/12 p-5 flex flex-col gap-3.5 shadow-2xl shadow-black/90 select-none transform-gpu contain-paint"
	>
		<!-- 1. Draggable Header -->
		<div
			class="flex items-center justify-between border-b border-white/10 pb-3 cursor-grab active:cursor-grabbing group/header touch-none"
			title="Click and drag to move"
			@pointerdown.stop.prevent="emit('drag-start', $event)"
		>
			<div class="flex items-center gap-2.5 min-w-0 pointer-events-none">
				<GripVerticalIcon
					class="w-4 h-4 text-slate-500 group-hover/header:text-slate-300 transition-colors shrink-0"
				/>
				<div
					class="w-8 h-8 rounded-xl bg-emerald-500/15 border border-emerald-500/25 flex items-center justify-center text-emerald-400 shrink-0 shadow-sm"
				>
					<UsersIcon class="w-4 h-4" />
				</div>
				<div class="flex flex-col min-w-0">
					<span class="text-sm font-semibold text-white/95 tracking-tight truncate"
						>Live Players & Roster</span
					>
					<span class="text-[11px] text-slate-400 font-mono truncate">
						{{ onlinePlayers.length }} Connected
					</span>
				</div>
			</div>

			<div class="flex items-center gap-2 shrink-0" @pointerdown.stop>
				<span
					class="text-[10px] uppercase font-bold tracking-wider px-2 py-0.5 rounded-full border bg-emerald-500/15 text-emerald-400 border-emerald-500/30"
				>
					{{ onlinePlayers.length }} Online
				</span>
				<button
					class="w-7 h-7 rounded-xl bg-white/5 hover:bg-white/15 text-slate-400 hover:text-white flex items-center justify-center transition-all cursor-pointer border-none shadow-sm"
					title="Hide widget"
					@click="overlayStore.showPlayerWidget = false"
				>
					<XIcon class="w-3.5 h-3.5" />
				</button>
			</div>
		</div>

		<!-- 2. Online Players List -->
		<div class="flex flex-col gap-2 max-h-60 overflow-y-auto pr-1">
			<div
				v-for="player in onlinePlayers"
				:key="player.name"
				class="p-3 rounded-xl bg-black/40 border border-white/5 hover:border-emerald-500/30 transition-all flex items-center justify-between gap-3 shadow-sm"
			>
				<!-- Avatar & Details -->
				<div class="flex items-center gap-2.5 min-w-0">
					<img
						:src="`https://mc-heads.net/avatar/${player.name}/32`"
						class="w-8 h-8 rounded-lg bg-surface-4 shrink-0 object-cover border border-white/10"
						:alt="player.name"
					/>
					<div class="flex flex-col min-w-0">
						<div class="flex items-center gap-1.5">
							<span class="text-xs font-semibold text-white/90 truncate">{{ player.name }}</span>
							<CrownIcon
								v-if="player.isOp"
								class="w-3.5 h-3.5 text-amber-400 shrink-0"
								title="Server Operator"
							/>
						</div>
						<div class="flex items-center gap-2 text-[10px] text-slate-400 font-mono">
							<span
								v-if="player.gamemode"
								class="capitalize px-1.5 py-0.2 rounded bg-white/5 border border-white/10"
							>
								{{ player.gamemode }}
							</span>
							<span v-if="player.pingMs">{{ player.pingMs }}ms</span>
						</div>
					</div>
				</div>

				<!-- Quick Operator Actions -->
				<div class="flex items-center gap-1 shrink-0" @pointerdown.stop>
					<!-- Quick Heal -->
					<button
						class="w-7 h-7 rounded-lg bg-white/5 hover:bg-emerald-500/20 text-slate-400 hover:text-emerald-400 border border-white/10 flex items-center justify-center transition-all cursor-pointer"
						title="Instant Heal & Feed"
						@click="quickHeal(player)"
					>
						<HeartIcon class="w-3.5 h-3.5" />
					</button>

					<!-- Toggle OP -->
					<button
						class="w-7 h-7 rounded-lg bg-white/5 hover:bg-amber-500/20 border border-white/10 flex items-center justify-center transition-all cursor-pointer"
						:class="player.isOp ? 'text-amber-400' : 'text-slate-400 hover:text-amber-300'"
						:title="player.isOp ? 'Revoke OP (Deop)' : 'Grant OP'"
						@click="toggleOp(player)"
					>
						<CrownIcon class="w-3.5 h-3.5" />
					</button>

					<!-- Kick Modal Trigger -->
					<button
						class="w-7 h-7 rounded-lg bg-white/5 hover:bg-amber-500/20 text-slate-400 hover:text-amber-400 border border-white/10 flex items-center justify-center transition-all cursor-pointer"
						title="Kick Player"
						@click="openConfirmDialog('kick', player)"
					>
						<UserXIcon class="w-3.5 h-3.5" />
					</button>

					<!-- Ban Modal Trigger -->
					<button
						class="w-7 h-7 rounded-lg bg-white/5 hover:bg-rose-500/20 text-slate-400 hover:text-rose-400 border border-white/10 flex items-center justify-center transition-all cursor-pointer"
						title="Ban Player"
						@click="openConfirmDialog('ban', player)"
					>
						<ShieldAlertIcon class="w-3.5 h-3.5" />
					</button>
				</div>
			</div>

			<!-- Empty State -->
			<div
				v-if="onlinePlayers.length === 0"
				class="p-4 rounded-xl bg-black/20 border border-white/5 text-center text-xs text-slate-500 italic"
			>
				No players currently connected to dedicated server.
			</div>
		</div>

		<!-- 3. Action Confirmation Modal Overlay -->
		<div
			v-if="confirmDialogAction && selectedPlayer"
			class="p-3.5 rounded-xl bg-slate-950/95 border border-amber-500/40 flex flex-col gap-2.5 shadow-xl"
			@pointerdown.stop
		>
			<div class="flex items-center justify-between text-xs font-semibold text-white">
				<span class="flex items-center gap-1.5">
					<ShieldIcon class="w-3.5 h-3.5 text-amber-400" />
					Confirm {{ confirmDialogAction.toUpperCase() }}: {{ selectedPlayer.name }}
				</span>
				<button
					class="text-slate-400 hover:text-white bg-transparent border-none cursor-pointer"
					@click="closeConfirmDialog"
				>
					<XIcon class="w-3.5 h-3.5" />
				</button>
			</div>

			<input
				v-model="actionReason"
				type="text"
				placeholder="Enter reason..."
				class="w-full px-2.5 py-1.5 rounded-lg bg-black/50 border border-white/15 text-xs text-white outline-none focus:border-amber-400"
			/>

			<div class="flex items-center justify-end gap-2 pt-1">
				<Button type="standard" class="!py-1 !px-2.5 text-xs" @click="closeConfirmDialog">
					Cancel
				</Button>
				<Button
					type="colored"
					:color="confirmDialogAction === 'ban' ? 'red' : 'brand'"
					class="!py-1 !px-3 text-xs !font-bold"
					@click="executeConfirmedAction"
				>
					Confirm {{ confirmDialogAction.toUpperCase() }}
				</Button>
			</div>
		</div>
	</div>
</template>
