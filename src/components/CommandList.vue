<template>
  <n-list bordered>
    <n-list-item
        v-for="cmd in commands"
        :key="cmd.letter"
        class="flex justify-between items-center"
    >
      <span>{{ cmd.letter }} — {{ cmd.description || '' }}</span>

      <div class="flex gap-1">
        <n-button size="tiny" quaternary @click="$emit('select', cmd)">
          Edit
        </n-button>

        <n-button
            size="tiny"
            quaternary
            type="error"
            @click="confirmDelete(cmd)"
        >
          Delete
        </n-button>
      </div>
    </n-list-item>
  </n-list>
</template>

<script setup lang="ts">
import { NList, NListItem, NButton, useDialog } from 'naive-ui'

defineProps<{ commands: any[] }>()
const emit = defineEmits(['select', 'delete'])
const dialog = useDialog()

function confirmDelete(cmd: any) {
  dialog.warning({
    title: 'Delete command',
    content: `Are you sure you want to delete “${cmd.letter}”?`,
    positiveText: 'Delete',
    negativeText: 'Cancel',
    onPositiveClick: () => emit('delete', cmd)
  })
}
</script>
