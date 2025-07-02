<template>
  <div v-if="result">
    <div
        v-for="(group, name) in groups"
        :key="name"
        class="mb-4"
    >
      <h4 class="text-lg font-semibold mb-2">
        {{ result[name]?._group_description ?? name }}
      </h4>
      <n-table :bordered="false" :single-line="true" size="small">
        <thead>
        <tr>
          <th>Field</th>
          <th>Type</th>
          <th>Value</th>
        </tr>
        </thead>
        <tbody>
        <tr v-for="[key, val] in Object.entries(group)" :key="key">
          <td>
            <n-tooltip trigger="hover" placement="top">
              <template #trigger>
                <span class="font-mono text-sm">{{ key }}</span>
              </template>
              {{ descOf(val) }}
            </n-tooltip>
          </td>
          <td>{{ typeOf(val) }}</td>
          <td>{{ valueOf(val) }}</td>
        </tr>
        </tbody>
      </n-table>
    </div>
  </div>

  <n-alert v-else-if="error" type="error" :show-icon="false">
    {{ error }}
  </n-alert>
</template>

<script setup lang="ts">
import {
  NTable,
  NTooltip,
  NAlert,
} from "naive-ui";
import { computed } from "vue";

const props = defineProps<{ result: any; error: string | null }>();

const groups = computed(() =>
    Object.fromEntries(
        Object.entries(props.result || {}).filter(
            ([k]) => !["cmd", "description", "variant_description"].includes(k)
        )
    )
);

function valueOf(v: any) {
  return v?.value ?? v;
}
function descOf(v: any) {
  return v?.description ?? "(no description)";
}
function typeOf(v: any) {
  if (typeof v !== "object" || v === null) return typeof v;
  if ("value" in v) return typeof v.value;
  return typeof v;
}
</script>

<style scoped>
.n-table td,
.n-table th {
  vertical-align: middle;
  padding-top: 0.75rem;
  padding-bottom: 0.75rem;
  font-size: 1rem;
}
</style>
