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
        <n-h4 class="sider-header">Command Families</n-h4>
        <n-menu
            :options="familyMenuOptions"
            :value="selectedFamily?.start"
            @update:value="handleFamilySelect"
            style="flex-grow: 1; overflow-y: auto; overflow-x: hidden;"
        />
        <div class="sider-footer">
          <n-button @click="handleNewFamily" block type="success" secondary>
            <template #icon>
              <n-icon :component="AddIcon" />
            </template>
            <span>New Family</span>
          </n-button>
        </div>
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
          <n-space>
            <n-button @click="handleEditFamily(selectedFamily)" type="primary">Edit Family</n-button>
            <n-button @click="handleDeleteFamily(selectedFamily)" type="error">Delete Family</n-button>
          </n-space>
        </div>

        <n-tabs type="line" animated v-model:value="activeTab" :pane-style="{ paddingTop: '12px' }">
          <n-tab-pane name="commands" tab="Commands">
            <CommandList
                :commands="commands"
                :family="selectedFamily"
                @refresh="fetchCommands"
                @delete="handleDeleteCommand"
            />
          </n-tab-pane>
          <n-tab-pane name="add-command" tab="Add New Command">
            <AddCommandBuilder
                mode="create"
                :family="selectedFamily"
                :existing-letters="existingLetters"
                @saved="handleCommandSaved"
            />
          </n-tab-pane>
        </n-tabs>
      </div>

      <div v-else class="centered-message">
        <n-empty description="Select a family to begin, or create a new one." />
      </div>
    </n-layout-content>
  </n-layout>

  <!-- Family Editor Modal -->
  <FamilyEditor
      v-model:show="showFamilyEditor"
      :family-data="editingFamily"
      @saved="fetchFamilies"
  />
</template>

<script setup lang="ts">
import { ref, onMounted, computed} from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
  NLayout, NLayoutSider, NLayoutContent, NMenu, NButton, NSpace,
  NSpin, NEmpty, NH2, NH4, NText, NTabs, NTabPane, useMessage, useDialog, NIcon
} from 'naive-ui';
import { Add as AddIcon } from '@vicons/ionicons5';
import CommandList from '../components/CommandList.vue';
import AddCommandBuilder from '../components/AddCommandBuilder.vue';
import FamilyEditor from '../components/FamilyEditor.vue';

const isLoading = ref(true);
const families = ref<any[]>([]);
const selectedFamily = ref<any | null>(null);
const commands = ref<any[]>([]);
const activeTab = ref('commands');
const siderCollapsed = ref(false);
const msg = useMessage();
const dialog = useDialog();
const showFamilyEditor = ref(false);
const editingFamily = ref<any | null>(null);

const familyMenuOptions = computed(() =>
    families.value.map(f => ({
      label: f.name,
      key: f.start,
    }))
);

const existingLetters = computed(() => commands.value.map(c => c.letter));

async function fetchFamilies() {
  try {
    isLoading.value = true;
    families.value = await invoke('get_families');
    if (families.value.length > 0) {
      const currentKey = selectedFamily.value?.start;
      const familyToSelect = families.value.find(f => f.start === currentKey) || families.value[0];
      await handleFamilySelect(familyToSelect.start);
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

async function handleFamilySelect(key: string) {
  selectedFamily.value = families.value.find(f => f.start === key) || null;
  activeTab.value = 'commands';
  await fetchCommands();
}

function handleNewFamily() {
  editingFamily.value = null;
  showFamilyEditor.value = true;
}

function handleEditFamily(family: any) {
  editingFamily.value = family;
  showFamilyEditor.value = true;
}

function handleDeleteFamily(family: any) {
  dialog.warning({
    title: 'Delete Family',
    content: `Are you sure you want to delete the "${family.name}" family? This will also delete all of its commands.`,
    positiveText: 'Delete',
    negativeText: 'Cancel',
    onPositiveClick: async () => {
      try {
        await invoke('delete_family', { start: family.start });
        msg.success('Family deleted.');
        selectedFamily.value = null;
        await fetchFamilies();
      } catch (err) {
        msg.error(`Failed to delete family: ${err}`);
      }
    },
  });
}

async function handleDeleteCommand(command: any) {
  if (!selectedFamily.value) return;
  try {
    await invoke('delete_command', {
      familyStart: selectedFamily.value.start,
      letter: command.letter
    });
    msg.success(`Command '${command.letter}' deleted.`);
    await fetchCommands();
  } catch (err) {
    msg.error(`Failed to delete command: ${err}`);
  }
}

async function handleCommandSaved() {
  await fetchCommands();
  activeTab.value = 'commands';
}

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
.sider-header {
  padding: 1rem 1rem 0.5rem 1rem;
  flex-shrink: 0;
}
.sider-footer {
  padding: 16px;
  flex-shrink: 0;
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
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  flex-wrap: wrap;
  gap: 1rem;
}
.centered-message {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>
