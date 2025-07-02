<template>
  <n-collapse v-if="result">
    <n-collapse-item
        v-for="(group, name) in groups"
        :key="name"
        :title="String(name)"
        :name="name"
    >
      <n-table :bordered="false" :single-line="true">
        <thead>
        <tr>
          <th>Field</th>
          <th>Type</th>
          <th>Value</th>
        </tr>
        </thead>
        <tbody>
        <tr
            v-for="[key, val] in Object.entries(group as Record<string, unknown>)"
            :key="key"
        >
          <td>
            <n-tooltip trigger="hover" placement="top">
              <template #trigger>
                <span class="font-mono text-sm">{{ key }}</span>
              </template>
              {{ descOf(val) }}
            </n-tooltip>
          </td>
          <td>{{ typeOf(val) }}</td>
          <td>{{ toDisplay(valueOf(val)) }}</td>
        </tr>
        </tbody>
      </n-table>
    </n-collapse-item>
  </n-collapse>

  <n-alert v-else-if="error" type="error" :show-icon="false">
    {{ error }}
  </n-alert>
</template>

<script setup lang="ts">
import { computed } from "vue";
import {
  NCollapse,
  NCollapseItem,
  NTable,
  NTooltip,
  NAlert,
} from "naive-ui";

/* ---------------- props ---------------- */
interface ResultObj {
  [key: string]: unknown;
}

const props = defineProps<{
  result: ResultObj | null;
  error: string | null;
}>();

/* ----- groups to show (exclude meta keys) ----- */
const groups = computed(() =>
    Object.fromEntries(
        Object.entries(props.result ?? {}).filter(
            ([k]) => !["cmd", "description", "variant_description"].includes(k)
        )
    )
);

/* ---------------- helpers ---------------- */
function valueOf(v: unknown): unknown {
  return typeof v === "object" && v !== null && "value" in v
      ? (v as any).value
      : v;
}

function descOf(v: unknown): string {
  return typeof v === "object" && v !== null && "description" in v
      ? (v as any).description
      : "(no description)";
}

function typeOf(v: unknown): string {
  const val = valueOf(v);
  return Array.isArray(val) ? "array" : typeof val;
}

/* stringify value for table cell */
function toDisplay(v: unknown): string {
  return typeof v === "object" ? JSON.stringify(v) : String(v);
}
</script>

<style scoped>
.n-table td,
.n-table th {
  vertical-align: middle;
}
</style>
