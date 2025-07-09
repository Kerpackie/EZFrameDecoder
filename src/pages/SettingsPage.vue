<template>
  <div class="page-padding">
    <n-space vertical>
      <div class="settings-wrapper">
        <n-card title="Application Settings">
          <n-space vertical size="large">
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

            <div class="setting-row">
              <div class="setting-info">
                <n-h3 style="margin: 0;">Dark Mode</n-h3>
                <n-text depth="3">
                  Switches the application to a dark color scheme.
                </n-text>
              </div>
              <n-switch :value="isDarkMode" @update:value="toggleDarkMode" />
            </div>

            <n-divider />

            <div class="setting-row">
              <div class="setting-info">
                <n-h3 style="margin: 0;">Spec File Location</n-h3>
                <n-text depth="3">
                  Current: {{ specFilePath || 'Default (spec_override.ezspec)' }}
                </n-text>
                <div v-if="isAdvancedMode" style="font-size: 13px;">
                  <strong>Default Override location:</strong>
                  <ul style="margin: 4px 0 0 16px; padding: 0; list-style: disc;">
                    <li><strong>Windows</strong>: <code>%APPDATA%/EZFrameDecoder/spec_override.ezspec</code></li>
                    <li><strong>macOS</strong>: <code>~/Library/Application Support/EZFrameDecoder/spec_override.ezspec</code></li>
                    <li><strong>Linux</strong>: <code>~/.config/EZFrameDecoder/spec_override.ezspec</code></li>
                  </ul>
                </div>

                <n-text depth="3" v-if="specFilePath && !isSpecFileValid" type="error">
                  (Last selected file was invalid. Using default.)
                </n-text>

              </div>
              <n-space v-if="isAdvancedMode">
                <n-upload
                    v-model:file-list="dummySpecFile"
                    :default-upload="false"
                    accept=".ezspec"
                    :max="1"
                    :before-upload="validateSpecFile"
                    :on-change="onSpecFileChange"
                    list-type="text"
                    class="spec-upload-button-wrapper"
                >
                  <n-button type="primary" class="spec-upload-btn" @click="$refs.specUploader?.open()">
                    Choose Spec File
                  </n-button>
                  <n-upload
                      ref="specUploader"
                      v-model:file-list="dummySpecFile"
                      :default-upload="false"
                      accept=".ezspec"
                      :max="1"
                      :before-upload="validateSpecFile"
                      :on-change="onSpecFileChange"
                      list-type="text"
                      style="display: none;"
                  />
                </n-upload>

                <n-button @click="exportSpecFile" type="default">
                  Export Spec
                </n-button>

                <n-button @click="resetSpecFile" :disabled="!specFilePath">
                  Reset to Default
                </n-button>
              </n-space>
            </div>

          </n-space>
        </n-card>
      </div>
      <div class="settings-wrapper">
        <n-card title="About EZ Frame Decoder">
          <n-space vertical>
            <n-p>
              <strong>EZ Frame Decoder</strong> is a high-performance utility designed to effortlessly parse and interpret data frames.
              Built for engineers and developers who need clarity and speed when debugging, this tool transforms raw data logs into human-readable information in an instant.
            </n-p>
            <n-h4 style="margin-bottom: 0;">Key Features:</n-h4>
            <n-ul>
              <n-li><strong>Dynamic Decoding:</strong> Paste raw frames for instant, real-time decoding based on the active specification.</n-li>
              <n-li><strong>Customizable Specs:</strong> Define your own command structures with a powerful, multi-family EZSpec format.</n-li>
              <n-li><strong>Advanced Editor:</strong> A dedicated, feature-rich editor for creating and managing command families and their unique protocols.</n-li>
              <n-li><strong>Flexible Overrides:</strong> Easily load and test different spec files for different hardware or software versions.</n-li>
              <n-li><strong>Modern Interface:</strong> A clean, themeable UI with both light and dark modes for your comfort.</n-li>
            </n-ul>
            <n-divider />
            <div class="footer-container">
              <n-text depth="3">
                Version 1.0.0 | Crafted with ❤️ using Rust, Tauri, and Vue.js.
                <span class="separator"> | </span>
                <a href="https://github.com/Kerpackie/EZFrameDecoder" target="_blank" class="footer-link">GitHub</a>
                <span class="separator"> | </span>
                <a href="https://github.com/Kerpackie/EZFrameDecoder/blob/main/LICENSE" target="_blank" class="footer-link">Released Under Apache 2.0 Licence</a>
              </n-text>
            </div>
          </n-space>
        </n-card>
      </div>
    </n-space>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  NCard, NSwitch, NH3, NText, NDivider, NSpace, NButton, useMessage,
  NUpload
} from 'naive-ui';
import type { UploadFileInfo } from 'naive-ui';
import { useSettingsStore } from '../stores/settingsStore';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

