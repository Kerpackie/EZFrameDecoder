<template>
  <n-collapse v-model:expanded-names="open" accordion>
    <n-collapse-item
        v-for="cmd in commands"
        :key="cmd.letter + cmd.familyStart"
        :name="cmd.letter + cmd.familyStart">
      <template #header>
        <div class="w-full flex justify-between">
          <div>
            <code>{{ cmd.familyStart }}</code>
            {{ cmd.letter }} — {{ cmd.description || 'no description' }}
          </div>
          <div class="flex gap-1">
            <n-button size="tiny" quaternary @click.stop="toggle(cmd)">
              {{ editing === cmd ? 'Close' : 'Edit' }}
            </n-button>
            <n-button size="tiny" quaternary type="error"
                      @click.stop="askDelete(cmd)">Delete</n-button>
          </div>
        </div>
      </template>

      <component :is="editing === cmd ? 'AddCommandBuilder' : 'CommandViewer'"
                 :command="cmd"
                 mode="edit"
                 :initial="cmd"
                 @saved="$emit('refresh')"
                 @cancel="editing = null"/>
    </n-collapse-item>
  </n-collapse>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useDialog, NCollapse, NCollapseItem, NButton } from 'naive-ui'
import CommandViewer     from './CommandViewer.vue'
import AddCommandBuilder from './AddCommandBuilder.vue'

defineProps<{ commands:any[] }>()
const emit = defineEmits(['edit','delete','refresh'])

const open    = ref<string[]>([])
const editing = ref<any|null>(null)

function toggle(cmd:any){
  editing.value = (editing.value === cmd ? null : cmd)
}

const dia = useDialog()
function askDelete(cmd:any){
  dia.warning({
    title:'Delete', content:`Delete ${cmd.letter}?`,
    positiveText:'Delete', negativeText:'Cancel',
    onPositiveClick:()=> emit('delete',cmd)
  })
}
</script>
