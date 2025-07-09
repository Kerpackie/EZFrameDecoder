<template>
  <!-- File uploader -->
  <n-upload
      v-model:file-list="dummy"
      :default-upload="false"
      accept=".txt,.log,.csv"
      :max="1"
      :before-upload="validateFile"
      :on-change="onFile"
  >
    <n-upload-dragger>
      Drop or click to load a <b>.txt</b>, <b>.log</b> or <b>.csv</b> file
    </n-upload-dragger>
  </n-upload>

  <!-- Scrollable frame list -->
  <div class="list-wrapper scroll-hide">
    <n-list class="frame-list">
      <n-list-item
          v-for="(f, i) in store.frames"
          :key="i"
          :class="{ selected: i === store.selected }"
          @click="select(i)"
      >
        {{ f }}
      </n-list-item>
    </n-list>
  </div>
</template>

<script setup lang="ts">
import {
  NUpload, NUploadDragger, NList, NListItem, useMessage
} from "naive-ui";
import type { UploadFileInfo } from "naive-ui";
import { ref, watch, onMounted } from "vue";
import { invoke } from '@tauri-apps/api/core';
import { useFrameStore } from "../stores/frameStore";
import { useSharedDecode } from "../composables/useSharedDecode";

const message = useMessage();
const store   = useFrameStore();
const { run } = useSharedDecode();

/* Component State */
const dummy = ref<UploadFileInfo[]>([]);
const families = ref<any[]>([]);

/* Fetch families on mount to know valid prefixes and terminators */
onMounted(async () => {
  try {
    families.value = await invoke('get_families');
  } catch (e) {
    message.error(`Failed to load families: ${e}`);
  }
});


/* Helpers */
function extOK(name = "") {
  return [".txt", ".log", ".csv"].some((e) =>
      name.toLowerCase().endsWith(e)
  );
}

/* Validate & load file */
function validateFile(file: UploadFileInfo) {
  if (!extOK(file.name)) {
    message.error("Only .txt, .log or .csv files are allowed");
    return false;
  }
  return true;
}

async function onFile(info: { file: UploadFileInfo }) {
  const txt = await info.file.file?.text();
  if (!txt) return;

  if (families.value.length === 0) {
    message.error("No valid frame families defined in the spec.");
    return;
  }

  const allFrames: string[] = [];
  const lines = txt.split(/[\r\n]+/);

  for (const line of lines) {
    if (!line.trim()) continue;

    const parts = line.split(/[\s,\|	]+/);

    for (const part of parts) {
      const trimmedPart = part.trim();

      // Find which family this part belongs to, if any
      const matchingFamily = families.value.find(f => trimmedPart.startsWith(f.start));

      if (matchingFamily) {
        // Use the terminator from the specific family
        const startIndex = 0; // We know it starts with the prefix
        const lastEndIndex = trimmedPart.lastIndexOf(matchingFamily.terminator);

        if (lastEndIndex > startIndex) {
          const potentialFrame = trimmedPart.substring(startIndex, lastEndIndex + matchingFamily.terminator.length);
          allFrames.push(potentialFrame);
          // Assuming one frame per line, we can break after finding the first one
          break;
        }
      }
    }
  }

  if (allFrames.length === 0) {
    message.warning("No valid frames found in the file matching the defined families.");
  }
  store.setFrames(allFrames);
}

/* Click handler */
function select(i: number) {
  if (i >= 0 && i < store.frames.length) {
    store.select(i);
    run(store.frames[i]);
  }
}

/* Keep decoding when selection changes elsewhere */
watch(
    () => store.selected,
    (idx) => {
      if (idx >= 0 && idx < store.frames.length) {
        run(store.frames[idx]);
      }
    }
);
</script>

<style scoped>
.list-wrapper {
  flex: 1 1 auto;
  overflow-y: auto;
  scrollbar-width: none;
}
.list-wrapper::-webkit-scrollbar {
  display: none;
}

</style>
