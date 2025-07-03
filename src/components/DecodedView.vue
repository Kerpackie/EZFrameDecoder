<template>
  <!-- When result exists render header + tables -->
  <div v-if="result" class="decoded-root">
    <!-- Header section -->
    <div class="command-header">
      <h3 class="cmd-letter">{{ result.cmd }}</h3>

      <p v-if="result.description" class="cmd-desc">
        {{ result.description }}
      </p>

      <p v-if="result.variant_description" class="cmd-variant">
        {{ result.variant_description }}
      </p>

      <p v-if="result.raw" class="cmd-raw">
        <span class="label">Decoded:</span>
        <code>{{ result.raw }}</code>
      </p>
    </div>

    <!-- One section per group -->
    <div
        v-for="(group, name) in groups"
        :key="name"
        class="group-section"
    >
      <h4 class="group-title">{{ name }}</h4>

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
        <tr v-for="(val, key) in group as Record<string, any>" :key="key">

        <!-- field name with tooltip -->
          <td>
            <n-tooltip trigger="hover" placement="top">
              <template #trigger>
                <span class="font-mono text-sm">{{ key }}</span>
              </template>
              {{ descOf(val) }}
            </n-tooltip>
          </td>

          <!-- raw hex slice -->
          <td><code>{{ hexOf(val) }}</code></td>

          <!-- type and value -->
          <td>{{ typeOf(val) }}</td>
          <td>{{ toDisplay(valueOf(val)) }}</td>
        </tr>
        </tbody>
      </n-table>
    </div>
  </div>

  <!-- Error state -->
  <n-alert v-else-if="error" type="error" :show-icon="false">
    {{ error }}
  </n-alert>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { NTable, NTooltip, NAlert } from "naive-ui";

/* -------- props from parent ---------- */
const props = defineProps<{
  result: Record<string, unknown> | null;
  error : string | null;
}>();

/* -------- groups to render ---------- */
const groups = computed(() =>
    Object.fromEntries(
        Object.entries(props.result ?? {}).filter(
            ([k]) =>
                ![
                  "cmd",
                  "description",
                  "variant_description",
                  "raw"
                ].includes(k)
        )
    )
);

/* ---------- helper functions ---------- */
function hexOf(v: any): string {
  return v?.hex ?? "";
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
/* header */
.command-header {
  margin-bottom: 1.5rem;
}
.cmd-letter {
  font-size: 1.6rem;
  font-weight: 700;
  margin: 0;
}
.cmd-desc {
  margin: 0.15rem 0 0.1rem;
  font-size: 0.95rem;
  color: #bbb;
}
.cmd-variant {
  margin: 0 0 0.4rem;
  font-size: 0.9rem;
  color: #aaa;
}
.cmd-raw {
  font-size: 0.85rem;
  color: #888;
}
.cmd-raw code {
  background: #222;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: monospace;
}
.label {
  font-weight: 500;
  margin-right: 6px;
}

/* tables */
.group-section {
  margin-bottom: 1.8rem;
}
.group-title {
  margin: 0 0 0.5rem;
  font-size: 1.1rem;
  font-weight: 600;
}
.n-table td,
.n-table th {
  vertical-align: middle;
}

/* optional: nicer code background in Hex column */
code {
  background: rgba(255, 255, 255, 0.05);
  padding: 0 4px;
  border-radius: 3px;
}
</style>
