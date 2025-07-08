<template>
  <n-card title="Application Settings">
    <n-space vertical size="large">
      <!-- Advanced Mode Setting -->
      <div class="setting-row">
        <div class="setting-info">
          <n-h3 style="margin: 0;">Advanced Mode</n-h3>
          <n-text depth="3">
            Enables full editing capabilities for command families and their specifications.
          </n-text>
        </div>
        <n-switch :value="isAdvancedMode" @update:value="toggleAdvancedMode" />
      </div>

      <n-divider />

      <!-- Dark Mode Setting (New) -->
      <div class="setting-row">
        <div class="setting-info">
          <n-h3 style="margin: 0;">Dark Mode</n-h3>
          <n-text depth="3">
            Switches the application to a dark color scheme.
          </n-text>
        </div>
        <n-switch :value="isDarkMode" @update:value="toggleDarkMode" />
      </div>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { NCard, NSwitch, NH3, NText, NDivider, NSpace } from 'naive-ui';
import { useSettingsStore } from '../stores/settingsStore';

const { isAdvancedMode, toggleAdvancedMode, isDarkMode, toggleDarkMode } = useSettingsStore();
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
