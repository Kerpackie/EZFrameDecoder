<template>
  <div class="flex gap-6">
    <!-- list -->
    <div class="w-64">
      <h3 class="font-semibold mb-2">Commands</h3>
      <CommandList
          :commands="commands"
          @select="selectCmd"
          @delete="deleteCmd"
      />
    </div>

    <!-- builder -->
    <div class="flex-1">
      <AddCommandBuilder
          v-if="selected"
          mode="edit"
          :initial="selected"
          @saved="reload"
          @cancel="selected = null"
      />
      <n-empty v-else description="Select a command to edit" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import CommandList from '../components/CommandList.vue'
import AddCommandBuilder from '../components/AddCommandBuilder.vue'
import { NEmpty, useMessage } from 'naive-ui'

const commands = ref<any[]>([])
const selected = ref<any|null>(null)
const msg      = useMessage()

const reload = () =>
    invoke('get_commands').then((v:any[]) => commands.value = v)

onMounted(reload)

function selectCmd(cmd:any){
  selected.value = JSON.parse(JSON.stringify(cmd)) // deep copy
}

async function deleteCmd(cmd:any){
  try {
    await invoke('delete_command', { letter: cmd.letter })
    msg.success(`Command ${cmd.letter} deleted`)
    selected.value = null
    await reload()
  } catch (e:any) {
    msg.error(String(e))
  }
}
</script>
