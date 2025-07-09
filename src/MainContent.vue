<template>
  <router-view/>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useMessage } from 'naive-ui';
import { useSettingsStore } from './stores/settingsStore';
import { invoke } from '@tauri-apps/api/core';

const message = useMessage();
const { setSpecFilePath } = useSettingsStore();

// --- Initial Spec File Load ---
onMounted(async () => {
  // On app startup, we always want to ensure the backend's in-memory spec
  // is loaded from the 'spec_override.json' file.
  // The 'specFilePath' in localStorage is just for UI display of the *source* file.
  try {
    // Attempt to reload the spec from the default path (spec_override.json)
    // This will also validate if spec_override.json is a valid spec.
    // If it's invalid, the backend will fall back to the embedded default,
    // but we won't get a direct error here unless the file system operation fails.
    // The backend's reload_spec_from_default_path handles the parsing error internally.
    await invoke('get_families'); // A simple command to trigger spec loading/validation in backend

    // If we successfully got families, it means the spec was loaded (either custom or default).
    // If specFilePath was set from a previous session, it means the backend should have loaded that content.
    // If specFilePath is null, it means the default spec_override.json is active.
    console.log('Initial spec loaded from spec_override.json (or embedded default if invalid).');

  } catch (error: any) {
    console.error('Failed to load initial spec on app startup:', error);
    message.error(`Failed to load initial spec on startup. Application might not function correctly. Error: ${error}`);
    // If there's a critical error loading the default spec, clear any saved path
    setSpecFilePath(null);
  }
});
</script>
