<template>
  <!-- accordion: only one command open at a time -->
  <n-collapse v-model:expanded-names="openKeys" accordion>
    <n-collapse-item
        v-for="cmd in commands"
        :key="cmd.letter"
        :name="cmd.letter"
        class="mb-1"
    >
      <!-- ───────── header row ───────── -->
      <template #header>
        <div class="flex justify-between items-center w-full">
          <span>{{ cmd.letter }} — {{ cmd.description || 'no description' }}</span>

          <!-- action buttons (conditionally rendered) -->
          <div v-if="!readOnly" class="flex gap-1">
            <n-button
                size="tiny"
                quaternary
                @click.stop="toggleEdit(cmd.letter)"
            >
              {{ editLetter === cmd.letter ? 'Close' : 'Edit' }}
            </n-button>

            <n-button
                size="tiny"
                quaternary
                type="error"
                @click.stop="confirmDelete(cmd)"
            >
              Delete
            </n-button>
          </div>
        </div>
      </template>

      <!-- ───────── expanded body ───────── -->
      <AddCommandBuilder
          v-if="!readOnly && editLetter === cmd.letter"
          mode="edit"
          :initial="cmd"
          :family="family"
          @saved="() => { editLetter = null; $emit('refresh'); }"
          @cancel="editLetter = null"
      />
      <CommandViewer
          v-else
          :command="cmd"
      />
    </n-collapse-item>
  </n-collapse>
</template>

<script setup lang="ts">
import {
  NCollapse,
  NCollapseItem,
  NButton,
  useDialog
} from 'naive-ui'
import { ref } from 'vue'
import CommandViewer from './CommandViewer.vue'
import AddCommandBuilder from './AddCommandBuilder.vue'

/* props & emits */
defineProps<{
  commands: any[],
  family: object,
  readOnly?: boolean // New prop to control edit/delete visibility
}>()
const emit = defineEmits(['refresh', 'delete'])

/* collapse state */
const openKeys = ref<string[]>([])
const editLetter = ref<string | null>(null)

function toggleEdit(letter: string) {
  editLetter.value = editLetter.value === letter ? null : letter
}

/* delete confirmation */
const dialog = useDialog()
function confirmDelete(cmd: any) {
  dialog.warning({
    title: 'Delete command',
    content: `Are you sure you want to delete command '${cmd.letter}'?`,
    positiveText: 'Delete',
    negativeText: 'Cancel',
    onPositiveClick: () => emit('delete', cmd)
  })
}
</script>
