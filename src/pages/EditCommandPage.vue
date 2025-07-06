<template>
  <div class="w-full">
    <h3 class="font-semibold mb-4">Commands</h3>
    <CommandList
        :commands="commands"
        @saved="reload"
        @delete="deleteCmd"
    />
    <n-empty v-if="!commands.length" description="No commands found" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { NEmpty, useMessage } from 'naive-ui'

import CommandList from '../components/CommandList.vue'

/* state */
const commands = ref<any[]>([])
const msg      = useMessage()

const reload = () =>
    invoke('get_commands').then((list: any[]) => (commands.value = list))

onMounted(reload)

/* delete handler */
async function deleteCmd(cmd: any) {
  try {
    await invoke('delete_command', { letter: cmd.letter })
    msg.success(`Command ${cmd.letter} deleted`)
    await reload()
  } catch (e: any) {
    msg.error(String(e))
  }
}
</script>
