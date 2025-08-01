<template>
  <div class="field-editor">
    <!-- rows -->
    <div
        v-for="(fld, idx) in fields"
        :key="idx"
        class="field-row"
    >
      <n-input
          v-model:value="fld.name"
          placeholder="Name"
          size="small"
          :disabled="isLocked(fld)"
          class="field-input"
      />

      <n-input-number
          v-model:value="fld.len"
          :min="1"
          :max="16"
          size="small"
          :disabled="isLocked(fld)"
          class="field-len"
      />

      <n-select
          v-model:value="fld.type"
          :options="TYPE_OPTS"
          size="small"
          :disabled="isLocked(fld)"
          class="field-type"
      />

      <n-select
          v-if="fld.type === 'number'"
          v-model:value="fld.base"
          :options="BASE_OPTS"
          size="small"
          :disabled="isLocked(fld)"
          class="field-base"
      />

      <n-input
          v-model:value="fld.description"
          placeholder="Description"
          size="small"
          class="field-desc"
      />

      <n-button
          v-if="!isLocked(fld)"
          circle quaternary size="small"
          @click="remove(idx)"
      >
        <template #icon>
          <n-icon><close-outline /></n-icon>
        </template>
      </n-button>
    </div>

    <!-- add -->
    <n-button
        size="tiny"
        quaternary
        @click="addField"
        class="mt-2"
    >
      + Add Field
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { CloseOutline } from '@vicons/ionicons5'
import type { PropType } from 'vue';

interface Field {
  name: string;
  len: number;
  type: 'number' | 'bool';
  base: 10 | 16;
  description: string;
}

const props = defineProps({
  fields: { type: Array as PropType<Field[]>, required: true },
  lockedNames: { type: Array as PropType<string[]>, default: () => [] }
})

function isLocked(f: Field) {
  return props.lockedNames.includes(f.name)
}

function remove(idx: number) {
  props.fields.splice(idx, 1)
}

function addField() {
  // auto-generate a unique field name
  let n = props.fields.length + 1
  let candidate = `field-${n}`
  const exists = (name: string) => props.fields.some((f: any) => f.name === name)
  while (exists(candidate)) {
    n += 1
    candidate = `field-${n}`
  }
  props.fields.push({
    name: candidate,
    len: 2,
    type: 'number',
    base: 16,
    description: ''
  })
}

/* ---------- select options ---------- */
const TYPE_OPTS = [
  { label: 'number', value: 'number' },
  { label: 'bool', value: 'bool' }
]
const BASE_OPTS = [
  { label: 'hex', value: 16 },
  { label: 'dec', value: 10 }
]
</script>

<style scoped>
.field-editor {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}
.field-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  align-items: flex-start;
}
.field-input {
  width: 160px;
}
.field-len {
  width: 70px;
}
.field-type {
  width: 110px;
}
.field-base {
  width: 90px;
}
.field-desc {
  flex: 1;
}
</style>
