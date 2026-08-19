<template>
	<FreePlayAccountRequiredModal ref="modal" :request-auth="requestAuth" />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import FreePlayAccountRequiredModal from './FreePlayAccountRequiredModal.vue'
import type { ModrinthAuthFlow } from '@/helpers/mr_auth'

defineProps<{
	requestAuth: (flow: ModrinthAuthFlow) => Promise<boolean>
}>()

const modal = ref<InstanceType<typeof FreePlayAccountRequiredModal>>()

function show(event?: MouseEvent) {
	return modal.value?.show(event)
}

function showSigningIn(flow: ModrinthAuthFlow = 'sign-in', event?: MouseEvent) {
	return modal.value?.showSigningIn(flow, event)
}

defineExpose({ show, showSigningIn })
</script>
