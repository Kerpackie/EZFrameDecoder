<template>
  <n-card class="about-card" embedded>
    <template #header>
      <n-h3 class="m-0">About EZ Frame Decoder</n-h3>
    </template>

    <n-space vertical size="small">
      <p>
        Lorem Ipsum....
      </p>

      <n-h4>How to use</n-h4>
      <ol>
        <li>Paste a single frame, or drag-and-drop a <code>.txt</code>, <code>.log</code>, or <code>.csv</code> file.</li>
        <li>Select a frame in the sidebar or click <kbd>Decode</kbd>.</li>
        <li>Review the breakdown — hover any field for its description.</li>
      </ol>

      <n-h4>Keyboard shortcuts</n-h4>
      <ul>
        <li><kbd>Enter</kbd> — Decode current frame</li>
      </ul>

      <n-h4>Version & Credits</n-h4>
      <p>
        EZ Frame Decoder v1.0<br>
        Built with Vue 3, Naive-UI, and Tauri.
      </p>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { NCard, NH3, NH4, NSpace } from "naive-ui";
</script>

<style scoped>
.about-card {
  max-width: 780px;
  margin: 0 auto;     /* center in content area */
  line-height: 1.6;
}
</style>
