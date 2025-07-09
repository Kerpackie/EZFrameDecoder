<template>
  <div class="page-padding">
    <n-space vertical>
      <div class="settings-wrapper">
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

            <!-- Dark Mode Setting -->
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

            <!-- Spec File Location Setting -->
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
              <!-- Conditionally render the spec file options based on advanced mode -->
              <n-space v-if="isAdvancedMode">
                <!-- CORRECTED: Use a simple button to trigger the Tauri dialog -->
                <n-button @click="chooseAndLoadSpecFile" type="primary">
                  Choose Spec File
                </n-button>

                <n-button @click="exportSpecFile" type="default">
                  Export Spec
                </n-button>

                <n-button @click="resetSpecFile">
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
} from 'naive-ui';
import { useSettingsStore } from '../stores/settingsStore';
import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

const message = useMessage();
const { isAdvancedMode, toggleAdvancedMode, isDarkMode, toggleDarkMode, specFilePath, setSpecFilePath } = useSettingsStore();
const isSpecFileValid = ref(true);

// This function uses the Tauri dialog plugin for a reliable way to get a file path.
async function chooseAndLoadSpecFile() {
  try {
    // 1. Open the native file dialog from Tauri.
    const selectedPath = await open({
      multiple: false,
      filters: [{
        name: 'EZFrameDecoder Spec',
        extensions: ['ezspec']
      }]
    });

    // 2. Check if the user selected a file (the result is a string path).
    if (typeof selectedPath === 'string' && selectedPath) {
      // 3. Call the backend with the guaranteed file path.
      await invoke('load_spec', { pathStr: selectedPath });

      // 4. Update the UI to show the new path.
      setSpecFilePath(selectedPath);
      isSpecFileValid.value = true;
      message.success(`Spec file loaded successfully!`);
    }
    // If the user cancels, `selectedPath` will be null, and we do nothing.
  } catch (error: any) {
    console.error('Error loading spec file:', error);
    isSpecFileValid.value = false;
    message.error(`Failed to load spec file: ${error}`);
    // If loading a custom file fails, it's safest to reset to the default.
    await resetSpecFile();
  }
}

// This function correctly resets the state in the UI.
async function resetSpecFile() {
  try {
    await invoke('reset_spec_to_default');
    // Set path to null to indicate the default spec is loaded.
    setSpecFilePath(null);
    isSpecFileValid.value = true;
    message.success('Spec has been reset to the default!');
  } catch (error: any) {
    console.error('Error resetting spec file:', error);
    message.error(`Failed to reset spec file: ${error}`);
  }
}

// This function uses the Tauri dialog plugin for saving files.
async function exportSpecFile() {
  try {
    const content: string = await invoke('get_spec_content');
    const filePath = await save({
      title: 'Export Spec File',
      defaultPath: 'spec_export.ezspec',
      filters: [{
        name: 'EZFrameDecoder Spec',
        extensions: ['ezspec']
      }]
    });

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
