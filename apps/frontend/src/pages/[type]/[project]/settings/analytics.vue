<template>
	<AnalyticsDashboard />
</template>

<script setup lang="ts">
import { commonProjectSettingsMessages } from '@freeplay/ui'

import AnalyticsDashboard from '~/components/analytics-dashboard/index.vue'

useProjectSettingsHeadTitle(commonProjectSettingsMessages.analytics)
</script>
