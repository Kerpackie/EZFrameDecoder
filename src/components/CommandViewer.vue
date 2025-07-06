<template>
  <!-- ───── top-level meta ───── -->
  <n-descriptions :column="1" size="small" class="mb-3">
    <n-descriptions-item label="Letter">
      <n-tag type="primary" size="small">{{ command.letter }}</n-tag>
    </n-descriptions-item>
    <n-descriptions-item label="Description">
      {{ command.description || '—' }}
    </n-descriptions-item>
  </n-descriptions>

  <!-- ───── iterate items ───── -->
  <div v-for="(item, idx) in command.items" :key="idx" class="mb-4">
    <!-- ── group ── -->
    <template v-if="'fields' in item">
      <n-h5 class="mt-0 mb-1">{{ item.name || 'Group' }}</n-h5>
      <FieldTable :fields="item.fields"/>
    </template>

    <!-- ── switch ── -->
    <template v-else>
      <n-h5 class="mt-0 mb-1">
        Switch on <code>{{ item.switch }}</code>
      </n-h5>

      <n-collapse size="small" class="ml-2">
        <!-- each case -->
        <n-collapse-item
            v-for="(variant, key) in item.cases"
            :key="key"
            :name="key"
            :title="`Case ${key}`"
        >
          <n-h6 v-if="variant.description" class="mt-0">{{ variant.description }}</n-h6>
          <template v-for="(g, gi) in variant.groups" :key="gi">
            <n-h6 class="mt-2 mb-1">{{ g.name || 'Group' }}</n-h6>
            <FieldTable :fields="g.fields"/>
          </template>
        </n-collapse-item>

        <!-- default -->
        <n-collapse-item
            v-if="item.default"
            name="__def"
            title="Default"
        >
          <n-h6 v-if="item.default.description" class="mt-0">{{ item.default.description }}</n-h6>
          <template v-for="(g, gi) in item.default.groups" :key="gi">
            <n-h6 class="mt-2 mb-1">{{ g.name || 'Group' }}</n-h6>
            <FieldTable :fields="g.fields"/>
          </template>
        </n-collapse-item>
      </n-collapse>
    </template>

    <n-divider v-if="idx < command.items.length-1"/>
  </div>
</template>

<script setup lang="ts">
import {
  NDescriptions,
  NDescriptionsItem,
  NTag,
  NH5,
  NH6,
  NDivider,
  NCollapse,
  NCollapseItem
} from 'naive-ui'
import FieldTable from './FieldTable.vue'
defineProps<{ command: any }>()
</script>

<style scoped>
code {
  background:#f2f2f2;
  padding:0 3px;
  border-radius:3px;
  font-family:monospace;
}
</style>
