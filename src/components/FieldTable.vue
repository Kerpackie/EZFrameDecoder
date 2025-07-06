<template>
  <n-table :bordered="false" size="small">
    <thead>
    <tr>
      <th>Name</th>
      <th>Type</th>
      <th>Len</th>
      <th>Base</th>
      <th>Description</th>
    </tr>
    </thead>
    <tbody>
    <tr v-for="(f, i) in fields" :key="i">
      <td><code>{{ f.name }}</code></td>
      <td>{{ f.type === 'bool' ? 'boolean' : 'number' }}</td>
      <td>{{ f.len }}</td>
      <td>{{ f.base ?? '—' }}</td>
      <td>{{ f.description || '—' }}</td>
    </tr>
    </tbody>
  </n-table>
</template>

<script setup lang="ts">
import { NTable } from 'naive-ui'
defineProps<{ fields: any[] }>()
</script>
