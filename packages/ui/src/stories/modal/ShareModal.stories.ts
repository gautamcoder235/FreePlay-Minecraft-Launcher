import type { StoryObj } from '@storybook/vue3-vite'
import { ref } from 'vue'

import { Button } from '../../components/base/buttons'
import ShareModal from '../../components/modal/ShareModal.vue'

const meta = {
	title: 'Modal/ShareModal',
	component: ShareModal,
}

export default meta
type Story = StoryObj<typeof ShareModal>

export const LinkShare: Story = {
	args: {
		header: 'Share link',
		shareTitle: 'FreePlay',
		shareText: 'Check this out on FreePlay',
		link: true,
	},
	render: (args) => ({
		components: { ShareModal, Button },
		setup() {
			const modalRef = ref<InstanceType<typeof ShareModal> | null>(null)
			const openModal = () => {
				modalRef.value?.show('https://freeplay.app')
			}
			return { args, modalRef, openModal }
		},
		template: `
			<div>
				<Button type="colored" color="brand" @click="openModal">Open Link Share Modal</Button>
				<ShareModal ref="modalRef" v-bind="args" />
			</div>
		`,
	}),
}

export const TextShare: Story = {
	args: {
		header: 'Share text',
		shareTitle: 'FreePlay',
		shareText: 'Invite your friends to try FreePlay.',
		link: false,
	},
	render: (args) => ({
		components: { ShareModal, Button },
		setup() {
			const modalRef = ref<InstanceType<typeof ShareModal> | null>(null)
			const openModal = () => {
				modalRef.value?.show('https://freeplay.app')
			}
			return { args, modalRef, openModal }
		},
		template: `
			<div>
				<Button type="colored" color="brand" @click="openModal">Open Text Share Modal</Button>
				<ShareModal ref="modalRef" v-bind="args" />
			</div>
		`,
	}),
}
