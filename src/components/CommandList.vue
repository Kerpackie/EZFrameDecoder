<template>
  <n-list bordered hoverable clickable>
    <n-list-item
        v-for="cmd in commands"
        :key="cmd.letter"
        @click="$emit('select', cmd)"
    >
      <template #prefix>
        <n-tag type="info">{{ cmd.letter }}</n-tag>
      </template>

      <div class="flex justify-between w-full items-center">
        <span class="font-medium">{{ cmd.description || '(no description)' }}</span>
        <n-button
            circle
            size="small"
            tertiary
            @click.stop="$emit('delete', cmd.letter)"
        >
          <template #icon><n-icon><trash-outline /></n-icon></template>
        </n-button>
      </div>
    </n-list-item>
  </n-list>
</template>

<script setup lang="ts">
import { NList, NListItem, NTag, NButton } from 'naive-ui'
import { TrashOutline } from '@vicons/ionicons5'

defineProps<{ commands: { letter: string; description?: string }[] }>()
defineEmits<{ (e: 'select', cmd: any): void; (e: 'delete', letter: string): void }>()
</script>
