<template>
  <n-card title="Application Settings">
    <div class="setting-row">
      <div class="setting-info">
        <n-h3 style="margin: 0;">Advanced Mode</n-h3>
        <n-text depth="3">
          Enables full editing capabilities for command families and their specifications.
        </n-text>
      </div>
      <n-switch :value="isAdvancedMode" @update:value="toggleAdvancedMode" />
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { NCard, NSwitch, NH3, NText } from 'naive-ui';
import { useSettingsStore } from '../stores/settingsStore';

const { isAdvancedMode, toggleAdvancedMode } = useSettingsStore();
</script>

<style scoped>
.setting-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.setting-info {
  margin-right: 2rem;
}
</style>
