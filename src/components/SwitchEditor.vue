<template>
  <n-card size="small" title="Switch Block" closable @close="onRemove?.()">
    <n-form label-width="100" class="mb-3">
      <n-form-item label="Switch Key">
        <n-input v-model:value="item.switch" placeholder="e.g. Opcode" />
      </n-form-item>
    </n-form>

    <n-tabs type="line" animated>
      <n-tab-pane v-for="(caseDef, key) in item.cases" :key="key" :name="key">
        <template #tab>
          {{ key }}
          <n-icon size="14" class="ml-1 text-red-500" @click.stop="removeCase(key)">
            <close-outline />
          </n-icon>
        </template>

        <n-form label-width="100" class="mb-2">
          <n-form-item label="Description">
            <n-input v-model:value="caseDef.description" />
          </n-form-item>
        </n-form>

        <FieldEditor v-model:fields="caseDef.groups" />
      </n-tab-pane>

      <n-tab-pane name="default" v-if="item.default">
        <template #tab>
          Default
          <n-icon size="14" class="ml-1 text-red-500" @click.stop="item.default = null">
            <close-outline />
          </n-icon>
        </template>

        <n-form label-width="100">
          <n-form-item label="Description">
            <n-input v-model:value="item.default.description" />
          </n-form-item>
        </n-form>
        <FieldEditor v-model:fields="item.default.groups" />
      </n-tab-pane>
    </n-tabs>

    <div class="mt-3 flex gap-2">
      <n-input v-model:value="newCaseKey" placeholder="New case key (e.g. 0x01)" size="small" />
      <n-button size="small" @click="addCase">Add Case</n-button>
      <n-button size="small" @click="addDefault" :disabled="!!item.default">Add Default</n-button>
    </div>
  </n-card>
</template>

<script setup>
import { ref } from 'vue'
import { CloseOutline } from '@vicons/ionicons5'
import FieldEditor from './FieldEditor.vue'

const props = defineProps({
  item: Object,
  onRemove: Function
})

const newCaseKey = ref('')

function addCase() {
  if (!newCaseKey.value || props.item.cases[newCaseKey.value]) return
  props.item.cases[newCaseKey.value] = {
    description: '',
    groups: []
  }
  newCaseKey.value = ''
}

function removeCase(key) {
  delete props.item.cases[key]
}

function addDefault() {
  props.item.default = {
    description: '',
    groups: []
  }
}
</script>
