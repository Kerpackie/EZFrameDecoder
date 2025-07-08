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
const { run } = useSharedDecode();       // shared decoder

/* Component State */
const dummy = ref<UploadFileInfo[]>([]);
const families = ref<any[]>([]);

/* Fetch families on mount to know valid prefixes */
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
function clean(line = "") {
  return line.split(/\s+/)[0];           // cut after first space
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

  const validStarts = families.value.map(f => f.start).filter(Boolean);
  if (validStarts.length === 0) {
    message.error("No valid frame families defined in the spec.");
    return;
  }

  const list =
      txt.split(/[\r\n]+/)
          .map((l) => clean(l.trim()))
          .filter((f) => validStarts.some(start => f.startsWith(start)) && f.length) || [];

  if (list.length === 0) {
    message.warning("No valid frames found in the file matching the defined families.");
  }
  store.setFrames(list);
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
  text-align: center;
}
.list-wrapper::-webkit-scrollbar {
  display: none;
}

</style>
