<template>
  <n-layout has-sider class="spec-editor-layout">
    <!-- Sidebar for Family List -->
    <n-layout-sider
        bordered
        collapse-mode="width"
        :collapsed-width="0"
        :width="240"
        show-trigger
        v-model:collapsed="siderCollapsed"
    >
      <div class="sider-content" v-if="!siderCollapsed">
        <n-h4 style="padding: 0 16px; flex-shrink: 0;">Command Families</n-h4>
        <n-menu
            :options="familyMenuOptions"
            :value="selectedFamily?.start"
            @update:value="handleFamilySelect"
            style="flex-grow: 1; overflow-y: auto; overflow-x: hidden;"
        />
      </div>
    </n-layout-sider>

    <!-- Main Content Area -->
    <n-layout-content class="main-content">
      <div v-if="isLoading" class="centered-message">
        <n-spin size="large" />
      </div>

      <div v-else-if="selectedFamily" class="content-wrapper">
        <!-- Family Header -->
        <div class="family-header">
          <div>
            <n-h2 style="margin: 0;">{{ selectedFamily.name }}</n-h2>
            <n-text depth="3">{{ selectedFamily.description }}</n-text>
          </div>
        </div>

        <!-- READ-ONLY Command List -->
        <CommandList
            :commands="commands"
            :family="selectedFamily"
            read-only
        />
      </div>

      <div v-else class="centered-message">
        <n-empty description="Select a family to view its commands." />
      </div>
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
  NLayout, NLayoutSider, NLayoutContent, NMenu,
  NSpin, NEmpty, NH2, NH4, NText, useMessage,
} from 'naive-ui';
import CommandList from '../components/CommandList.vue';

// --- State ---
const isLoading = ref(true);
const families = ref<any[]>([]);
const selectedFamily = ref<any | null>(null);
const commands = ref<any[]>([]);
const siderCollapsed = ref(false);
const msg = useMessage();

// --- Computed Properties ---
const familyMenuOptions = computed(() =>
    families.value.map(f => ({
      label: f.name,
      key: f.start,
    }))
);

// --- Data Fetching ---
async function fetchFamilies() {
  try {
    isLoading.value = true;
    families.value = await invoke('get_families');
    if (families.value.length > 0) {
      await handleFamilySelect(families.value[0].start);
    } else {
      selectedFamily.value = null;
      commands.value = [];
    }
  } catch (err) {
    msg.error(`Failed to load families: ${err}`);
  } finally {
    isLoading.value = false;
  }
}

async function fetchCommands() {
  if (!selectedFamily.value) return;
  try {
    commands.value = await invoke('get_commands', { familyStart: selectedFamily.value.start });
  } catch (err) {
    msg.error(`Failed to load commands: ${err}`);
    commands.value = [];
  }
}

// --- Event Handlers ---
async function handleFamilySelect(key: string) {
  selectedFamily.value = families.value.find(f => f.start === key) || null;
  await fetchCommands();
}

// --- Lifecycle ---
onMounted(() => {
  fetchFamilies();
});
</script>

<style scoped>
.spec-editor-layout {
  height: 100%;
  overflow: hidden;
}
.sider-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}
.main-content {
  padding: 24px;
  overflow-y: auto;
}
.content-wrapper {
  padding: 24px;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.12);
}
.family-header {
  margin-bottom: 24px;
}
.centered-message {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>
