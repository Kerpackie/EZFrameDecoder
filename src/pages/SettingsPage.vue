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
                  Current: {{ specFilePath || 'Default (spec_override.json)' }}
                </n-text>
                <n-text depth="3" v-if="isAdvancedMode" style="font-size: 13px;">
                  <br>
                  <span>
                  <strong>Default Override location:</strong>
                  </span>
                  <ul style="margin: 4px 0 0 16px; padding: 0; list-style: disc;">
                  <li><strong>Windows</strong>: <code>%APPDATA%/EZFrameDecoder/spec_override.json</code></li>
                  <li><strong>macOS</strong>: <code>~/Library/Application Support/EZFrameDecoder/spec_override.json</code></li>
                  <li><strong>Linux</strong>: <code>~/.config/EZFrameDecoder/spec_override.json</code></li>
                  </ul>
                </n-text>
                <n-text depth="3" v-if="specFilePath && !isSpecFileValid" type="error">
                  (Last selected file was invalid. Using default.)
                </n-text>
              </div>
              <!-- Conditionally render the spec file options based on advanced mode -->
              <n-space v-if="isAdvancedMode">
                <!-- File uploader for spec file, styled as a button -->
                <n-upload
                    v-model:file-list="dummySpecFile"
                    :default-upload="false"
                    accept=".json"
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
                    accept=".json"
                    :max="1"
                    :before-upload="validateSpecFile"
                    :on-change="onSpecFileChange"
                    list-type="text"
                    style="display: none;"
                  />
                </n-upload>

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
              <n-li><strong>Customizable Specs:</strong> Define your own command structures with a powerful, multi-family JSON spec format.</n-li>
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
  NUpload, NUploadDragger
} from 'naive-ui';
import type { UploadFileInfo } from 'naive-ui'; // Import UploadFileInfo
import { useSettingsStore } from '../stores/settingsStore';
import { invoke } from '@tauri-apps/api/core';

const message = useMessage();
const { isAdvancedMode, toggleAdvancedMode, isDarkMode, toggleDarkMode, specFilePath, setSpecFilePath } = useSettingsStore();

// This ref will track if the currently displayed specFilePath is valid and loaded by the backend.
// It's primarily for UI feedback, the backend handles the actual fallback.
const isSpecFileValid = ref(true);

// Dummy file list for n-upload
const dummySpecFile = ref<UploadFileInfo[]>([]);

// Helper to validate file extension
function validateSpecFile(file: UploadFileInfo): boolean {
  if (!file.name.toLowerCase().endsWith('.json')) {
    message.error('Only .json files are allowed for spec files.');
    return false;
  }
  return true;
}

// Handler for when a spec file is selected/dropped
async function onSpecFileChange(info: { file: UploadFileInfo }) {
  const file = info.file.file;
  if (!file) return;

  try {
    const content = await file.text(); // Read file content as string on frontend

    // Invoke backend command with the file content
    await invoke('set_spec_from_content', { content: content });

    // Update frontend store with the original file path for display
    setSpecFilePath(file.path || file.name); // Use file.path if available (Tauri), fallback to file.name
    isSpecFileValid.value = true;
    message.success('Spec file loaded and updated successfully!');
  } catch (error: any) {
    console.error('Error loading spec file:', error);
    isSpecFileValid.value = false; // Mark as invalid for UI feedback
    message.error(`Failed to load spec file: ${error}`);
    // If loading fails, revert specFilePath in store to null to indicate default/invalid
    setSpecFilePath(null);
  } finally {
    // Clear the file list in the uploader after processing
    dummySpecFile.value = [];
  }
}

// Function to handle resetting to the default spec file
async function resetSpecFile() {
  try {
    // Invoke the new backend command to reset to default
    await invoke('reset_spec_to_default');
    setSpecFilePath(null); // Clear the stored path in frontend
    isSpecFileValid.value = true;
    message.success('Spec file reset to default!');
  } catch (error: any) {
    console.error('Error resetting spec file:', error);
    message.error(`Failed to reset spec file: ${error}`);
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
