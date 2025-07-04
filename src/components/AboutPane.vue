<template>
  <n-card class="about-card" embedded>
    <template #header>
      <n-h3 class="m-0">About EZ Frame Decoder</n-h3>
    </template>

    <n-space vertical size="small">
      <p>
        EZ Frame Decoder is a tool for parsing 23-byte ASCII frames used by Abrel
        production testers. Paste a single frame or load a log file to see
        a human-readable breakdown.
      </p>

      <n-h4>How to use</n-h4>
      <ol>
        <li>Drag-and-drop a <code>.txt</code>, <code>.log</code>, or <code>.csv</code> file
          onto the sidebar, or paste a single frame into the input box.</li>
        <li>Click a frame in the list or press <kbd>Decode</kbd>.</li>
        <li>Scroll the decoded tables; tool-tips show field descriptions.</li>
      </ol>

      <n-h4>Keyboard shortcuts</n-h4>
      <ul>
        <li><kbd>Enter</kbd> in the input field &nbsp;—&nbsp; decode current frame</li>
      </ul>

      <n-h4>Version & Credits</n-h4>
      <p>
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
  max-width: 800px;
  margin: 0 auto;
  line-height: 1.55;
}
</style>
