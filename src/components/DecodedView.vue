<template>
  <!-- decoded result -->
  <div v-if="result" class="decoded-root">
    <!-- meta card -->
    <n-card embedded class="meta-card" size="small">
      <template #header>
        <span class="font-semibold">Command Type: {{ result.cmd }}</span>
      </template>

      <div class="meta-details">
        <div class="meta-section" v-if="result.description">
          <n-text strong> Description: <br></n-text>
          <n-text depth="3">
            {{ result.description }}
          </n-text>
        </div>

        <div class="meta-section" v-if="result.variant_description">
          <n-text depth="3">
            <n-text strong> Sub-Variant: <br> </n-text>
            {{ result.variant_description }}
          </n-text>
        </div>
      </div>

      <div class="mt-2">
        <n-text strong>Decoded: </n-text>
        <n-tag type="primary" size="small" bordered>{{ result.raw }}</n-tag>
      </div>
    </n-card>

    <!-- groups -->
    <div
        v-for="(group, name) in groups"
        :key="name"
        class="group-section"
    >
      <n-h4 class="group-title">{{ toTitleCase(String(name)) }}</n-h4>

      <n-table :bordered="false" :single-line="true" size="small">
        <thead>
        <tr>
          <th>Field</th>
          <th>Hex</th>
          <th>Type</th>
          <th>Value</th>
        </tr>
        </thead>
        <tbody>
        <tr
            v-for="(val, key) in group as Record<string, any>"
            :key="key"
        >
          <!-- field name -->
          <td>
            <n-tooltip trigger="hover" placement="top">
              <template #trigger>
                <n-text code>{{ key }}</n-text>
              </template>
              {{ descOf(val) }}
            </n-tooltip>
          </td>

          <!-- raw hex -->
          <td>
            <n-tag size="small" bordered>{{ hexOf(val) }}</n-tag>
          </td>

          <!-- type / value -->
          <td>{{ typeOf(val) }}</td>
          <td>{{ toDisplay(valueOf(val)) }}</td>
        </tr>
        </tbody>
      </n-table>
    </div>
  </div>

  <!-- error -->
  <n-alert v-else-if="error" type="error" :show-icon="false">
    {{ error }}
  </n-alert>
</template>

<script setup lang="ts">
import { computed } from "vue";
import {
  NCard,
  NTable,
  NTooltip,
  NAlert,
  NTag,
  NText,
  NH4
} from "naive-ui";

const props = defineProps<{
  result: Record<string, unknown> | null;
  error: string | null;
}>();

/* groups to render (skip meta keys) */
const groups = computed(() =>
    Object.fromEntries(
        Object.entries(props.result ?? {}).filter(([k]) =>
            !["cmd", "description", "variant_description", "raw"].includes(k))
    )
);

/* helpers */
function hexOf(v: any): string {
  return v?.hex ?? "";
}
function toTitleCase(text: string): string {
  return text.replace(/\w\S*/g, w => w.charAt(0).toUpperCase() + w.slice(1));
}
function valueOf(v: any): unknown {
  return v?.value ?? v;
}
function descOf(v: any): string {
  return v?.description ?? "(no description)";
}
function typeOf(v: any): string {
  const raw = valueOf(v);
  return Array.isArray(raw) ? "array" : typeof raw;
}
function toDisplay(v: unknown): string {
  return typeof v === "object" ? JSON.stringify(v) : String(v);
}
</script>

<style scoped>
.meta-card {
  margin-bottom: 1.5rem;
}

.group-section {
  margin-bottom: 2rem;
}

.group-title {
  margin-bottom: 0.5rem;
}

.meta-section {
  margin-bottom: 0.5rem;
}
</style>