const message = useMessage();
const { isAdvancedMode, toggleAdvancedMode, isDarkMode, toggleDarkMode, specFilePath, setSpecFilePath } = useSettingsStore();
const isSpecFileValid = ref(true);
const dummySpecFile = ref<UploadFileInfo[]>([]);

function validateSpecFile(data: { file: UploadFileInfo, fileList: UploadFileInfo[] }): boolean {
  if (!data.file.name.toLowerCase().endsWith('.ezspec')) {
    message.error('Only .ezspec files are allowed for spec files.');
    return false;
  }
  return true;
}

async function onSpecFileChange(info: { file: UploadFileInfo }) {
  const file = info.file.file;
  if (!file) return;

  try {
    const content = await file.text();
    await invoke('set_spec_from_content', { content: content });
    setSpecFilePath(file.path || file.name);
    isSpecFileValid.value = true;
    message.success('Spec file loaded and updated successfully!');
  } catch (error: any) {
    console.error('Error loading spec file:', error);
    isSpecFileValid.value = false;
    message.error(`Failed to load spec file: ${error}`);
    setSpecFilePath(null);
  } finally {
    dummySpecFile.value = [];
  }
}

async function resetSpecFile() {
  try {
    await invoke('reset_spec_to_default');
    setSpecFilePath(null);
    isSpecFileValid.value = true;
    message.success('Spec file reset to default!');
  } catch (error: any) {
    console.error('Error resetting spec file:', error);
    message.error(`Failed to reset spec file: ${error}`);
  }
}

async function exportSpecFile() {
  try {
    // 1. Get the current spec content from the backend
    const content: string = await invoke('get_spec_content');

    // 2. Open a native "save file" dialog
    const filePath = await save({
      title: 'Export Spec File',
      defaultPath: 'spec_export.ezspec',
      filters: [{
        name: 'EZFrameDecoder Spec',
        extensions: ['ezspec']
      }]
    });

    // 3. If the user selected a path (didn't cancel), write the file
    if (filePath) {
      await writeTextFile(filePath, content);
      message.success(`Spec exported successfully to ${filePath}`);
    }
  } catch (error: any) {
    console.error('Error exporting spec file:', error);
    message.error(`Failed to export spec file: ${error}`);
  }
}

</script>

<style scoped>
.page-padding {
  padding: 1rem;
}

.settings-wrapper {
  display: flex;
  flex-direction: column;
  gap: 1.5rem; /* Replicates n-space large */
}

.setting-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.setting-info {
  margin-right: 2rem;
}

/* Styles to make n-upload-dragger look like a button */
.spec-upload-button-wrapper :deep(.n-upload-dragger) {
  display: flex;
  align-items: center;
  justify-content: center;
  /* Basic button styling */
  padding: 8px 15px; /* Adjust padding to match NButton */
  border-radius: 6px; /* Match NButton's border-radius */
  font-size: 14px; /* Match NButton's font size */
  font-weight: 500; /* Match NButton's font weight */
  cursor: pointer;
  transition: background-color 0.3s, border-color 0.3s, color 0.3s, box-shadow 0.3s;

  /* Mimic Naive UI primary button colors */
  background-color: var(--n-primary-color);
  color: var(--n-button-text-color); /* Usually white or a light color for primary */
  border: 1px solid var(--n-primary-color); /* Solid border matching background */
  box-shadow: var(--n-box-shadow); /* Optional: Add a subtle shadow */
}

.spec-upload-button-wrapper :deep(.n-upload-dragger:hover) {
  background-color: var(--n-primary-color-hover);
  border-color: var(--n-primary-color-hover);
}

.spec-upload-button-wrapper :deep(.n-upload-dragger:active) {
  background-color: var(--n-primary-color-pressed);
  border-color: var(--n-primary-color-pressed);
}

/* Hide the default file list if it appears */
.spec-upload-button-wrapper :deep(.n-upload-file-list) {
  display: none;
}

.about-card{
  line-height: 1.6;
}

.footer-container {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
  text-align: center;
}
.footer-link {
  color: var(--n-text-color-3);
  text-decoration: none;
  transition: color 0.3s;
}
.footer-link:hover {
  color: var(--n-primary-color-hover);
  text-decoration: underline;
}
</style>