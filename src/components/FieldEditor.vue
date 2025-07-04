<template>
  <div class="field-editor">
    <n-dynamic-input
        v-model:value="fields"
        :on-create="createField"
        preset="pair"
        key-placeholder="Name"
        value-placeholder="Length"
        :min="0"
    >
      <template #default="{ value, index }">
        <n-space :wrap="false" align="center">
          <n-input v-model:value="value.name" placeholder="Name" style="width: 120px" />

          <n-input-number v-model:value="value.len" :min="1" :max="16" placeholder="Len" style="width: 80px" />

          <n-select v-model:value="value.type" :options="typeOpts" style="width: 100px" />

          <n-select
              v-model:value="value.base"
              :options="baseOpts"
              :disabled="value.type === 'bool'"
              style="width: 80px"
          />

          <n-input
              v-model:value="value.description"
              type="text"
              placeholder="Description"
              style="flex: 1"
          />
        </n-space>
      </template>
    </n-dynamic-input>
  </div>
</template>

<script setup lang="ts">
import {
  NDynamicInput,
  NInput,
  NInputNumber,
  NSelect,
  NSpace
} from "naive-ui";
import type { Field } from "../types"; // You can inline this if needed

const props = defineProps<{
  fields: Field[];
}>();
const emit = defineEmits(["update:fields"]);

const fields = defineModel<Field[]>('fields', { required: true });

const typeOpts = [
  { label: "Number", value: "number" },
  { label: "Bool", value: "bool" }
];
const baseOpts = [
  { label: "Hex", value: 16 },
  { label: "Decimal", value: 10 }
];

function createField(): Field {
  return {
    name: "",
    len: 2,
    type: "number",
    base: 16,
    description: ""
  };
}
</script>

<style scoped>
.field-editor {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
</style>
